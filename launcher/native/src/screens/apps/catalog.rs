//! The Apps catalog (launcher/src/apps/catalog.ts over catalog.json, shared with the native side).
//! `status_for` mirrors the native side's planning so fixtures show the same states.

use rare_render::Src;
use serde::Deserialize;
use std::sync::OnceLock;

use rare_i18n::tr;

static CATALOG_JSON: &str = include_str!("catalog.json");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Controller {
    Focus,
    Pointer,
    Gamepad,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WayKind {
    Webview,
    ChromeApp,
    Chrome,
    Flatpak,
    Exec,
    Steam,
    Battlenet,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Way {
    pub webview: Option<String>,
    pub chrome_app: Option<String>,
    pub chrome: Option<bool>,
    pub flatpak: Option<String>,
    pub exec: Option<Vec<String>>,
    /// A steam:// URL: Steam's own UI, an app like the others.
    pub steam: Option<String>,
    /// Blizzard's Battle.net app through umu-run in its prefix; Rare installs it when missing.
    pub battlenet: Option<bool>,
    pub label: String,
    pub controller: Option<Controller>,
    /// "escape" or "alt+left".
    pub back: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub about: String,
    #[serde(default)]
    pub drm: bool,
    /// What playing needs (a subscription, an account), in plain words.
    #[serde(default)]
    pub plan: Option<String>,
    /// Plain setup steps (the streaming apps), numbered on the app's page and in Library > Streaming.
    #[serde(default)]
    pub steps: Vec<String>,
    pub ways: Vec<Way>,
}

impl AppEntry {
    /// What the app is, in the player's language (the JSON text is English and the fallback for
    /// an app the catalog doesn't know).
    pub fn about_text(&self) -> &str {
        let a = tr().apps;
        match self.id.as_str() {
            "steam" => a.about_steam,
            "battlenet" => a.about_battlenet,
            "telegram" => a.about_telegram,
            "x" => a.about_x,
            "netflix" => a.about_netflix,
            "youtube" => a.about_youtube,
            "spotify" => a.about_spotify,
            "twitch" => a.about_twitch,
            "discord" => a.about_discord,
            "xcloud" => a.about_xcloud,
            "geforcenow" => a.about_geforcenow,
            "chiaki" => a.about_chiaki,
            "steamlink" => a.about_steamlink,
            "chrome" => a.about_chrome,
            _ => &self.about,
        }
    }

    /// What playing needs, in the player's language.
    pub fn plan_text(&self) -> Option<&str> {
        let a = tr().apps;
        match self.id.as_str() {
            "battlenet" => Some(a.plan_battlenet),
            "xcloud" => Some(a.plan_xcloud),
            "geforcenow" => Some(a.plan_geforcenow),
            "chiaki" => Some(a.plan_chiaki),
            "steamlink" => Some(a.plan_steamlink),
            _ => self.plan.as_deref(),
        }
    }

    /// The setup steps, in the player's language.
    pub fn step_texts(&self) -> Vec<&str> {
        let a = tr().apps;
        match self.id.as_str() {
            "chiaki" => vec![a.chiaki_step_1, a.chiaki_step_2, a.chiaki_step_3],
            "steamlink" => vec![a.steamlink_step_1, a.steamlink_step_2, a.steamlink_step_3, a.steamlink_step_4],
            _ => self.steps.iter().map(|s| s.as_str()).collect(),
        }
    }
}

/// The cloud app a Library cloud tile opens (`cloud:xbox:<id>`, `cloud:gfn:<id>`).
pub fn cloud_app(game_id: &str) -> Option<&'static AppEntry> {
    let rest = game_id.strip_prefix("cloud:")?;
    app_by_id(match rest.split(':').next()? {
        "xbox" => "xcloud",
        "gfn" => "geforcenow",
        _ => return None,
    })
}

/// What `Offer::flatpak` is when the offer is Battle.net (native `catalog::BATTLENET`): Blizzard's
/// installer through umu-run, with no one-line command.
pub const BATTLENET: &str = "battlenet";

/// Something to install first, with the exact one-line command (native `Offer`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub flatpak: String,
    pub name: String,
    pub command: String,
}

/// An app on this device (native `Status`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppStatus {
    pub id: String,
    /// The way A opens, or None when something must be installed first.
    pub way: Option<usize>,
    pub opens_in: Option<String>,
    pub other_ways: Vec<usize>,
    pub offer: Option<Offer>,
    pub controller: Option<Controller>,
}

#[derive(Deserialize)]
struct File {
    apps: Vec<AppEntry>,
}

/// The catalog, parsed once.
pub fn apps() -> &'static [AppEntry] {
    static APPS: OnceLock<Vec<AppEntry>> = OnceLock::new();
    APPS.get_or_init(|| serde_json::from_str::<File>(CATALOG_JSON).expect("catalog.json").apps)
}

pub fn app_by_id(id: &str) -> Option<&'static AppEntry> {
    apps().iter().find(|a| a.id == id)
}

/// Apps run as "app:<id>" in the launcher's running list.
pub fn run_id(id: &str) -> String {
    format!("app:{id}")
}

pub fn app_by_run_id(rid: &str) -> Option<&'static AppEntry> {
    rid.strip_prefix("app:").and_then(app_by_id)
}

/// A way's name for the player: brands stay as written.
pub fn way_label(w: &Way) -> String {
    if w.steam.as_deref() == Some("steam://open/games") {
        tr().apps.steam_library.to_string()
    } else {
        w.label.clone()
    }
}

pub fn way_kind(w: &Way) -> Option<WayKind> {
    let mut set = Vec::new();
    if w.webview.is_some() {
        set.push(WayKind::Webview);
    }
    if w.chrome_app.is_some() {
        set.push(WayKind::ChromeApp);
    }
    if w.chrome == Some(true) {
        set.push(WayKind::Chrome);
    }
    if w.flatpak.is_some() {
        set.push(WayKind::Flatpak);
    }
    if w.exec.as_ref().is_some_and(|e| !e.is_empty()) {
        set.push(WayKind::Exec);
    }
    if w.steam.is_some() {
        set.push(WayKind::Steam);
    }
    if w.battlenet == Some(true) {
        set.push(WayKind::Battlenet);
    }
    if set.len() == 1 {
        Some(set[0])
    } else {
        None
    }
}

pub fn controller_of(w: &Way) -> Controller {
    w.controller.unwrap_or(match way_kind(w) {
        Some(WayKind::Webview) => Controller::Focus,
        Some(WayKind::Steam) => Controller::Gamepad,
        _ => Controller::Pointer,
    })
}

struct Browser {
    name: &'static str,
    flatpak: &'static str,
    drm: bool,
}

/// Browsers, best first (native BROWSERS). drm: has Widevine.
const BROWSERS: [Browser; 4] = [
    Browser { name: "Google Chrome", flatpak: "com.google.Chrome", drm: true },
    Browser { name: "Microsoft Edge", flatpak: "com.microsoft.Edge", drm: true },
    Browser { name: "Chromium", flatpak: "org.chromium.Chromium", drm: false },
    Browser { name: "Brave", flatpak: "com.brave.Browser", drm: false },
];

pub fn install_command(flatpak: &str, user_flathub: bool) -> String {
    let install = format!("flatpak install --user -y flathub {flatpak}");
    if user_flathub {
        install
    } else {
        format!("flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo && {install}")
    }
}

enum Need {
    Ready,
    Flatpak(String),
    Never,
}

/// The same answer as the native `status`, from a list of installed Flatpaks (and `battlenet`
/// when Battle.net is installed).
pub fn status_for(app: &AppEntry, installed: &[&str], user_flathub: bool) -> AppStatus {
    let browser = BROWSERS.iter().find(|b| (!app.drm || b.drm) && installed.contains(&b.flatpak));
    let need = |w: &Way| -> Need {
        match way_kind(w) {
            Some(WayKind::Webview) | Some(WayKind::Steam) => Need::Ready,
            Some(WayKind::ChromeApp) | Some(WayKind::Chrome) => {
                if browser.is_some() {
                    Need::Ready
                } else {
                    Need::Flatpak("com.google.Chrome".into())
                }
            }
            Some(WayKind::Flatpak) => {
                let f = w.flatpak.as_deref().unwrap();
                if installed.contains(&f) {
                    Need::Ready
                } else {
                    Need::Flatpak(f.into())
                }
            }
            Some(WayKind::Battlenet) => {
                if installed.contains(&BATTLENET) {
                    Need::Ready
                } else {
                    Need::Flatpak(BATTLENET.into())
                }
            }
            _ => Need::Never,
        }
    };
    let ready: Vec<usize> = app.ways.iter().enumerate().filter(|(_, w)| matches!(need(w), Need::Ready)).map(|(i, _)| i).collect();
    let way = ready.first().copied();
    let offer = match need(&app.ways[0]) {
        Need::Flatpak(f) if f == BATTLENET => Some(Offer { flatpak: f, name: "Battle.net".into(), command: String::new() }),
        Need::Flatpak(f) => {
            let name = BROWSERS
                .iter()
                .find(|b| b.flatpak == f)
                .map(|b| b.name.to_string())
                .or_else(|| app.ways.iter().find(|w| w.flatpak.as_deref() == Some(f.as_str())).map(way_label))
                .unwrap_or_else(|| f.clone());
            Some(Offer { command: install_command(&f, user_flathub), flatpak: f, name })
        }
        _ => None,
    };
    let w = way.map(|i| &app.ways[i]);
    let k = w.and_then(way_kind);
    AppStatus {
        id: app.id.clone(),
        way,
        opens_in: w.map(|w| match (k, browser) {
            (Some(WayKind::ChromeApp) | Some(WayKind::Chrome), Some(b)) => b.name.to_string(),
            _ => way_label(w),
        }),
        other_ways: ready.iter().skip(1).copied().collect(),
        offer,
        controller: w.map(controller_of),
    }
}

/// One row of the controller layout. `b` is a pad button ("a", "lt", "start"...), or a stick's name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlRow {
    pub b: &'static str,
    pub does: &'static str,
}

const fn cr(b: &'static str, does: &'static str) -> ControlRow {
    ControlRow { b, does }
}

pub fn controls_for(controller: Controller, way: Option<&Way>) -> Vec<ControlRow> {
    let a = tr().apps;
    let k = way.and_then(way_kind);
    let back = way
        .and_then(|w| w.back.as_deref())
        .unwrap_or(if matches!(k, Some(WayKind::ChromeApp) | Some(WayKind::Chrome)) { "alt+left" } else { "escape" });
    match controller {
        // Chrome itself (the Browser, with tabs): rare_launch apps::pointer's browser layout.
        Controller::Pointer if k == Some(WayKind::Chrome) => vec![
            cr("Right stick", a.ctl_pointer),
            cr("a", a.ctl_click),
            cr("lt", a.ctl_right_click),
            cr("Left stick", a.ctl_scrolls),
            cr("b", a.ctl_back_page),
            cr("D-pad", a.ctl_dpad_back_forward),
            cr("lb", a.ctl_prev_tab),
            cr("rb", a.ctl_next_tab),
            cr("start", a.ctl_new_tab),
            cr("select", a.ctl_address_bar),
            cr("x", a.ctl_close_tab),
            cr("y", a.ctl_keyboard),
        ],
        Controller::Pointer => vec![
            cr("Right stick", a.ctl_pointer),
            cr("a", a.ctl_click),
            cr("lt", a.ctl_right_click),
            cr("Left stick", a.ctl_scrolls),
            cr("D-pad", a.ctl_arrow_keys),
            cr("b", if back == "alt+left" { a.ctl_back_page } else { a.ctl_back_escape }),
            cr("x", a.ctl_space),
            cr("y", a.ctl_keyboard),
            cr("start", a.ctl_enter),
        ],
        Controller::Focus => vec![
            cr("D-pad", a.ctl_moves_buttons),
            cr("a", a.ctl_select),
            cr("b", a.ctl_back_escape),
            cr("select", a.ctl_keyboard),
            cr("Right stick", a.ctl_moves_a_pointer),
        ],
        Controller::Gamepad => vec![cr("D-pad", a.ctl_app_reads)],
    }
}

// ---- icons (launcher/public/apps) ----

macro_rules! icons {
    ($($f:literal),*) => {
        /// The icon as the service ships it (an apple-touch icon).
        pub fn icon_src(path: &str) -> Option<Src> {
            match path {
                $( concat!("apps/", $f) => Some(Src::Static(concat!("apps/", $f), include_bytes!(concat!("../../../../public/apps/", $f)))), )*
                _ => None,
            }
        }
    };
}
icons!("steam.png", "battlenet.png", "telegram.png", "x.png", "netflix.png", "youtube.png", "spotify.png", "twitch.png", "discord.png", "xcloud.png", "geforcenow.png", "chiaki.png", "steamlink.png", "chrome.png");

#[cfg(test)]
mod tests {
    use super::*;

    fn get(id: &str) -> &'static AppEntry {
        apps().iter().find(|a| a.id == id).unwrap()
    }

    fn public(p: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../public").join(p)
    }

    #[test]
    fn has_the_media_row_each_app_valid() {
        let ids: Vec<&str> = apps().iter().map(|a| a.id.as_str()).collect();
        for want in ["steam", "telegram", "x", "netflix", "chrome", "youtube", "discord", "spotify", "twitch"] {
            assert!(ids.contains(&want), "{want}");
        }
        let mut u = ids.clone();
        u.sort();
        u.dedup();
        assert_eq!(u.len(), apps().len());
        for a in apps() {
            assert!(public(&a.icon).exists(), "{} icon {}", a.id, a.icon);
            assert!(icon_src(&a.icon).is_some(), "{} icon embedded", a.id);
            assert!(!a.ways.is_empty(), "{}", a.id);
            for w in &a.ways {
                let k = way_kind(w);
                assert!(k.is_some(), "{}: a way sets exactly one kind", a.id);
                assert!(!w.label.is_empty(), "{}", a.id);
                if a.drm {
                    assert!(matches!(k, Some(WayKind::ChromeApp) | Some(WayKind::Chrome)), "{}: DRM opens in Chrome", a.id);
                }
                if controller_of(w) == Controller::Focus {
                    assert_eq!(k, Some(WayKind::Webview), "{}: focus is for Rare's webview", a.id);
                }
                for u in [w.webview.as_ref(), w.chrome_app.as_ref()].into_iter().flatten() {
                    assert!(u.starts_with("https://"), "{u}");
                }
            }
        }
        assert!(public("apps/NOTICE").exists());
    }

    #[test]
    fn maps_run_ids() {
        assert_eq!(run_id("netflix"), "app:netflix");
        assert_eq!(app_by_run_id("app:netflix").map(|a| a.name.as_str()), Some("Netflix"));
        assert!(app_by_run_id("steam:1").is_none());
    }

    #[test]
    fn plans_like_the_native_side() {
        // Netflix needs Chrome with Widevine; Chromium is not enough.
        let n = status_for(get("netflix"), &["org.chromium.Chromium"], true);
        assert_eq!(n.way, None);
        assert_eq!(n.offer, Some(Offer { flatpak: "com.google.Chrome".into(), name: "Google Chrome".into(), command: "flatpak install --user -y flathub com.google.Chrome".into() }));
        let n = status_for(get("netflix"), &["com.google.Chrome"], true);
        assert_eq!((n.way, n.opens_in.as_deref(), n.controller, n.offer), (Some(0), Some("Google Chrome"), Some(Controller::Pointer), None));
        // Telegram: the desktop app when installed, else Telegram Web with the app on offer.
        let t = status_for(get("telegram"), &[], true);
        assert_eq!((t.way, t.opens_in.as_deref(), t.controller), (Some(1), Some("Telegram Web"), Some(Controller::Focus)));
        assert_eq!(t.offer.unwrap().flatpak, "org.telegram.desktop");
        let t = status_for(get("telegram"), &["org.telegram.desktop"], true);
        assert_eq!((t.way, t.opens_in.as_deref(), t.other_ways, t.offer), (Some(0), Some("Telegram Desktop"), vec![1], None));
        // X opens in Rare; Chrome becomes a second way once installed.
        assert!(status_for(get("x"), &[], true).other_ways.is_empty());
        assert_eq!(status_for(get("x"), &["com.google.Chrome"], true).other_ways, vec![1]);
        // Battle.net: Rare installs it (no Flatpak, no command), then it opens with the pad as a mouse.
        let b = status_for(get("battlenet"), &[], true);
        assert_eq!((b.way, b.offer), (None, Some(Offer { flatpak: BATTLENET.into(), name: "Battle.net".into(), command: String::new() })));
        let b = status_for(get("battlenet"), &[BATTLENET], true);
        assert_eq!((b.way, b.opens_in.as_deref(), b.controller, b.offer), (Some(0), Some("Battle.net"), Some(Controller::Pointer), None));
        assert_eq!(
            install_command("org.telegram.desktop", false),
            "flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo && flatpak install --user -y flathub org.telegram.desktop"
        );
    }

    #[test]
    fn describes_the_controls() {
        let back = |id: &str| controls_for(Controller::Pointer, Some(&get(id).ways[0])).into_iter().find(|r| r.b == "b").unwrap().does;
        assert_eq!(back("netflix"), "Back a page");
        assert_eq!(back("telegram"), "Back (Escape)");
        assert!(controls_for(Controller::Focus, None).iter().any(|r| r.b == "select"));
        // Chrome itself: tabs and the address bar on the pad.
        let c = controls_for(Controller::Pointer, Some(&get("chrome").ways[0]));
        let does = |b: &str| c.iter().find(|r| r.b == b).map(|r| r.does);
        assert_eq!(does("start"), Some("New tab"));
        assert_eq!(does("select"), Some("Address bar"));
        assert_eq!(does("x"), Some("Close tab"));
        assert_eq!(does("b"), Some("Back a page"));
    }
}
