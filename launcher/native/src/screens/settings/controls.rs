//! The Controls section (SystemSections.tsx ControlsSection): the SuiPlay's buttons, sticks, gyro,
//! rumble.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::screens::overlay::common::*;
use rare_render::Id;

/// What `native.inputStatus()` says.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PadStatus {
    pub available: bool,
    pub name: Option<String>,
    pub motion: bool,
}

pub const ACTIONS: [(&str, &str); 5] = [("capture", "Capture"), ("keyboard", "Keyboard"), ("perf", "Performance overlay"), ("shortcut", "Key shortcut"), ("none", "Nothing")];

/// LC or RC is set to Key shortcut in one of its rows (Settings or in games).
fn uses_shortcut(cfg: &Config, lc: bool) -> bool {
    let (a, g) = if lc { (&cfg.lc_action, &cfg.lc_game_action) } else { (&cfg.rc_action, &cfg.rc_game_action) };
    a.as_deref() == Some("shortcut") || g.as_deref() == Some("shortcut")
}

/// The button's saved shortcut, as people read it.
fn shortcut_label(cfg: &Config, lc: bool) -> Option<String> {
    let s = if lc { &cfg.lc_shortcut } else { &cfg.rc_shortcut };
    s.as_deref().and_then(rare_input::shortcut::Shortcut::parse).map(|s| s.label())
}

/// The Key shortcut row for LC or RC: the keys, and Change for the sheet (screens/settings/shortcut.rs).
fn shortcut_row(cfg: &Config, cx: &mut Cx, nav: &mut Nav, parent: Id, lc: bool) {
    let name = if lc { "LC" } else { "RC" };
    let body = match shortcut_label(cfg, lc) {
        Some(l) => format!("{l}. Held down for as long as you hold {name}, in Rare, games and Desktop mode."),
        None => "No keys picked yet.".to_string(),
    };
    let row = setting_row(cx, parent, &format!("{name} key shortcut"), Some(&body), false);
    let sid = if lc { "shortcut-lc" } else { "shortcut-rc" };
    let b = button(cx, row.right, "Change", Variant::Secondary, Sz::Md, false).sid(sid).hint("Change").tag(3);
    nav.add(b.group(10));
}

fn action_idx(v: &str) -> Option<usize> {
    ACTIONS.iter().position(|(k, _)| *k == v)
}

pub struct ControlsSection {
    rows: RowSet,
}

impl ControlsSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, now: f64) -> ControlsSection {
        let pad = match super::live::get() {
            Some(s) => s.pad,
            None => Some(PadStatus { available: true, name: Some("Rare Pad".into()), motion: true }),
        };
        ControlsSection::build_with(cx, nav, parent, cfg, pad.as_ref(), now)
    }

    pub fn build_with(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, pad: Option<&PadStatus>, _now: f64) -> ControlsSection {
        let r = cx.r();
        let col = cx.scene.parent(parent).map(|_| parent).unwrap_or(parent);
        let _ = r;
        let mut rows = RowSet::default();
        let labels: Vec<&str> = ACTIONS.iter().map(|(_, l)| *l).collect();
        let pct = |v: Option<f32>, d: f32| v.unwrap_or(d);

        let p1 = panel(cx, col);
        setting_row(cx, p1, "Buttons", Some("The SuiPlay's own buttons. Rare and Quick always do the same thing, so no game or app can take them."), true);
        setting_row(cx, p1, "Rare", Some("Press for the Rare menu, press again for Home, hold for the Switcher. Fixed."), false);
        setting_row(cx, p1, "Quick", Some("Quick Settings. Fixed."), false);
        rows.choice(cx, nav, p1, "btn-lc", "LC", Some("The small button at the top left. Press."), &labels, action_idx(cfg.lc_action.as_deref().unwrap_or("capture")), false);
        rows.choice(cx, nav, p1, "btn-rc", "RC", Some("The small button at the top right. Press."), &labels, action_idx(cfg.rc_action.as_deref().unwrap_or("keyboard")), false);
        for lc in [true, false] {
            if uses_shortcut(cfg, lc) {
                shortcut_row(cfg, cx, nav, p1, lc);
            }
        }
        layout_row(cx, nav, p1);
        setting_row(cx, p1, "Back paddles", Some("This device has none."), false);
        setting_row(cx, p1, "Holds", Some("Holding LC saves the last 30 seconds and holding RC changes the performance overlay, unless the button is set to Nothing or a key shortcut. A key shortcut holds its keys down while you hold the button, for push-to-talk apps like dictation. In games both do nothing until you pick something below, except the keyboard: a button set to Keyboard opens it over games and apps too. Inside Steam Game Mode Steam keeps Rare and Quick."), false);

        // In games: the standard map (features/gamebuttons.rs), and LC and RC there (nothing until picked).
        let pg = panel(cx, col);
        let row = setting_row(cx, pg, "In games", Some("Menu is Start, View is Select, A B X Y are the console's buttons by where they sit. Rare and Quick never reach the game. Every emulator follows it."), true);
        let b = button(cx, row.right, "Show", Variant::Secondary, Sz::Md, false).sid("game-buttons").hint("Show").tag(2);
        nav.add(b.group(10));
        rows.choice(cx, nav, pg, "btn-lc-game", "LC in games", Some("Press, while a game has the screen. A game's own page can pick its own."), &labels, action_idx(cfg.lc_game_action.as_deref().unwrap_or(crate::side::game_default(rare_input::side::Key::Lc, cfg.lc_action.as_deref()))), false);
        rows.choice(cx, nav, pg, "btn-rc-game", "RC in games", Some("Press, while a game or app has the screen. Keyboard types into it, Steam's sign-in too."), &labels, action_idx(cfg.rc_game_action.as_deref().unwrap_or(crate::side::game_default(rare_input::side::Key::Rc, cfg.rc_action.as_deref()))), false);

        // Desktop mode (features/desktop.rs): what the controller does there. Nothing to set.
        let pd = panel(cx, col);
        setting_row(cx, pd, "In Desktop mode", Some("The right stick moves the pointer and the left stick scrolls. A clicks, B right-clicks, Y opens the keyboard, the d-pad is the arrow keys. Rare opens the app menu. LC and RC do what you picked above."), true);
        setting_row(cx, pd, "Games in Desktop mode", Some("A game or app in full screen gets the controller as a normal controller. Press Quick to switch between the two yourself."), false);

        // Rows InputPlumber can't apply yet are left out (rare_input::ip::controls_support), so
        // nothing here saves a value that does nothing.
        let support = rare_input::ip::controls_support();
        let applies = |key: &str| support.iter().find(|c| c.key == key).is_none_or(|c| c.applies);
        let sticks = ["deadzoneLeft", "deadzoneRight", "invertLeftY", "invertRightY"];
        if sticks.iter().any(|k| applies(k)) {
            let p2 = panel(cx, col);
            let mut first = true;
            let mut lead = || std::mem::replace(&mut first, false);
            if applies("deadzoneLeft") {
                rows.slider(cx, nav, p2, "deadzone-left", "Left stick deadzone", None, pct(cfg.deadzone_left, 10.0), 0.0, 40.0, 1.0, "%", lead());
            }
            if applies("deadzoneRight") {
                rows.slider(cx, nav, p2, "deadzone-right", "Right stick deadzone", None, pct(cfg.deadzone_right, 10.0), 0.0, 40.0, 1.0, "%", lead());
            }
            if applies("invertLeftY") {
                rows.switch(cx, nav, p2, "invert-left", "Invert left stick, up and down", "Pushing up moves down.", cfg.invert_left_y.unwrap_or(false), lead());
            }
            if applies("invertRightY") {
                rows.switch(cx, nav, p2, "invert-right", "Invert right stick, up and down", "For looking around in games.", cfg.invert_right_y.unwrap_or(false), lead());
            }
        }

        let p3 = panel(cx, col);
        let gyro_body = match pad {
            None => "Motion controls.",
            Some(p) if p.available && p.motion => "On. The controller carries motion right now.",
            Some(p) if p.available => "Tilt the device to aim in games that support it.",
            Some(_) => "The controller service isn't running. Restart the device and try again.",
        };
        rows.switch(cx, nav, p3, "gyro-on", "Gyro", gyro_body, cfg.gyro_enabled.unwrap_or(true), true);
        if applies("gyroSensitivity") {
            rows.slider(cx, nav, p3, "gyro-sens", "Gyro sensitivity", None, pct(cfg.gyro_sensitivity, 100.0), 25.0, 200.0, 5.0, "%", false);
        }
        rows.switch(cx, nav, p3, "gyro", "Gyro in Steam games", "Motion controls in Steam games. The controller shows up as a Steam Deck while a Steam game runs. Set the sensitivity in the game's Steam controller settings.", cfg.gyro_in_steam.unwrap_or(false), false);

        let p4 = panel(cx, col);
        rows.switch(cx, nav, p4, "rumble", "Rumble", "A light buzz on the same moves, on controllers that support it.", cfg.rumble.unwrap_or(true), true);
        rows.slider(cx, nav, p4, "rumble-strength", "Rumble strength", None, pct(cfg.rumble_strength, 100.0), 0.0, 100.0, 10.0, "%", false);
        ControlsSection { rows }
    }
}

/// Button layout (rare_input::remap): what it is now, and Change for the sheet.
fn layout_row(cx: &mut Cx, nav: &mut Nav, parent: Id) {
    use rare_input::remap::{self, Status};
    let live = super::live::get().is_some();
    let layout = if live { remap::load() } else { remap::Layout::default() };
    let mut body = format!("{} Applies to all games and the launcher.", layout.summary());
    if live && !layout.is_default() {
        match remap::status() {
            Status::NeedsUpdate => body = format!("{} Takes effect after the next system update.", layout.summary()),
            Status::TooOld => body = format!("{} This version of Rare OS can't change buttons yet. An update brings it.", layout.summary()),
            _ => {}
        }
    }
    let row = setting_row(cx, parent, "Button layout", Some(&body), false);
    let b = button(cx, row.right, "Change", Variant::Secondary, Sz::Md, false).sid("remap").hint("Change").tag(1);
    nav.add(b.group(10));
}

impl SectionView for ControlsSection {
    fn press(&mut self, cx: &mut Cx, nav: &mut Nav, cfg: &mut Config, now: f64) -> SectionOut {
        if nav.current().is_some_and(|f| f.sid == "remap") {
            return SectionOut::Act("controls:remap-open".into());
        }
        if nav.current().is_some_and(|f| f.sid == "game-buttons") {
            return SectionOut::Act("open:game-buttons".into());
        }
        if let Some(f) = nav.current().filter(|f| f.sid == "shortcut-lc" || f.sid == "shortcut-rc") {
            let k = if f.sid == "shortcut-lc" { "lc" } else { "rc" };
            return SectionOut::Act(format!("controls:shortcut:{k}:"));
        }
        match self.rows.press(cx, nav, now) {
            Some((sid, Val::Bool(v))) => {
                match sid.as_str() {
                    "invert-left" => cfg.invert_left_y = Some(v),
                    "invert-right" => cfg.invert_right_y = Some(v),
                    "gyro-on" => cfg.gyro_enabled = Some(v),
                    "gyro" => cfg.gyro_in_steam = Some(v),
                    "rumble" => cfg.rumble = Some(v),
                    _ => return SectionOut::None,
                }
                SectionOut::Changed
            }
            Some((sid, Val::Idx(i))) => {
                let v = Some(ACTIONS[i].0.to_string());
                // Key shortcut with no keys picked yet: the sheet picks them and saves both.
                if ACTIONS[i].0 == "shortcut" {
                    let lc = sid.starts_with("btn-lc");
                    if shortcut_label(cfg, lc).is_none() {
                        return SectionOut::Act(format!("controls:shortcut:{}:{sid}", if lc { "lc" } else { "rc" }));
                    }
                }
                match sid.as_str() {
                    "btn-lc" => cfg.lc_action = v,
                    "btn-rc" => cfg.rc_action = v,
                    "btn-lc-game" => cfg.lc_game_action = v,
                    "btn-rc-game" => cfg.rc_game_action = v,
                    _ => return SectionOut::None,
                }
                SectionOut::Changed
            }
            _ => SectionOut::None,
        }
    }

    fn nudge(&mut self, cx: &mut Cx, nav: &mut Nav, cfg: &mut Config, dir: i32, _now: f64) -> SectionOut {
        match self.rows.nudge(cx, nav, dir) {
            Some((sid, Val::Num(v))) => {
                match sid.as_str() {
                    "deadzone-left" => cfg.deadzone_left = Some(v),
                    "deadzone-right" => cfg.deadzone_right = Some(v),
                    "gyro-sens" => cfg.gyro_sensitivity = Some(v),
                    "rumble-strength" => cfg.rumble_strength = Some(v),
                    _ => return SectionOut::None,
                }
                SectionOut::Changed
            }
            Some(_) => SectionOut::None,
            None if nav.current().is_some_and(|f| self.rows.is_slider(&f.sid)) => SectionOut::None,
            None => SectionOut::Pass,
        }
    }
}
