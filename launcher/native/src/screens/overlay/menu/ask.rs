//! The Rare menu's Ask tab (Ask Rare): talk or type, the answer as it streams, each action Rare
//! takes on its own line. The newest exchange is on top so it reads without scrolling over a game;
//! earlier ones follow. Over a game the picture switch sends a still of the game with the question
//! (the in-game helper), when the owner allowed it in Settings.
//!
//! Sids: `ask-talk` (A starts or sends; X held talks), `ask-type` (the keyboard), `ask-stop`,
//! `ask-shot` (the picture switch), `ask-setup` (Settings > Ask Rare), `ask-clear`, `ask-line:<n>`
//! (answer blocks, focusable so a long answer scrolls).

use rare_i18n::{tr, trf};
use super::super::common::*;
use super::super::lay::s;
use crate::features::agent::{Line, StepState, View};
use rare_render::theme::tok;
use rare_render::{Id, Node};

pub const G_ASK: u32 = 41;

/// What the tab knows besides the conversation.
#[derive(Clone, Debug, Default)]
pub struct AskInfo {
    /// A provider is chosen in Settings.
    pub ready: bool,
    pub provider: String,
    pub voice: bool,
    /// Over a game with a still: the picture switch shows.
    pub can_shoot: bool,
    pub shoot: bool,
    pub over_game: bool,
}

pub fn info() -> AskInfo {
    let s = crate::features::agent::setup();
    AskInfo {
        ready: s.kind.is_some(),
        provider: s.kind.map(|k| k.name().to_string()).unwrap_or_default(),
        voice: s.voice,
        can_shoot: crate::features::agent::can_shoot(),
        shoot: s.shots,
        over_game: false,
    }
}

fn para(cx: &mut Cx, parent: Id, t: (f32, f32), w: u16, c: rare_render::Color, msg: &str) -> Id {
    let r = cx.r();
    cx.scene.add_text(cx.t, parent, s().wp(1.0).done(), msg, ts(r, t.0, t.1, w).wrap(), c)
}

/// The status line under the buttons.
pub fn status(v: &View) -> Option<String> {
    if let Some(p) = v.download {
        return Some(trf!(tr().overlay.ask_status_download, pct = format!("{:.0}", (p * 100.0).clamp(0.0, 100.0))));
    }
    if v.listening {
        return Some(tr().overlay.ask_status_listening.into());
    }
    if v.hearing {
        return Some(tr().overlay.ask_status_hearing.into());
    }
    if v.busy {
        return Some(tr().overlay.ask_status_busy.into());
    }
    None
}

/// The lines split into exchanges (each starts at a question), newest first.
pub fn exchanges(lines: &[Line]) -> Vec<&[Line]> {
    let mut starts: Vec<usize> = lines.iter().enumerate().filter(|(_, l)| matches!(l, Line::You { .. })).map(|(i, _)| i).collect();
    if starts.first() != Some(&0) && !lines.is_empty() {
        starts.insert(0, 0);
    }
    let mut out: Vec<&[Line]> = starts.iter().enumerate().map(|(k, &a)| &lines[a..starts.get(k + 1).copied().unwrap_or(lines.len())]).collect();
    out.reverse();
    out
}

/// Builds the tab into `parent`; returns the sid focus should land on.
pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, v: &View, i: &AskInfo) -> &'static str {
    let r = cx.r();
    let col = cx.scene.add(parent, s().col().wp(1.0).gap(0.9 * r).pad(0.75 * r, 0.4 * r, 0.75 * r, 0.0).done(), Node::default());
    if !i.ready {
        let card = cx.scene.add(col, s().col().wp(1.0).gap(0.6 * r).pad(1.25 * r, 1.1 * r, 1.25 * r, 1.2 * r).done(), Node::bg(tok::L2).radius(0.5 * r));
        text(cx, card, ty::ROW, 600, tok::FG, tr().overlay.ask_title);
        para(cx, card, ty::SMALL, 400, tok::MUTED, tr().overlay.ask_intro);
        let row = cx.scene.add(card, s().row().mt(0.4 * r).done(), Node::default());
        let b = button(cx, row, tr().overlay.ask_setup, Variant::Primary, Sz::Md, false).group(G_ASK).hint(tr().common.open).sid("ask-setup");
        nav.add(b);
        return "ask-setup";
    }

    // Controls.
    let row = cx.scene.add(col, s().row().wp(1.0).gap(0.6 * r).done(), Node::default());
    let talk_label = if v.listening { tr().overlay.ask_send } else { tr().overlay.ask_talk };
    if i.voice {
        let b = button(cx, row, talk_label, Variant::Primary, Sz::Md, false).group(G_ASK).hint(talk_label).sid("ask-talk");
        nav.add(b);
    }
    let b = button(cx, row, tr().overlay.ask_type, if i.voice { Variant::Secondary } else { Variant::Primary }, Sz::Md, false).group(G_ASK).hint(tr().overlay.ask_type_hint).sid("ask-type");
    nav.add(b);
    if v.busy {
        let b = button(cx, row, tr().overlay.ask_stop, Variant::Secondary, Sz::Md, false).group(G_ASK).hint(tr().overlay.ask_stop).sid("ask-stop");
        nav.add(b);
    }
    let meta = cx.scene.add(row, s().row().items_center().ml_auto().no_shrink_().done(), Node::default());
    text(cx, meta, ty::META, 400, tok::MUTED, &i.provider);
    if let Some(st) = status(v) {
        para(cx, col, ty::SMALL, 400, tok::FG.alpha(0.8), &st);
    }
    if i.can_shoot {
        let sw = cx.scene.add(col, s().row().items_center().gap(0.75 * r).wp(1.0).min_h(3.25 * r).px(0.75 * r).done(), Node::default().radius(0.5 * r));
        let words = cx.scene.add(sw, s().col().grow().done(), Node::default());
        text_tr(cx, words, ty::SMALL, 500, tok::FG, tr().overlay.ask_picture);
        text_tr(cx, words, ty::LABEL, 400, tok::MUTED, tr().overlay.ask_picture_sub);
        toggle(cx, sw, i.shoot);
        nav.add(Focusable { id: sw, kind: Kind::Row, group: G_ASK, hint: if i.shoot { tr().common.turn_off.into() } else { tr().common.turn_on.into() }, tag: 0, sid: "ask-shot".into() });
    }

    // The conversation, newest exchange first.
    let ex = exchanges(&v.lines);
    if ex.is_empty() {
        let c = cx.scene.add(col, s().col().wp(1.0).gap(0.35 * r).pad(0.75 * r, 0.6 * r, 0.75 * r, 0.0).done(), Node::default());
        text(cx, c, ty::LABEL, 500, tok::MUTED, tr().overlay.ask_try);
        let tries: &[&str] = if i.over_game { &[tr().overlay.ask_try_boss, tr().overlay.ask_try_brightness, tr().overlay.ask_try_invite] } else { &[tr().overlay.ask_try_start, tr().overlay.ask_try_dnd, tr().overlay.ask_try_space] };
        for t in tries {
            para(cx, c, ty::SMALL, 400, tok::FG.alpha(0.8), t);
        }
    }
    let mut n = 0u32;
    for (k, lines) in ex.iter().enumerate() {
        let block = cx.scene.add(col, s().col().wp(1.0).gap(0.45 * r).done(), Node::default());
        if k == 1 {
            let lab = cx.scene.add(block, s().pad(0.75 * r, 0.6 * r, 0.75 * r, 0.1 * r).done(), Node::default());
            edge(cx, lab, 0, tok::HAIR);
            text(cx, lab, ty::LABEL, 500, tok::MUTED, tr().overlay.ask_earlier);
        }
        let dim = if k == 0 { 1.0 } else { 0.7 };
        for l in lines.iter() {
            match l {
                Line::You { text: t, shot } => {
                    let y = cx.scene.add(block, s().col().wp(1.0).gap(0.15 * r).px(0.75 * r).done(), Node::default());
                    text(cx, y, ty::LABEL, 500, tok::MUTED, if *shot { tr().overlay.ask_you_picture } else { tr().overlay.ask_you });
                    para(cx, y, ty::BODY, 500, tok::FG.alpha(dim), t);
                }
                Line::Step { label, state, .. } => {
                    let st = cx.scene.add(block, s().row().items_center().gap(0.55 * r).wp(1.0).px(0.75 * r).done(), Node::default());
                    let (c, sub) = match state {
                        StepState::Running => (tok::MUTED, None),
                        StepState::Done(x) => (tok::FG.alpha(0.85), Some(x.as_str())),
                        StepState::Failed(x) => (tok::NEG, Some(x.as_str())),
                    };
                    dot(cx, st, c);
                    let words = cx.scene.add(st, s().col().grow().shrink().done(), Node::default());
                    text_tr(cx, words, ty::SMALL, 500, tok::FG.alpha(0.9 * dim), label);
                    if let Some(sub) = sub.filter(|x| !x.is_empty() && *x != label) {
                        text_tr(cx, words, ty::LABEL, 400, if matches!(state, StepState::Failed(_)) { tok::NEG } else { tok::MUTED }, sub);
                    }
                }
                Line::Answer(a) => {
                    let id = cx.scene.add(block, s().col().wp(1.0).px(0.75 * r).py(0.5 * r).done(), Node::default().radius(0.5 * r));
                    para(cx, id, ty::BODY, 400, tok::FG.alpha(dim), a.trim());
                    nav.add(Focusable { id, kind: Kind::Row, group: G_ASK, hint: String::new(), tag: n, sid: format!("ask-line:{n}") });
                    n += 1;
                }
                Line::Note { text: t, error } => {
                    let c = cx.scene.add(block, s().col().wp(1.0).px(0.75 * r).done(), Node::default());
                    para(cx, c, ty::SMALL, 400, if *error { tok::NEG } else { tok::MUTED }, t);
                }
            }
        }
    }
    if !v.lines.is_empty() && !v.busy {
        let row = cx.scene.add(col, s().row().pad(0.0, 0.3 * r, 0.0, 0.6 * r).done(), Node::default());
        let b = button(cx, row, tr().overlay.ask_new_chat, Variant::Ghost, Sz::Sm, false).group(G_ASK).hint(tr().overlay.ask_start_over).sid("ask-clear");
        nav.add(b);
    }
    if i.voice {
        "ask-talk"
    } else {
        "ask-type"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchanges_newest_first() {
        let you = |t: &str| Line::You { text: t.into(), shot: false };
        let lines = vec![Line::Note { text: "n".into(), error: false }, you("a"), Line::Answer("1".into()), you("b"), Line::Answer("2".into())];
        let ex = exchanges(&lines);
        assert_eq!(ex.len(), 3);
        assert_eq!(ex[0][0], you("b"));
        assert_eq!(ex[2][0], Line::Note { text: "n".into(), error: false });
        assert!(exchanges(&[]).is_empty());
    }

    #[test]
    fn status_lines() {
        assert_eq!(status(&View::default()), None);
        assert!(status(&View { listening: true, ..Default::default() }).unwrap().starts_with("Listening"));
        assert!(status(&View { download: Some(0.42), ..Default::default() }).unwrap().contains("42%"));
    }
}
