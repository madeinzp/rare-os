//! Epic Games and GOG in the native launcher, over rare-stores (legendary and gogdl): sign-in from
//! Settings > Accounts in the web window (the store's own login page; the sign-in code is taken
//! from the page as it finishes), the owned lists kept fresh, Install with a drive choice into the
//! downloads queue, Update, Check files, cloud saves (fetched before a game starts, sent after it closes).
//!
//! Every action is an `Act::Legacy("stores:...")` the app hands to `act`:
//! `stores:signin:<epic|gog>`, `stores:signout:<store>`, `stores:install:<id>` (the drive sheet),
//! `stores:install-to:<id>|<drive root>`, `stores:pause:<id>`, `stores:resume:<id>`,
//! `stores:cancel:<id>`, `stores:update:<id>`, `stores:verify:<id>`, `stores:sync:<id>`,
//! `stores:menu:<id>` (a Downloads row's sheet).

use crate::act::Act;
use crate::state::{OptionItem, OptionsSpec, State, Tone};
use rare_stores::jobs::{self, Job, Kind, Phase};
use rare_stores::{drives, saves, Store};
use rare_library::storage::{self, DriveKind};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use rare_i18n::{tr, trf};

/// Moves on every store change (jobs, accounts, owned lists, updates); pages put it in their stamp.
pub fn ver() -> u64 {
    rare_stores::version().wrapping_add(LOCAL.load(Ordering::Relaxed) << 32)
}

/// Sizes, cloud saves and updates learned here.
static LOCAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn bump() {
    LOCAL.fetch_add(1, Ordering::Relaxed);
    super::bus::wake();
}

/// An Epic or GOG game id.
pub fn is_store(id: &str) -> bool {
    Store::of(id).is_some()
}

#[derive(Default)]
struct Known {
    /// Installed games with an update waiting.
    updates: HashSet<String>,
    /// (download, disk) bytes before an install; None while asked or unknown.
    sizes: HashMap<String, Option<(u64, u64)>>,
    /// Has cloud saves (asked once per game page).
    cloud: HashMap<String, bool>,
}

fn known() -> &'static Mutex<Known> {
    static K: OnceLock<Mutex<Known>> = OnceLock::new();
    K.get_or_init(Default::default)
}

fn library_changed() {
    super::bus::ctx().send(rare_core::Event::LibraryChanged);
}

/// Once at start: change notices to the frame loop, the finished-job notices, and the owned lists
/// and update checks in the background (after start-up, then every six hours).
pub fn boot() {
    rare_stores::set_waker(Box::new(super::bus::wake));
    jobs::on_done(Box::new(|job: &Job, r: &Result<(), String>| {
        match r {
            Ok(()) => {
                let line = match job.kind {
                    Kind::Install { .. } => trf!(tr().features_more.stores_finished_downloading, title = job.title),
                    Kind::Update => trf!(tr().features_more.stores_up_to_date, title = job.title),
                    Kind::Repair => trf!(tr().features_more.stores_files_fixed, title = job.title),
                    Kind::Saves => trf!(tr().features_more.stores_saves_synced, title = job.title),
                    Kind::Setup => trf!(tr().features_more.stores_is_set_up, title = job.title),
                };
                if matches!(job.kind, Kind::Update | Kind::Repair) {
                    known().lock().unwrap().updates.remove(&job.id);
                }
                if matches!(job.kind, Kind::Install { .. }) {
                    // A finished install is a notice (features/downloads.rs).
                    super::bus::ctx().send(rare_core::Event::Other { name: "store-installed".into(), payload: serde_json::json!({ "title": job.title }) });
                } else {
                    super::bus::note(line, false);
                }
            }
            Err(e) => super::bus::note(format!("{}: {e}", job.title), true),
        }
        library_changed();
    }));
    // Screenshots: RR_STORE_JOB=<id>|<title>|<install|update|repair|setup>|<percent> shows a job.
    if let Ok(v) = std::env::var("RR_STORE_JOB") {
        let p: Vec<&str> = v.split('|').collect();
        if p.len() == 4 {
            let kind = match p[2] {
                "update" => Kind::Update,
                "repair" => Kind::Repair,
                "setup" => Kind::Setup,
                _ => Kind::Install { base: "/tmp".into() },
            };
            let pct = p[3].parse().unwrap_or(40.0);
            let total = 12_300_000_000u64;
            jobs::show(Job { id: p[0].into(), title: p[1].into(), setup: kind == Kind::Setup, kind, phase: Phase::Running, pct, done: (total as f64 * pct as f64 / 100.0) as u64, total, speed: 24_600_000.0, eta: Some(400), verifying: false });
        }
    }
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let _ = std::thread::Builder::new().name("stores-refresh".into()).spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        loop {
            refresh_all();
            std::thread::sleep(Duration::from_secs(6 * 3600));
        }
    });
}

/// The owned lists and the update checks for the stores signed in.
fn refresh_all() {
    let mut changed = false;
    for store in [Store::Epic, Store::Gog] {
        if !store.signed_in() {
            continue;
        }
        let r = match store {
            Store::Epic => rare_stores::epic::refresh_owned(),
            Store::Gog => rare_stores::gog::refresh_owned(),
        };
        match r {
            Ok(n) => {
                log::info!("{}: {n} games owned", store.name());
                changed = true;
            }
            Err(e) => log::warn!("{} owned list: {e}", store.name()),
        }
        let ups = match store {
            Store::Epic => rare_stores::epic::updates().map(|v| v.into_iter().map(|a| format!("epic:{a}")).collect::<Vec<_>>()),
            Store::Gog => rare_stores::gog::updates().map(|v| v.into_iter().map(|a| format!("gog:{a}")).collect::<Vec<_>>()),
        };
        if let Ok(ups) = ups {
            let mut k = known().lock().unwrap();
            k.updates.retain(|id| !id.starts_with(store.key()));
            k.updates.extend(ups);
        }
    }
    if changed {
        library_changed();
    }
    bump();
}

pub fn has_update(id: &str) -> bool {
    known().lock().unwrap().updates.contains(id)
}

/// (download, disk) bytes for an install, asked in the background the first time.
pub fn size(id: &str) -> Option<(u64, u64)> {
    let mut k = known().lock().unwrap();
    if let Some(s) = k.sizes.get(id) {
        return *s;
    }
    k.sizes.insert(id.to_string(), None);
    let id = id.to_string();
    std::thread::spawn(move || {
        let s = match Store::of(&id) {
            Some((Store::Epic, app)) => rare_stores::epic::size(app),
            Some((Store::Gog, gid)) => rare_stores::gog::info(gid).ok().map(|(d, k, _)| (d, k)),
            None => None,
        };
        if s.is_some() {
            known().lock().unwrap().sizes.insert(id, s);
            bump();
        }
    });
    None
}

/// Whether an installed game has cloud saves (asked in the background the first time).
pub fn cloud(id: &str) -> bool {
    let mut k = known().lock().unwrap();
    if let Some(c) = k.cloud.get(id) {
        return *c;
    }
    k.cloud.insert(id.to_string(), false);
    let id = id.to_string();
    std::thread::spawn(move || {
        if saves::supported(&id) {
            known().lock().unwrap().cloud.insert(id, true);
            bump();
        }
    });
    false
}

pub fn job(id: &str) -> Option<Job> {
    jobs::get(id)
}

/// The downloads queue's rows (id, title, done, total, paused), for the launcher's downloads list.
pub fn downloads() -> Vec<(String, String, u64, u64, bool)> {
    jobs::downloads()
}

/// "Installing 42% · 1.2 GB of 12.3 GB · 8 min left"
pub fn job_line(j: &Job) -> String {
    let mut parts = vec![match j.phase {
        Phase::Running if j.pct > 0.0 => format!("{} {}%", status_text(j), whole_pct(j)),
        _ => status_text(j),
    }];
    if j.setup {
        // Installers run one by one; there are no bytes or times to show.
        parts.push(tr().features_more.stores_installing_needs.into());
        return parts.join(" \u{b7} ");
    }
    if matches!(j.phase, Phase::Running | Phase::Paused) && j.total > 0 && !j.verifying {
        parts.push(trf!(tr().features_more.stores_bytes_of, done = rare_stores::fmt_bytes(j.done), total = rare_stores::fmt_bytes(j.total)));
    }
    if j.phase == Phase::Running {
        if let Some(eta) = j.eta.filter(|e| *e > 0) {
            parts.push(if eta >= 3600 { trf!(tr().features_more.stores_eta_hours, h = eta / 3600, m = (eta % 3600) / 60) } else { trf!(tr().features_more.stores_eta_min, n = (eta / 60).max(1)) });
        }
    }
    parts.join(" \u{b7} ")
}

/// Before an Epic or GOG game starts (worker thread): its first-start setup when something is
/// missing, then newer cloud saves (never blocks the start for long and never stops it). Err stops
/// the start: the game can't run without its prefix and Proton.
pub fn before_launch(id: &str, title: &str) -> Result<(), String> {
    if !is_store(id) {
        return Ok(());
    }
    // The prefix, a Proton and umu's runtime (rare-stores prep: a first umu-run downloads Proton
    // and the runtime, minutes with no window, which inside the game's start read as "didn't
    // start" and Try again stopped the download), and for GOG the runtimes the game lists, once
    // per prefix. Both normally happen right after the install; a game from Heroic, or a setup
    // that failed then, gets them here. The start waits; the step shows in Downloads.
    let prep = rare_stores::prep::needed(id);
    let redist = match Store::of(id) {
        Some((Store::Gog, gid)) => !rare_stores::redist::needed(gid, &rare_stores::redist::deps_of(gid)).is_empty(),
        _ => false,
    };
    if prep || redist {
        super::bus::note(trf!(tr().features_more.stores_first_start, title = title), false);
        if let Err(e) = jobs::run_now(id, title, Kind::Setup) {
            log::warn!("setup before {id}: {e}");
            if rare_stores::prep::needed(id) {
                return Err(e);
            }
        }
    }
    let id2 = id.to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(saves::before_launch(&id2));
    });
    match rx.recv_timeout(Duration::from_secs(25)) {
        Ok(Err(e)) => {
            log::warn!("saves before {id}: {e}");
            super::bus::note(tr().features_more.stores_cloud_fetch_failed, true);
        }
        Err(_) => log::warn!("saves before {id}: still syncing, starting anyway"),
        Ok(Ok(())) => {}
    }
    Ok(())
}

/// What ran last time (to see Epic and GOG games close).
static RUNNING: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Backend events: an Epic or GOG game that closed sends its saves.
pub fn on_event(ev: &rare_core::Event) {
    let rare_core::Event::GameState(r) = ev else { return };
    let mut prev = RUNNING.lock().unwrap();
    let gone: Vec<String> = prev.iter().filter(|id| is_store(id) && !r.running.contains(id)).cloned().collect();
    *prev = r.running.clone();
    drop(prev);
    for id in gone {
        std::thread::spawn(move || {
            // Let the game finish writing.
            std::thread::sleep(Duration::from_secs(3));
            if let Err(e) = saves::after_exit(&id) {
                log::warn!("saves after {id}: {e}");
                super::bus::note(tr().features_more.stores_cloud_send_failed, true);
            }
        });
    }
}

fn sign_in(store: Store) -> Result<(), String> {
    super::webgame::install();
    let (url, script) = match store {
        Store::Epic => (rare_stores::epic::LOGIN_URL, rare_stores::epic::LOGIN_SCRIPT),
        Store::Gog => (rare_stores::gog::LOGIN_URL, ""),
    };
    let got = Arc::new(AtomicBool::new(false));
    let g = got.clone();
    let nav = Arc::new(move |u: &str| -> bool {
        let Some(code) = rare_stores::login_code(store, u) else { return true };
        if !g.swap(true, Ordering::Relaxed) {
            std::thread::spawn(move || {
                rare_webgame::close_external();
                super::bus::note(format!("Signing in to {}", store.name()), false);
                let r = match store {
                    Store::Epic => rare_stores::epic::sign_in(&code),
                    Store::Gog => rare_stores::gog::sign_in(&code),
                };
                match r {
                    Ok(name) => {
                        super::bus::note(format!("Signed in to {} as {name}. Your games are in the Library.", store.name()), false);
                        refresh_all();
                    }
                    Err(e) => super::bus::note(e, true),
                }
            });
        }
        false
    });
    let closed = Arc::new(|| super::bus::wake());
    rare_webgame::open_sign_in(&format!("Sign in to {}", store.name()), url, script, nav, closed)
}

fn store_named(s: &str) -> Option<Store> {
    match s {
        "epic" => Some(Store::Epic),
        "gog" => Some(Store::Gog),
        _ => None,
    }
}

/// A job's percent as its downloads row shows it: bytes done of the total when the size is known
/// (`jobs::downloads` gives the row those), else the tool's percent; whole, rounded down. legendary's
/// own percent counts chunks, so at 936 MB of 54.8 GB the row read 1 % and the line 2 %.
fn whole_pct(j: &Job) -> u32 {
    let p = if j.total > 0 { j.done.min(j.total) as f64 * 100.0 / j.total as f64 } else { j.pct as f64 };
    p.clamp(0.0, 100.0).floor() as u32
}

/// A download's status in the UI language: rare-stores words it in English (`Job::status`).
/// A failure rare-stores words itself (`jobs::friendly`) is translated; any other error text, from
/// the tool, stays as it came.
pub fn status_text(j: &Job) -> String {
    let t = tr();
    match &j.phase {
        Phase::Queued => t.features.dl_queued.into(),
        Phase::Paused => t.features.dl_paused.into(),
        Phase::Failed(e) => match e.as_str() {
            "Not enough space on that drive." => t.features_more.stores_err_space.into(),
            "Sign in again in Settings > Accounts." => t.features_more.stores_err_sign_in.into(),
            "No connection. Check Wi-Fi and try again." => t.features_more.stores_err_offline.into(),
            "It stopped with an error. Try again." => t.features_more.stores_err_generic.into(),
            _ => e.clone(),
        },
        Phase::Running if j.setup => t.game.setting_up.into(),
        Phase::Running if j.verifying => t.features_more.verify_checking.into(),
        Phase::Running => match j.kind {
            Kind::Install { .. } => t.library.installing.into(),
            Kind::Update => t.features.dl_updating.into(),
            Kind::Repair => t.features_more.stores_repairing.into(),
            Kind::Saves => t.features_more.stores_syncing_saves.into(),
            Kind::Setup => t.game.setting_up.into(),
        },
    }
}

/// The drive sheet for an install: every drive Settings > Storage lists, with its free space.
fn install_sheet(st: &mut State, id: &str) {
    let Some((store, _)) = Store::of(id) else { return };
    let title = st.game(id).map(|g| g.title.clone()).unwrap_or_default();
    let need = size(id).map(|s| s.1);
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default();
    let items: Vec<OptionItem> = storage::drives()
        .into_iter()
        .map(|d| {
            let short = need.is_some_and(|n| n > d.free);
            let label = if short { trf!(tr().features_more.stores_drive_short, drive = d.label, free = rare_stores::fmt_bytes(d.free)) } else { trf!(tr().features_more.stores_drive_free, drive = d.label, free = rare_stores::fmt_bytes(d.free)) };
            let root = if d.kind == DriveKind::Internal { home.clone() } else { d.mount.clone() };
            OptionItem { label, on: None, act: Act::Legacy(format!("stores:install-to:{id}|{}", drives::games_dir(&root, store).display())) }
        })
        .collect();
    let sub = match need {
        Some(n) => trf!(tr().features_more.stores_install_where_needs, size = rare_stores::fmt_bytes(n)),
        None => tr().features_more.stores_install_where.into(),
    };
    st.options = Some(OptionsSpec { title, sub: Some(sub), items });
}

/// Runs a `stores:` action. True when it was one.
pub fn act(st: &mut State, a: &str, now: f64) -> bool {
    let Some(rest) = a.strip_prefix("stores:") else { return false };
    let (verb, arg) = rest.split_once(':').unwrap_or((rest, ""));
    let title = |st: &State, id: &str| st.game(id).map(|g| g.title.clone()).unwrap_or_else(|| id.to_string());
    match verb {
        "signin" => {
            let Some(store) = store_named(arg) else { return true };
            if let Err(e) = sign_in(store) {
                log::warn!("sign-in window: {e}");
                st.toast(format!("Couldn't open the {} sign-in. Try again.", store.name()), Tone::Error, now);
            }
        }
        "signout" => {
            let Some(store) = store_named(arg) else { return true };
            std::thread::spawn(move || {
                let r = match store {
                    Store::Epic => rare_stores::epic::sign_out(),
                    Store::Gog => rare_stores::gog::sign_out(),
                };
                match r {
                    Ok(()) => super::bus::note(format!("Signed out of {}. Installed games stay.", store.name()), false),
                    Err(e) => {
                        log::warn!("sign out: {e}");
                        super::bus::note(format!("Signed out of {}.", store.name()), false)
                    }
                }
                library_changed();
            });
        }
        "install" => install_sheet(st, arg),
        "menu" => row_sheet(st, arg),
        "install-to" => {
            let Some((id, dir)) = arg.split_once('|') else { return true };
            let t = title(st, id);
            match jobs::add(id, &t, Kind::Install { base: dir.into() }) {
                Ok(()) => st.toast(trf!(tr().features_more.stores_in_downloads, title = t), Tone::Info, now),
                Err(e) => st.toast(e, Tone::Error, now),
            }
        }
        "update" | "verify" => {
            let t = title(st, arg);
            let kind = if verb == "update" { Kind::Update } else { Kind::Repair };
            if let Err(e) = jobs::add(arg, &t, kind) {
                st.toast(e, Tone::Error, now);
            }
        }
        "pause" => jobs::pause(arg),
        "resume" => jobs::resume(arg),
        "cancel" => jobs::cancel(arg),
        "sync" => {
            let id = arg.to_string();
            let t = title(st, &id);
            st.toast(trf!(tr().features_more.stores_syncing, title = t), Tone::Info, now);
            std::thread::spawn(move || match saves::sync_now(&id) {
                Ok(()) => super::bus::note(trf!(tr().features_more.stores_saves_synced, title = t), false),
                Err(e) => {
                    log::warn!("sync {id}: {e}");
                    super::bus::note(tr().features_more.stores_sync_failed, true)
                }
            });
        }
        _ => {}
    }
    true
}

/// Settings > Accounts: (store, its row's text, signed in).
pub fn account_rows() -> Vec<(Store, String, bool)> {
    let a = rare_stores::accounts();
    [(Store::Epic, a.epic), (Store::Gog, a.gog)]
        .into_iter()
        .map(|(store, name)| match name {
            Some(n) => (store, trf!(tr().features_more.stores_signed_in_as, name = n, store = store.short()), true),
            None => (store, trf!(tr().features_more.stores_sign_in_to_see, store = store.short()), false),
        })
        .collect()
}

// ---------------------------------------------------------------- the Downloads view's rows

/// A download's row in the Downloads view (features/downloads.rs), the same look as Steam's rows.
/// A opens its sheet (Pause or Resume, Cancel, Game page).
pub fn download_row(ui: &mut crate::kit::Ui, st: &State, acts: &mut crate::page::Acts, j: &Job, def: bool) -> rare_render::view::El {
    use crate::kit::{self, s, tok, Ty, T};
    use crate::screens::game::parts::{self, Variant};
    let g = st.game(&j.id);
    let sources: Vec<String> = g.map(|g| if g.art.hero.is_empty() { g.art.cover.clone() } else { g.art.hero.clone() }).unwrap_or_default();
    let art = ui.bx(s().wh(8.6, 4.0)).bg(tok::L3).radius(ui.r(0.4)).clip().child(parts::art(ui, &sources, &j.title, Variant::Header, 8.6, 4.0, 0.5));
    let live = j.phase == Phase::Running;
    let pct = if j.pct > 0.0 { format!("{}%", j.pct.floor() as u32) } else { String::new() };
    let head = ui
        .bx(s().row().items_center().gap(0.75))
        .child(ui.text_s(s().grow().min0(), j.title.clone(), T::new(Ty::Body).w(600).trunc()))
        .child(ui.text(status_text(j), T::new(Ty::Small).c(tok::MUTED)))
        .child(ui.text_s(s().w(3.2), pct, T::new(Ty::Small).w(600).tab().right()));
    let mut line = job_line(j);
    if live && j.speed > 0.0 {
        line = format!("{line} \u{b7} {}/s", rare_stores::fmt_bytes(j.speed as u64));
    }
    let col = ui
        .bx(s().col().grow().min0().gap(0.45))
        .child(head)
        .child(super::downloads::bar(ui, None, j.pct / 100.0, !live))
        .child(ui.text(line, T::new(Ty::Small).c(tok::MUTED).tab().trunc()));
    let el = ui.bx(s().row().items_center().gap(1.1).h(5.5).wp(1.0).px(0.85)).radius(ui.r(0.5)).child(art).child(col);
    kit::nav(kit::row_look(el), acts.game(Act::Legacy(format!("stores:menu:{}", j.id)), &j.id), Some(tr().common.options), None, def).key(rare_render::view::key(&j.id))
}

/// The sheet behind a download row.
fn row_sheet(st: &mut State, id: &str) {
    let Some(j) = jobs::get(id) else { return };
    let mut items = vec![match j.phase {
        Phase::Running | Phase::Queued => OptionItem { label: tr().features_more.stores_pause.into(), on: None, act: Act::Legacy(format!("stores:pause:{id}")) },
        _ => OptionItem { label: tr().common.resume.into(), on: None, act: Act::Legacy(format!("stores:resume:{id}")) },
    }];
    items.push(OptionItem { label: if matches!(j.kind, Kind::Install { .. }) { tr().features_more.stores_cancel_install.into() } else { tr().common.cancel.into() }, on: None, act: Act::Legacy(format!("stores:cancel:{id}")) });
    if st.game(id).is_some() {
        items.push(OptionItem { label: tr().features_more.stores_game_page.into(), on: None, act: Act::OpenGame(id.to_string()) });
    }
    st.options = Some(OptionsSpec { title: j.title.clone(), sub: Some(status_text(&j)), items });
}

/// The Rare menu's Downloads line for Epic and GOG ("2 downloading, 45%"), None when idle.
pub fn menu_line() -> Option<String> {
    let list: Vec<Job> = jobs::list().into_iter().filter(|j| matches!(j.phase, Phase::Running | Phase::Queued)).collect();
    let first = list.first()?;
    Some(if list.len() == 1 { format!("{}, {}%", status_text(first), whole_pct(first)) } else { trf!(tr().features_more.stores_n_downloading, n = list.len(), pct = whole_pct(first)) })
}

/// A finished Epic or GOG install: the "finished downloading" notice Steam's installs post.
pub fn on_installed(st: &mut State, payload: &serde_json::Value) {
    let title = payload.get("title").and_then(|t| t.as_str()).unwrap_or(tr().features_more.stores_a_game);
    st.feat.downloads.notices.push(rare_notify::LocalNotice::new(rare_notify::types::NoticeKind::DownloadDone, trf!(tr().features_more.stores_finished_downloading, title = title), rare_notify::types::NoticeAction::Downloads));
}
