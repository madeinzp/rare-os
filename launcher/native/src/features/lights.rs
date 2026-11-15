//! The stick lights: Settings > Lighting's choices (launcher.json `lightsMode`, `lightsEffect`,
//! `lightsLeft`, `lightsRight`, `lightsBrightness`) sent to rare-fand (org.rare.Lights), which
//! draws them on the rings and keeps them across reboots, sleep and Desktop mode.
//!
//! The spec is rare-fand's (image/rare-fand/src/lights.rs):
//! `mode=on effect=breathe left=4da2ff right=4da2ff brightness=80`. Nothing is sent while
//! `lightsMode` is unset, so a device nobody configured keeps the controller's own lights.
//! Sends run on one worker thread and only the newest waiting spec is sent, so scrolling through
//! colors previews live without queueing up.

use crate::screens::settings::config::Config;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Condvar, Mutex, Once};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightsMode {
    Default,
    On,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Solid,
    Breathe,
    Cycle,
}

/// Sui blue: both rings when lighting is first turned on.
pub const DEFAULT_COLOR: [u8; 3] = [0x4d, 0xa2, 0xff];

pub fn mode(c: &Config) -> LightsMode {
    match c.lights_mode.as_deref() {
        Some("on") => LightsMode::On,
        Some("off") => LightsMode::Off,
        _ => LightsMode::Default,
    }
}

pub fn effect(c: &Config) -> Effect {
    match c.lights_effect.as_deref() {
        Some("breathe") => Effect::Breathe,
        Some("cycle") => Effect::Cycle,
        _ => Effect::Solid,
    }
}

pub fn brightness(c: &Config) -> u8 {
    c.lights_brightness.map_or(100, |b| b.round().clamp(0.0, 100.0) as u8)
}

pub fn hex(c: [u8; 3]) -> String {
    format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

pub fn parse_hex(s: &str) -> Option<[u8; 3]> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 || !s.is_ascii() {
        return None;
    }
    let b = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).ok();
    Some([b(0)?, b(2)?, b(4)?])
}

/// (left, right).
pub fn colors(c: &Config) -> ([u8; 3], [u8; 3]) {
    let p = |v: &Option<String>| v.as_deref().and_then(parse_hex).unwrap_or(DEFAULT_COLOR);
    (p(&c.lights_left), p(&c.lights_right))
}

/// Full saturation and value at hue `h` degrees (rare-fand's `hue`).
pub fn hue(h: f32) -> [u8; 3] {
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

/// The hue of a color in degrees (0 for greys).
pub fn hue_of(c: [u8; 3]) -> f32 {
    let [r, g, b] = c.map(|v| v as f32 / 255.0);
    let max = r.max(g).max(b);
    let d = max - r.min(g).min(b);
    if d <= f32::EPSILON {
        return 0.0;
    }
    let h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    h * 60.0
}

/// rare-fand's spec for this config.
pub fn spec(c: &Config) -> String {
    let m = match mode(c) {
        LightsMode::Default => "default",
        LightsMode::On => "on",
        LightsMode::Off => "off",
    };
    let e = match effect(c) {
        Effect::Solid => "solid",
        Effect::Breathe => "breathe",
        Effect::Cycle => "cycle",
    };
    let (l, r) = colors(c);
    format!("mode={m} effect={e} left={} right={} brightness={}", hex(l), hex(r), brightness(c))
}

// ------------------------------------------------------------------ rare-fand

/// 0 not known yet, 1 the device has stick lights, 2 it has none.
static AVAILABLE: AtomicU8 = AtomicU8::new(0);
static PENDING: Mutex<Option<String>> = Mutex::new(None);
static WAKE: Condvar = Condvar::new();
static LAST: Mutex<Option<String>> = Mutex::new(None);
static START: Once = Once::new();

/// None until rare-fand answered (Settings shows the rows meanwhile).
pub fn available() -> Option<bool> {
    match AVAILABLE.load(Ordering::Relaxed) {
        1 => Some(true),
        2 => Some(false),
        _ => None,
    }
}

/// Asks rare-fand; None when it isn't on the bus (yet).
fn probe() -> Option<bool> {
    let a = rare_system::system::lights_available()?;
    AVAILABLE.store(if a { 1 } else { 2 }, Ordering::Relaxed);
    Some(a)
}

fn worker() {
    // rare-fand may come up after the launcher at boot: a minute of tries, then "none here".
    let mut tries = 0;
    while probe().is_none() {
        tries += 1;
        if tries >= 12 {
            AVAILABLE.store(2, Ordering::Relaxed);
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    loop {
        let spec = {
            let mut g = PENDING.lock().unwrap_or_else(|e| e.into_inner());
            loop {
                if let Some(s) = g.take() {
                    break s;
                }
                g = WAKE.wait(g).unwrap_or_else(|e| e.into_inner());
            }
        };
        if AVAILABLE.load(Ordering::Relaxed) != 1 && probe() != Some(true) {
            *LAST.lock().unwrap_or_else(|e| e.into_inner()) = None;
            continue;
        }
        match rare_system::system::set_lights(&spec) {
            Ok(()) => rare_input::log(&format!("[lights] {spec}")),
            Err(e) => {
                rare_input::log(&format!("[lights] {spec}: {e}"));
                *LAST.lock().unwrap_or_else(|e| e.into_inner()) = None;
                crate::screens::settings::live::toast_pub(rare_i18n::tr().lights.failed, true);
            }
        }
    }
}

/// Sends the config's lights to rare-fand when they changed since the last send. At start (and
/// whenever Settings saves). Unset lights are never sent: rare-fand keeps what it has.
pub fn sync(c: &Config) {
    if !cfg!(target_os = "linux") {
        return;
    }
    START.call_once(|| {
        let _ = std::thread::Builder::new().name("lights".into()).spawn(worker);
    });
    if c.lights_mode.is_none() {
        return;
    }
    let s = spec(c);
    {
        let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
        if last.as_deref() == Some(s.as_str()) {
            return;
        }
        *last = Some(s.clone());
    }
    *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some(s);
    WAKE.notify_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_matches_rare_fand() {
        let mut c = Config::default();
        // Unset: the controller's own lights.
        assert_eq!(spec(&c), "mode=default effect=solid left=4da2ff right=4da2ff brightness=100");
        c.lights_mode = Some("on".into());
        c.lights_effect = Some("breathe".into());
        c.lights_left = Some("ff2a6d".into());
        c.lights_right = Some("0000ff".into());
        c.lights_brightness = Some(80.0);
        // The same line rare-fand's lights::tests round-trip.
        assert_eq!(spec(&c), "mode=on effect=breathe left=ff2a6d right=0000ff brightness=80");
        c.lights_left = Some("nope".into());
        assert_eq!(colors(&c).0, DEFAULT_COLOR);
    }

    #[test]
    fn hues() {
        for h in [0.0, 30.0, 120.0, 210.0, 300.0, 355.0] {
            assert!((hue_of(hue(h)) - h).abs() < 1.5, "{h}");
        }
        assert_eq!(hue_of([255, 255, 255]), 0.0);
        assert_eq!(hue(240.0), [0, 0, 255]);
    }
}
