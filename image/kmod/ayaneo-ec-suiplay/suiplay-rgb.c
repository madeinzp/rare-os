// SPDX-License-Identifier: GPL-2.0+
/*
 * Rare OS: the SuiPlay0X1's stick rings, through the EC's LED registers.
 *
 * The protocol is the "legacy" AYANEO one the AYANEO 2S (and so the SuiPlay0X1) uses, as in
 * ShadowBlip's ayaneo-platform (GPL-2.0+; Derek J. Clark, JELOS, Sebastian Kranz, Trevor
 * Heslop; derived from Maya Matuszczyk's ayaled). Stock GameOS loads that driver on the
 * SuiPlay0X1, pinned at 9b2a602 "Add support for SuiPlay0X1 RGB":
 *
 *   one subpixel: 0x6d = group (1 left ring, 2 right ring, 3 both), 0xb1 = position,
 *                 0xb2 = value, 0xbf = 0x10 (write), about 2 ms, 0xbf = 0xfe (hold)
 *   0xbf = 0x00   gives the LEDs back to the EC
 *   position 2    0xb1 on, 0x31 off, 0xc0 reset
 *   positions 3, 6, 9, 12: the four zones of a ring, red, green, blue at +0, +1, +2
 *   group 3, position 0, value 0 ends a batch. Values go up to 192 (0..255 scaled, as there).
 *
 * Differences from ayaneo-platform:
 *   - nothing is written until userspace asks. From boot the rings keep the EC's own behaviour
 *     (rgb_mode "oem"), so a device nobody configures looks as it always did;
 *   - each ring and each of its four zones takes its own color (rgb_zones), only subpixels that
 *     changed are written, and the writes run in a work item that sleeps between them (no busy
 *     wait), so an effect at 10 frames a second costs next to no CPU;
 *   - in suspend the rings go dark unless the EC has them; on resume the last frame comes back;
 *   - at shutdown and unload the EC gets them back.
 *
 * sysfs, on /sys/class/leds/ayaneo:rgb:joystick_rings (the name InputPlumber and hhd look for):
 *   brightness, multi_intensity, multi_index
 *              the standard multicolor LED: both rings one color, brightness 0 is "off"
 *   rgb_mode   "oem" (the EC's own lights), "on" (the frame in rgb_zones), "off" (dark, held)
 *   rgb_zones  eight RRGGBB words: the left ring's zones 1-4, then the right ring's. One word
 *              sets all eight. Writing it sets rgb_mode to "on".
 */

#include <linux/acpi.h>
#include <linux/ctype.h>
#include <linux/delay.h>
#include <linux/device.h>
#include <linux/hex.h>
#include <linux/kernel.h>
#include <linux/led-class-multicolor.h>
#include <linux/leds.h>
#include <linux/module.h>
#include <linux/mutex.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/sysfs.h>
#include <linux/workqueue.h>

#include "suiplay-rgb.h"

static bool rgb = true;
module_param(rgb, bool, 0444);
MODULE_PARM_DESC(rgb, "Stick ring lights on the SuiPlay0X1 (default on; the EC keeps them until userspace writes)");

#define LED_GROUP_REG		0x6d
#define LED_POS_REG		0xb1
#define LED_VAL_REG		0xb2
#define LED_MODE_REG		0xbf
#define LED_MODE_RELEASE	0x00
#define LED_MODE_WRITE		0x10
#define LED_MODE_HOLD		0xfe

#define GROUP_LEFT		0x01
#define GROUP_RIGHT		0x02
#define GROUP_BOTH		0x03

#define POS_ENABLE		0x02
#define ENABLE_ON		0xb1
#define ENABLE_OFF		0x31
#define ENABLE_RESET		0xc0

#define RINGS			2
#define ZONES			4
#define MAX_VALUE		192
#define GLOBAL_LOCK_MS		500

static const u8 zone_pos[ZONES] = { 3, 6, 9, 12 };
static const u8 ring_group[RINGS] = { GROUP_LEFT, GROUP_RIGHT };

enum rgb_mode { MODE_OEM, MODE_ON, MODE_OFF };
static const char * const mode_names[] = {
	[MODE_OEM] = "oem",
	[MODE_ON] = "on",
	[MODE_OFF] = "off",
};

/* What the EC was last left showing. UNKNOWN: after a resume, before the next write. */
enum hw_state { HW_UNKNOWN, HW_OEM, HW_ON, HW_OFF };

struct suiplay_rgb {
	struct device *dev;
	struct led_classdev_mc mc;
	struct mc_subled subled[3];
	struct work_struct work;

	/* What userspace asked for. */
	struct mutex lock;
	enum rgb_mode want_mode;
	u8 want[RINGS][ZONES][3];
	bool stopped;

	/* The worker's view of the EC (the work, suspend, resume and shutdown take it). */
	struct mutex hw_lock;
	enum hw_state hw;
	bool hw_valid;
	bool settle;
	u8 shown[RINGS][ZONES][3];
};

static struct suiplay_rgb *the_rgb;

/* ------------------------------------------------------------------ the EC */

static int rgb_reg(u8 reg, u8 val)
{
	u32 handle;
	int ret;

	if (ACPI_FAILURE(acpi_acquire_global_lock(GLOBAL_LOCK_MS, &handle)))
		return -EBUSY;
	ret = ec_write(reg, val);
	acpi_release_global_lock(handle);
	return ret;
}

static int rgb_set(u8 group, u8 pos, u8 val)
{
	u32 handle;
	int ret;

	if (ACPI_FAILURE(acpi_acquire_global_lock(GLOBAL_LOCK_MS, &handle)))
		return -EBUSY;
	ret = ec_write(LED_GROUP_REG, group);
	if (!ret)
		ret = ec_write(LED_POS_REG, pos);
	if (!ret)
		ret = ec_write(LED_VAL_REG, val);
	if (!ret)
		ret = ec_write(LED_MODE_REG, LED_MODE_WRITE);
	acpi_release_global_lock(handle);
	if (ret)
		return ret;
	usleep_range(2000, 3000);
	return rgb_reg(LED_MODE_REG, LED_MODE_HOLD);
}

/* Position 2 of both rings, then the end of the batch. */
static int rgb_enable(u8 val)
{
	int ret;

	ret = rgb_set(GROUP_LEFT, POS_ENABLE, val);
	if (!ret)
		ret = rgb_set(GROUP_RIGHT, POS_ENABLE, val);
	if (!ret)
		ret = rgb_set(GROUP_BOTH, 0, 0);
	return ret;
}

/* Hold, reset, off: ours, dark. */
static int rgb_take(void)
{
	int ret;

	ret = rgb_reg(LED_MODE_REG, LED_MODE_HOLD);
	if (!ret)
		ret = rgb_enable(ENABLE_RESET);
	if (!ret)
		ret = rgb_enable(ENABLE_OFF);
	return ret;
}

/* Reset, release: the EC's own lights again. */
static int rgb_release(void)
{
	int ret;

	ret = rgb_enable(ENABLE_RESET);
	if (!ret)
		ret = rgb_reg(LED_MODE_REG, LED_MODE_RELEASE);
	return ret;
}

static u8 rgb_scale(u8 v)
{
	u32 s = (u32)v * MAX_VALUE / 255;

	/* A dim channel stays lit rather than rounding to 0. */
	return v && !s ? 1 : s;
}

/* Brings the EC to `mode` (and the frame `want` for "on"). Called with hw_lock held. */
static int rgb_apply(struct suiplay_rgb *r, enum rgb_mode mode, u8 want[RINGS][ZONES][3])
{
	int ring, zone, ch, ret = 0, n = 0;

	if (r->settle) {
		/* After a resume the EC needs a moment (ayaneo-platform waits 100 ms). */
		r->settle = false;
		msleep(100);
	}
	switch (mode) {
	case MODE_OEM:
		if (r->hw == HW_OEM)
			return 0;
		r->hw_valid = false;
		ret = rgb_release();
		if (!ret)
			r->hw = HW_OEM;
		return ret;
	case MODE_OFF:
		if (r->hw == HW_OFF)
			return 0;
		r->hw_valid = false;
		ret = (r->hw == HW_ON) ? rgb_enable(ENABLE_OFF) : rgb_take();
		if (!ret)
			r->hw = HW_OFF;
		return ret;
	case MODE_ON:
		break;
	}
	if (r->hw == HW_OEM || r->hw == HW_UNKNOWN) {
		r->hw_valid = false;
		ret = rgb_take();
		if (ret)
			return ret;
		r->hw = HW_OFF;
	}
	if (r->hw == HW_OFF) {
		r->hw_valid = false;
		ret = rgb_enable(ENABLE_ON);
		if (ret)
			return ret;
		r->hw = HW_ON;
	}
	for (ring = 0; ring < RINGS; ring++) {
		for (zone = 0; zone < ZONES; zone++) {
			for (ch = 0; ch < 3; ch++) {
				u8 v = rgb_scale(want[ring][zone][ch]);

				if (r->hw_valid && r->shown[ring][zone][ch] == v)
					continue;
				ret = rgb_set(ring_group[ring], zone_pos[zone] + ch, v);
				if (ret) {
					r->hw_valid = false;
					return ret;
				}
				r->shown[ring][zone][ch] = v;
				n++;
			}
		}
	}
	if (n)
		ret = rgb_set(GROUP_BOTH, 0, 0);
	r->hw_valid = !ret;
	return ret;
}

static void rgb_work(struct work_struct *work)
{
	struct suiplay_rgb *r = container_of(work, struct suiplay_rgb, work);
	u8 want[RINGS][ZONES][3];
	enum rgb_mode mode;
	int ret;

	mutex_lock(&r->hw_lock);
	mutex_lock(&r->lock);
	if (r->stopped) {
		mutex_unlock(&r->lock);
		mutex_unlock(&r->hw_lock);
		return;
	}
	mode = r->want_mode;
	memcpy(want, r->want, sizeof(want));
	mutex_unlock(&r->lock);
	ret = rgb_apply(r, mode, want);
	mutex_unlock(&r->hw_lock);
	if (ret)
		dev_warn_ratelimited(r->dev, "stick lights: EC write failed (%d)\n", ret);
}

static void rgb_kick(struct suiplay_rgb *r)
{
	queue_work(system_long_wq, &r->work);
}

/* ------------------------------------------------------------------ sysfs */

static struct suiplay_rgb *rgb_of(struct device *led_dev)
{
	struct led_classdev *cdev = dev_get_drvdata(led_dev);

	return container_of(lcdev_to_mccdev(cdev), struct suiplay_rgb, mc);
}

static int rgb_brightness_set(struct led_classdev *cdev, enum led_brightness b)
{
	struct led_classdev_mc *mc = lcdev_to_mccdev(cdev);
	struct suiplay_rgb *r = container_of(mc, struct suiplay_rgb, mc);
	int ring, zone, ch;

	led_mc_calc_color_components(mc, b);
	mutex_lock(&r->lock);
	if (b == LED_OFF) {
		r->want_mode = MODE_OFF;
	} else {
		for (ring = 0; ring < RINGS; ring++)
			for (zone = 0; zone < ZONES; zone++)
				for (ch = 0; ch < 3; ch++)
					r->want[ring][zone][ch] = min_t(u32, mc->subled_info[ch].brightness, 255);
		r->want_mode = MODE_ON;
	}
	mutex_unlock(&r->lock);
	rgb_kick(r);
	return 0;
}

static ssize_t rgb_mode_show(struct device *dev, struct device_attribute *attr, char *buf)
{
	struct suiplay_rgb *r = rgb_of(dev);
	enum rgb_mode mode;

	mutex_lock(&r->lock);
	mode = r->want_mode;
	mutex_unlock(&r->lock);
	return sysfs_emit(buf, "%s\n", mode_names[mode]);
}

static ssize_t rgb_mode_store(struct device *dev, struct device_attribute *attr,
			      const char *buf, size_t count)
{
	struct suiplay_rgb *r = rgb_of(dev);
	int mode = sysfs_match_string(mode_names, buf);

	if (mode < 0)
		return mode;
	mutex_lock(&r->lock);
	r->want_mode = mode;
	mutex_unlock(&r->lock);
	rgb_kick(r);
	return count;
}
static DEVICE_ATTR_RW(rgb_mode);

/* One RRGGBB word (all eight zones) or eight, separated by white space. */
static int rgb_parse(const char *buf, size_t count, u8 out[RINGS][ZONES][3])
{
	u8 cols[RINGS * ZONES][3];
	size_t i = 0;
	int n = 0, k;

	while (i < count) {
		while (i < count && isspace(buf[i]))
			i++;
		if (i >= count)
			break;
		if (n == RINGS * ZONES || count - i < 6)
			return -EINVAL;
		if (hex2bin(cols[n], buf + i, 3))
			return -EINVAL;
		i += 6;
		if (i < count && !isspace(buf[i]))
			return -EINVAL;
		n++;
	}
	if (n == 1) {
		for (k = 1; k < RINGS * ZONES; k++)
			memcpy(cols[k], cols[0], 3);
	} else if (n != RINGS * ZONES) {
		return -EINVAL;
	}
	memcpy(out, cols, sizeof(cols));
	return 0;
}

static ssize_t rgb_zones_show(struct device *dev, struct device_attribute *attr, char *buf)
{
	struct suiplay_rgb *r = rgb_of(dev);
	ssize_t len = 0;
	int ring, zone;

	mutex_lock(&r->lock);
	for (ring = 0; ring < RINGS; ring++)
		for (zone = 0; zone < ZONES; zone++)
			len += sysfs_emit_at(buf, len, "%02x%02x%02x%c", r->want[ring][zone][0],
					     r->want[ring][zone][1], r->want[ring][zone][2],
					     ring == RINGS - 1 && zone == ZONES - 1 ? '\n' : ' ');
	mutex_unlock(&r->lock);
	return len;
}

static ssize_t rgb_zones_store(struct device *dev, struct device_attribute *attr,
			       const char *buf, size_t count)
{
	struct suiplay_rgb *r = rgb_of(dev);
	u8 frame[RINGS][ZONES][3];
	int ret;

	ret = rgb_parse(buf, count, frame);
	if (ret)
		return ret;
	mutex_lock(&r->lock);
	memcpy(r->want, frame, sizeof(frame));
	r->want_mode = MODE_ON;
	mutex_unlock(&r->lock);
	rgb_kick(r);
	return count;
}
static DEVICE_ATTR_RW(rgb_zones);

static struct attribute *rgb_attrs[] = {
	&dev_attr_rgb_mode.attr,
	&dev_attr_rgb_zones.attr,
	NULL
};

static const struct attribute_group rgb_group = {
	.attrs = rgb_attrs,
};

static const struct attribute_group *rgb_groups[] = {
	&rgb_group,
	NULL
};

/* ------------------------------------------------------------------ life cycle */

/* Stops the work and gives the LEDs back to the EC (unload, shutdown). */
static void rgb_stop(struct suiplay_rgb *r)
{
	mutex_lock(&r->lock);
	r->stopped = true;
	mutex_unlock(&r->lock);
	cancel_work_sync(&r->work);
	mutex_lock(&r->hw_lock);
	if (r->hw != HW_OEM) {
		if (!rgb_release())
			r->hw = HW_OEM;
	}
	mutex_unlock(&r->hw_lock);
}

static void rgb_teardown(void *data)
{
	struct suiplay_rgb *r = data;

	rgb_stop(r);
	if (the_rgb == r)
		the_rgb = NULL;
}

int suiplay_rgb_probe(struct device *dev)
{
	struct suiplay_rgb *r;
	int ret;

	if (!rgb)
		return 0;
	r = devm_kzalloc(dev, sizeof(*r), GFP_KERNEL);
	if (!r)
		return -ENOMEM;
	r->dev = dev;
	mutex_init(&r->lock);
	mutex_init(&r->hw_lock);
	INIT_WORK(&r->work, rgb_work);
	r->want_mode = MODE_OEM;
	r->hw = HW_OEM;

	r->subled[0].color_index = LED_COLOR_ID_RED;
	r->subled[0].channel = 0;
	r->subled[1].color_index = LED_COLOR_ID_GREEN;
	r->subled[1].channel = 1;
	r->subled[2].color_index = LED_COLOR_ID_BLUE;
	r->subled[2].channel = 2;
	r->mc.subled_info = r->subled;
	r->mc.num_colors = ARRAY_SIZE(r->subled);
	r->mc.led_cdev.name = "ayaneo:rgb:joystick_rings";
	r->mc.led_cdev.color = LED_COLOR_ID_RGB;
	r->mc.led_cdev.max_brightness = 255;
	r->mc.led_cdev.brightness_set_blocking = rgb_brightness_set;
	r->mc.led_cdev.groups = rgb_groups;
	/* Unregistering must not write "off"; rgb_teardown hands the LEDs to the EC instead. */
	r->mc.led_cdev.flags = LED_RETAIN_AT_SHUTDOWN;

	/* Registered first, so it runs after the LED is gone (devm runs in reverse). */
	ret = devm_add_action_or_reset(dev, rgb_teardown, r);
	if (ret)
		return ret;
	ret = devm_led_classdev_multicolor_register(dev, &r->mc);
	if (ret) {
		/* The fan matters more than the lights: carry on without them. */
		dev_warn(dev, "stick lights not registered (%d)\n", ret);
		return 0;
	}
	the_rgb = r;
	dev_info(dev, "stick lights at %s; the EC keeps them until written\n", r->mc.led_cdev.name);
	return 0;
}

void suiplay_rgb_suspend(struct device *dev)
{
	struct suiplay_rgb *r = the_rgb;

	if (!r)
		return;
	mutex_lock(&r->lock);
	r->stopped = true;
	mutex_unlock(&r->lock);
	cancel_work_sync(&r->work);
	mutex_lock(&r->hw_lock);
	/* Dark while asleep; the EC's own lights (oem) are left to the EC. */
	if (r->hw == HW_ON || r->hw == HW_UNKNOWN) {
		if (!rgb_take())
			r->hw = HW_OFF;
	}
	mutex_unlock(&r->hw_lock);
}

void suiplay_rgb_resume(struct device *dev)
{
	struct suiplay_rgb *r = the_rgb;

	if (!r)
		return;
	mutex_lock(&r->hw_lock);
	if (r->hw != HW_OEM) {
		/* The EC may have reset its LED state: take it again and write the whole frame. */
		r->hw = HW_UNKNOWN;
		r->hw_valid = false;
		r->settle = true;
	}
	mutex_unlock(&r->hw_lock);
	mutex_lock(&r->lock);
	r->stopped = false;
	mutex_unlock(&r->lock);
	rgb_kick(r);
}

void suiplay_rgb_shutdown(struct device *dev)
{
	if (the_rgb)
		rgb_stop(the_rgb);
}
