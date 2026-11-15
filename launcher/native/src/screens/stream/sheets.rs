//! PC streaming sheets (stream/sheets.tsx): StreamSetup (get Moonlight, find PCs, add one, pair
//! with a big PIN) and StreamAppSheet (a PC game: Play, Resume, Quit, wake, stream quality).

use rare_i18n::{tr, trf};
use super::logic::*;
use super::state::StreamState;
use super::view::*;
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_render::theme::tok;
use rare_render::{Id, Node};

fn sheet_layer(cx: &mut Cx, parent: Id, title: &str, sub: Option<&str>, now: f64) -> (Layer, Id, Id) {
    let p = sheet(cx, parent, title, sub, true, true, now);
    (Layer::new(p.card, Some(p.card)), p.body, p.footer.unwrap())
}

/// Moonlight from Flathub: Rare gets it the first time streaming opens, with progress here; the
/// command for Desktop mode only after a failed install.
fn get_moonlight(cx: &mut Cx, parent: Id, st: &StreamState, waiting: Option<&str>) {
    let Some(offer) = &st.offer else { return };
    let r = cx.r();
    let busy = st.installing.as_ref().is_some_and(|i| i.error.is_none());
    let failed = st.installing.as_ref().and_then(|i| i.error.as_ref());
    let col = cx.scene.add(parent, s().col().gap(0.6 * r).mb(1.4 * r).done(), Node::default());
    text(cx, col, ty::BODY, 600, tok::FG, if busy { tr().stream.getting_moonlight } else { "Moonlight" });
    let why = match waiting {
        Some(name) if busy => trf!(tr().stream.moonlight_why_waiting, name = name),
        _ if busy => tr().stream.moonlight_why_busy.to_string(),
        _ => tr().stream.moonlight_why.to_string(),
    };
    wrap_text(cx, col, ty::SMALL, 400, tok::MUTED, &why);
    // The bar's row keeps its place before, during and after, so nothing below moves.
    let row_ = cx.scene.add(col, s().row().items_center().gap(0.9 * r).h(1.25 * r).done(), Node::default());
    if busy {
        let p = st.installing.as_ref().unwrap().progress;
        install_bar(cx, row_, p);
        let pct = cx.scene.add(row_, s().w(3.0 * r).row().justify_end().done(), Node::default());
        text(cx, pct, ty::SMALL, 400, tok::FG, &p.map(|p| format!("{}%", (p * 100.0).round() as u32)).unwrap_or_default());
    }
    if let Some(e) = failed {
        wrap_text(cx, col, ty::SMALL, 400, tok::NEG, e);
        text(cx, col, ty::LABEL, 400, tok::MUTED, tr().stream.or_desktop_terminal);
        let code = cx.scene.add(col, s().col().px(0.9 * r).py(0.6 * r).done(), Node::bg(tok::L2).radius(0.5 * r));
        let ts_ = ts(r, ty::META.0, ty::META.1, 400).mono().wrap();
        cx.scene.add_text(cx.t, code, s().done(), &offer.command, ts_, tok::FG);
    }
}

fn host_row(cx: &mut Cx, layer: &mut Layer, parent: Id, st: &StreamState, host: &Host, def: bool) {
    let r = cx.r();
    let id = cx.scene.add(parent, s().row().items_center().gap(0.9 * r).h(3.75 * r).wp(1.0).px(0.75 * r).done(), Node::default().radius(0.5 * r));
    let circle = cx.scene.add(id, s().wh(2.25 * r, 2.25 * r).items_center().justify_center().done(), Node::bg(tok::L3).radius(1.125 * r));
    let k = cx.img.request(svg_req("monitor", MONITOR, 1.1 * r));
    cx.scene.add(circle, s().wh(1.1 * r, 1.1 * r).done(), Node::default().icon(k, tok::MUTED));
    let col = cx.scene.add(id, s().col().grow().done(), Node::default());
    text_tr(cx, col, ty::BODY, 500, tok::FG, &host.name);
    let sub: Vec<String> = [host.address.clone(), host.software.clone()].into_iter().flatten().collect();
    mono_text(cx, col, ty::META, 400, tok::MUTED, &sub.join(" \u{b7} "), true);
    let right = cx.scene.add(id, s().row().items_center().gap(0.45 * r).no_shrink_().done(), Node::default());
    dot(cx, right, if host.online { tok::FG } else { tok::MUTED });
    let state = if host.paired {
        tr().stream.paired
    } else if host.online {
        tr().stream.not_paired
    } else {
        tr().stream.not_answering
    };
    text(cx, right, ty::SMALL, 400, tok::MUTED, state);
    let hint = if host.paired { tr().stream.show_games } else { tr().stream.pair };
    let t = layer.reg(Focusable { id, kind: Kind::Row, group: 10, hint: hint.into(), tag: 0, sid: host.uuid.clone() }, Act::HostRow(host.uuid.clone()));
    if def {
        layer.default = Some(t);
    }
    let _ = st;
}

/// Find PCs and pair.
pub fn build_setup(cx: &mut Cx, st: &StreamState, parent: Id, now: f64) -> Layer {
    if st.pairing.is_some() {
        return build_pair(cx, st, parent, now);
    }
    let r = cx.r();
    let (mut layer, body, footer) = sheet_layer(cx, parent, tr().stream.stream_from_pc, None, now);
    let busy = st.installing.as_ref().is_some_and(|i| i.error.is_none());
    let waiting = st.pending_pair.as_deref().and_then(|u| st.host(u)).map(|h| h.name.clone());
    get_moonlight(cx, body, st, waiting.as_deref());
    let list = cx.scene.add(body, s().col().done(), Node::default());
    let head = cx.scene.add(list, s().row().items_center().gap(0.75 * r).mb(0.5 * r).done(), Node::default());
    text(cx, head, ty::BODY, 600, tok::FG, tr().stream.pcs_on_network);
    text(cx, head, ty::SMALL, 400, tok::MUTED, if st.scanning { tr().stream.looking } else { "" });
    load_bar(cx, list, st.scanning);
    let hosts = cx.scene.add(list, s().col().done(), Node::default());
    let unpaired = st.hosts.iter().position(|h| !h.paired && h.online);
    for (i, h) in st.hosts.iter().enumerate() {
        let def = st.offer.is_none() && unpaired.map(|u| i == u).unwrap_or(i == 0);
        host_row(cx, &mut layer, hosts, st, h, def);
    }
    if !st.scanning && st.hosts.is_empty() {
        let p = cx.scene.add(list, s().col().py(0.9 * r).done(), Node::default());
        wrap_text(cx, p, ty::SMALL, 400, tok::MUTED, tr().stream.no_pcs_yet);
    }
    let note = cx.scene.add(body, s().col().mt(1.1 * r).done(), Node::default());
    let (note_a, note_b) = tr().stream.sunshine_note.split_once("{link}").unwrap_or((tr().stream.sunshine_note, ""));
    rich(
        cx,
        note,
        ty::SMALL,
        tok::MUTED,
        &[(note_a, false), ("github.com/LizardByte/Sunshine", true), (note_b, false)],
    );
    // Footer.
    if st.offer.is_some() && !busy {
        // Only after a failed install: Rare starts it on its own when streaming opens.
        let b = btn(cx, footer, tr().common.try_again, Variant::Primary, tr().stream.get_moonlight, false, None);
        reg_btn(&mut layer, b, Act::GetMoonlight, 11, true, false);
    }
    let b = btn(cx, footer, tr().stream.add_by_address, Variant::Secondary, tr().stream.add_by_address, false, Some(("plus", PLUS, 1.0)));
    reg_btn(&mut layer, b, Act::AddAddress, 11, false, false);
    let b = btn(cx, footer, tr().stream.look_again, Variant::Secondary, tr().stream.look_again, st.scanning, None);
    reg_btn(&mut layer, b, Act::LookAgain, 11, false, st.scanning);
    layer
}

/// The PIN, big, and where to type it on the PC.
fn build_pair(cx: &mut Cx, st: &StreamState, parent: Id, now: f64) -> Layer {
    let r = cx.r();
    let p = st.pairing.clone().unwrap();
    let title = trf!(tr().stream.pair_with, name = p.name.clone());
    let (mut layer, body, footer) = sheet_layer(cx, parent, &title, Some(tr().stream.pairing_once), now);
    let gfe = p.software == "GeForce Experience";
    let top = cx.scene.add(body, s().row().items_center().gap(2.5 * r).done(), Node::default());
    let pin = cx.scene.add(top, s().row().gap(0.6 * r).no_shrink_().done(), Node::default());
    for d in p.pin.chars() {
        let b = cx.scene.add(pin, s().wh(4.25 * r, 5.5 * r).items_center().justify_center().done(), Node::bg(tok::L2).radius(0.5 * r));
        cx.scene.add_text(cx.t, b, s().done(), &d.to_string(), ts(r, 2.75, 3.25, 600).tabular(), tok::FG);
    }
    let ol = cx.scene.add(top, s().col().grow().gap(0.7 * r).done(), Node::default());
    let steps: Vec<Vec<(String, bool)>> = if gfe {
        vec![
            vec![(trf!(tr().stream.gfe_step1, name = p.name.clone()), false)],
            vec![(tr().stream.gfe_step2.into(), false)],
            vec![(tr().stream.finishes_on_own.into(), false)],
        ]
    } else {
        vec![
            sun_step1(&p.name),
            vec![(tr().stream.sunshine_step2.into(), false)],
            vec![(tr().stream.sunshine_step3.into(), false)],
        ]
    };
    for (i, segs) in steps.iter().enumerate() {
        let li = cx.scene.add(ol, s().row().gap(0.75 * r).done(), Node::default());
        let n = cx.scene.add(li, s().w(1.0 * r).no_shrink_().done(), Node::default());
        cx.scene.add_text(cx.t, n, s().done(), &(i + 1).to_string(), ts(r, ty::BODY.0, ty::BODY.1, 400).tabular(), tok::MUTED);
        let txt = cx.scene.add(li, s().col().grow().done(), Node::default());
        let segs: Vec<(&str, bool)> = segs.iter().map(|(t, c)| (t.as_str(), *c)).collect();
        rich(cx, txt, ty::BODY, tok::FG, &segs);
    }
    let bottom = cx.scene.add(body, s().col().mt(1.6 * r).pt(1.0 * r).done(), Node::default());
    edge(cx, bottom, 0, tok::HAIR);
    load_bar(cx, bottom, p.phase == PairPhase::Waiting);
    let row_ = cx.scene.add(bottom, s().row().items_center().gap(0.5 * r).done(), Node::default());
    let (tone, msg, col) = match p.phase {
        PairPhase::Paired => (Tone::Primary, trf!(tr().stream.paired_with, name = p.name.clone()), tok::FG.alpha(0.8)),
        PairPhase::Failed => (Tone::Negative, p.error.clone().unwrap_or_default(), tok::NEG),
        PairPhase::Waiting => (Tone::Muted, trf!(tr().stream.waiting_for, name = p.name.clone()), tok::FG.alpha(0.8)),
    };
    status_dot(cx, row_, tone);
    text(cx, row_, ty::SMALL, 400, col, &msg);
    match p.phase {
        PairPhase::Paired => {
            let b = btn(cx, footer, tr().stream.show_games, Variant::Primary, tr().stream.show_games, false, None);
            reg_btn(&mut layer, b, Act::ShowGames, 11, true, false);
        }
        PairPhase::Failed => {
            let b = btn(cx, footer, tr().common.try_again, Variant::Primary, tr().common.try_again, false, None);
            reg_btn(&mut layer, b, Act::TryAgain(p.uuid.clone()), 11, true, false);
            let b = btn(cx, footer, tr().common.back, Variant::Secondary, "", false, None);
            reg_btn(&mut layer, b, Act::BackFromPair, 11, false, false);
        }
        PairPhase::Waiting => {
            let b = btn(cx, footer, tr().common.cancel, Variant::Secondary, tr().common.cancel, false, None);
            reg_btn(&mut layer, b, Act::CancelPair, 11, true, false);
        }
    }
    layer
}

/// A focusable setting row: A moves to the next value.
fn choice(cx: &mut Cx, layer: &mut Layer, parent: Id, label: &str, items: &[String], on: usize, act: Act, def: bool) {
    let r = cx.r();
    let id = cx.scene.add(parent, s().row().items_center().gap(1.0 * r).h(3.25 * r).wp(1.0).px(0.75 * r).done(), Node::default().radius(0.5 * r));
    let l = cx.scene.add(id, s().grow().done(), Node::default());
    text(cx, l, ty::BODY, 400, tok::FG, label);
    chips(cx, id, items, on);
    let t = layer.reg(Focusable { id, kind: Kind::Row, group: 12, hint: tr().common.change.into(), tag: 0, sid: label.to_lowercase() }, act);
    if def {
        layer.default = Some(t);
    }
}

/// What the sheet's status line says for each state.
pub fn app_line(host: &Host, state: PlayState, waking: bool, other: &str) -> String {
    match state {
        PlayState::Playing => tr().stream.streaming_here.into(),
        PlayState::OnPc => trf!(tr().stream.line_on_pc, name = host.name.clone()),
        PlayState::Busy => trf!(tr().stream.line_busy, game = other, name = host.name.clone()),
        PlayState::Asleep => {
            if waking {
                trf!(tr().stream.waking_host, name = host.name.clone())
            } else {
                trf!(tr().stream.line_asleep, name = host.name.clone())
            }
        }
        PlayState::Offline => trf!(tr().stream.line_offline, name = host.name.clone()),
        PlayState::Ready => trf!(tr().stream.line_ready, name = host.name.clone()),
    }
}

/// Sunshine pairing, step 1: the sentence ends with the address of its web page.
fn sun_step1(name: &str) -> Vec<(String, bool)> {
    let line = trf!(tr().stream.sunshine_step1, name = name.to_string(), url = "\u{1}");
    match line.split_once('\u{1}') {
        Some((a, b)) => vec![(a.to_string(), false), ("https://localhost:47990".into(), true), (b.to_string(), false)],
        None => vec![(line, false)],
    }
}

/// A PC game's page. None when the PC or the game isn't known.
pub fn build_app(cx: &mut Cx, st: &StreamState, uuid: &str, app_id: u32, parent: Id, now: f64) -> Option<Layer> {
    let host = st.host(uuid)?.clone();
    let app = st.apps_of(uuid).iter().find(|a| a.id == app_id)?.clone();
    let r = cx.r();
    let state = play_state(&host, &app, &st.running);
    let other = host.current_game.map(|g| st.apps_of(uuid).iter().find(|a| a.id == g).map(|a| a.name.clone()).unwrap_or_else(|| tr().stream.another_game.into())).unwrap_or_default();
    let waking = st.waking.as_deref() == Some(uuid);
    let sub = trf!(tr().stream.on_host, name = host.name.clone()) + &host.software.as_ref().map(|s| format!(" \u{b7} {s}")).unwrap_or_default();
    let (mut layer, body, footer) = sheet_layer(cx, parent, &app.name, Some(&sub), now);
    let top = cx.scene.add(body, s().row().gap(1.75 * r).done(), Node::default());
    let c = cx.scene.add(top, s().wh(9.0 * r, 13.5 * r).no_shrink_().done(), Node::bg(tok::L3).radius(0.5 * r).clip());
    cover(cx, c, app.art.as_ref(), &app.name, 9.0 * r, 13.5 * r);
    let col = cx.scene.add(top, s().col().grow().done(), Node::default());
    let line = cx.scene.add(col, s().row().items_center().gap(0.5 * r).done(), Node::default());
    dot(cx, line, if matches!(state, PlayState::Playing | PlayState::OnPc) { tok::FG } else { tok::MUTED });
    text(cx, line, ty::SMALL, 400, tok::FG.alpha(0.8), &app_line(&host, state, waking, &other));
    let q = cx.scene.add(col, s().col().mt(1.2 * r).mb(0.35 * r).done(), Node::default());
    text(cx, q, ty::BODY, 600, tok::FG, tr().stream.stream_quality);
    let set = st.settings.clone();
    let auto = auto_codec(st.decode, host.codecs);
    let list = cx.scene.add(col, s().col().done(), Node::default());
    let res: Vec<String> = RESOLUTIONS.iter().map(|r| r.1.to_string()).collect();
    choice(cx, &mut layer, list, tr().stream.resolution, &res, RESOLUTIONS.iter().position(|r| r.0 == set.resolution).unwrap_or(0), Act::ChooseResolution, false);
    let br: Vec<String> = BITRATES.iter().map(|m| format!("{m} Mbps")).collect();
    choice(cx, &mut layer, list, tr().stream.bitrate, &br, BITRATES.iter().position(|m| *m == set.bitrate_mbps).unwrap_or(0), Act::ChooseBitrate, false);
    let codecs: Vec<String> = CODECS.iter().map(|c| if *c == Codec::Auto && auto.is_some() { trf!(tr().stream.auto_codec, codec = codec_label(auto.unwrap())) } else { codec_label(*c).to_string() }).collect();
    choice(cx, &mut layer, list, tr().stream.video, &codecs, CODECS.iter().position(|c| *c == set.codec).unwrap_or(0), Act::ChooseCodec, false);
    let note = cx.scene.add(col, s().col().mt(0.8 * r).done(), Node::default());
    wrap_text(
        cx,
        note,
        ty::SMALL,
        400,
        tok::MUTED,
        if st.guest { tr().stream.quality_note_guest } else { tr().stream.quality_note },
    );
    // Footer, per state.
    let add = |cx: &mut Cx, layer: &mut Layer, label: &str, v: Variant, hint: &str, act: Act, def: bool, disabled: bool| {
        let b = btn(cx, footer, label, v, hint, disabled, None);
        reg_btn(layer, b, act, 11, def, disabled);
    };
    if let Some(b) = st.busy.clone() {
        add(cx, &mut layer, &b, Variant::Primary, &b, Act::Play, true, true);
    } else if st.moonlight.is_none() {
        if st.installing.as_ref().is_some_and(|i| i.error.is_none()) {
            add(cx, &mut layer, tr().stream.getting_moonlight, Variant::Primary, tr().stream.getting_moonlight, Act::Play, true, true);
        } else {
            add(cx, &mut layer, tr().stream.set_up_streaming, Variant::Primary, tr().stream.set_up_streaming, Act::SetupFromApp, true, false);
        }
    } else {
        match state {
            PlayState::Playing | PlayState::OnPc => {
                add(cx, &mut layer, tr().common.resume, Variant::Primary, tr().common.resume, Act::Resume, true, false);
                add(cx, &mut layer, tr().stream.quit_game, Variant::Secondary, tr().stream.quit_game, Act::QuitGame, false, false);
            }
            PlayState::Busy => {
                let l = trf!(tr().stream.quit_and_play, game = other.clone());
                add(cx, &mut layer, &l, Variant::Primary, &l, Act::QuitAndPlay, true, false);
            }
            PlayState::Asleep => {
                let l = if waking { trf!(tr().stream.waking_host, name = host.name.clone()) } else { tr().stream.wake_and_play.to_string() };
                add(cx, &mut layer, &l, Variant::Primary, tr().stream.wake_and_play, Act::WakeAndPlay, true, waking);
            }
            PlayState::Offline => add(cx, &mut layer, tr().stream.look_again, Variant::Secondary, tr().stream.look_again, Act::LookAgainApp, true, false),
            PlayState::Ready => add(cx, &mut layer, tr().common.play, Variant::Primary, tr().common.play, Act::Play, true, false),
        }
    }
    // The quality rows sit above the footer in focus order only by position; default is the footer.
    Some(layer)
}
