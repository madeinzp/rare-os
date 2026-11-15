//! The on-screen keyboard (components/Osk.tsx), driven by the controller or touch: the d-pad
//! moves (rows wrap), A types, X deletes, Y is Shift (#+= / 123 on the symbol pages), LT switches
//! letters and symbols (?123 / abc), Menu finishes, B cancels; held X and a held A on a key that
//! types repeat (features/kbrepeat.rs), as does a finger held on one. The layout is
//! features/kblayout.rs, shared with the keyboard over apps. A bottom sheet sliding up over a
//! dimmed screen; the focused key white.

use rare_i18n::{tr, trf};
use crate::act::Act;
use crate::features::kblayout::{self, Field, Host, Key, COLS, SPECIAL_ROW};
use crate::kit::{self, s, tok, GSize, Pb, Ty, Ui, T};
use crate::page::{hash, Acts, Btn, Feedback, Fx, Page};
use crate::state::{OskFor, OskReq, State};
use rare_input::Pad;
use rare_render::focus::Focusable;
use rare_render::scene::Shadow;
use rare_render::view::{key, El, Motion};
use std::sync::atomic::Ordering;

/// Special keys' tags start here (character keys are row * 10 + column).
const SPECIAL_TAG: u32 = 40;

pub struct Osk {
    kb: kblayout::Kb,
    /// The kb was set up for the request in front (its field kind).
    ready: bool,
    /// The key to focus after the next build (a tag).
    want: Option<u32>,
}

impl Default for Osk {
    fn default() -> Self {
        Osk {
            kb: kblayout::Kb::new(Field::Text, Host::Launcher),
            ready: false,
            want: Some(10),
        }
    }
}

fn tag_of(pos: (usize, usize)) -> u32 {
    if pos.0 >= SPECIAL_ROW {
        SPECIAL_TAG + pos.1 as u32
    } else {
        (pos.0 * COLS + pos.1) as u32
    }
}

fn pos_of(tag: u32) -> (usize, usize) {
    if tag >= SPECIAL_TAG {
        (SPECIAL_ROW, (tag - SPECIAL_TAG) as usize)
    } else {
        ((tag as usize) / COLS, (tag as usize) % COLS)
    }
}

fn pb(p: Pad) -> Pb {
    match p {
        Pad::Y => Pb::Y,
        Pad::X => Pb::X,
        Pad::Lt => Pb::Lt,
        _ => Pb::Start,
    }
}

/// The pad buttons the shared keyboard knows.
fn pad_of(b: Btn) -> Option<Pad> {
    Some(match b {
        Btn::Up => Pad::Up,
        Btn::Down => Pad::Down,
        Btn::Left => Pad::Left,
        Btn::Right => Pad::Right,
        Btn::A => Pad::A,
        Btn::X => Pad::X,
        Btn::Y => Pad::Y,
        Btn::Lt => Pad::Lt,
        Btn::Start => Pad::Start,
        _ => return None,
    })
}

/// What a finished keyboard hands back (the web's askText promise).
pub fn finish(st: &mut State, req: OskReq, value: Option<String>) {
    match (req.purpose, value) {
        (OskFor::Settings(k), v) => {
            st.settings_input = Some((k, v));
        }
        (OskFor::Search, Some(v)) => {
            st.search_before = None;
            if st.search != v {
                st.search = v;
                st.ver.search += 1;
            }
        }
        (OskFor::Search, None) => {
            if let Some(b) = st.search_before.take() {
                if st.search != b {
                    st.search = b;
                    st.ver.search += 1;
                }
            }
            // Cancelled with no search yet: the Search page goes too.
            if st.search.is_empty() {
                st.close();
            }
        }
        (OskFor::WebGameUrl, Some(v)) if !v.trim().is_empty() => match crate::features::webgame::normalise_url(&v) {
            Ok(url) => {
                let host = url.split("://").nth(1).and_then(|r| r.split(['/', '?', '#']).next()).unwrap_or("");
                let host = host.trim_start_matches("www.").to_string();
                st.osk = Some(OskReq::new(tr().osk.name_game, host, None, OskFor::WebGameTitle { url }));
            }
            Err(e) => st.toast(crate::screens::settings::live::failed(tr().osk.not_web_address, &e, None), crate::state::Tone::Error, rare_render::clock::secs()),
        },
        (OskFor::WebGameTitle { url }, Some(t)) if !t.trim().is_empty() => {
            st.acts.push(crate::act::Act::AddWebGame { id: None, title: t.trim().to_string(), url, cover: None, hero: None });
        }
        (OskFor::Screen(k), v) => st.osk_answer = Some((k, v)),
        (OskFor::SameGame(id), Some(q)) if !q.trim().is_empty() => st.acts.push(crate::act::Act::SameGameFind(id, q.trim().to_string())),
        (OskFor::Collection { rename, game }, Some(name)) => {
            let now = rare_render::clock::secs();
            let r = match &rename {
                Some(old) => st.user.rename(old, &name).map(|n| trf!(tr().osk.renamed_to, name = n)),
                None => st.user.create(&name, game.as_deref()).map(|n| if game.is_some() { trf!(tr().osk.added_to, name = n) } else { trf!(tr().osk.created, name = n) }),
            };
            match r {
                Ok(text) => {
                    st.user_changed();
                    st.toast(text, crate::state::Tone::Info, now);
                }
                Err(e) => st.toast(e, crate::state::Tone::Error, now),
            }
        }
        _ => {}
    }
}

impl Osk {
    /// Sets the layout up for the request in front (once per request: the slot is new for each).
    fn sync(&mut self, st: &State) {
        if !self.ready {
            if let Some(req) = &st.osk {
                self.kb = kblayout::Kb::new(req.field, Host::Launcher);
                self.ready = true;
            }
        }
        // The pad thread's view of it: whether a held A repeats on the focused key.
        crate::features::kbrepeat::LAUNCHER_A.store(self.kb.repeats(Pad::A), Ordering::Relaxed);
    }

    fn apply(&mut self, st: &mut State, out: kblayout::Out) {
        let Some(req) = st.osk.as_mut() else { return };
        match out {
            kblayout::Out::Type(t) => req.value.push_str(t),
            kblayout::Out::Delete => {
                req.value.pop();
            }
            kblayout::Out::Done => {
                let req = st.osk.take().unwrap();
                let v = req.value.clone();
                finish(st, req, Some(v));
            }
            kblayout::Out::Paste => {
                match rare_system::ssh::clipboard_text().filter(|c| !c.trim().is_empty()) {
                    Some(c) => req.value.push_str(c.trim()),
                    // Say so, instead of a key that seems to do nothing.
                    None => st.toast(
                        tr().osk.nothing_to_paste,
                        crate::state::Tone::Info,
                        rare_render::clock::secs(),
                    ),
                }
            }
            kblayout::Out::Close | kblayout::Out::Redraw | kblayout::Out::Nothing => {}
        }
    }
}

impl Page for Osk {
    fn stamp(&self, st: &State) -> u64 {
        hash((
            st.osk
                .as_ref()
                .map(|o| (&o.title, &o.value, &o.status, o.field)),
            self.kb.layer,
            self.kb.shift,
        ))
    }

    fn view(&mut self, ui: &mut Ui, st: &State, acts: &mut Acts) -> El {
        self.sync(st);
        let Some(req) = &st.osk else {
            return ui.bx(s());
        };
        let kb = self.kb;
        let shown = if req.secret {
            "\u{2022}".repeat(req.value.chars().count())
        } else {
            req.value.clone()
        };
        let mut field = ui
            .bx(s().row().items_center().h(3.25).px(1.0).mb(1.0))
            .bg(tok::BG)
            .radius(ui.r(0.5))
            .border(1.0, tok::BORDER_LIGHT);
        field = if shown.is_empty() {
            field.child(ui.text(
                req.placeholder.clone().unwrap_or_default(),
                T::new(Ty::Row).c(tok::MUTED.alpha(0.7)),
            ))
        } else {
            field.child(ui.text(shown, T::new(Ty::Row).mono().trunc()))
        };
        let caret_name = ui.loop_name(&CARET, 2.0);
        field = field.child(
            ui.bx(s().w_px(2.0).h(1.5).ml(1.0 / ui.rem))
                .bg(tok::FG)
                .name(caret_name),
        );

        let mut keys = ui.bx(s().col().gap(0.45));
        for (ri, row) in kb.rows().iter().enumerate() {
            let mut r = ui.bx(s().row().gap(0.45));
            for (ci, k) in row.iter().enumerate() {
                let tag = tag_of((ri, ci));
                let _ = acts.tag(Act::Local(tag));
                let b = ui
                    .bx(s().row().items_center().justify_center().h(2.9).grow())
                    .bg(tok::L3)
                    .radius(ui.r(0.5))
                    .child(ui.text(*k, T::new(Ty::Row).w(500)).focus_color(tok::BG));
                let fl = b.look.clone().bg(tok::FG).scale(1.06);
                // Keyed by place: Shift and the layers change labels, focus stays on the key.
                r = r.child(
                    kit::nav(
                        b.focus_look(fl)
                            .transition(rare_render::view::Transition::pro(0.1)),
                        tag,
                        Some(tr().osk.type_key),
                        None,
                        false,
                    )
                    .key(key(&(ri, ci))),
                );
            }
            keys = keys.child(r);
        }
        let mut sp = ui.bx(s().row().gap(0.45));
        let weights = kb.weights();
        for (ci, k) in kb.special().into_iter().enumerate() {
            let tag = tag_of((SPECIAL_ROW, ci));
            let _ = acts.tag(Act::Local(tag));
            let done = k == Key::Done;
            let fill = if done {
                tok::BRAND
            } else if k == Key::Shift && kb.lit() {
                // Shift on (or the #+= page): the key stays lit.
                tok::fg(0.3)
            } else {
                tok::L4
            };
            let mut b = ui
                .bx(s()
                    .row()
                    .items_center()
                    .justify_center()
                    .gap(0.5)
                    .h(2.9)
                    .flex(weights[ci]))
                .bg(fill)
                .radius(ui.r(0.5));
            if let Some(g) = kblayout::Kb::button(k) {
                b = b.child(kit::glyph(ui, pb(g), GSize::Sm));
            }
            let ty = if matches!(k, Key::Char(_)) {
                T::new(Ty::Row).w(500)
            } else {
                T::new(Ty::Body).w(500)
            };
            b = b.child(
                ui.text(kb.label(k), ty.c(if done { tok::WHITE } else { tok::FG }))
                    .focus_color(tok::BG),
            );
            let fl = b.look.clone().bg(tok::FG).scale(1.04);
            sp = sp.child(
                kit::nav(
                    b.focus_look(fl)
                        .transition(rare_render::view::Transition::pro(0.1)),
                    tag,
                    Some(tr().osk.type_key),
                    None,
                    false,
                )
                .key(key(&k)),
            );
        }
        keys = keys.child(sp);
        let small = T::new(Ty::Small).c(tok::fg(0.75));
        let hints = ui
            .bx(s().row().justify_end().gap(1.3).mt(0.9))
            .child(kit::hint(ui, Pb::A, tr().osk.type_key, small))
            .child(kit::hint(ui, Pb::B, tr().common.cancel, small));
        let mut inner = ui
            .bx(s().col().wp(1.0).max_w(64.0).mx_auto())
            .child(ui.text_s(
                s().mb(0.5),
                req.title.clone(),
                T::new(Ty::Small).c(tok::MUTED),
            ))
            .child(field);
        if let Some(line) = &req.status {
            // The asking screen's live line (askText's status), between the field and the keys.
            inner = inner.child(ui.text_s(
                s().mt(-0.5).mb(0.75),
                line.clone(),
                T::new(Ty::Small).c(tok::MUTED).trunc(),
            ));
        }
        let inner = inner.child(keys).child(hints);
        // rounded-t-sheet: the bottom corners sit below the screen edge.
        let sheet = ui
            .bx(s().col().px(2.5).pb(1.1 + 1.25).pt(1.2).mb(-1.25))
            .bg(tok::L1)
            .radius(ui.r(1.25))
            .border(1.0, tok::BORDER_LIGHT)
            .shadow(&[Shadow::new(
                0.0,
                -ui.r(1.0),
                ui.r(3.0),
                0.0,
                tok::BLACK.alpha(0.5),
            )])
            .enter(Motion::SLIDE_UP)
            .child(inner);
        ui.bx(s().fill().col().justify_end())
            .bg(tok::BLACK.alpha(0.55))
            .enter(Motion::FADE)
            .child(sheet)
    }

    fn pad(
        &mut self,
        st: &mut State,
        fx: &mut Fx,
        b: Btn,
        cur: Option<(&Focusable, Option<&Act>)>,
    ) -> bool {
        self.sync(st);
        // The focused key is where the keyboard is (a touch puts focus on a key without moving here).
        if let Some((f, _)) = cur.filter(|(f, _)| f.scope == crate::app::SCOPE_OSK) {
            let p = pos_of(f.tag);
            if p.1
                < if p.0 == SPECIAL_ROW {
                    self.kb.special().len()
                } else {
                    COLS
                }
            {
                self.kb.pos = p;
            }
        }
        if b == Btn::B {
            if let Some(req) = st.osk.take() {
                finish(st, req, None);
            }
            return true;
        }
        let Some(p) = pad_of(b) else { return true };
        let before = self.kb.pos;
        let out = self.kb.press(p, false);
        if self.kb.pos != before {
            fx.fb(Feedback::Move);
        }
        // Focus stays where the keyboard is (a layer change rebuilds the keys).
        self.want = Some(tag_of(self.kb.pos));
        self.apply(st, out);
        // A layer with fewer special keys never leaves focus on a key that is gone.
        self.kb.pos.1 = self.kb.pos.1.min(if self.kb.pos.0 == SPECIAL_ROW {
            self.kb.special().len() - 1
        } else {
            COLS - 1
        });
        crate::features::kbrepeat::LAUNCHER_A.store(self.kb.repeats(Pad::A), Ordering::Relaxed);
        true
    }

    fn take_focus(&mut self, cur: Option<&Focusable>) -> Option<u32> {
        // A rebuild under the sheet (the page below loading, a layer change) can drop focus; it
        // goes back to the key the keyboard is on, never to the first key.
        let lost = cur.is_none_or(|c| c.scope != crate::app::SCOPE_OSK);
        self.want.take().or(lost.then(|| tag_of(self.kb.pos)))
    }
}

/// animate-pulse: opacity 1 → 0.5 → 1 over 2 s.
pub static CARET: rare_render::anim::Keys = rare_render::anim::Keys { frames: &[(0.0, 1.0), (0.5, 0.5), (1.0, 1.0)], ease: rare_render::Ease(0.4, 0.0, 0.6, 1.0) };
