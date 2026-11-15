//! The Steam Cloud card on a Steam game's page. Read from Steam's local files on a worker
//! (rare-library steam_cloud), so the page never waits on the disk; it shows "Checking" until the
//! answer lands and the card drops out for games without cloud files.

use crate::data::model::Game;
use crate::kit::{self, Ui};
use rare_i18n::{tr, trf};
use crate::page::Fx;
use rare_library::steam_cloud::{CloudInfo, CloudStatus};
use rare_render::view::El;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Cloud {
    info: Arc<Mutex<Option<CloudInfo>>>,
    ver: Arc<AtomicU64>,
}

impl Cloud {
    pub fn ver(&self) -> u64 {
        self.ver.load(Ordering::Relaxed)
    }
}

fn appid(g: &Game) -> Option<u32> {
    (g.source == "steam" && !g.foreign).then(|| g.id.strip_prefix("steam:")?.parse().ok()).flatten()
}

/// First look at the page: reads the cloud state off the UI thread.
pub fn load(c: &Cloud, fx: &mut Fx, g: &Game) {
    let Some(appid) = appid(g) else { return };
    let (info, ver) = (c.info.clone(), c.ver.clone());
    fx.spawn(move |_| {
        let roots = rare_library::steam::find_roots();
        let found = rare_library::steam_cloud::root_for(&roots, appid)
            .map(|r| rare_library::steam_cloud::info(r, appid))
            .unwrap_or(CloudInfo { status: CloudStatus::Unsupported, synced_at: None, files: 0, bytes: 0 });
        Box::new(move |_, _| {
            *info.lock().unwrap() = Some(found);
            ver.fetch_add(1, Ordering::Relaxed);
        })
    });
}

/// (value, sub text) for a status; None hides the card.
fn text(i: &CloudInfo) -> Option<(&'static str, Option<String>)> {
    let when = i.synced_at.map(|t| trf!(tr().game.last_sync, when = super::parts::when_ago(t)));
    match i.status {
        CloudStatus::Unsupported => None,
        CloudStatus::Synced => Some((tr().game.cloud_synced, when)),
        CloudStatus::OutOfDate => Some((tr().game.cloud_not_synced, when)),
        CloudStatus::Conflict => Some((tr().game.cloud_conflict, Some(tr().game.cloud_conflict_sub.into()))),
        CloudStatus::Off => Some((tr().common.off, None)),
    }
}

/// Whether a game has Steam Cloud files: a few file checks, decided before the page's first
/// paint (and remembered), so the card never appears and then drops out.
fn has_cloud(appid: u32) -> bool {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static ROOTS: OnceLock<Vec<rare_library::steam::SteamRoot>> = OnceLock::new();
    static SEEN: OnceLock<Mutex<HashMap<u32, bool>>> = OnceLock::new();
    let seen = SEEN.get_or_init(Default::default);
    if let Some(v) = seen.lock().unwrap().get(&appid) {
        return *v;
    }
    let roots = ROOTS.get_or_init(rare_library::steam::find_roots);
    let v = rare_library::steam_cloud::root_for(roots, appid).is_some();
    seen.lock().unwrap().insert(appid, v);
    v
}

/// The card, for Steam games with cloud files only. Its space is decided before the first paint:
/// "Checking" holds it until the status lands.
pub fn card(ui: &mut Ui, c: &Cloud, g: &Game) -> Option<El> {
    let id = appid(g)?;
    match c.info.lock().unwrap().as_ref() {
        None if has_cloud(id) => Some(kit::info_card(ui, tr().game.steam_cloud, tr().game.checking, None, None)),
        None => None,
        // Unsupported only when the files went away since the first look: keep the space.
        Some(i) => Some(match text(i) {
            Some((v, sub)) => kit::info_card(ui, tr().game.steam_cloud, v, sub.as_deref(), None),
            None => kit::info_card(ui, tr().game.steam_cloud, tr().game.cloud_not_used, None, None),
        })
        .filter(|_| has_cloud(id) || text(i).is_some()),
    }
}
