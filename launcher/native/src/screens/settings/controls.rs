//! The Controls section (SystemSections.tsx ControlsSection): the SuiPlay's buttons, sticks, gyro,
//! rumble.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::screens::overlay::common::*;
use rare_i18n::{tr, trf};
use rare_render::Id;

/// What `native.inputStatus()` says.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PadStatus {
    pub available: bool,
    pub name: Option<String>,
    pub motion: bool,
}

/// What LC and RC can do: (id, English label). Show `actions()`' labels, which follow the language.
pub const ACTIONS: [(&str, &str); 5] = [("capture", "Capture"), ("keyboard", "Keyboard"), ("perf", "Performance overlay"), ("shortcut", "Key shortcut"), ("none", "Nothing")];

/// `ACTIONS` with the labels in the current language.
pub fn actions() -> [(&'static str, &'static str); 5] {
    let t = &tr().settings;
    [("capture", t.act_capture), ("keyboard", t.act_keyboard), ("perf", t.act_perf), ("shortcut", t.act_shortcut), ("none", t.act_nothing)]
}

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
    let t = &tr().settings;
    let body = match shortcut_label(cfg, lc) {
        Some(l) => trf!(t.shortcut_held, keys = l, button = name),
        None => t.shortcut_none.to_string(),
    };
    let row = setting_row(cx, parent, &trf!(t.shortcut_title, button = name), Some(&body), false);
    let sid = if lc { "shortcut-lc" } else { "shortcut-rc" };
    let b = button(cx, row.right, tr().common.change, Variant::Secondary, Sz::Md, false).sid(sid).hint(tr().common.change).tag(3);
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
        let t = &tr().settings;
        let labels: Vec<&str> = actions().iter().map(|(_, l)| *l).collect();
        let pct = |v: Option<f32>, d: f32| v.unwrap_or(d);

        let p1 = panel(cx, col);
        setting_row(cx, p1, t.row_buttons, Some(t.buttons_body), true);
        setting_row(cx, p1, t.row_rare_btn, Some(t.rare_btn_body), false);
        setting_row(cx, p1, t.row_quick_btn, Some(t.quick_btn_body), false);
        rows.choice(cx, nav, p1, "btn-lc", "LC", Some(t.lc_body), &labels, action_idx(cfg.lc_action.as_deref().unwrap_or("capture")), false);
        rows.choice(cx, nav, p1, "btn-rc", "RC", Some(t.rc_body), &labels, action_idx(cfg.rc_action.as_deref().unwrap_or("keyboard")), false);
        for lc in [true, false] {
            if uses_shortcut(cfg, lc) {
                shortcut_row(cfg, cx, nav, p1, lc);
            }
        }
        layout_row(cx, nav, p1);
        setting_row(cx, p1, t.row_paddles, Some(t.paddles_body), false);
        setting_row(cx, p1, t.row_holds, Some(t.holds_body), false);

        // In games: the standard map (features/gamebuttons.rs), and LC and RC there (nothing until picked).
        let pg = panel(cx, col);
        let row = setting_row(cx, pg, t.row_in_games, Some(t.in_games_body), true);
        let b = button(cx, row.right, t.show, Variant::Secondary, Sz::Md, false).sid("game-buttons").hint(t.show).tag(2);
        nav.add(b.group(10));
        rows.choice(cx, nav, pg, "btn-lc-game", t.row_lc_game, Some(t.lc_game_body), &labels, action_idx(cfg.lc_game_action.as_deref().unwrap_or(crate::side::game_default(rare_input::side::Key::Lc, cfg.lc_action.as_deref()))), false);
        rows.choice(cx, nav, pg, "btn-rc-game", t.row_rc_game, Some(t.rc_game_body), &labels, action_idx(cfg.rc_game_action.as_deref().unwrap_or(crate::side::game_default(rare_input::side::Key::Rc, cfg.rc_action.as_deref()))), false);

        // Desktop mode (features/desktop.rs): what the controller does there. Nothing to set.
        let pd = panel(cx, col);
        setting_row(cx, pd, t.row_desktop, Some(t.desktop_body), true);
        setting_row(cx, pd, t.row_games_desktop, Some(t.games_desktop_body), false);

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
                rows.slider(cx, nav, p2, "deadzone-left", t.row_deadzone_left, None, pct(cfg.deadzone_left, 10.0), 0.0, 40.0, 1.0, "%", lead());
            }
            if applies("deadzoneRight") {
                rows.slider(cx, nav, p2, "deadzone-right", t.row_deadzone_right, None, pct(cfg.deadzone_right, 10.0), 0.0, 40.0, 1.0, "%", lead());
            }
            if applies("invertLeftY") {
                rows.switch(cx, nav, p2, "invert-left", t.row_invert_left, t.invert_left_body, cfg.invert_left_y.unwrap_or(false), lead());
            }
            if applies("invertRightY") {
                rows.switch(cx, nav, p2, "invert-right", t.row_invert_right, t.invert_right_body, cfg.invert_right_y.unwrap_or(false), lead());
            }
        }

        let p3 = panel(cx, col);
        let gyro_body = match pad {
            None => t.gyro_none,
            Some(p) if p.available && p.motion => t.gyro_active,
            Some(p) if p.available => t.gyro_tilt,
            Some(_) => t.gyro_down,
        };
        rows.switch(cx, nav, p3, "gyro-on", t.row_gyro, gyro_body, cfg.gyro_enabled.unwrap_or(true), true);
        if applies("gyroSensitivity") {
            rows.slider(cx, nav, p3, "gyro-sens", t.row_gyro_sens, None, pct(cfg.gyro_sensitivity, 100.0), 25.0, 200.0, 5.0, "%", false);
        }
        rows.switch(cx, nav, p3, "gyro", t.row_gyro_steam, t.gyro_steam_body, cfg.gyro_in_steam.unwrap_or(false), false);

        let p4 = panel(cx, col);
        rows.switch(cx, nav, p4, "rumble", t.row_rumble, t.rumble_body, cfg.rumble.unwrap_or(true), true);
        rows.slider(cx, nav, p4, "rumble-strength", t.row_rumble_strength, None, pct(cfg.rumble_strength, 100.0), 0.0, 100.0, 10.0, "%", false);
        ControlsSection { rows }
    }
}

/// Button layout (rare_input::remap): what it is now, and Change for the sheet.
fn layout_row(cx: &mut Cx, nav: &mut Nav, parent: Id) {
    use rare_input::remap::{self, Status};
    let live = super::live::get().is_some();
    let layout = if live { remap::load() } else { remap::Layout::default() };
    let t = &tr().settings;
    let mut body = trf!(t.layout_applies, summary = layout.summary());
    if live && !layout.is_default() {
        match remap::status() {
            Status::NeedsUpdate => body = trf!(t.layout_after_update, summary = layout.summary()),
            Status::TooOld => body = trf!(t.layout_too_old, summary = layout.summary()),
            _ => {}
        }
    }
    let row = setting_row(cx, parent, t.row_layout, Some(&body), false);
    let b = button(cx, row.right, tr().common.change, Variant::Secondary, Sz::Md, false).sid("remap").hint(tr().common.change).tag(1);
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
