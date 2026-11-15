//! The SuiPlay's LC and RC buttons and the capture chords, in the native launcher (the Tauri
//! shell's sysbuttons.rs and share.rs): what a press or a hold means comes from rare-input's
//! `side` module and Settings > Controls (`lcAction`, `rcAction` in launcher.json, read at the
//! moment of the press so a change applies at once); this module runs the action.
//!
//! Captures run on their own threads. Anything that needs the app (a toast, the keyboard) is
//! queued here and taken by `App::update` through [`take`].

use rare_i18n::{tr, trf};
use rare_input::side::{Action, Fire, HoldAction, Key};
use rare_share::ShareHost;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// What a button asks of the app.
#[derive(Clone, Debug, PartialEq)]
pub enum Req {
    /// The on-screen keyboard (RC's default press).
    Keyboard,
    /// The keyboard over the app in front for a web address (the browser's address bar, a new
    /// tab): opens it, or switches it to web address keys; never closes it.
    KeyboardUrl,
    /// A message for the toasts: (text, is an error).
    Toast(String, bool),
}

static QUEUE: Mutex<Vec<Req>> = Mutex::new(Vec::new());
static WAKER: OnceLock<Arc<dyn Fn() + Send + Sync>> = OnceLock::new();
static RUNNING: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
static FRONT: AtomicBool = AtomicBool::new(true);

pub fn set_waker(w: Arc<dyn Fn() + Send + Sync>) {
    let _ = WAKER.set(w);
}

fn push(r: Req) {
    if let Ok(mut q) = QUEUE.lock() {
        if q.len() < 32 {
            q.push(r);
        }
    }
    if let Some(w) = WAKER.get() {
        w();
    }
}

/// A message for the toasts from a worker thread.
pub fn toast(text: &str, err: bool) {
    push(Req::Toast(text.to_string(), err));
}

/// The games running now (id, title), as the capture tags and Discord read them.
pub fn running_titles() -> Vec<(String, String)> {
    RUNNING.lock().map(|r| r.clone()).unwrap_or_default()
}

/// What the app queued up for itself since the last call.
pub fn take() -> Vec<Req> {
    QUEUE.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default()
}

/// The games running now (id, title) and whether the launcher is the window in front, for the
/// capture's game tag and where its message shows.
pub fn set_running(running: Vec<(String, String)>) {
    FRONT.store(running.is_empty(), Ordering::Relaxed);
    if let Ok(mut r) = RUNNING.lock() {
        *r = running;
    }
}

fn config() -> crate::screens::settings::config::Config {
    crate::screens::settings::config::load()
}

/// Rumble strength, 0 to 1, from Settings > Controls (off: 0).
pub fn rumble_strength() -> f32 {
    let c = config();
    if c.rumble == Some(false) {
        0.0
    } else {
        (c.rumble_strength.unwrap_or(100.0) / 100.0).clamp(0.0, 1.0)
    }
}

struct NativeShare;

impl ShareHost for NativeShare {
    fn clips_enabled(&self) -> bool {
        crate::features::capture::replay_on()
    }
    fn replay_seconds(&self) -> u32 {
        crate::features::capture::replay_seconds()
    }
    fn clips_cap_bytes(&self) -> Option<u64> {
        Some((crate::features::capture::cap_gb() as u64) << 30)
    }
    fn running(&self) -> Vec<String> {
        RUNNING.lock().map(|r| r.iter().map(|(id, _)| id.clone()).collect()).unwrap_or_default()
    }
    fn title_of(&self, id: &str) -> Option<String> {
        RUNNING.lock().ok().and_then(|r| r.iter().find(|(i, _)| i == id).map(|(_, t)| t.clone()))
    }
    fn rumble_enabled(&self) -> bool {
        config().rumble != Some(false)
    }
    fn rumble(&self) {
        let _ = rare_input::rumble::play("capture", rumble_strength());
    }
    fn launcher_front(&self) -> bool {
        FRONT.load(Ordering::Relaxed)
    }
    fn toast(&self, text: &str) {
        push(Req::Toast(text.to_string(), false));
    }
}

/// The host the Share screen and the buttons give rare-share.
pub fn share_host() -> Arc<dyn ShareHost> {
    Arc::new(NativeShare)
}

/// Starts the clip recorder supervisor (it records only while a game runs and instant replay is
/// on), Discord, the Sunshine host and the network announcement.
pub fn start_share() {
    // The shortcut keyboard is made before the first press, so the compositor knows it by then.
    if uses_shortcut() {
        std::thread::spawn(rare_launch::apps::pointer::prepare_keys);
    }
    rare_share::set_host_env(|cmd| {
        rare_system::host::env(cmd);
    });
    rare_share::start(share_host());
    if let Some(w) = WAKER.get() {
        crate::features::capture::start(w.clone());
    }
}

/// Takes a screenshot or saves a clip on its own thread; the result is a toast.
pub fn capture(clip: bool) {
    std::thread::spawn(move || {
        let host = NativeShare;
        let r = if clip { rare_share::save_clip(&host) } else { rare_share::take_screenshot(&host) };
        match r {
            Ok(cap) => {
                if host.rumble_enabled() {
                    host.rumble();
                }
                push(Req::Toast(match cap.game.as_ref().map(|g| g.name.trim()).filter(|n| !n.is_empty()) {
                    Some(n) => trf!(if clip { tr().app.clip_saved_game } else { tr().app.screenshot_saved_game }, game = n),
                    None => (if clip { tr().app.clip_saved } else { tr().app.screenshot_saved }).to_string(),
                }, false));
            }
            Err(e) => push(Req::Toast(e, true)),
        }
    });
}

/// Steps the performance overlay to its next level and saves it, as quick settings does.
pub fn perf_next() {
    use rare_system::perfhud::{self, HudLevel};
    use rare_system::power::PowerSettings;
    std::thread::spawn(|| {
        let path = rare_core::paths::config_file();
        let mut v: serde_json::Value = std::fs::read(&path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_else(|| serde_json::json!({}));
        let cur: PowerSettings = serde_json::from_value(v.clone()).unwrap_or_default();
        let next = perfhud::next(cur.perf_hud.unwrap_or(HudLevel::Off));
        if let (Some(obj), Ok(level)) = (v.as_object_mut(), serde_json::to_value(next)) {
            obj.insert("perfHud".into(), level);
            let tmp = path.with_extension("json.tmp");
            if std::fs::write(&tmp, serde_json::to_vec_pretty(&v).unwrap_or_default()).is_ok() {
                let _ = std::fs::rename(tmp, &path);
            }
        }
        perfhud::apply(next);
        let name = match next {
            HudLevel::Off => tr().common.off,
            HudLevel::Fps => tr().app.hud_fps,
            HudLevel::Basic => tr().app.hud_basic,
            HudLevel::Full => tr().app.hud_full,
        };
        push(Req::Toast(trf!(tr().app.perf_overlay, level = name), false));
    });
}

/// What LC or RC in games does until it is picked: the keyboard when the button is the keyboard
/// in the launcher (RC unless changed), else nothing. The keyboard over a game takes nothing from
/// the game (LC and RC never reach it), and it is how a game or Steam's sign-in gets typed into.
pub fn game_default(k: Key, launcher: Option<&str>) -> &'static str {
    match rare_input::side::action(k, launcher) {
        Action::Keyboard => "keyboard",
        _ => "none",
    }
}

/// LC's or RC's action: the running game's own (its page's Controls), else Settings > Controls.
/// While a game has the screen it is Settings > Controls > LC / RC in games, which until picked is
/// `game_default` (the standard in-game map, rare_emu::controls::STANDARD).
fn side_setting(k: Key) -> Option<String> {
    let running: Vec<String> = RUNNING.lock().map(|r| r.iter().map(|(id, _)| id.clone()).collect()).unwrap_or_default();
    if let Some(a) = crate::data::gameset::side_action(&running, k == Key::Lc) {
        return Some(a);
    }
    let c = config();
    let in_game = !running.is_empty() && !rare_launch::apps::front::launcher_front();
    match (k, in_game) {
        (Key::Lc, true) => Some(c.lc_game_action.unwrap_or_else(|| game_default(Key::Lc, c.lc_action.as_deref()).into())),
        (Key::Rc, true) => Some(c.rc_game_action.unwrap_or_else(|| game_default(Key::Rc, c.rc_action.as_deref()).into())),
        (Key::Lc, false) => c.lc_action,
        (Key::Rc, false) => c.rc_action,
    }
}

/// What a press of LC or RC does with the config as it is now.
pub fn tap_action(k: Key) -> Action {
    rare_input::side::action(k, side_setting(k).as_deref())
}

/// The keys LC's or RC's Key shortcut holds (Settings > Controls), if one is picked.
pub fn shortcut_of(k: Key) -> Option<rare_input::shortcut::Shortcut> {
    let c = config();
    let saved = match k {
        Key::Lc => c.lc_shortcut,
        Key::Rc => c.rc_shortcut,
    };
    saved.as_deref().and_then(rare_input::shortcut::Shortcut::parse)
}

/// Some setting (launcher, in games, a game's own) sets LC or RC to a key shortcut.
pub fn uses_shortcut() -> bool {
    let c = config();
    [&c.lc_action, &c.rc_action, &c.lc_game_action, &c.rc_game_action].iter().any(|a| a.as_deref() == Some("shortcut"))
}

/// The keys LC and RC hold down right now: let go on the button's release, whatever the setting
/// or the window in front is by then.
static HELD: Mutex<[Option<Vec<u16>>; 2]> = Mutex::new([None, None]);

fn held_idx(k: Key) -> usize {
    match k {
        Key::Lc => 0,
        Key::Rc => 1,
    }
}

/// What holding LC or RC does, if anything.
pub fn hold_action(k: Key) -> Option<HoldAction> {
    rare_input::side::hold_action(k, side_setting(k).as_deref())
}

fn tap(k: Key) {
    match tap_action(k) {
        Action::Capture => capture(false),
        Action::Keyboard => push(Req::Keyboard),
        Action::Perf => perf_next(),
        // Held in `press`, never a tap.
        Action::Nothing | Action::Shortcut => {}
    }
}

fn hold(k: Key) {
    // RC held with Ask Rare's "Hold RC to talk" on: Rare listens until RC goes up.
    if k == Key::Rc && crate::features::agent::rc_hold() {
        return;
    }
    match hold_action(k) {
        Some(HoldAction::Clip) => capture(true),
        Some(HoldAction::Perf) => perf_next(),
        None => {}
    }
}

/// LC or RC went down or up (InputPlumber's `ui_screenshot` / `ui_osk` actions).
pub fn press(k: Key, down: bool) {
    if k == Key::Rc && !down {
        crate::features::agent::rc_released();
    }
    // A key shortcut: down with the button, up with it (push-to-talk), no tap and no hold.
    let i = held_idx(k);
    if down {
        if tap_action(k) == Action::Shortcut {
            match shortcut_of(k) {
                Some(s) => {
                    let keys = s.keys().to_vec();
                    rare_launch::apps::pointer::hold_keys(&keys, true);
                    if let Ok(mut h) = HELD.lock() {
                        h[i] = Some(keys);
                    }
                }
                None => toast(&trf!(tr().app.pick_keys, path = format!("{} > {}", tr().common.settings, tr().settings.sec_controls)), false),
            }
            return;
        }
    } else if let Some(keys) = HELD.lock().ok().and_then(|mut h| h[i].take()) {
        rare_launch::apps::pointer::hold_keys(&keys, false);
        return;
    }
    rare_input::side::press(
        k,
        down,
        Arc::new(|k, f| match f {
            Fire::Tap => tap(k),
            Fire::Hold => hold(k),
        }),
    );
}

/// A chord from the pad: Rare (or Guide) + Y or View captures, + X the keyboard.
pub fn chord(s: rare_input::chords::Share) {
    match s {
        rare_input::chords::Share::Keyboard => keyboard(),
        s => capture(matches!(s, rare_input::chords::Share::Clip)),
    }
}

/// The keyboard: over the game or app in front (features/typekb.rs), else the launcher's own.
pub fn keyboard() {
    push(Req::Keyboard);
}

/// Y (`url` false), or View and Menu in the browser (`url`), in an app the pad drives as a mouse.
pub fn app_keyboard(url: bool) {
    push(if url { Req::KeyboardUrl } else { Req::Keyboard });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_queue_hands_over_once() {
        push(Req::Keyboard);
        let a = take();
        assert!(a.contains(&Req::Keyboard));
        assert!(take().is_empty());
    }

    #[test]
    fn keyboard_in_games_follows_the_launcher_until_picked() {
        // RC is the keyboard unless changed: in games too.
        assert_eq!(game_default(Key::Rc, None), "keyboard");
        assert_eq!(game_default(Key::Rc, Some("keyboard")), "keyboard");
        assert_eq!(game_default(Key::Rc, Some("capture")), "none");
        // LC captures unless changed: nothing in games; set to Keyboard, the keyboard there too.
        assert_eq!(game_default(Key::Lc, None), "none");
        assert_eq!(game_default(Key::Lc, Some("keyboard")), "keyboard");
    }
}
