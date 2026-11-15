//! The game page's Manage row (features/gamemgmt): Performance (this game's own power, frame
//! limit, upscaling, render size and refresh rate, in a sheet), Check files with its progress in
//! plain words on the same line, Rename (the keyboard) and Art (the art page).

use crate::act::Act;
use rare_i18n::{tr, trf};
use crate::data::model::Game;
use crate::features::gamemgmt::verify::{self, Phase, Status};
use crate::features::gamemgmt::GamePerf;
use crate::kit::{self, s, tok, Btn as B, BtnIcon, Ty, Ui, T};
use crate::page::{Acts, Btn, Feedback, Fx};
use crate::state::{State, Tone};
use rare_core::Profile;
use rare_library::custom::ArtKind;
use rare_render::focus::Group;
use rare_render::view::{key, El};
use rare_system::gamefx::{FgStatus, Filters, FrameGen};
use rare_system::power::GamePower;
use rare_system::upscale::{Mode, Upscale};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// `Act::Local` codes from 2000 are this file's (panels.rs has 1..399, settings.rs 1000..1299).
pub const BASE: u32 = 2000;
const ACT_PERF: u32 = 2000;
const ACT_CHECK: u32 = 2001;
const ACT_RENAME: u32 = 2002;
const ACT_ART: u32 = 2003;
const ACT_KEEP: u32 = 2004;
const ACT_LOSSLESS: u32 = 2005;
const ACT_CHIP: u32 = 2100;
const ACT_RESET: u32 = 2290;
const ACT_DONE: u32 = 2291;
/// The Performance sheet's focus scope (over the page, under Options and the keyboard).
const SCOPE: u32 = 3;
const RENAME_KEY: &str = "gm-rename";

#[derive(Default)]
pub struct Manage {
    sheet: bool,
    perf: Arc<Mutex<Option<GamePerf>>>,
    ver: Arc<AtomicU64>,
    asked: bool,
    verify: Option<Status>,
    /// The Performance button's tag in the last build, and whether focus goes back to it now
    /// (the sheet just closed: its focused chip is gone, so without this the focus engine waits
    /// for its fallback and the page sits still for a few frames).
    perf_tag: std::cell::Cell<Option<u32>>,
    refocus: bool,
}

/// Steam games use Steam's own check; games outside the stores Rare's file list, unless their
/// folder is a general one (an .exe picked from Downloads).
fn can_check(g: &Game) -> bool {
    // Nothing to check before it is installed.
    if !g.installed {
        return false;
    }
    if g.source == "steam" {
        return !g.foreign && g.id.starts_with("steam:");
    }
    if !matches!(g.source.as_str(), "local" | "itch" | "rare") {
        return false;
    }
    let Some(d) = g.install_dir.as_deref() else { return false };
    let home = rare_core::paths::home();
    let p = std::path::Path::new(d);
    let general = p == home || ["Downloads", "Desktop", "Games", "Documents"].iter().any(|x| p == home.join(x)) || p.parent().is_none_or(|x| x.as_os_str().len() <= 1);
    !general && rare_library::roots::mount_points().iter().all(|m| m != p)
}

const FPS: [Option<u32>; 5] = [None, Some(60), Some(30), Some(20), Some(15)];
fn modes() -> [(Mode, &'static str); 3] {
    [(Mode::Off, tr().common.off), (Mode::Fsr, "FSR"), (Mode::Nis, "NIS")]
}
fn sharp() -> [(u32, &'static str); 3] {
    [(4, tr().game.sharp_soft), (10, tr().game.sharp_medium), (16, tr().game.sharp_sharp)]
}
const SIZES: [(u32, u32, &str); 3] = [(0, 0, "1200p"), (1280, 800, "800p"), (960, 600, "600p")];
const HZ: [Option<u32>; 3] = [None, Some(60), Some(40)];
/// Frame generation (lsfg-vk): multipliers (1 is off) and flow scales.
fn mults() -> [(u8, &'static str); 3] {
    [(1, tr().common.off), (2, "2x"), (3, "3x")]
}
const FLOWS: [(u8, &str); 3] = [(50, "50%"), (75, "75%"), (100, "100%")];
/// Filters (vkBasalt): CAS sharpening and the colour LUT.
fn sharpen_names() -> [&'static str; 4] {
    [tr().common.off, tr().game.level_low, tr().game.sharp_medium, tr().game.level_high]
}
fn colour_names() -> [&'static str; 3] {
    [tr().game.colour_natural, tr().game.colour_vivid, tr().game.colour_bold]
}
/// Rows from here on are frame generation and filters (notes go under them).
const FX_ROWS: usize = 6;

/// One row of the sheet: label, chips, the one that is on, whether it can change, whether it shows.
struct Row {
    label: &'static str,
    chips: Vec<String>,
    on: Option<usize>,
    off: bool,
    hidden: bool,
}

fn row(label: &'static str, chips: Vec<String>, on: Option<usize>, off: bool) -> Row {
    Row { label, chips, on, off, hidden: false }
}

fn now_of(p: &GamePerf) -> (GamePower, Upscale) {
    let g = p.saved.unwrap_or(GamePower::DEFAULT);
    (g, g.upscale.unwrap_or(p.global_upscale))
}

fn rows(p: &GamePerf) -> Vec<Row> {
    let (g, u) = now_of(p);
    let w = crate::kit::format::watts;
    let mut power = vec![trf!(tr().game.power_quiet_w, watts = w(p.watts[0])), trf!(tr().game.power_balanced_w, watts = w(p.watts[1])), trf!(tr().game.power_performance_w, watts = w(p.watts[2]))];
    let power_on = match g.profile {
        Profile::Quiet => Some(0),
        Profile::Balanced => Some(1),
        Profile::Performance => Some(2),
        Profile::Custom => {
            power.push(trf!(tr().game.power_custom_w, watts = w(g.tdp.unwrap_or(15.0) as f32)));
            Some(3)
        }
    };
    let sharp_on = sharp().iter().enumerate().min_by_key(|(_, (v, _))| (*v as i64 - u.sharpness as i64).abs()).map(|(i, _)| i);
    let fg = g.framegen.unwrap_or_default();
    let fl = g.filters.unwrap_or_default();
    let fg_usable = matches!(p.fx.framegen, FgStatus::Ready | FgStatus::OldBranch);
    let strs = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    vec![
        row(tr().game.row_power, power, power_on, false),
        row(tr().game.row_frame_limit, FPS.iter().map(|f| f.map(|n| format!("{n} fps")).unwrap_or_else(|| tr().common.off.into())).collect(), FPS.iter().position(|f| *f == g.fps), false),
        row(tr().game.row_upscaling, modes().iter().map(|m| m.1.to_string()).collect(), modes().iter().position(|m| m.0 == u.mode), false),
        row(tr().game.row_sharpness, sharp().iter().map(|m| m.1.to_string()).collect(), sharp_on.filter(|_| u.mode != Mode::Off), u.mode == Mode::Off),
        row(tr().game.row_render_size, SIZES.iter().map(|m| m.2.to_string()).collect(), SIZES.iter().position(|m| m.0 == u.width && m.1 == u.height), false),
        row(tr().game.row_refresh_rate, HZ.iter().map(|h| h.map(|n| format!("{n} Hz")).unwrap_or_else(|| tr().game.screen_default.into())).collect(), HZ.iter().position(|h| *h == g.refresh), false),
        row(tr().game.row_frame_generation, mults().iter().map(|m| m.1.to_string()).collect(), Some(mults().iter().position(|m| m.0 == fg.mult.min(3)).unwrap_or(0)).filter(|_| fg_usable), !fg_usable),
        row(tr().game.row_flow_scale, FLOWS.iter().map(|m| m.1.to_string()).collect(), FLOWS.iter().position(|m| m.0 == fg.flow).filter(|_| fg_usable && fg.on()), !(fg_usable && fg.on())),
        row(tr().game.row_sharpening, strs(&sharpen_names()), Some(fl.sharpen.min(3) as usize).filter(|_| p.fx.filters), !p.fx.filters),
        row(tr().game.row_colour, strs(&colour_names()), Some(fl.color.min(2) as usize).filter(|_| p.fx.filters), !p.fx.filters),
        Row { hidden: p.fx.reshade == 0 && !fl.reshade, ..row("ReShade", strs(&[tr().common.off, tr().common.on]), Some(fl.reshade as usize).filter(|_| p.fx.filters), !p.fx.filters) },
    ]
}

/// The lines under frame generation and the filters: what they need, when changes apply.
fn fx_notes(p: &GamePerf) -> (Vec<&'static str>, bool) {
    let g = p.saved.unwrap_or(GamePower::DEFAULT);
    let mut v = Vec::new();
    let ask_steam = match p.fx.framegen {
        FgStatus::NoLayer => {
            v.push(tr().game.fx_no_layer);
            false
        }
        FgStatus::NoDll => {
            v.push(tr().game.fx_no_dll);
            true
        }
        FgStatus::OldBranch => {
            v.push(tr().game.fx_old_branch);
            true
        }
        FgStatus::Ready => {
            v.push(tr().game.fx_ready);
            false
        }
    };
    if p.fx.filters {
        v.push(tr().game.fx_filters_next);
        if p.fx.reshade == 0 && !g.filters.is_some_and(|f| f.reshade) {
            v.push(tr().game.fx_reshade_dir);
        }
    }
    (v, ask_steam)
}

/// A chip press: the game's settings after it.
fn pressed(p: &GamePerf, row: usize, i: usize) -> GamePower {
    let (g, u) = now_of(p);
    let profiles = [Profile::Quiet, Profile::Balanced, Profile::Performance];
    match row {
        0 => match profiles.get(i) {
            Some(pr) => g.with_perf(*pr, None, None),
            None => g,
        },
        1 => GamePower { fps: FPS[i], ..g },
        2 => GamePower { upscale: Some(Upscale { mode: modes()[i].0, ..u }), ..g },
        3 => GamePower { upscale: Some(Upscale { sharpness: sharp()[i].0, ..u }), ..g },
        4 => GamePower { upscale: Some(Upscale { width: SIZES[i].0, height: SIZES[i].1, ..u }), ..g },
        5 => GamePower { refresh: HZ[i], ..g },
        6 => GamePower { framegen: Some(FrameGen { mult: mults()[i].0, ..g.framegen.unwrap_or_default() }), ..g },
        7 => GamePower { framegen: Some(FrameGen { flow: FLOWS[i].0, ..g.framegen.unwrap_or_default() }), ..g },
        // Filters all off are no filters (nothing kept for them).
        8 => GamePower { filters: Some(Filters { sharpen: i as u8, ..g.filters.unwrap_or_default() }).filter(Filters::on), ..g },
        9 => GamePower { filters: Some(Filters { color: i as u8, ..g.filters.unwrap_or_default() }).filter(Filters::on), ..g },
        _ => GamePower { filters: Some(Filters { reshade: i == 1, ..g.filters.unwrap_or_default() }).filter(Filters::on), ..g },
    }
}

impl Manage {
    /// What the row and sheet show, for the page's stamp.
    pub fn stamp(&self) -> (bool, u64, Option<Status>) {
        (self.sheet, self.ver.load(Ordering::Relaxed), self.verify.clone())
    }

    pub fn sheet_open(&self) -> bool {
        self.sheet
    }

    fn load(&mut self, fx: &mut Fx, id: &str) {
        let (perf, ver, id) = (self.perf.clone(), self.ver.clone(), id.to_string());
        fx.spawn(move |b| {
            let p = b.game_perf(&id);
            Box::new(move |_, _| {
                *perf.lock().unwrap() = Some(p);
                ver.fetch_add(1, Ordering::Relaxed);
            })
        });
    }

    /// Timers: the check's progress, the keyboard's answer for Rename.
    pub fn tick(&mut self, st: &mut State, fx: &mut Fx, g: &Game, now: f64) -> Option<f64> {
        if !self.asked {
            self.asked = true;
            self.load(fx, &g.id);
        }
        if let Some(ans) = st.take_answer(RENAME_KEY) {
            if let Some(name) = ans {
                rename(fx, &g.id, name);
            }
        }
        let v = verify::status(&g.id);
        if v != self.verify {
            if let (Some(old), Some(new)) = (&self.verify, &v) {
                if old.busy() && !new.busy() {
                    st.toast(new.text.clone(), if new.phase == Phase::Failed { Tone::Error } else { Tone::Info }, now);
                }
            }
            self.verify = v;
        }
        self.verify.as_ref().filter(|s| s.busy()).map(|_| now + 0.5)
    }

    /// The Manage row, laid out like the page's other settings rows (settings.rs): Performance,
    /// Check files, Rename, Art, and the check's line at its end.
    pub fn section(&self, ui: &mut Ui, st: &State, acts: &mut Acts, g: &Game) -> El {
        let running = st.is_running(&g.id);
        let busy = self.verify.as_ref().is_some_and(|s| s.busy());
        let mut kids = vec![
            ui.text_s(s().mr(0.4).w(7.5), tr().game.manage, T::new(Ty::Small).w(600)),
            {
                let t = acts.tag(Act::Local(ACT_PERF));
                self.perf_tag.set(Some(t));
                kit::button(ui, B::new(tr().game.performance, t).ghost().hint(tr().game.performance_hint))
            },
        ];
        if can_check(g) {
            kids.push(kit::button(ui, B::new(tr().game.check_files, acts.tag(Act::Local(ACT_CHECK))).ghost().disabled(busy || running).hint(tr().game.check_files)));
        } else if g.installed && super::stores::managed(g) {
            // Epic and GOG: the store's own check, which fetches what is missing or broken (its
            // progress shows under the game's buttons).
            let checking = crate::features::stores::job(&g.id).is_some();
            kids.push(kit::button(ui, B::new(tr().game.check_files, acts.tag(Act::Legacy(format!("stores:verify:{}", g.id)))).ghost().disabled(checking || running).hint(tr().game.check_files)));
        }
        kids.push(kit::button(ui, B::new(tr().game.rename, acts.tag(Act::Local(ACT_RENAME))).ghost()));
        kids.push(kit::button(ui, B::new(tr().game.art, acts.tag(Act::Local(ACT_ART))).ghost().hint(tr().game.art_hint)));
        if self.verify.as_ref().is_some_and(|s| s.phase == Phase::Differs) {
            kids.push(kit::button(ui, B::new(tr().game.keep_good_copy, acts.tag(Act::Local(ACT_KEEP))).ghost()));
        }
        // The check's line sits at the end of the same row, so nothing moves when it shows.
        let failed = self.verify.as_ref().is_some_and(|v| v.phase == Phase::Failed);
        let line = self.verify.as_ref().map(|v| v.text.clone()).unwrap_or_default();
        let text = ui.text_s(s().min0().shrink().ml(0.5), line, T::new(Ty::Small).c(if failed { tok::ATTN } else { tok::MUTED }).trunc());
        ui.bx(s().row().items_center().gap(0.5).max_w(75.0)).group(Group::new("game-manage", [0.0; 4])).kids(kids).child(text.key(key("gm-line")))
    }

    /// The Performance sheet, over the page.
    pub fn sheet(&self, ui: &mut Ui, acts: &mut Acts, g: &Game) -> Option<El> {
        if !self.sheet {
            return None;
        }
        let p = self.perf.lock().unwrap().clone();
        let mut body = Vec::new();
        match &p {
            None => body.push(ui.text_s(s().h(16.0), tr().common.loading, T::new(Ty::Small).c(tok::MUTED))),
            Some(p) => {
                for (r, row) in rows(p).into_iter().enumerate() {
                    if row.hidden {
                        continue;
                    }
                    if r == FX_ROWS {
                        body.push(ui.text_s(s().mt(0.25).wp(1.0), tr().game.hz_note, T::new(Ty::Meta).c(tok::MUTED).wrap()).key(key("gm-perf-hz-note")));
                        body.push(ui.text_s(s().mt(1.0).mb(0.25), tr().game.fx_head, T::new(Ty::Label).w(500).c(tok::MUTED)).key(key("gm-perf-fx-head")));
                    }
                    let mut line = ui.bx(s().row().items_center().gap(0.5).h(2.75)).group(Group::new(&format!("gm-perf-{r}"), [0.0; 4]));
                    line = line.child(ui.text_s(s().w(8.5), row.label, T::new(Ty::Small).w(500).c(if row.off { tok::MUTED } else { tok::FG })));
                    for (i, c) in row.chips.iter().enumerate() {
                        let on = row.on == Some(i);
                        let b = B::new(c.clone(), acts.tag(Act::Local(ACT_CHIP + r as u32 * 10 + i as u32))).sm().disabled(row.off).def(r == 0 && on);
                        // The chosen chip carries a check, so it still reads as chosen under the focus ring.
                        line = line.child(kit::button(ui, if on { b.primary().icon(BtnIcon::Lucide("check", 0.9, false)) } else { b.ghost() }));
                    }
                    body.push(line.key(key(&("gm-perf-row", r))));
                }
                let (notes, ask_steam) = fx_notes(p);
                for (i, n) in notes.into_iter().enumerate() {
                    body.push(ui.text_s(s().mt(if i == 0 { 0.75 } else { 0.4 }).wp(1.0), n, T::new(Ty::Meta).c(tok::MUTED).wrap()).key(key(&("gm-perf-note", i))));
                }
                if ask_steam {
                    let b = kit::button(ui, B::new(tr().game.lossless_in_steam, acts.tag(Act::Local(ACT_LOSSLESS))).sm().ghost());
                    body.push(ui.bx(s().row().mt(0.6)).group(Group::new("gm-perf-ls", [0.0; 4])).child(b).key(key("gm-perf-ls")));
                }
            }
        }
        let saved = p.as_ref().is_some_and(|p| p.saved.is_some());
        let footer = vec![
            kit::button(ui, B::new(tr().common.done, acts.tag(Act::Local(ACT_DONE))).primary().def(p.is_none())),
            kit::button(ui, B::new(tr().game.use_rare_settings, acts.tag(Act::Local(ACT_RESET))).ghost().disabled(!saved)),
        ];
        let sub = trf!(tr().game.perf_sub, game = g.title);
        Some(kit::sheet(ui, tr().game.performance, Some(&sub), body, footer, true, SCOPE))
    }

    /// B and the buttons that would act behind the sheet. True when used.
    pub fn pad(&mut self, fx: &mut Fx, b: Btn) -> bool {
        if !self.sheet {
            return false;
        }
        match b {
            Btn::B => {
                fx.fb(Feedback::Back);
                self.close();
                true
            }
            Btn::Start | Btn::Select | Btn::X | Btn::Y | Btn::Lb | Btn::Rb | Btn::Lt | Btn::Rt => true,
            _ => false,
        }
    }

    pub fn act(&mut self, st: &mut State, fx: &mut Fx, g: &Game, code: u32) {
        match code {
            ACT_PERF => {
                self.sheet = true;
                self.load(fx, &g.id);
                self.ver.fetch_add(1, Ordering::Relaxed);
            }
            ACT_DONE => self.close(),
            ACT_RESET => self.save(fx, &g.id, None),
            ACT_LOSSLESS => {
                fx.fb(Feedback::Open);
                fx.spawn(|b| {
                    let r = b.steam_store(rare_system::gamefx::LOSSLESS_APPID);
                    Box::new(move |st: &mut State, now| {
                        if let Err(e) = r {
                            log::warn!("lossless: {e}");
                            st.toast(tr().game.steam_open_failed, Tone::Error, now);
                        }
                    })
                });
            }
            c if (ACT_CHIP..ACT_RESET).contains(&c) => {
                let Some(p) = self.perf.lock().unwrap().clone() else { return };
                let (row, i) = (((c - ACT_CHIP) / 10) as usize, ((c - ACT_CHIP) % 10) as usize);
                let r = rows(&p);
                let now = rare_render::clock::secs();
                let Some(rw) = r.get(row).filter(|r| i < r.chips.len()) else { return };
                if rw.on == Some(i) {
                    return st.toast(trf!(tr().game.already_set, label = rw.label, value = rw.chips[i]), Tone::Info, now);
                }
                if rw.off {
                    let why = match row {
                        3 => tr().game.why_sharpness,
                        7 => tr().game.why_flow,
                        6 => tr().game.why_framegen,
                        _ => tr().game.why_update,
                    };
                    return st.toast(why, Tone::Info, now);
                }
                fx.fb(Feedback::Toggle);
                self.save(fx, &g.id, Some(pressed(&p, row, i)));
            }
            ACT_CHECK | ACT_KEEP => {
                if st.is_running(&g.id) {
                    st.toast(tr().game.close_first, Tone::Info, rare_render::clock::secs());
                    return;
                }
                let (g, keep) = (g.clone(), code == ACT_KEEP);
                fx.spawn(move |b| {
                    let r = b.verify_start(&g, keep);
                    Box::new(move |st: &mut State, now| {
                        if let Err(e) = r {
                            st.toast(e, Tone::Error, now);
                        }
                    })
                });
            }
            ACT_RENAME => {
                st.ask_text(RENAME_KEY, tr().game.rename_title, g.title.clone(), Some(tr().game.rename_hint));
                fx.fb(Feedback::Open);
            }
            ACT_ART => crate::features::gamemgmt::artpage::open(st, &g.id, ArtKind::Cover),
            _ => {}
        }
    }

    fn close(&mut self) {
        self.sheet = false;
        self.refocus = true;
        self.ver.fetch_add(1, Ordering::Relaxed);
    }

    /// After a build: the Performance button once the sheet has closed.
    pub fn take_focus(&mut self) -> Option<u32> {
        if self.sheet || !std::mem::take(&mut self.refocus) {
            return None;
        }
        self.perf_tag.get()
    }

    /// Shows the change at once and saves it; the saved value comes back after.
    fn save(&mut self, fx: &mut Fx, id: &str, p: Option<GamePower>) {
        if let Some(cur) = self.perf.lock().unwrap().as_mut() {
            cur.saved = p;
        }
        self.ver.fetch_add(1, Ordering::Relaxed);
        let (perf, ver, id) = (self.perf.clone(), self.ver.clone(), id.to_string());
        fx.spawn(move |b| {
            let r = b.set_game_perf(&id, p);
            let fresh = b.game_perf(&id);
            Box::new(move |st: &mut State, now| {
                *perf.lock().unwrap() = Some(fresh);
                ver.fetch_add(1, Ordering::Relaxed);
                if let Err(e) = r {
                    log::warn!("game perf: {e}");
                    st.toast(tr().game.save_failed, Tone::Error, now);
                }
            })
        });
    }
}

/// Keeps a new name (empty: the original).
fn rename(fx: &mut Fx, id: &str, name: String) {
    let id = id.to_string();
    fx.spawn(move |b| {
        let mut c = b.game_custom(&id);
        let name = name.trim().to_string();
        c.title = (!name.is_empty()).then_some(name.clone());
        let r = b.set_game_custom(&id, c.clone());
        Box::new(move |st: &mut State, now| match r {
            Ok(()) if name.is_empty() => {
                st.rescan_at = Some(now);
                st.toast(tr().game.name_back, Tone::Info, now);
            }
            Ok(()) => {
                crate::features::gamemgmt::show_custom(st, &id, &c);
                st.toast(trf!(tr().game.renamed_to, name = name), Tone::Info, now);
            }
            Err(e) => {
                log::warn!("rename: {e}");
                st.toast(tr().game.rename_failed, Tone::Error, now);
            }
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perf(saved: Option<GamePower>) -> GamePerf {
        GamePerf { saved, global_upscale: Upscale { mode: Mode::Fsr, ..Default::default() }, watts: [10.0, 15.0, 25.0], fx: rare_system::gamefx::Status { framegen: FgStatus::Ready, filters: true, reshade: 0 } }
    }

    #[test]
    fn rows_show_what_applies() {
        let r = rows(&perf(None));
        assert_eq!(r[0].chips[1], "Balanced 15 W");
        assert_eq!(r[0].on, Some(1));
        assert_eq!(r[1].on, Some(0));
        // Nothing saved: Settings' upscaling is what the game gets.
        assert_eq!(r[2].on, Some(1));
        assert_eq!(r[5].on, Some(0));
        assert_eq!(r[5].chips[0], "Screen default");
    }

    #[test]
    fn presses_change_only_their_part() {
        let p = perf(None);
        let a = pressed(&p, 1, 2);
        assert_eq!(a.fps, Some(30));
        assert_eq!(a.profile, Profile::Balanced);
        let b = pressed(&perf(Some(a)), 2, 0);
        assert_eq!(b.upscale.map(|u| u.mode), Some(Mode::Off));
        assert_eq!(b.fps, Some(30));
        let c = pressed(&perf(Some(b)), 5, 2);
        assert_eq!(c.refresh, Some(40));
        let d = pressed(&perf(Some(c)), 0, 2);
        assert_eq!((d.profile, d.fps, d.refresh), (Profile::Performance, Some(30), Some(40)));
        // Sharpness can't change with upscaling off.
        assert!(rows(&perf(Some(b)))[3].off);
    }

    #[test]
    fn frame_generation_and_filters() {
        let p = perf(None);
        let r = rows(&p);
        assert_eq!((r[6].label, r[6].on, r[6].off), ("Frame generation", Some(0), false));
        // Flow scale waits for frame generation; ReShade shows only with shaders.
        assert!(r[7].off && r[10].hidden);
        let a = pressed(&p, 6, 2);
        assert_eq!(a.framegen, Some(FrameGen { mult: 3, flow: 75, perf: true }));
        let b = pressed(&perf(Some(a)), 7, 0);
        assert_eq!(b.framegen.unwrap().flow, 50);
        assert!(!rows(&perf(Some(b)))[7].off);
        let c = pressed(&perf(Some(b)), 8, 3);
        let c = pressed(&perf(Some(c)), 9, 1);
        assert_eq!(c.filters, Some(Filters { sharpen: 3, color: 1, reshade: false }));
        assert_eq!(c.framegen.unwrap().mult, 3);
        // Every filter back off: nothing kept for them.
        let d = pressed(&perf(Some(c)), 8, 0);
        assert_eq!(pressed(&perf(Some(d)), 9, 0).filters, None);
        // No Lossless Scaling: frame generation can't be picked, and the sheet says why.
        let mut none = perf(Some(c));
        none.fx.framegen = FgStatus::NoDll;
        assert!(rows(&none)[6].off && rows(&none)[6].on.is_none());
        assert!(fx_notes(&none).1);
        none.fx.reshade = 2;
        assert!(!rows(&none)[10].hidden);
    }
}
