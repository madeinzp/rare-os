//! tr().overlay.paused_title (Switcher.tsx PausedFull): leaving a game would make three paused.

use rare_i18n::{tr, trf};
use super::model::{FreezeFull, RunItem};
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_render::input::Pad;
use rare_render::theme::tok;
use rare_render::{Id, Node};

pub struct PausedFull {
    nav: Nav,
    pending: String,
    frozen: Vec<String>,
}

impl PausedFull {
    pub fn build(cx: &mut Cx, ask: &FreezeFull, all: &[RunItem], now: f64) -> PausedFull {
        let r = cx.r();
        let root = cx.scene.root;
        let name = |rid: &str| all.iter().find(|x| x.rid == rid).map(|x| x.name.clone()).unwrap_or_else(|| tr().overlay.a_game.into());
        let sub = trf!(tr().overlay.paused_sub, name = name(&ask.pending));
        let sh = sheet(cx, root, tr().overlay.paused_title, Some(&sub), true, false, now);
        let mut nav = Nav::default();
        let n = ask.frozen.len();
        for (i, rid) in ask.frozen.iter().enumerate() {
            let it = all.iter().find(|x| &x.rid == rid);
            let row_ = cx.scene.add(sh.body, s().row().items_center().gap(0.9 * r).py(0.7 * r).done(), Node::default());
            if i + 1 < n {
                edge(cx, row_, 2, tok::HAIR);
            }
            let col = cx.scene.add(row_, s().col().grow().done(), Node::default());
            cx.scene.add_text(cx.t, col, s().done(), &name(rid), ts(r, ty::BODY.0, ty::BODY.1, 500).truncate(), tok::FG);
            text(cx, col, ty::SMALL, 400, tok::MUTED, tr().overlay.paused);
            let label = if it.is_some_and(|x| x.app) { tr().common.close } else { tr().overlay.quit_game };
            let hint = if it.is_some_and(|x| x.app) { tr().overlay.close_app } else { tr().overlay.quit_game };
            let b = button(cx, row_, label, Variant::Secondary, Sz::Sm, false).tag(i as u32).hint(hint).group(1);
            nav.add(b);
        }
        let keep = trf!(tr().overlay.keep_running, name = name(&ask.pending));
        let b = button(cx, sh.footer.unwrap(), &keep, Variant::Secondary, Sz::Md, false).tag(100).hint(tr().overlay.keep_running_hint).group(2);
        nav.add(b);
        let first = nav.items[0].id;
        nav.focus(cx.scene, first, now, false);
        PausedFull { nav, pending: ask.pending.clone(), frozen: ask.frozen.clone() }
    }
}

impl Screen for PausedFull {
    fn touch_focus(&mut self, cx: &mut Cx, x: f32, y: f32, now: f64) -> bool {
        self.nav.focus_at(cx.scene, x, y, now)
    }
    fn pad(&mut self, cx: &mut Cx, pad: Pad, now: f64) -> Out {
        match pad {
            // B keeps the new one running.
            Pad::B => Out::Act(format!("keep:{}", self.pending)),
            Pad::A => match self.nav.current().map(|f| f.tag) {
                Some(100) => Out::Act(format!("keep:{}", self.pending)),
                Some(i) => Out::Act(format!("quit:{}", self.frozen[i as usize])),
                None => Out::None,
            },
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
}

#[allow(dead_code)]
fn _unused(_: Id) {}
