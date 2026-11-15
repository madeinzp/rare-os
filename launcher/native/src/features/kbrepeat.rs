//! Held keys on Rare's keyboards: a held X deletes again and again, a held A (or a finger held on
//! a key) types that key again, the way a held direction moves. rare-input repeats only
//! directions, for every screen; this repeats X and A only while a keyboard is up and only on keys
//! that type (kblayout `repeats`), so a held A never flips Shift or finishes twice.
//!
//! The pad thread reports presses and releases (`down`, `up`); a timer thread sends the repeats
//! back through the same path as a press with `repeat` set (`start`'s `fire`). Any other press, the
//! keyboard closing, or `LIMIT` ends a repeat, so a release that never arrives (InputPlumber drops
//! some across a mode change) cannot type forever.

use rare_input::Pad;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

/// First repeat, as a phone's delete key.
pub const DELAY: Duration = Duration::from_millis(420);
/// Then this often (about 14 a second).
pub const EVERY: Duration = Duration::from_millis(70);
/// A hold this long stops repeating on its own.
pub const LIMIT: Duration = Duration::from_secs(8);

/// The launcher's own keyboard is up, and whether A on its focused key repeats (screens/osk.rs
/// keeps these current; the pad thread can't see the app's state).
pub static LAUNCHER_OPEN: AtomicBool = AtomicBool::new(false);
pub static LAUNCHER_A: AtomicBool = AtomicBool::new(false);

/// A hold: the button, when it started, when the next repeat is due.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hold {
    pub pad: Pad,
    pub since: Instant,
    pub due: Instant,
}

/// The pure part: what a press, a release and the clock do to the hold.
#[derive(Default, Debug)]
pub struct Timer {
    pub hold: Option<Hold>,
}

impl Timer {
    /// A fresh press. `wanted`: a keyboard is up and `p` repeats on it.
    pub fn down(&mut self, p: Pad, wanted: bool, now: Instant) {
        self.hold = wanted.then_some(Hold {
            pad: p,
            since: now,
            due: now + DELAY,
        });
    }

    pub fn up(&mut self, p: Pad) {
        if self.hold.is_some_and(|h| h.pad == p) {
            self.hold = None;
        }
    }

    /// The repeat due at `now`, if any (and when the one after is due).
    pub fn tick(&mut self, now: Instant) -> Option<Pad> {
        let h = self.hold.as_mut()?;
        if now.duration_since(h.since) >= LIMIT {
            self.hold = None;
            return None;
        }
        if now < h.due {
            return None;
        }
        h.due = now + EVERY;
        Some(h.pad)
    }

    pub fn next_due(&self) -> Option<Instant> {
        self.hold.map(|h| h.due)
    }
}

struct Shared {
    timer: Mutex<Timer>,
    wake: Condvar,
}

fn shared() -> &'static Shared {
    static S: OnceLock<Shared> = OnceLock::new();
    S.get_or_init(|| Shared {
        timer: Mutex::new(Timer::default()),
        wake: Condvar::new(),
    })
}

fn timer() -> std::sync::MutexGuard<'static, Timer> {
    shared().timer.lock().unwrap_or_else(|e| e.into_inner())
}

/// Starts the timer thread (once). `fire` sends a repeat of the held button as a press with
/// `repeat` set: to the keyboard over an app, or into the launcher's input queue.
pub fn start(fire: impl Fn(Pad) + Send + 'static) {
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("rare-kbrepeat".into())
        .spawn(move || {
            let s = shared();
            let mut t = timer();
            loop {
                let now = Instant::now();
                if let Some(p) = t.tick(now) {
                    // Still up, and the key still repeats (focus may have moved under a held A).
                    if keyboard_up() && wants(p) {
                        drop(t);
                        fire(p);
                        t = timer();
                    } else {
                        t.hold = None;
                    }
                    continue;
                }
                t = match t.next_due() {
                    Some(due) => {
                        s.wake
                            .wait_timeout(t, due.saturating_duration_since(now))
                            .unwrap_or_else(|e| e.into_inner())
                            .0
                    }
                    None => s.wake.wait(t).unwrap_or_else(|e| e.into_inner()),
                };
            }
        });
}

fn keyboard_up() -> bool {
    super::typekb::is_open() || LAUNCHER_OPEN.load(Ordering::Relaxed)
}

/// Whether a held `p` repeats on the keyboard that is up.
fn wants(p: Pad) -> bool {
    if super::typekb::is_open() {
        return super::typekb::repeats(p);
    }
    LAUNCHER_OPEN.load(Ordering::Relaxed)
        && (p == Pad::X || (p == Pad::A && LAUNCHER_A.load(Ordering::Relaxed)))
}

/// A fresh press from the pad thread (after the keyboard saw it). Any press ends the hold before.
pub fn down(p: Pad) {
    let wanted = matches!(p, Pad::A | Pad::X) && keyboard_up() && wants(p);
    timer().down(p, wanted, Instant::now());
    shared().wake.notify_all();
}

/// A finger held on a key of the launcher's keyboard: A repeats until it lifts (`up(Pad::A)`).
pub fn touch_hold() {
    timer().down(
        Pad::A,
        LAUNCHER_OPEN.load(Ordering::Relaxed) && LAUNCHER_A.load(Ordering::Relaxed),
        Instant::now(),
    );
    shared().wake.notify_all();
}

/// A release from the pad thread (or the finger lifting).
pub fn up(p: Pad) {
    timer().up(p);
}

/// The keyboard went: nothing repeats.
pub fn stop() {
    timer().hold = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeats_after_the_delay_until_released() {
        let t0 = Instant::now();
        let mut t = Timer::default();
        t.down(Pad::X, true, t0);
        assert_eq!(t.tick(t0 + DELAY - Duration::from_millis(1)), None);
        assert_eq!(t.tick(t0 + DELAY), Some(Pad::X));
        assert_eq!(t.tick(t0 + DELAY + Duration::from_millis(10)), None);
        assert_eq!(t.tick(t0 + DELAY + EVERY), Some(Pad::X));
        // Another button's release changes nothing; its own ends it.
        t.up(Pad::A);
        assert!(t.hold.is_some());
        t.up(Pad::X);
        assert_eq!(t.tick(t0 + DELAY + EVERY * 3), None);
    }

    #[test]
    fn unwanted_or_another_press_or_too_long_ends_it() {
        let t0 = Instant::now();
        let mut t = Timer::default();
        // A on Shift: no repeat.
        t.down(Pad::A, false, t0);
        assert_eq!(t.tick(t0 + DELAY * 4), None);
        // A press of something else ends a hold.
        t.down(Pad::X, true, t0);
        t.down(Pad::Right, false, t0 + Duration::from_millis(100));
        assert_eq!(t.tick(t0 + DELAY * 2), None);
        // A lost release: it stops by itself.
        t.down(Pad::X, true, t0);
        let mut n = 0;
        let mut now = t0;
        while now < t0 + LIMIT + Duration::from_secs(2) {
            if t.tick(now).is_some() {
                n += 1;
            }
            now += Duration::from_millis(10);
        }
        assert!(t.hold.is_none());
        let most = ((LIMIT - DELAY).as_millis() / EVERY.as_millis()) as usize + 2;
        assert!(n > 50 && n <= most, "{n}");
    }
}
