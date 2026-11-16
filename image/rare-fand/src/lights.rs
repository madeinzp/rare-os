//! The stick ring lights (SuiPlay0X1): what Settings > Lighting picked, drawn as frames on the
//! ayaneo_ec_suiplay LED (image/kmod/ayaneo-ec-suiplay/suiplay-rgb.c).
//!
//! sysfs, /sys/class/leds/ayaneo:rgb:joystick_rings:
//! - rgb_mode: "oem" (the EC's own lights), "on" (rgb_zones), "off" (dark, held)
//! - rgb_zones: eight RRGGBB words, the left ring's four zones then the right ring's
//! - rgb_rings: how many rings the driver can color apart. 1 (the default until a SuiPlay0X1
//!   has shown two, see suiplay-rgb.c): both rings take the left color, and Settings offers
//!   one color. Missing (Rare OS 0.1.3's module): 2
//! - brightness, multi_intensity: the standard multicolor LED (both rings one color). Used when
//!   the driver is another one (ShadowBlip's ayaneo-platform has no rgb_zones).
//!
//! The spec is one line of `key=value` words, the same on the bus (SetLights) and in
//! /var/lib/rare/lights: `mode=on effect=breathe left=4da2ff right=4da2ff brightness=80`.
//! "default" leaves the rings to the EC, which is what a device nobody configured shows.
//!
//! Cost: a static color is written once; breathe redraws every 100 ms and cycle every 200 ms,
//! and the driver writes only the subpixels that changed, sleeping between EC writes.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const LED: &str = "sys/class/leds/ayaneo:rgb:joystick_rings";
pub const STATE_FILE: &str = "/var/lib/rare/lights";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// The EC's own lights (nothing written).
    Default,
    On,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Solid,
    /// The color fades down and up, 4 s a breath.
    Breathe,
    /// A rainbow turning around each ring, 8 s a turn.
    Cycle,
}

pub type Rgb = [u8; 3];

/// Sui blue, the color a ring starts with when lighting is first turned on.
pub const DEFAULT_COLOR: Rgb = [0x4d, 0xa2, 0xff];

#[derive(Clone, Debug, PartialEq)]
pub struct Spec {
    pub mode: Mode,
    pub effect: Effect,
    pub left: Rgb,
    pub right: Rgb,
    /// Percent, 0..=100.
    pub brightness: u8,
}

impl Default for Spec {
    fn default() -> Self {
        Spec { mode: Mode::Default, effect: Effect::Solid, left: DEFAULT_COLOR, right: DEFAULT_COLOR, brightness: 100 }
    }
}

pub fn hex(c: Rgb) -> String {
    format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

pub fn parse_hex(s: &str) -> Option<Rgb> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 || !s.is_ascii() {
        return None;
    }
    let b = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).ok();
    Some([b(0)?, b(2)?, b(4)?])
}

impl Spec {
    pub fn encode(&self) -> String {
        let mode = match self.mode {
            Mode::Default => "default",
            Mode::On => "on",
            Mode::Off => "off",
        };
        let effect = match self.effect {
            Effect::Solid => "solid",
            Effect::Breathe => "breathe",
            Effect::Cycle => "cycle",
        };
        format!("mode={mode} effect={effect} left={} right={} brightness={}", hex(self.left), hex(self.right), self.brightness)
    }

    /// Unknown keys are skipped (a newer launcher may send more); a bad value is an error.
    pub fn parse(s: &str) -> Result<Spec, String> {
        let mut spec = Spec::default();
        for word in s.split_whitespace() {
            let Some((k, v)) = word.split_once('=') else { return Err(format!("{word:?} is not key=value")) };
            match k {
                "mode" => {
                    spec.mode = match v {
                        "default" => Mode::Default,
                        "on" => Mode::On,
                        "off" => Mode::Off,
                        _ => return Err(format!("mode {v:?}: use default, on or off")),
                    }
                }
                "effect" => {
                    spec.effect = match v {
                        "solid" => Effect::Solid,
                        "breathe" => Effect::Breathe,
                        "cycle" => Effect::Cycle,
                        _ => return Err(format!("effect {v:?}: use solid, breathe or cycle")),
                    }
                }
                "left" => spec.left = parse_hex(v).ok_or_else(|| format!("left {v:?} is not RRGGBB"))?,
                "right" => spec.right = parse_hex(v).ok_or_else(|| format!("right {v:?} is not RRGGBB"))?,
                "brightness" => {
                    spec.brightness = v.parse::<u8>().ok().filter(|b| *b <= 100).ok_or_else(|| format!("brightness {v:?}: 0 to 100"))?;
                }
                _ => {}
            }
        }
        Ok(spec)
    }
}

/// Eight zones: the left ring's four, then the right ring's.
pub type Frame = [Rgb; 8];

#[derive(Clone, Debug, PartialEq)]
pub enum Out {
    Oem,
    Off,
    Frame(Frame),
}

fn scale(c: Rgb, k: f32) -> Rgb {
    let k = k.clamp(0.0, 1.0);
    [(c[0] as f32 * k).round() as u8, (c[1] as f32 * k).round() as u8, (c[2] as f32 * k).round() as u8]
}

/// Hue in degrees, full saturation and value.
pub fn hue(h: f32) -> Rgb {
    let h = h.rem_euclid(360.0) / 60.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    [(r * 255.0f32).round() as u8, (g * 255.0f32).round() as u8, (b * 255.0f32).round() as u8]
}

pub const BREATH: f32 = 4.0;
pub const TURN: f32 = 8.0;
/// The dimmest point of a breath, as a share of the brightness: the ring never goes fully dark.
const BREATH_FLOOR: f32 = 0.08;

/// The spec as `rings` rings can show it: with one, both take the left ring's color.
pub fn for_rings(spec: &Spec, rings: u8) -> Spec {
    if rings >= 2 {
        return spec.clone();
    }
    Spec { right: spec.left, ..spec.clone() }
}

/// What the rings show `t` seconds into the effect.
pub fn render(spec: &Spec, t: f32) -> Out {
    match spec.mode {
        Mode::Default => return Out::Oem,
        Mode::Off => return Out::Off,
        Mode::On => {}
    }
    let k = spec.brightness as f32 / 100.0;
    let mut f = [[0u8; 3]; 8];
    match spec.effect {
        Effect::Solid | Effect::Breathe => {
            let k = if spec.effect == Effect::Breathe {
                let wave = 0.5 - 0.5 * (std::f32::consts::TAU * t / BREATH).cos();
                k * (BREATH_FLOOR + (1.0 - BREATH_FLOOR) * wave)
            } else {
                k
            };
            for (i, z) in f.iter_mut().enumerate() {
                *z = scale(if i < 4 { spec.left } else { spec.right }, k);
            }
        }
        Effect::Cycle => {
            let base = 360.0 * (t / TURN);
            for (i, z) in f.iter_mut().enumerate() {
                *z = scale(hue(base + (i % 4) as f32 * 90.0), k);
            }
        }
    }
    Out::Frame(f)
}

/// How long until the frame changes again; None for a still one.
pub fn interval(spec: &Spec) -> Option<Duration> {
    match (spec.mode, spec.effect) {
        (Mode::On, Effect::Breathe) => Some(Duration::from_millis(100)),
        (Mode::On, Effect::Cycle) => Some(Duration::from_millis(200)),
        _ => None,
    }
}

pub fn encode_frame(f: &Frame) -> String {
    let mut s = String::with_capacity(56);
    for (i, z) in f.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        let _ = write!(s, "{}", hex(*z));
    }
    s
}

/// The LED in sysfs.
pub struct Led {
    pub dir: PathBuf,
    /// ayaneo_ec_suiplay's rgb_zones and rgb_mode are there (else the plain multicolor files).
    zones: bool,
    /// Rings that take their own color: 1 or 2.
    pub rings: u8,
}

impl Led {
    pub fn find(root: &Path) -> Option<Led> {
        let dir = root.join(LED);
        if !dir.join("multi_intensity").exists() && !dir.join("rgb_zones").exists() {
            return None;
        }
        let zones = dir.join("rgb_zones").exists() && dir.join("rgb_mode").exists();
        let rings = if !zones {
            1
        } else {
            match fs::read_to_string(dir.join("rgb_rings")).ok().and_then(|s| s.trim().parse::<u8>().ok()) {
                Some(1) => 1,
                _ => 2,
            }
        };
        Some(Led { dir, zones, rings })
    }

    pub fn write(&self, out: &Out) -> io::Result<()> {
        if self.zones {
            return match out {
                Out::Oem => fs::write(self.dir.join("rgb_mode"), "oem"),
                Out::Off => fs::write(self.dir.join("rgb_mode"), "off"),
                Out::Frame(f) => fs::write(self.dir.join("rgb_zones"), encode_frame(f)),
            };
        }
        // Another driver: one color for both rings (the left ring's first zone), no EC mode.
        match out {
            Out::Oem => Ok(()),
            Out::Off => fs::write(self.dir.join("brightness"), "0"),
            Out::Frame(f) => {
                fs::write(self.dir.join("multi_intensity"), format!("{} {} {}", f[0][0], f[0][1], f[0][2]))?;
                fs::write(self.dir.join("brightness"), "255")
            }
        }
    }
}

pub fn load(path: &Path) -> Spec {
    match fs::read_to_string(path) {
        Ok(s) => Spec::parse(s.trim()).unwrap_or_else(|e| {
            eprintln!("rare-fand: {}: {e}; lights left to the EC", path.display());
            Spec::default()
        }),
        Err(_) => Spec::default(),
    }
}

pub fn save(path: &Path, spec: &Spec) -> io::Result<()> {
    if let Some(d) = path.parent() {
        fs::create_dir_all(d)?;
    }
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, format!("{}\n", spec.encode()))?;
    fs::rename(&tmp, path)
}

/// The rings back to the EC (ExecStopPost, a panic): an effect never freezes mid-frame.
pub fn release(root: &Path) -> io::Result<bool> {
    match Led::find(root) {
        Some(l) if l.zones => l.write(&Out::Oem).map(|_| true),
        _ => Ok(false),
    }
}

pub struct Lights {
    led: Option<Led>,
    path: PathBuf,
    spec: Spec,
    /// The effect's time zero (a new spec restarts it).
    t0: Instant,
    last: Option<Out>,
    next: Option<Instant>,
    error: Option<String>,
}

impl Lights {
    pub fn new(root: &Path, path: &Path) -> Lights {
        let led = Led::find(root);
        let spec = load(path);
        if let Some(l) = &led {
            eprintln!("rare-fand: stick lights at {}, {} ring{}, {}", l.dir.display(), l.rings, if l.rings == 1 { "" } else { "s" }, spec.encode());
        }
        Lights { led, path: path.to_path_buf(), spec, t0: Instant::now(), last: None, next: None, error: None }
    }

    pub fn available(&self) -> bool {
        self.led.is_some()
    }

    /// Rings that take their own color (0 without the LED).
    pub fn rings(&self) -> u8 {
        self.led.as_ref().map_or(0, |l| l.rings)
    }

    pub fn spec(&self) -> &Spec {
        &self.spec
    }

    /// Applied on the next tick and saved.
    pub fn set(&mut self, spec: Spec, now: Instant) {
        if spec == self.spec {
            return;
        }
        if let Err(e) = save(&self.path, &spec) {
            eprintln!("rare-fand: saving {}: {e}", self.path.display());
        }
        // The same effect keeps its phase (a color or brightness change mid-breath does not jump).
        if spec.effect != self.spec.effect || spec.mode != self.spec.mode {
            self.t0 = now;
        }
        eprintln!("rare-fand: lights {}", spec.encode());
        self.spec = spec;
        self.next = None;
    }

    /// Draws the frame due at `now`; cheap when nothing is due.
    pub fn tick(&mut self, now: Instant) {
        let Some(led) = &self.led else { return };
        if self.next.is_some_and(|n| now < n) {
            return;
        }
        let out = render(&for_rings(&self.spec, led.rings), (now - self.t0).as_secs_f32());
        self.next = interval(&self.spec).map(|d| now + d);
        if self.last.as_ref() == Some(&out) {
            return;
        }
        match led.write(&out) {
            Ok(()) => {
                self.last = Some(out);
                self.error = None;
            }
            Err(e) => {
                let msg = format!("writing the stick lights: {e}");
                if self.error.as_deref() != Some(&msg) {
                    eprintln!("rare-fand: {msg}");
                }
                self.error = Some(msg);
                self.last = None;
                self.next = Some(now + Duration::from_secs(5));
            }
        }
    }

    /// Back to the EC, on exit.
    pub fn shutdown(&mut self) {
        if let Some(l) = &self.led {
            if l.zones {
                let _ = l.write(&Out::Oem);
            }
        }
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_round_trips_and_checks() {
        let s = Spec { mode: Mode::On, effect: Effect::Breathe, left: [0xff, 0x2a, 0x6d], right: [0, 0, 0xff], brightness: 80 };
        assert_eq!(s.encode(), "mode=on effect=breathe left=ff2a6d right=0000ff brightness=80");
        assert_eq!(Spec::parse(&s.encode()), Ok(s));
        assert_eq!(Spec::parse(""), Ok(Spec::default()));
        // A newer launcher's keys are skipped.
        assert_eq!(Spec::parse("mode=off rumble=1").unwrap().mode, Mode::Off);
        assert!(Spec::parse("mode=loud").is_err());
        assert!(Spec::parse("left=12345").is_err());
        assert!(Spec::parse("brightness=101").is_err());
        assert!(Spec::parse("mode").is_err());
        assert_eq!(parse_hex("#4DA2FF"), Some(DEFAULT_COLOR));
    }

    #[test]
    fn frames() {
        let mut s = Spec { mode: Mode::On, effect: Effect::Solid, left: [200, 100, 0], right: [0, 0, 255], brightness: 50 };
        let Out::Frame(f) = render(&s, 3.0) else { panic!() };
        assert_eq!(f[0], [100, 50, 0]);
        assert_eq!(f[3], [100, 50, 0]);
        assert_eq!(f[4], [0, 0, 128]);
        assert_eq!(interval(&s), None);
        s.effect = Effect::Breathe;
        s.brightness = 100;
        let Out::Frame(lo) = render(&s, 0.0) else { panic!() };
        let Out::Frame(hi) = render(&s, BREATH / 2.0) else { panic!() };
        assert_eq!(hi[4], [0, 0, 255]);
        assert!(lo[4][2] > 0 && lo[4][2] < 30, "the breath's floor is dim, not dark: {:?}", lo[4]);
        s.effect = Effect::Cycle;
        let Out::Frame(c) = render(&s, 0.0) else { panic!() };
        assert_eq!(c[0], [255, 0, 0]);
        assert_eq!(c[2], [0, 255, 255]);
        assert_eq!(c[4], c[0]);
        assert_eq!(render(&Spec::default(), 0.0), Out::Oem);
        assert_eq!(render(&Spec { mode: Mode::Off, ..s }, 0.0), Out::Off);
        assert_eq!(hue(120.0), [0, 255, 0]);
        assert_eq!(hue(-120.0), [0, 0, 255]);
    }

    fn fake(name: &str, zones: bool) -> PathBuf {
        let root = std::env::temp_dir().join(format!("rare-fand-lights-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let led = root.join(LED);
        fs::create_dir_all(&led).unwrap();
        for f in ["brightness", "multi_intensity"] {
            fs::write(led.join(f), "0").unwrap();
        }
        if zones {
            fs::write(led.join("rgb_mode"), "oem").unwrap();
            fs::write(led.join("rgb_zones"), "").unwrap();
        }
        root
    }

    #[test]
    fn writes_only_changes_and_persists() {
        let root = fake("persist", true);
        let state = root.join("var/lib/rare/lights");
        let led = root.join(LED);
        let t = Instant::now();
        let mut l = Lights::new(&root, &state);
        assert!(l.available());
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("rgb_mode")).unwrap(), "oem");
        let on = Spec { mode: Mode::On, effect: Effect::Solid, left: [255, 0, 0], right: [0, 255, 0], brightness: 100 };
        l.set(on.clone(), t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), "ff0000 ff0000 ff0000 ff0000 00ff00 00ff00 00ff00 00ff00");
        // A still frame is not written again.
        fs::write(led.join("rgb_zones"), "x").unwrap();
        l.tick(t + Duration::from_secs(1));
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), "x");
        // Saved: a restart (reboot) shows it again.
        assert_eq!(load(&state), on);
        let mut again = Lights::new(&root, &state);
        again.tick(t);
        assert!(fs::read_to_string(led.join("rgb_zones")).unwrap().starts_with("ff0000"));
        // Breathe redraws on its interval only.
        l.set(Spec { effect: Effect::Breathe, ..on }, t);
        l.tick(t);
        let a = fs::read_to_string(led.join("rgb_zones")).unwrap();
        l.tick(t + Duration::from_millis(50));
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), a);
        l.tick(t + Duration::from_millis(1000));
        assert_ne!(fs::read_to_string(led.join("rgb_zones")).unwrap(), a);
        // Exit and release give the rings back to the EC.
        l.shutdown();
        assert_eq!(fs::read_to_string(led.join("rgb_mode")).unwrap(), "oem");
        fs::write(led.join("rgb_mode"), "on").unwrap();
        assert_eq!(release(&root).unwrap(), true);
        assert_eq!(fs::read_to_string(led.join("rgb_mode")).unwrap(), "oem");
        let _ = fs::remove_dir_all(root);
    }

    /// Settings' Left and Right rows: one ring's color changes, the other ring's zones stay, and
    /// the left ring is always the first four words of rgb_zones (suiplay-rgb.c's order).
    #[test]
    fn left_and_right_land_on_their_own_zones() {
        let root = fake("rings2", true);
        let led = root.join(LED);
        let t = Instant::now();
        let mut l = Lights::new(&root, &root.join("lights"));
        assert_eq!(l.rings(), 2, "0.1.3's module has no rgb_rings: two rings, as before");
        let both = Spec { mode: Mode::On, effect: Effect::Solid, left: [0, 0, 255], right: [0, 0, 255], brightness: 100 };
        l.set(both.clone(), t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), ["0000ff"; 8].join(" "));
        l.set(Spec { left: [255, 0, 0], ..both.clone() }, t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), "ff0000 ff0000 ff0000 ff0000 0000ff 0000ff 0000ff 0000ff");
        l.set(Spec { left: [255, 0, 0], right: [0, 255, 0], ..both }, t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), "ff0000 ff0000 ff0000 ff0000 00ff00 00ff00 00ff00 00ff00");
        // Rainbow: zone n of both rings takes the same hue.
        let Out::Frame(c) = render(&Spec { effect: Effect::Cycle, ..l.spec().clone() }, 1.0) else { panic!() };
        assert_eq!(&c[..4], &c[4..]);
        let _ = fs::remove_dir_all(root);
    }

    /// A driver that can color only both rings together (rgb_rings 1): the left color everywhere.
    #[test]
    fn one_ring_driver_draws_the_left_color_on_both() {
        let root = fake("rings1", true);
        let led = root.join(LED);
        fs::write(led.join("rgb_rings"), "1\n").unwrap();
        let t = Instant::now();
        let mut l = Lights::new(&root, &root.join("lights"));
        assert_eq!(l.rings(), 1);
        l.set(Spec { mode: Mode::On, effect: Effect::Breathe, left: [255, 0, 0], right: [0, 255, 0], brightness: 100 }, t);
        l.tick(t + Duration::from_secs(2));
        assert_eq!(fs::read_to_string(led.join("rgb_zones")).unwrap(), ["ff0000"; 8].join(" "));
        // The saved spec keeps both colors (a driver with two rings shows them again).
        assert_eq!(l.spec().right, [0, 255, 0]);
        fs::write(led.join("rgb_rings"), "2\n").unwrap();
        assert_eq!(Lights::new(&root, &root.join("lights")).rings(), 2);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn plain_multicolor_driver() {
        let root = fake("plain", false);
        let led = root.join(LED);
        let t = Instant::now();
        let mut l = Lights::new(&root, &root.join("lights"));
        assert_eq!(l.rings(), 1);
        l.set(Spec { mode: Mode::On, left: [10, 20, 30], ..Spec::default() }, t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("multi_intensity")).unwrap(), "10 20 30");
        assert_eq!(fs::read_to_string(led.join("brightness")).unwrap(), "255");
        l.set(Spec { mode: Mode::Off, ..Spec::default() }, t);
        l.tick(t);
        assert_eq!(fs::read_to_string(led.join("brightness")).unwrap(), "0");
        assert_eq!(release(&root).unwrap(), false);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn no_led_no_writes() {
        let root = std::env::temp_dir().join(format!("rare-fand-lights-{}-none", std::process::id()));
        let mut l = Lights::new(&root, &root.join("lights"));
        assert!(!l.available());
        assert_eq!(l.rings(), 0);
        l.tick(Instant::now());
    }
}
