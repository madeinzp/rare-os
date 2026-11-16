// SPDX-License-Identifier: GPL-2.0+
/*
 * Rare OS: the SuiPlay0X1's stick rings, through the EC's LED registers.
 *
 * The protocol is the "legacy" AYANEO one the AYANEO 2S (and so the SuiPlay0X1) uses, as in
 * ShadowBlip's ayaneo-platform (GPL-2.0+; Derek J. Clark, JELOS, Sebastian Kranz, Trevor
 * Heslop; derived from Maya Matuszczyk's ayaled). Stock GameOS loads that driver on the
 * SuiPlay0X1, pinned at 9b2a602 "Add support for SuiPlay0X1 RGB":
 *
 *   one subpixel: 0x6d = group, 0xb1 = position, 0xb2 = value, 0xbf = 0x10 (write),
 *                 about 2 ms, 0xbf = 0xfe (hold)
 *   0xbf = 0x00   gives the LEDs back to the EC
 *   position 2    0xb1 on, 0x31 off, 0xc0 reset
 *   positions 3, 6, 9, 12: the four zones of a ring, red, green, blue at +0, +1, +2
 *   group 3, position 0, value 0 ends a batch. Values go up to 192 (0..255 scaled, as there).
 *
 * The group register (0x6d), as the sources name it:
 *   ayaneo-platform 9b2a602   AYANEO_LED_GROUP_LEFT 1, _RIGHT 2, _LEFT_RIGHT 3, _BUTTON 4 (KUN);
 *                             the 2S path writes one color, group 1 then 2, each closed by 3/0/0
 *   HandheldCompanion         LEDGroup StickLeft 1, StickRight 2, StickBoth 3, AYA 4; on the
 *     (AYANEODeviceCEc.cs,    "CEc" devices (SuiPlay0X1 included) every subpixel is followed by
 *      SuiPlay0X1.cs)         3/0/0, 5 ms each, "AYASpace does this"
 *   HueSync (honjow)          the same 1/2/3, per-zone colors on SuiPlay0X1 through the 2S path
 *   ayaled (Maccraft123,      Joystick Left 1, Right 2, ALL 3
 *     Gawah)
 * None of them has been seen to show two colors on a SuiPlay0X1: stock GameOS, hhd and
 * InputPlumber only drive the one multicolor LED (both rings, one color), so 1 and 2 always get
 * the same values and nothing tells left from right. A report on Rare OS 0.1.3 (Left lit both
 * rings, Right changed neither) was first read as "group 1 reaches both rings, group 2 neither",
 * but this driver never ran there: the kernel's ayaneo_ec had taken the "ayaneo-ec" device
 * (rare-market/rare-os#8, now /usr/lib/modprobe.d/rare-suiplay-ec.conf), and rare-fand drew the
 * left color on ayaneo-platform's one-color LED, which is exactly what was seen. So 1 = left,
 * 2 = right is still the sources' word, not yet seen on a SuiPlay0X1, and:
 *   - rgb_map says where each of the eight zones is written (group:position, "-" for none), at
 *     load (module parameter) or live (sysfs). Default: the sources' 1:3 1:6 1:9 1:12, 2:3 ...;
 *   - rgb_rings tells userspace how many rings it can color apart (default 1, Settings then
 *     offers one color for both) until a SuiPlay0X1 shows two;
 *   - rgb_raw writes "group position value" triples as they are, for rare-lights-test, which
 *     walks a person through the groups and zones in under a minute and prints the map.
 *
 * Differences from ayaneo-platform:
 *   - nothing is written until userspace asks. From boot the rings keep the EC's own behaviour
 *     (rgb_mode "oem"), so a device nobody configures looks as it always did;
 *   - each ring and each of its four zones takes its own color (rgb_zones). A ring whose colors
 *     changed is written whole, closed by 3/0/0, the order ayaneo-platform and HueSync use; an
 *     unchanged ring is skipped. The writes run in a work item that sleeps between them (no busy
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
 *   rgb_map    eight group:position words (or "-"), comma or space separated; root writes it
 *   rgb_rings  1 or 2; root writes it
 *   rgb_raw    root only, write only: "group position value" triples (0x.. or decimal)
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

#define DEFAULT_MAP		"1:3,1:6,1:9,1:12,2:3,2:6,2:9,2:12"
static char rgb_map[64] = DEFAULT_MAP;
module_param_string(rgb_map, rgb_map, sizeof(rgb_map), 0444);
MODULE_PARM_DESC(rgb_map, "Where the eight zones (left ring 1-4, right ring 1-4) are written: group:position each, - for none (default " DEFAULT_MAP ")");

static uint rgb_rings = 1;
module_param(rgb_rings, uint, 0444);
MODULE_PARM_DESC(rgb_rings, "Rings the map colors apart, for userspace: 1 (one color for both, default) or 2");

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
#define GROUP_MAX		0x04	/* 4: the KUN's button; higher ones are unknown */

#define POS_ENABLE		0x02
#define ENABLE_ON		0xb1
#define ENABLE_OFF		0x31
#define ENABLE_RESET		0xc0

#define RINGS			2
#define ZONES			4
#define MAX_VALUE		192
#define GLOBAL_LOCK_MS		500
#define RAW_MAX			48

/* Where one zone's red goes (green and blue follow). pos 0: the zone is not written. */
struct zone_addr {
	u8 group;
	u8 pos;
};

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
	struct zone_addr map[RINGS][ZONES];
	unsigned int rings;
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
static int rgb_apply(struct suiplay_rgb *r, enum rgb_mode mode, u8 want[RINGS][ZONES][3],
		     struct zone_addr map[RINGS][ZONES])
{
	int ring, zone, ch, ret = 0;
	bool valid;

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
	/* A ring that changed is written whole and closed by 3/0/0 (ayaneo-platform's order). */
	valid = r->hw_valid;
	for (ring = 0; ring < RINGS; ring++) {
		u8 v[ZONES][3];
		bool dirty = !valid;
		int n = 0;

		for (zone = 0; zone < ZONES; zone++) {
			for (ch = 0; ch < 3; ch++) {
				v[zone][ch] = rgb_scale(want[ring][zone][ch]);
				if (v[zone][ch] != r->shown[ring][zone][ch])
					dirty = true;
			}
		}
		if (!dirty)
			continue;
		r->hw_valid = false;
		for (zone = 0; zone < ZONES; zone++) {
			const struct zone_addr *a = &map[ring][zone];

			if (!a->pos)
				continue;
			for (ch = 0; ch < 3; ch++) {
				ret = rgb_set(a->group, a->pos + ch, v[zone][ch]);
				if (ret)
					return ret;
				n++;
			}
		}
		if (n)
			ret = rgb_set(GROUP_BOTH, 0, 0);
		if (ret)
			return ret;
		memcpy(r->shown[ring], v, sizeof(v));
	}
	r->hw_valid = true;
	return 0;
}

static void rgb_work(struct work_struct *work)
{
	struct suiplay_rgb *r = container_of(work, struct suiplay_rgb, work);
	u8 want[RINGS][ZONES][3];
	struct zone_addr map[RINGS][ZONES];
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
	memcpy(map, r->map, sizeof(map));
	mutex_unlock(&r->lock);
	ret = rgb_apply(r, mode, want, map);
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

/* "G:P" words (or "-"), eight of them, separated by commas or white space. */
static int map_parse(const char *buf, size_t count, struct zone_addr out[RINGS][ZONES])
{
	struct zone_addr m[RINGS * ZONES];
	char *dup, *p, *tok;
	int n = 0, ret = 0;

	dup = kstrndup(buf, count, GFP_KERNEL);
	if (!dup)
		return -ENOMEM;
	p = dup;
	while ((tok = strsep(&p, ", \t\n"))) {
		char *colon;
		u8 g, pos;

		if (!*tok)
			continue;
		if (n == RINGS * ZONES) {
			ret = -EINVAL;
			break;
		}
		if (!strcmp(tok, "-")) {
			m[n].group = 0;
			m[n++].pos = 0;
			continue;
		}
		colon = strchr(tok, ':');
		if (!colon) {
			ret = -EINVAL;
			break;
		}
		*colon = '\0';
		/* Only the zone positions: 0..2 and 15.. are commands (enable, pattern, fade, ...). */
		if (kstrtou8(tok, 0, &g) || kstrtou8(colon + 1, 0, &pos) || g < 1 || g > GROUP_MAX ||
		    (pos != 3 && pos != 6 && pos != 9 && pos != 12)) {
			ret = -EINVAL;
			break;
		}
		m[n].group = g;
		m[n++].pos = pos;
	}
	kfree(dup);
	if (!ret && n != RINGS * ZONES)
		ret = -EINVAL;
	if (!ret)
		memcpy(out, m, sizeof(m));
	return ret;
}

static ssize_t rgb_map_show(struct device *dev, struct device_attribute *attr, char *buf)
{
	struct suiplay_rgb *r = rgb_of(dev);
	ssize_t len = 0;
	int ring, zone;

	mutex_lock(&r->lock);
	for (ring = 0; ring < RINGS; ring++) {
		for (zone = 0; zone < ZONES; zone++) {
			const struct zone_addr *a = &r->map[ring][zone];
			char sep = ring == RINGS - 1 && zone == ZONES - 1 ? '\n' : ',';

			if (a->pos)
				len += sysfs_emit_at(buf, len, "%u:%u%c", a->group, a->pos, sep);
			else
				len += sysfs_emit_at(buf, len, "-%c", sep);
		}
	}
	mutex_unlock(&r->lock);
	return len;
}

static ssize_t rgb_map_store(struct device *dev, struct device_attribute *attr,
			     const char *buf, size_t count)
{
	struct suiplay_rgb *r = rgb_of(dev);
	struct zone_addr m[RINGS][ZONES];
	int ret;

	ret = map_parse(buf, count, m);
	if (ret)
		return ret;
	mutex_lock(&r->lock);
	memcpy(r->map, m, sizeof(m));
	mutex_unlock(&r->lock);
	/* The whole frame again, at the new places. */
	mutex_lock(&r->hw_lock);
	r->hw_valid = false;
	mutex_unlock(&r->hw_lock);
	rgb_kick(r);
	return count;
}
static DEVICE_ATTR_RW(rgb_map);

static ssize_t rgb_rings_show(struct device *dev, struct device_attribute *attr, char *buf)
{
	struct suiplay_rgb *r = rgb_of(dev);

	return sysfs_emit(buf, "%u\n", READ_ONCE(r->rings));
}

static ssize_t rgb_rings_store(struct device *dev, struct device_attribute *attr,
			       const char *buf, size_t count)
{
	struct suiplay_rgb *r = rgb_of(dev);
	unsigned int v;

	if (kstrtouint(buf, 0, &v) || v < 1 || v > RINGS)
		return -EINVAL;
	WRITE_ONCE(r->rings, v);
	return count;
}
static DEVICE_ATTR_RW(rgb_rings);

/*
 * "group position value" triples, written as they are (no scaling, no batch end: send 3 0 0
 * yourself). For finding where the zones are (rare-lights-test). The next frame takes the
 * LEDs again and writes everything, so whatever this left is drawn over.
 */
static ssize_t rgb_raw_store(struct device *dev, struct device_attribute *attr,
			     const char *buf, size_t count)
{
	struct suiplay_rgb *r = rgb_of(dev);
	u8 t[RAW_MAX][3];
	char *dup, *p, *tok;
	int n = 0, i, ret = 0;
	bool stopped;

	dup = kstrndup(buf, count, GFP_KERNEL);
	if (!dup)
		return -ENOMEM;
	p = dup;
	while ((tok = strsep(&p, ", \t\n"))) {
		if (!*tok)
			continue;
		if (n == RAW_MAX * 3 || kstrtou8(tok, 0, &t[n / 3][n % 3])) {
			ret = -EINVAL;
			break;
		}
		n++;
	}
	kfree(dup);
	if (!ret && (n == 0 || n % 3))
		ret = -EINVAL;
	for (i = 0; !ret && i < n / 3; i++)
		if (t[i][0] < 1 || t[i][0] > GROUP_MAX)
			ret = -EINVAL;
	if (ret)
		return ret;

	mutex_lock(&r->lock);
	stopped = r->stopped;
	mutex_unlock(&r->lock);
	if (stopped)
		return -EBUSY;
	mutex_lock(&r->hw_lock);
	for (i = 0; !ret && i < n / 3; i++)
		ret = rgb_set(t[i][0], t[i][1], t[i][2]);
	r->hw = HW_UNKNOWN;
	r->hw_valid = false;
	mutex_unlock(&r->hw_lock);
	return ret ? ret : count;
}
static DEVICE_ATTR_WO(rgb_raw);

static struct attribute *rgb_attrs[] = {
	&dev_attr_rgb_mode.attr,
	&dev_attr_rgb_zones.attr,
	&dev_attr_rgb_map.attr,
	&dev_attr_rgb_rings.attr,
	&dev_attr_rgb_raw.attr,
	NULL
};

static const struct attribute_group rgb_group = {
	.attrs = rgb_attrs,
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

static void rgb_remove_group(void *data)
{
	struct suiplay_rgb *r = data;

	sysfs_remove_group(&r->mc.led_cdev.dev->kobj, &rgb_group);
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
	if (map_parse(rgb_map, strlen(rgb_map), r->map)) {
		dev_warn(dev, "stick lights: rgb_map \"%s\" is not eight group:position words; using %s\n",
			 rgb_map, DEFAULT_MAP);
		map_parse(DEFAULT_MAP, strlen(DEFAULT_MAP), r->map);
	}
	r->rings = clamp_val(rgb_rings, 1, RINGS);

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
	/*
	 * rgb_mode, rgb_zones, rgb_map, rgb_rings and rgb_raw go on the LED once it is registered:
	 * led_classdev_multicolor_register_ext() sets led_cdev.groups to its own (multi_intensity,
	 * multi_index), so groups set before it never appeared (Rare OS 0.1.3: rare-fand found no
	 * rgb_zones and drew one color). Removed before the LED goes (devm runs in reverse).
	 */
	ret = sysfs_create_group(&r->mc.led_cdev.dev->kobj, &rgb_group);
	if (!ret)
		ret = devm_add_action_or_reset(dev, rgb_remove_group, r);
	if (ret)
		dev_warn(dev, "stick lights: no rgb_zones (%d), one color only\n", ret);
	the_rgb = r;
	dev_info(dev, "stick lights at %s (map %s, %u ring%s); the EC keeps them until written\n",
		 r->mc.led_cdev.name, rgb_map, r->rings, r->rings == 1 ? "" : "s");
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
