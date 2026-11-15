//! Keys from a keyboard for a sheet that records them (Settings > Controls > Key shortcut): while
//! it is on, the window's key events come here as Linux key codes instead of moving the focus
//! (main.rs), and the sheet takes them on its tick.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static ON: AtomicBool = AtomicBool::new(false);
static KEYS: Mutex<Vec<(u16, bool)>> = Mutex::new(Vec::new());

pub fn set(on: bool) {
    ON.store(on, Ordering::Relaxed);
    if let Ok(mut k) = KEYS.lock() {
        k.clear();
    }
}

pub fn active() -> bool {
    ON.load(Ordering::Relaxed)
}

/// A key went down or up (`code` is the Linux key code).
pub fn push(code: u16, down: bool) {
    if !active() {
        return;
    }
    if let Ok(mut k) = KEYS.lock() {
        if k.len() < 64 {
            k.push((code, down));
        }
    }
}

pub fn take() -> Vec<(u16, bool)> {
    KEYS.lock().map(|mut k| std::mem::take(&mut *k)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_pass_only_while_on() {
        push(29, true);
        assert!(take().is_empty());
        set(true);
        push(29, true);
        push(57, true);
        assert_eq!(take(), vec![(29, true), (57, true)]);
        set(false);
        push(57, false);
        assert!(take().is_empty());
    }
}
