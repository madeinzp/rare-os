//! Games Rare runs through umu-run (Epic through legendary, GOG and other Windows builds): their
//! Wine prefix and their Proton.
//!
//! Prefix: Rare's own (`~/.local/share/rare/prefixes/<id>`), unless the game came from Heroic and
//! only Heroic's prefix exists (its GamesConfig `winePrefix`): then that one, so the saves made
//! under Heroic carry on.
//!
//! Proton: the player's per-game pick (`~/.config/rare/umu-proton.json`, by game id and Steam's
//! tool name, the names the game page's Proton row shows), else Valve's newest Proton when Steam
//! has one, else umu's own default. `protonpath` is what goes in PROTONPATH.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn read_json(p: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(p).ok()?).ok()
}

/// Rare's own prefix for a game.
pub fn rare_prefix(id: &str) -> PathBuf {
    dirs::data_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".local/share")).join("rare/prefixes").join(id.replace(':', "-"))
}

/// The prefix Heroic keeps for a game (`heroic/GamesConfig/<app>.json` winePrefix), when it exists.
pub fn heroic_prefix(app: &str) -> Option<PathBuf> {
    for c in crate::heroic::config_dirs() {
        let Some(v) = read_json(&c.join(format!("heroic/GamesConfig/{app}.json"))) else { continue };
        if let Some(p) = v.get(app).and_then(|g| g.get("winePrefix")).and_then(Value::as_str).filter(|p| !p.is_empty()) {
            let p = PathBuf::from(p.replace('~', &dirs::home_dir().unwrap_or_default().to_string_lossy()));
            if p.is_dir() {
                return Some(p);
            }
        }
    }
    None
}

/// The prefix a game runs in (see the module notes).
pub fn prefix(id: &str) -> PathBuf {
    let own = rare_prefix(id);
    if own.join("drive_c").is_dir() || own.join("pfx/drive_c").is_dir() {
        return own;
    }
    let app = id.split_once(':').map(|(_, a)| a).unwrap_or(id);
    heroic_prefix(app).unwrap_or(own)
}

/// Where drive_c is: the prefix itself (umu, Wine) or its `pfx` (a prefix Proton made directly).
pub fn wine_root(prefix: &Path) -> PathBuf {
    if !prefix.join("drive_c").is_dir() && prefix.join("pfx/drive_c").is_dir() {
        prefix.join("pfx")
    } else {
        prefix.to_path_buf()
    }
}

fn choices_path() -> PathBuf {
    dirs::config_dir().unwrap_or_default().join("rare/umu-proton.json")
}

/// The Proton the player picked for this game (Steam's tool name), if any.
pub fn choice(id: &str) -> Option<String> {
    read_json(&choices_path())?.get(id)?.as_str().map(String::from)
}

/// Picks one (None: back to the default).
pub fn set_choice(id: &str, tool: Option<&str>) -> Result<(), String> {
    if let Some(t) = tool {
        if tool_path(t).is_none() {
            return Err(format!("{t} is not installed."));
        }
    }
    let path = choices_path();
    let mut v = read_json(&path).unwrap_or_else(|| json!({}));
    if let Some(o) = v.as_object_mut() {
        match tool {
            Some(t) => {
                o.insert(id.to_string(), json!(t));
            }
            None => {
                o.remove(id);
            }
        }
    }
    if let Some(d) = path.parent() {
        let _ = fs::create_dir_all(d);
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(&v).unwrap_or_default()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

fn steam_root() -> Option<PathBuf> {
    let roots = crate::steam::find_roots();
    roots.iter().find(|r| r.primary).or(roots.first()).map(|r| r.path.clone())
}

/// Valve's Protons in the Steam libraries: (major, minor, folder).
fn valve(libs: &[PathBuf]) -> Vec<(u32, u32, PathBuf)> {
    let mut out = Vec::new();
    for lib in libs {
        let Ok(rd) = fs::read_dir(lib.join("steamapps/common")) else { continue };
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            let Some((a, b)) = n.strip_prefix("Proton ").and_then(|v| v.split_once('.')) else { continue };
            let (Ok(a), Ok(b)) = (a.parse::<u32>(), b.parse::<u32>()) else { continue };
            if e.path().join("proton").is_file() {
                out.push((a, b, e.path()));
            }
        }
    }
    out.sort();
    out
}

/// The folder of a Proton by Steam's tool name: `proton_N` (Valve's newest N.x),
/// `proton_experimental`, or a tool in compatibilitytools.d (by its compatibilitytool.vdf name).
pub fn tool_path(name: &str) -> Option<PathBuf> {
    let root = steam_root()?;
    let libs = crate::steam::registered_libraries(&root);
    if let Some(major) = name.strip_prefix("proton_").and_then(|m| m.parse::<u32>().ok()) {
        return valve(&libs).into_iter().rev().find(|(a, _, _)| *a == major).map(|(_, _, p)| p);
    }
    if name == "proton_experimental" {
        return libs.iter().map(|l| l.join("steamapps/common/Proton - Experimental")).find(|p| p.join("proton").is_file());
    }
    let rd = fs::read_dir(root.join("compatibilitytools.d")).ok()?;
    for e in rd.flatten() {
        let dir = e.path();
        let Ok(text) = fs::read_to_string(dir.join("compatibilitytool.vdf")) else { continue };
        if (e.file_name().to_string_lossy() == name || text.contains(&format!("\"{name}\""))) && dir.join("proton").is_file() {
            return Some(dir);
        }
    }
    None
}

/// The default: Valve's newest Proton (Steam's tool name), when Steam has one.
pub fn default_tool() -> Option<String> {
    let root = steam_root()?;
    valve(&crate::steam::registered_libraries(&root)).last().map(|(a, _, _)| format!("proton_{a}"))
}

/// Where umu-run keeps the Protons it downloads (its STEAM_COMPAT), whether or not Steam is installed.
pub fn umu_compat_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(".local/share/Steam/compatibilitytools.d")
}

/// The numbers in a tool's folder name, for ordering ("UMU-Proton-10.0-4" -> [10, 0, 4]).
fn version_key(name: &str) -> Vec<u64> {
    name.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).filter_map(|s| s.parse().ok()).collect()
}

/// The newest UMU-Proton (else GE-Proton) umu-run downloaded before, in `dir`.
pub fn newest_umu_proton(dir: &Path) -> Option<PathBuf> {
    let Ok(rd) = fs::read_dir(dir) else { return None };
    let mut found: Vec<(u8, Vec<u64>, PathBuf)> = rd
        .flatten()
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            let rank = if n.starts_with("UMU-Proton-") {
                1
            } else if n.starts_with("GE-Proton") {
                0
            } else {
                return None;
            };
            e.path().join("proton").is_file().then(|| (rank, version_key(&n), e.path()))
        })
        .collect();
    found.sort();
    found.pop().map(|(_, _, p)| p)
}

/// PROTONPATH for a game: its pick, else Valve's newest, else the newest Proton umu-run already
/// downloaded. None (nothing on the device yet) leaves umu to download UMU-Proton. With no
/// PROTONPATH umu asks GitHub for a newer UMU-Proton on every start and downloads it first, so a
/// game would show nothing for minutes whenever a new build came out.
pub fn protonpath(id: &str) -> Option<PathBuf> {
    choice(id).and_then(|t| tool_path(&t)).or_else(|| default_tool().and_then(|t| tool_path(&t))).or_else(|| newest_umu_proton(&umu_compat_dir()))
}

fn umu_local() -> PathBuf {
    dirs::data_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".local/share")).join("umu")
}

/// The Steam Runtime a Proton runs in, as umu-run names its folder: from the Proton's
/// `toolmanifest.vdf` ("require_tool_appid": 1628350 sniper for Proton 8 to 10, 4183110 steamrt4
/// for Proton 11).
pub fn runtime_of(proton: &Path) -> Option<&'static str> {
    let text = fs::read_to_string(proton.join("toolmanifest.vdf")).ok()?;
    let line = text.lines().find(|l| l.contains("require_tool_appid"))?;
    let appid: String = line.split('"').filter(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit())).collect();
    match appid.as_str() {
        "1391110" => Some("steamrt2"),
        "1628350" => Some("steamrt3"),
        "4183110" => Some("steamrt4"),
        _ => None,
    }
}

/// umu's Steam Runtime for this Proton is on the device (`~/.local/share/umu/<steamrtN>`, set up
/// by umu-run's first start with it); with no Proton or manifest to go by, any runtime counts.
pub fn runtime_ready(proton: Option<&Path>) -> bool {
    let local = umu_local();
    match proton.and_then(runtime_of) {
        Some(rt) => local.join(rt).join("_v2-entry-point").is_file(),
        None => fs::read_dir(&local).map(|rd| rd.flatten().any(|e| e.path().join("_v2-entry-point").is_file())).unwrap_or(false),
    }
}

/// Everything a game's start needs before the game itself: its prefix, a Proton and umu's
/// runtime for it. When one is missing, umu-run downloads Proton (about 490 MB) and the runtime
/// and makes the prefix first, with no window for minutes; Rare does that as a setup step of its
/// own (rare-stores prep) instead of inside the game's start.
pub fn ready(id: &str) -> bool {
    let Some(proton) = protonpath(id) else { return false };
    wine_root(&prefix(id)).join("drive_c").is_dir() && runtime_ready(Some(&proton))
}

/// The log of a store game's setup and last start (`~/.cache/rare/<store>/<app>.log`): umu-run's,
/// Proton's and the store tool's output.
pub fn log_path(id: &str) -> PathBuf {
    let (store, app) = id.split_once(':').unwrap_or(("games", id));
    let app: String = app.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '_' }).collect();
    dirs::cache_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".cache")).join("rare").join(store).join(format!("{app}.log"))
}

/// A log's last words worth showing: umu-run's (or legendary's, gogdl's) ERROR/CRITICAL line
/// ("Could not download..."), without its logger prefix. umu-run with no Proton and no network
/// says only that PROTONPATH is empty ("Possible reason: ... or network error"), so that one is
/// put plainly.
pub fn last_error(log: &str) -> Option<String> {
    if log.contains("GE-Proton or UMU-Proton not found") || log.contains("Environment variable not set or is empty: PROTONPATH") {
        return Some("Proton couldn't be downloaded. Check Wi-Fi.".into());
    }
    let line = log.lines().rev().find(|l| l.contains("CRITICAL:") || l.contains("ERROR:"))?;
    let msg = line.split_once("CRITICAL:").or_else(|| line.split_once("ERROR:")).map(|(_, m)| m.trim()).unwrap_or(line.trim());
    let mut msg: String = msg.chars().take(160).collect();
    if msg.is_empty() {
        return None;
    }
    if !msg.ends_with('.') && !msg.ends_with('!') {
        msg.push('.');
    }
    Some(msg)
}

/// The log as a path to read out to the player: `~/...` under the home folder.
pub fn show_path(p: &Path) -> String {
    let home = dirs::home_dir().unwrap_or_default();
    match p.strip_prefix(&home) {
        Ok(rest) if !home.as_os_str().is_empty() => format!("~/{}", rest.display()),
        _ => p.display().to_string(),
    }
}

/// The environment umu-run needs for a game (prefix, Proton, store).
pub fn env(id: &str) -> Vec<(String, String)> {
    let mut v = vec![("WINEPREFIX".to_string(), prefix(id).to_string_lossy().into_owned()), ("GAMEID".to_string(), "umu-default".to_string())];
    let store = if id.starts_with("gog:") {
        "gog"
    } else if id.starts_with("epic:") {
        "egs"
    } else {
        "none"
    };
    v.push(("STORE".to_string(), store.to_string()));
    if let Some(p) = protonpath(id) {
        v.push(("PROTONPATH".to_string(), p.to_string_lossy().into_owned()));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wine_root_follows_pfx() {
        let t = tempfile::tempdir().unwrap();
        fs::create_dir_all(t.path().join("a/pfx/drive_c")).unwrap();
        fs::create_dir_all(t.path().join("b/drive_c")).unwrap();
        assert_eq!(wine_root(&t.path().join("a")), t.path().join("a/pfx"));
        assert_eq!(wine_root(&t.path().join("b")), t.path().join("b"));
    }

    #[test]
    fn newest_umu_proton_wins() {
        let t = tempfile::tempdir().unwrap();
        for n in ["UMU-Proton-9.0-3", "UMU-Proton-10.0-4", "UMU-Proton-10.0-10", "GE-Proton11-12", "UMU-Proton-11.0-1"] {
            fs::create_dir_all(t.path().join(n)).unwrap();
            if n != "UMU-Proton-11.0-1" {
                // Still downloading: no proton script yet.
                fs::write(t.path().join(n).join("proton"), "").unwrap();
            }
        }
        assert_eq!(newest_umu_proton(t.path()), Some(t.path().join("UMU-Proton-10.0-10")));
        fs::remove_dir_all(t.path().join("UMU-Proton-10.0-10")).unwrap();
        fs::remove_dir_all(t.path().join("UMU-Proton-10.0-4")).unwrap();
        fs::remove_dir_all(t.path().join("UMU-Proton-9.0-3")).unwrap();
        assert_eq!(newest_umu_proton(t.path()), Some(t.path().join("GE-Proton11-12")));
        assert_eq!(newest_umu_proton(&t.path().join("none")), None);
    }

    #[test]
    fn runtime_from_the_toolmanifest() {
        let t = tempfile::tempdir().unwrap();
        // UMU-Proton 10.0-4's and Valve's Proton Experimental 11's manifests.
        fs::write(t.path().join("toolmanifest.vdf"), "\"manifest\"\n{\n  \"version\" \"2\"\n  \"commandline\" \"/proton %verb%\"\n  \"require_tool_appid\" \"1628350\"\n  \"use_sessions\" \"1\"\n}\n").unwrap();
        assert_eq!(runtime_of(t.path()), Some("steamrt3"));
        fs::write(t.path().join("toolmanifest.vdf"), "\"manifest\"\n{\n\t\"require_tool_appid\"\t\"4183110\"\n}\n").unwrap();
        assert_eq!(runtime_of(t.path()), Some("steamrt4"));
        assert_eq!(runtime_of(&t.path().join("none")), None);
    }

    #[test]
    fn log_paths() {
        let p = log_path("epic:Fortnite");
        assert!(p.ends_with("rare/epic/Fortnite.log"), "{}", p.display());
        assert!(log_path("gog:1207658924").ends_with("rare/gog/1207658924.log"));
        assert!(log_path("epic:a/../b").ends_with("rare/epic/a_.._b.log"));
    }
}
