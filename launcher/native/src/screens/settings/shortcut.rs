//! Settings > Controls > LC or RC key shortcut: which keys the button holds down while it is held
//! (rare_input::shortcut). A sheet over Settings. The keys can be pressed on a keyboard (any
//! keyboard the launcher's window gets keys from, features/keycapture.rs), or picked with the
//! pad: modifiers toggle, one key ends it. Save writes the shortcut and sets the button to it.

use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_input::shortcut::{self, Shortcut, MODIFIERS};
use rare_render::input::Pad;
use rare_render::theme::tok;
use rare_render::{Id, Node};

const HINT: &str = "Press the keys on a keyboard, or pick them below.";

const T_SAVE: u32 = 100;
const T_CANCEL: u32 = 101;
/// A modifier chip: 200 + its index in MODIFIERS.
const T_MOD: u32 = 200;
/// No key, modifiers only.
const T_NOKEY: u32 = 299;
/// A key chip: 300 + its index in shortcut::KEYS.
const T_KEY: u32 = 300;

pub struct ShortcutSheet {
    nav: Nav,
    body: Id,
    footer: Id,
    shown: Id,
    lc: bool,
    /// The Controls row whose choice opened the sheet (`btn-lc`, `btn-rc-game`...): Save also sets
    /// that row to Key shortcut. None from the Change button.
    row: Option<String>,
    mods: Vec<u16>,
    key: Option<u16>,
    /// Keys held on a keyboard now, and every key pressed since none were held.
    held: Vec<u16>,
    chord: Vec<u16>,
    live: bool,
    message: Option<String>,
}

impl ShortcutSheet {
    pub fn open(cx: &mut Cx, lc: bool, row: Option<String>, saved: Option<Shortcut>, live: bool, now: f64) -> ShortcutSheet {
        let r = cx.r();
        let root = cx.scene.root;
        let name = if lc { "LC" } else { "RC" };
        let sub = format!("{name} holds these keys down for as long as you hold it. Dictation apps and other push-to-talk apps listen for a key like this.");
        let sh = sheet(cx, root, &format!("{name} key shortcut"), Some(&sub), true, true, now);
        let top = cx.scene.add(sh.body, s().col().gap(0.25 * r).mb(1.0 * r).done(), Node::default());
        let shown = cx.scene.add_text(cx.t, top, s().done(), "", ts(r, ty::ROW.0, ty::ROW.1, 600), tok::FG);
        cx.scene.add_text(cx.t, top, s().done(), HINT, ts(r, ty::SMALL.0, ty::SMALL.1, 400).wrap(), tok::MUTED);
        let body = cx.scene.add(sh.body, s().col().wp(1.0).gap(0.75 * r).done(), Node::default());
        let (mods, key) = saved.map(|s| (s.mods(), s.key())).unwrap_or_else(|| (Vec::new(), None));
        let mut me = ShortcutSheet { nav: Nav::default(), body, footer: sh.footer.unwrap(), shown, lc, row, mods, key, held: Vec::new(), chord: Vec::new(), live, message: None };
        if live {
            crate::features::keycapture::set(true);
        }
        me.draw(cx, now, None);
        me
    }

    fn current(&self) -> Option<Shortcut> {
        let mut codes = self.mods.clone();
        codes.extend(self.key);
        Shortcut::new(&codes)
    }

    fn show(&mut self, cx: &mut Cx) {
        let t = match self.current() {
            Some(s) => s.label(),
            None => "No keys picked".into(),
        };
        cx.scene.set_text(cx.t, self.shown, &t);
    }

    fn chip(&mut self, cx: &mut Cx, parent: Id, label: &str, on: bool, tag: u32, hint: &str) {
        let r = cx.r();
        let (base, fg) = if on { (tok::FG, tok::BG) } else { (tok::L2, tok::FG) };
        let chip = cx.scene.add(parent, s().row().items_center().justify_center().h(2.4 * r).min_w(4.0 * r).px(0.9 * r).done(), Node::bg(base).radius(0.25 * r));
        let tid = cx.scene.add_text(cx.t, chip, s().done(), label, ts(r, ty::SMALL.0, ty::SMALL.1, 500), fg);
        self.nav.add(Focusable { id: chip, kind: Kind::Btn { base, fg, text: tid }, group: 1, hint: hint.into(), tag, sid: String::new() });
    }

    /// Builds the body and footer and focuses the item tagged `focus` (else the picked key).
    fn draw(&mut self, cx: &mut Cx, now: f64, focus: Option<u32>) {
        let r = cx.r();
        self.nav.clear();
        cx.scene.clear_children(self.body);
        cx.scene.clear_children(self.footer);
        self.show(cx);

        text(cx, self.body, ty::SMALL, 600, tok::MUTED, "Hold with");
        let row = cx.scene.add(self.body, s().row().wrap().gap(0.5 * r).wp(1.0).done(), Node::default());
        for (i, m) in MODIFIERS.iter().enumerate() {
            let on = self.mods.contains(m);
            self.chip(cx, row, shortcut::label_of(*m).unwrap_or("?"), on, T_MOD + i as u32, if on { "Remove" } else { "Add" });
        }

        text(cx, self.body, ty::SMALL, 600, tok::MUTED, "Key");
        let grid = cx.scene.add(self.body, s().row().wrap().gap(0.5 * r).wp(1.0).done(), Node::default());
        let none_on = self.key.is_none() && !self.mods.is_empty();
        self.chip(cx, grid, "None", none_on, T_NOKEY, "Pick");
        for (i, (code, _, label)) in shortcut::KEYS.iter().enumerate() {
            if MODIFIERS.contains(code) {
                continue;
            }
            self.chip(cx, grid, label, self.key == Some(*code), T_KEY + i as u32, "Pick");
        }

        let can_save = self.current().is_some();
        self.nav.add(button(cx, self.footer, "Cancel", Variant::Secondary, Sz::Md, false).hint("Cancel").tag(T_CANCEL).group(2));
        self.nav.add(button(cx, self.footer, "Save", if can_save { Variant::Primary } else { Variant::Dimmed }, Sz::Md, false).hint("Save").tag(T_SAVE).group(2));

        let key_tag = self.key.and_then(|k| shortcut::KEYS.iter().position(|x| x.0 == k)).map(|i| T_KEY + i as u32);
        let want = focus.or(key_tag);
        let target = want.and_then(|t| self.nav.items.iter().find(|f| f.tag == t)).or_else(|| self.nav.items.first()).map(|f| f.id);
        if let Some(id) = target {
            self.nav.focus(cx.scene, id, now, false);
            self.nav.reveal(cx.scene, id, now);
        }
    }

    /// Keys from a keyboard: every key pressed until all are let go makes the shortcut.
    fn keys_in(&mut self, cx: &mut Cx, now: f64) -> bool {
        let evs = crate::features::keycapture::take();
        if evs.is_empty() {
            return false;
        }
        let mut changed = false;
        for (code, down) in evs {
            if down {
                if !self.held.contains(&code) {
                    self.held.push(code);
                }
                if !self.chord.contains(&code) {
                    self.chord.push(code);
                }
                // Live: what is held so far.
                if let Some(s) = Shortcut::new(&self.chord) {
                    self.mods = s.mods();
                    self.key = s.key();
                    changed = true;
                }
            } else {
                self.held.retain(|c| *c != code);
                if self.held.is_empty() {
                    self.chord.clear();
                }
            }
        }
        if changed {
            let focus = self.nav.current().map(|f| f.tag);
            self.draw(cx, now, focus.filter(|t| *t == T_SAVE || *t == T_CANCEL).or(Some(T_SAVE)));
        }
        changed
    }

    /// Writes the shortcut (and the row's choice, when a row opened the sheet) into launcher.json.
    fn save(&mut self) -> Result<(), String> {
        let Some(s) = self.current() else { return Err("Pick at least one key.".into()) };
        if !self.live {
            return Ok(());
        }
        use crate::screens::settings::config;
        let old = config::load();
        let mut new = old.clone();
        let v = Some(s.to_saved());
        if self.lc {
            new.lc_shortcut = v;
        } else {
            new.rc_shortcut = v;
        }
        let pick = Some("shortcut".to_string());
        match self.row.as_deref() {
            Some("btn-lc") => new.lc_action = pick,
            Some("btn-rc") => new.rc_action = pick,
            Some("btn-lc-game") => new.lc_game_action = pick,
            Some("btn-rc-game") => new.rc_game_action = pick,
            _ => {}
        }
        config::save_diff(&old, &new)?;
        std::thread::spawn(rare_launch::apps::pointer::prepare_keys);
        Ok(())
    }
}

impl Drop for ShortcutSheet {
    fn drop(&mut self) {
        if self.live {
            crate::features::keycapture::set(false);
        }
    }
}

impl Screen for ShortcutSheet {
    fn touch_focus(&mut self, cx: &mut Cx, x: f32, y: f32, now: f64) -> bool {
        self.nav.focus_at(cx.scene, x, y, now)
    }

    fn pad(&mut self, cx: &mut Cx, pad: Pad, now: f64) -> Out {
        match pad {
            Pad::B => Out::Act("controls:shortcut-done".into()),
            Pad::A => {
                let Some(tag) = self.nav.current().map(|f| f.tag) else { return Out::None };
                match tag {
                    T_CANCEL => Out::Act("controls:shortcut-done".into()),
                    T_SAVE => match self.save() {
                        Ok(()) => Out::Act("controls:shortcut-saved".into()),
                        Err(e) => {
                            self.message = Some(e);
                            Out::None
                        }
                    },
                    T_NOKEY => {
                        self.key = None;
                        self.draw(cx, now, Some(tag));
                        Out::Moved
                    }
                    t if (T_MOD..T_MOD + MODIFIERS.len() as u32).contains(&t) => {
                        let m = MODIFIERS[(t - T_MOD) as usize];
                        if self.mods.contains(&m) {
                            self.mods.retain(|x| *x != m);
                        } else {
                            self.mods.push(m);
                        }
                        self.draw(cx, now, Some(tag));
                        Out::Moved
                    }
                    t if (T_KEY..T_KEY + shortcut::KEYS.len() as u32).contains(&t) => {
                        self.key = Some(shortcut::KEYS[(t - T_KEY) as usize].0);
                        self.draw(cx, now, Some(tag));
                        Out::Moved
                    }
                    _ => Out::None,
                }
            }
            Pad::Up | Pad::Down | Pad::Left | Pad::Right => {
                if self.nav.step(cx.scene, pad, now) {
                    Out::Moved
                } else {
                    Out::None
                }
            }
            _ => Out::None,
        }
    }

    fn tick(&mut self, cx: &mut Cx, now: f64) {
        if self.live {
            self.keys_in(cx, now);
        }
    }

    fn next_deadline(&self) -> Option<f64> {
        // Keys from a keyboard arrive on the event loop; the window wakes for them.
        None
    }

    fn bar(&self) -> Option<Vec<rare_render::focus::hints::Hint>> {
        use rare_render::focus::hints::{hint, Key};
        let a = self.nav.current().map(|f| f.hint.clone()).filter(|h| !h.is_empty()).unwrap_or_else(|| "Select".into());
        Some(vec![hint(Key::A, &a), hint(Key::B, "Cancel")])
    }

    fn take_message(&mut self) -> Option<String> {
        self.message.take()
    }

    fn focus_id(&self) -> Option<Id> {
        self.nav.cur
    }
}

/// Fixtures: `settings-shortcut` (RC set to Ctrl + Alt + Space) and `settings-shortcut-empty`.
pub fn fixture(name: &str, cx: &mut Cx, now: f64) -> Option<Box<dyn Screen>> {
    match name {
        "settings-shortcut" => Some(Box::new(ShortcutSheet::open(cx, false, None, Shortcut::parse("ctrl+alt+space"), false, now))),
        "settings-shortcut-empty" => Some(Box::new(ShortcutSheet::open(cx, true, Some("btn-lc".into()), None, false, now))),
        _ => None,
    }
}
