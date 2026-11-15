//! Rare cloud saves on the game page (features/cloudsync.rs does the work): the Saves card, the
//! Saves sheet (status, turn on, versions to restore) and the conflict sheet that asks which save
//! to keep when this device and the cloud both changed.

use crate::act::Act;
use rare_i18n::{tr, trf};
use crate::data::model::Game;
use crate::features::cloudsync::{self as cs, Keep, VersionRow};
use crate::kit::{self, s, tok, Btn as B, DotTone, Ty, Ui, T};
use crate::page::{hash, Acts, Btn, Fx, Page};
use crate::state::State;
use rare_render::focus::Focusable;
use rare_render::view::El;

/// The game page's code for the Saves card.
pub const ACT_SAVES: u32 = 9000;

fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// (year, day of year, hour, minute, day of month, month 0-11) in local time.
fn local(ms: i64) -> (i32, i32, i32, i32, i32, i32) {
    let t = (ms / 1000) as libc::time_t;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&t, &mut tm) };
    (tm.tm_year, tm.tm_yday, tm.tm_hour, tm.tm_min, tm.tm_mday, tm.tm_mon)
}

/// "today 21:40", "yesterday 18:02", "12 Oct 09:15".
pub fn when(ms: i64, now: i64) -> String {
    let (y, d, h, m, md, mo) = local(ms);
    let (ny, nd, ..) = local(now);
    let hm = format!("{h:02}:{m:02}");
    if (y, d) == (ny, nd) {
        trf!(tr().game.today_at, time = hm)
    } else if (y == ny && d + 1 == nd) || (y + 1 == ny && nd == 0 && d >= 364) {
        trf!(tr().game.yesterday_at, time = hm)
    } else {
        let e = &tr().emulators;
        let mon = [e.month_jan, e.month_feb, e.month_mar, e.month_apr, e.month_may, e.month_jun, e.month_jul, e.month_aug, e.month_sep, e.month_oct, e.month_nov, e.month_dec];
        trf!(tr().game.date_at, day = md, month = mon[mo.clamp(0, 11) as usize], time = hm)
    }
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// (value, sub) for the card and the sheet's status line.
fn status(g: &Game) -> (String, Option<String>, DotTone) {
    let now = now_ms();
    if let Some(b) = cs::busy(&g.id) {
        return (b.to_string(), None, DotTone::Primary);
    }
    let Some(v) = cs::game(&g.id) else { return (tr().game.checking.into(), None, DotTone::Muted) };
    if v.sets == 0 {
        return (tr().game.saves_not_found.into(), Some(tr().game.saves_not_found_sub.into()), DotTone::Muted);
    }
    if !cs::is_on() {
        let sub = v.versions.first().map(|r| trf!(tr().game.kept_here, when = when(r.created, now)));
        return (tr().common.off.into(), sub.or(Some(tr().game.cloud_saves_off.into())), DotTone::Muted);
    }
    if v.conflict {
        return (tr().game.cloud_conflict.into(), Some(tr().game.changed_both.into()), DotTone::Attention);
    }
    if let Some(e) = cs::error(&g.id) {
        return (tr().game.cloud_not_synced.into(), Some(e), DotTone::Attention);
    }
    if v.pending {
        return (tr().game.cloud_not_synced.into(), Some(tr().game.uploads_after.into()), DotTone::Muted);
    }
    (tr().game.cloud_synced.into(), v.synced_at.map(|t| trf!(tr().game.synced_when, when = when(t, now))), DotTone::Muted)
}

/// The Saves card (games whose saves Rare syncs itself). A opens the Saves sheet.
pub fn card(ui: &mut Ui, acts: &mut Acts, g: &Game) -> Option<El> {
    if !cs::eligible(g) {
        return None;
    }
    let (v, sub, _) = status(g);
    Some(kit::info_card(ui, tr().game.cloud_saves, &v, sub.as_deref(), Some(acts.tag(Act::Local(ACT_SAVES)))))
}

pub fn open(st: &mut State, g: &Game) {
    cs::open_versions(st, &g.id);
}

fn reason(r: &VersionRow) -> &'static str {
    match r.reason.as_str() {
        "played" => tr().game.reason_played,
        "download" => tr().game.reason_download,
        "restore" => tr().game.reason_restore,
        "conflict" => tr().game.reason_conflict,
        "cloud" => tr().game.reason_cloud,
        "restored" => tr().game.reason_restored,
        _ => tr().game.reason_kept,
    }
}

fn place(r: &VersionRow) -> &'static str {
    match (r.local.is_some(), r.cloud.is_some()) {
        (true, true) => tr().game.place_both,
        (true, false) => tr().game.this_device,
        _ => tr().game.cloud,
    }
}

// ---------------------------------------------------------------- the Saves sheet

const V_ON: u32 = 1;
const V_CONFLICT: u32 = 2;
const V_FUNDS: u32 = 3;
const V_ROW: u32 = 100;

#[derive(Default)]
pub struct VersionsPage {
    /// the row armed for restore (second A restores)
    armed: Option<usize>,
    rows: Vec<VersionRow>,
    asked: bool,
}

impl Page for VersionsPage {
    fn stamp(&self, st: &State) -> u64 {
        hash((cs::ver(), st.ver.library, self.armed, cs::sheet_game()))
    }

    fn tick(&mut self, st: &mut State, _fx: &mut Fx, _now: f64) -> Option<f64> {
        if !self.asked {
            if let Some(g) = cs::sheet_game().and_then(|id| st.game(&id).cloned()) {
                self.asked = true;
                cs::load(st, &g);
            }
        }
        None
    }

    fn view(&mut self, ui: &mut Ui, st: &State, acts: &mut Acts) -> El {
        let Some(g) = cs::sheet_game().and_then(|id| st.game(&id).cloned()) else { return ui.bx(s()) };
        let now = now_ms();
        let (value, sub, tone) = status(&g);
        let line = match &sub {
            Some(sub) => trf!(tr().game.status_line_sub, value = value, sub = sub),
            None => trf!(tr().game.status_line, value = value),
        };
        let mut body: Vec<El> = Vec::new();
        body.push(ui.bx(s().row().items_center().gap(0.6)).child(kit::dot(ui, s(), tone, cs::busy(&g.id).is_some())).child(ui.text(&line, T::new(Ty::Small).c(tok::fg(0.8)).wrap())));
        let view = cs::game(&g.id);
        let mut actions: Vec<El> = Vec::new();
        if !cs::is_on() {
            body.push(ui.text_s(s().mt(0.75).max_w(40.0), tr().game.turn_on_body, T::new(Ty::Small).c(tok::MUTED).wrap()));
            let label = if cs::unlocking() { tr().game.waiting_wallet } else { tr().game.turn_on_cloud_saves };
            actions.push(kit::button(ui, B::new(label, acts.tag(Act::Local(V_ON))).primary().def(true).disabled(cs::unlocking())));
        } else {
            if view.as_ref().is_some_and(|v| v.conflict) {
                actions.push(kit::button(ui, B::new(tr().game.choose_save, acts.tag(Act::Local(V_CONFLICT))).primary().def(true)));
            }
            if let Some(f) = cs::funds() {
                let n = cs::uploads_left(f);
                let t = if n == 0 { tr().game.no_funds.to_string() } else { trf!(tr().game.funds_about, n = n) };
                body.push(ui.text_s(s().mt(0.4), &t, T::new(Ty::Small).c(tok::MUTED)));
                if n < 5 {
                    actions.push(kit::button(ui, B::new(tr().game.add_funds, acts.tag(Act::Local(V_FUNDS)))));
                }
            }
        }
        if !actions.is_empty() {
            body.push(ui.bx(s().row().gap(0.5).mt(1.0)).kids(actions));
        }
        self.rows = view.map(|v| v.versions).unwrap_or_default();
        body.push(ui.text_s(s().mt(1.5).mb(0.5), tr().game.versions, T::new(Ty::Body).w(600)));
        if self.rows.is_empty() {
            body.push(ui.text(if cs::game(&g.id).is_none() { tr().game.looking_for_saves } else { tr().game.no_copies }, T::new(Ty::Small).c(tok::MUTED)));
        }
        let mut list: Vec<El> = Vec::new();
        for (i, r) in self.rows.iter().enumerate() {
            let armed = self.armed == Some(i);
            let mut title = cap(&when(r.created, now));
            if !r.set_label.is_empty() {
                title = format!("{} \u{b7} {title}", r.set_label);
            }
            let meta = format!("{} \u{b7} {} \u{b7} {}", reason(r), crate::screens::sui::saves::fmt_bytes(r.bytes), place(r));
            let right = if r.current { tr().game.current } else if armed { tr().game.press_again } else { "" };
            let row = ui
                .bx(s().row().items_center().gap(1.0).px(1.0).py(0.7).wp(1.0))
                .radius(ui.r(0.5))
                .child(ui.bx(s().col().grow()).child(ui.text(&title, T::new(Ty::Body).w(500).trunc())).child(ui.text(&meta, T::new(Ty::Small).c(tok::MUTED).trunc())))
                .child(ui.text_s(s().min_w(12.0), right, T::new(Ty::Small).c(if armed { tok::FG } else { tok::MUTED }).right()));
            let hint = if r.current { None } else { Some(tr().game.restore) };
            list.push(kit::row_look(kit::nav(row, acts.tag(Act::Local(V_ROW + i as u32)), hint, None, i == 0 && cs::is_on() && !view_conflict(&g))));
        }
        body.push(ui.bx(s().col().gap(0.25)).kids(list));
        let footer = vec![kit::button(ui, B::new(tr().common.close, acts.tag(Act::Close)).hint(tr().common.close))];
        kit::sheet(ui, &trf!(tr().game.saves_title, game = g.title), Some(tr().game.saves_sub), body, footer, true, 1)
    }

    fn act(&mut self, st: &mut State, _fx: &mut Fx, code: u32) {
        let Some(id) = cs::sheet_game() else { return };
        match code {
            V_ON => cs::turn_on(),
            V_CONFLICT => cs::ask_conflict(st, &id),
            V_FUNDS => cs::add_funds(),
            c if c >= V_ROW => {
                let i = (c - V_ROW) as usize;
                let Some(r) = self.rows.get(i).cloned() else { return };
                let now = rare_render::clock::secs();
                if r.current {
                    return st.toast(tr().game.save_in_use, crate::state::Tone::Info, now);
                }
                if cs::busy(&id).is_some() {
                    return st.toast(tr().game.wait_sync, crate::state::Tone::Info, now);
                }
                if self.armed == Some(i) {
                    self.armed = None;
                    cs::restore(st, &id, &r);
                } else {
                    self.armed = Some(i);
                }
            }
            _ => {}
        }
    }

    fn pad(&mut self, _st: &mut State, _fx: &mut Fx, b: Btn, _cur: Option<(&Focusable, Option<&Act>)>) -> bool {
        if b != Btn::A {
            self.armed = None;
        }
        false
    }
}

fn view_conflict(g: &Game) -> bool {
    cs::game(&g.id).is_some_and(|v| v.conflict)
}

// ---------------------------------------------------------------- the conflict sheet

const C_DEVICE: u32 = 1;
const C_CLOUD: u32 = 2;
const C_LATER: u32 = 3;

#[derive(Default)]
pub struct ConflictPage;

fn side(ui: &mut Ui, acts: &mut Acts, code: u32, head: &str, date: &str, size: &str, def: bool) -> El {
    let card = ui
        .bx(s().col().grow().px(1.25).py(1.0).gap(0.25))
        .bg(tok::L1)
        .radius(ui.r(0.75))
        .child(ui.text(head, T::new(Ty::Small).w(500).c(tok::MUTED)))
        .child(ui.text(&cap(date), T::new(Ty::Title).w(600).tab()))
        .child(ui.text(size, T::new(Ty::Small).c(tok::MUTED)));
    kit::row_look(kit::nav(card, acts.tag(Act::Local(code)), Some(if code == C_DEVICE { tr().game.keep_this } else { tr().game.use_this }), None, def))
}

impl Page for ConflictPage {
    fn stamp(&self, _st: &State) -> u64 {
        hash(cs::ver())
    }

    fn view(&mut self, ui: &mut Ui, _st: &State, acts: &mut Acts) -> El {
        let Some(c) = cs::conflict() else { return ui.bx(s()) };
        let now = now_ms();
        let local_t = c.sets.iter().filter_map(|s| s.local.modified).max().unwrap_or(now);
        let cloud_t = c.sets.iter().map(|s| s.cloud.meta.modified.unwrap_or(s.cloud.meta.created)).max().unwrap_or(now);
        let local_b: u64 = c.sets.iter().map(|s| s.local.bytes).sum();
        let cloud_b: u64 = c.sets.iter().map(|s| s.cloud.meta.raw.unwrap_or(s.cloud.meta.bytes)).sum();
        let newer_here = local_t >= cloud_t;
        let line = trf!(tr().game.conflict_line, device = when(local_t, now), cloud = when(cloud_t, now));
        let sides = ui
            .bx(s().row().gap(1.0).wp(1.0).mt(1.0))
            .child(side(ui, acts, C_DEVICE, tr().game.this_device, &when(local_t, now), &crate::screens::sui::saves::fmt_bytes(local_b), newer_here))
            .child(side(ui, acts, C_CLOUD, tr().game.cloud, &when(cloud_t, now), &crate::screens::sui::saves::fmt_bytes(cloud_b), !newer_here));
        let body = vec![
            ui.text(&line, T::new(Ty::Body).c(tok::fg(0.85)).wrap()),
            sides,
            ui.text_s(s().mt(1.0), tr().game.conflict_note, T::new(Ty::Small).c(tok::MUTED).wrap()),
        ];
        let footer = vec![kit::button(ui, B::new(tr().signin.not_now, acts.tag(Act::Local(C_LATER))).hint(tr().signin.not_now))];
        kit::sheet(ui, tr().game.conflict_title, Some(&trf!(tr().game.conflict_sub, game = c.title)), body, footer, false, 1)
    }

    fn act(&mut self, st: &mut State, _fx: &mut Fx, code: u32) {
        match code {
            C_DEVICE => cs::resolve(st, Some(Keep::Device)),
            C_CLOUD => cs::resolve(st, Some(Keep::Cloud)),
            C_LATER => cs::resolve(st, None),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_read_plainly() {
        let now = now_ms();
        assert!(when(now, now).starts_with("today "));
        assert!(when(now - 86_400_000, now).starts_with("yesterday "));
        let old = when(now - 40 * 86_400_000, now);
        assert!(!old.starts_with("today") && !old.starts_with("yesterday") && old.len() >= 10, "{old}");
        assert_eq!(cap("today 21:40"), "Today 21:40");
    }
}
