//! Getting Battle.net (Apps > Battle.net > Get Battle.net): Blizzard's installer, downloaded from
//! Blizzard, runs through umu-run in Rare's prefix for the app (rare-library battlenet), the way
//! GOG's Windows builds run. Its window takes the screen as the app `app:battlenet`, with the pad
//! as a mouse; it installs by itself and then opens the app's sign-in. Progress goes out as
//! `app-install` events: the download's percent, then a plain loading bar until the installer is
//! done.
//!
//! The first umu-run on a device downloads Proton (UMU-Proton, about 490 MB, when Steam has no
//! Valve Proton) and its Steam runtime before the installer can start: 77 s to the first window on
//! a wired link, minutes on Wi-Fi. So the run is owned here (`Runner::track_child_owned`): the
//! runner never calls it a start that failed, Rare keeps the screen with its loading bar until
//! the installer's window shows, and everything umu-run, Proton and the installer print goes to
//! `log_path()`, which every error names.

use super::catalog::{self, Back};
use rare_core::events::InstallEvent;
use rare_core::{Ctx, Error, Event};
use rare_library::battlenet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static BUSY: Mutex<bool> = Mutex::new(false);

/// How long the installer may take before Rare closes it (Proton's first download on a slow
/// connection, then Blizzard's).
const WAIT: Duration = Duration::from_secs(45 * 60);

/// The installer's file name (what its Wine process is called, `is_setup`).
const SETUP: &str = "Battle.net-Setup.exe";

fn dir() -> PathBuf {
    dirs::cache_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".cache")).join("rare/battlenet")
}

fn setup_path() -> PathBuf {
    dir().join(SETUP)
}

/// What umu-run, Proton and Blizzard's installer printed on the last try (~/.cache/rare/battlenet/install.log).
pub fn log_path() -> PathBuf {
    dir().join("install.log")
}

/// A path as the player reads it: the home folder as ~.
fn shown(p: &Path, home: Option<&Path>) -> String {
    match home.and_then(|h| p.strip_prefix(h).ok()) {
        Some(rest) => format!("~/{}", rest.display()),
        None => p.display().to_string(),
    }
}

fn log_note() -> String {
    format!("Details: {}", shown(&log_path(), dirs::home_dir().as_deref()))
}

/// Starts it; progress and the end arrive as `app-install` events for `battlenet`.
pub fn start(ctx: Ctx, runner: crate::launch::Runner) -> Result<(), Error> {
    if !rare_library::steam::which("umu-run") {
        return Err("Battle.net needs umu-run (Proton), and Rare couldn't find it.".into());
    }
    {
        let mut b = BUSY.lock().unwrap_or_else(|e| e.into_inner());
        if *b {
            return Ok(());
        }
        *b = true;
    }
    std::thread::spawn(move || {
        let result = run(&ctx, &runner);
        if let Err(e) = &result {
            rare_core::host::global().log(&format!("[battlenet] install: {e}"));
        }
        *BUSY.lock().unwrap_or_else(|e| e.into_inner()) = false;
        ctx.send(Event::AppInstall(InstallEvent { id: battlenet::APP_ID.into(), progress: result.is_ok().then_some(1.0), done: true, error: result.err() }));
    });
    Ok(())
}

fn emit(ctx: &Ctx, progress: Option<f32>) {
    ctx.send(Event::AppInstall(InstallEvent { id: battlenet::APP_ID.into(), progress, done: false, error: None }));
}

/// Blizzard's installer into the cache (kept for a second try), with the download's percent.
fn download(ctx: &Ctx, to: &PathBuf) -> Result<(), String> {
    let _ = std::fs::create_dir_all(to.parent().unwrap_or(to));
    let resp = ureq::AgentBuilder::new().timeout_connect(Duration::from_secs(15)).timeout_read(Duration::from_secs(60)).build().get(battlenet::SETUP_URL).set("User-Agent", "RareLauncher/1.0").call().map_err(|e| {
        rare_core::host::global().log(&format!("[battlenet] download: {e}"));
        "Couldn't download Battle.net. Check Wi-Fi and try again.".to_string()
    })?;
    let total: Option<u64> = resp.header("Content-Length").and_then(|v| v.parse().ok());
    let tmp = to.with_extension("part");
    let mut out = std::fs::File::create(&tmp).map_err(|e| e.to_string())?;
    let mut r = resp.into_reader();
    let (mut buf, mut done, mut last) = (vec![0u8; 64 * 1024], 0u64, -1i32);
    loop {
        let n = r.read(&mut buf).map_err(|_| "The download stopped. Try again.".to_string())?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n]).map_err(|e| e.to_string())?;
        done += n as u64;
        if let Some(t) = total.filter(|t| *t > 0) {
            let pct = (done * 100 / t) as i32;
            if pct != last {
                last = pct;
                emit(ctx, Some(pct as f32 / 100.0));
            }
        }
    }
    drop(out);
    // A Windows program starts with MZ; anything else is an error page.
    let mut head = [0u8; 2];
    std::fs::File::open(&tmp).and_then(|mut f| f.read_exact(&mut head)).map_err(|e| e.to_string())?;
    if &head != b"MZ" {
        let _ = std::fs::remove_file(&tmp);
        return Err("Blizzard sent something that isn't the installer. Try again later.".into());
    }
    std::fs::rename(&tmp, to).map_err(|e| e.to_string())
}

/// The installer's own Wine process: comm `Battle.net-Setu` (the kernel keeps 15 characters), or
/// a command line whose program is a Windows path to it (`S:\...\Battle.net-Setup.exe`).
/// Proton's wrappers (`c:\windows\system32\steam.exe`, umu.exe, python) only carry its path as an
/// argument.
pub fn is_setup(comm: &str, cmd: &str) -> bool {
    let setup = SETUP.to_ascii_lowercase();
    if setup.starts_with(&comm.to_ascii_lowercase()) && comm.len() >= 15 {
        return true;
    }
    let c = cmd.trim_start().to_ascii_lowercase().replace('\\', "/");
    let windows = c.as_bytes().get(1) == Some(&b':') && c.as_bytes().first().is_some_and(|b| b.is_ascii_alphabetic());
    windows && c.find(".exe").is_some_and(|i| c[..i + 4].ends_with(&format!("/{setup}")))
}

fn setup_running() -> bool {
    rare_core::proc::cmdlines().iter().any(|(_, comm, cmd)| is_setup(comm, cmd))
}

/// The installer's last words worth showing: umu-run's ERROR/CRITICAL lines ("Could not
/// download..."), without its logger prefix. umu-run with no Proton and no network says only that
/// PROTONPATH is empty ("Possible reason: ... or network error"), so that one is put plainly.
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
    if !msg.ends_with('.') {
        msg.push('.');
    }
    Some(msg)
}

/// The error for a run that ended before Battle.net was there.
fn ended(log: &Path) -> String {
    let why = std::fs::read_to_string(log).ok().and_then(|t| last_error(&t)).map(|e| format!(" {e}")).unwrap_or_default();
    format!("Battle.net's installer closed before it finished.{why} Try again. {}", log_note())
}

fn run(ctx: &Ctx, runner: &crate::launch::Runner) -> Result<(), String> {
    let prefix = battlenet::rare_prefix();
    let run_id = catalog::run_id(battlenet::APP_ID);
    let log = log_path();
    // An installer from an earlier try still running (Rare restarted, or it was taking long):
    // wait for that one rather than start a second in the same prefix.
    let already = runner.running().contains(&run_id);
    if !already {
        if battlenet::client_in(&prefix).is_some() {
            return Ok(());
        }
        emit(ctx, Some(0.0));
        let setup = setup_path();
        if !setup.is_file() {
            download(ctx, &setup)?;
        }
        emit(ctx, None);
        std::fs::create_dir_all(&prefix).map_err(|e| e.to_string())?;
        let mut args = vec![setup.to_string_lossy().into_owned()];
        args.extend(battlenet::setup_args(&std::env::var("LANG").unwrap_or_default()));
        let (mut cmd, unit) = crate::launch::host_command_unit("umu-run", &args, &run_id);
        let env = battlenet::prefix_env(&prefix);
        // The log: what Rare ran, then umu-run's, Proton's and the installer's own output.
        let out = std::fs::File::create(&log).and_then(|mut f| {
            let proton = env.iter().find(|(k, _)| k == "PROTONPATH").map(|(_, v)| v.as_str()).unwrap_or("umu-run's default (UMU-Proton)");
            writeln!(f, "Get Battle.net: umu-run {}\nWINEPREFIX={}\nPROTONPATH={proton}\n", args.join(" "), prefix.display())?;
            Ok(f)
        });
        if let Ok(f) = out {
            if let Ok(f2) = f.try_clone() {
                cmd.stdout(f).stderr(f2);
            }
        }
        cmd.envs(env);
        super::pointer::prepare();
        let child = cmd.spawn().map_err(|e| format!("Could not start Battle.net's installer: {e}"))?;
        runner.track_child_owned(&run_id, child, unit);
        super::front::set_pointer(&run_id, Some(Back::Escape));
    }
    // Rare keeps the screen, with its loading bar, until the installer has a window (Proton's
    // first download comes before it); then the window gets it, unless the player has started
    // something else meanwhile. It stays in the Running list either way.
    let gen = super::front::generation();
    let mut handed = false;
    let t0 = Instant::now();
    loop {
        if !handed && runner.has_window(&run_id) {
            handed = true;
            if super::front::to_front_if(gen, &run_id) {
                super::step_aside(ctx);
            }
        }
        std::thread::sleep(Duration::from_secs(2));
        let alive = runner.running().contains(&run_id);
        // Done once the app's program is there and the installer has finished (it writes
        // Battle.net.exe at about 90%); it then opens the app's sign-in itself, which stays the
        // running app.
        if battlenet::client_in(&prefix).is_some() && (!alive || !setup_running()) {
            return Ok(());
        }
        if !alive {
            return Err(ended(&log));
        }
        if t0.elapsed() > WAIT {
            let _ = runner.quit(ctx, &run_id);
            return Err(format!("Battle.net's installer didn't finish in 45 minutes, so Rare closed it. Try again. {}", log_note()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_installer_process_not_its_wrappers() {
        // As seen on the build machine (umu-run 1.4.4, Proton Experimental 11 and GE-Proton 11-12).
        assert!(is_setup("Battle.net-Setu", "S:\\bnet-test\\dl\\Battle.net-Setup.exe --lang=enUS --installpath=C:\\Program Files (x86)\\Battle.net "));
        assert!(is_setup("wine64-preload", "Z:\\home\\bazzite\\.cache\\rare\\battlenet\\Battle.net-Setup.exe --lang=enUS "));
        assert!(!is_setup("steam.exe", "c:\\windows\\system32\\steam.exe /home/bazzite/.cache/rare/battlenet/Battle.net-Setup.exe --lang=enUS "));
        assert!(!is_setup("umu.exe", "c:\\windows\\system32\\umu.exe /home/bazzite/.cache/rare/battlenet/Battle.net-Setup.exe "));
        assert!(!is_setup("python3", "python3 /usr/libexec/rare/stores/umu-run /home/bazzite/.cache/rare/battlenet/Battle.net-Setup.exe "));
        assert!(!is_setup("python3", "python3 /home/b/.local/share/Steam/steamapps/common/Proton 11.0/proton waitforexitandrun /home/b/.cache/rare/battlenet/Battle.net-Setup.exe "));
        // The app it installs, and its agent.
        assert!(!is_setup("Battle.net.exe", "C:/Program Files (x86)/Battle.net/Battle.net.exe --detected-region=US "));
        assert!(!is_setup("Agent.exe", "C:/ProgramData/Battle.net/Agent/Agent.9873/Agent.exe --locale=enUS "));
        assert!(!is_setup("Battle", ""));
    }

    #[test]
    fn umu_errors_read_plainly() {
        let log = "Get Battle.net: umu-run ...\n[umu.umu_run:1047] DEBUG: ('...')\n[umu.umu_runtime:163] INFO: Downloading steamrt4, please wait...\n[umu.umu_runtime:400] CRITICAL: Could not reach repo.steampowered.com\nProton: Upgrading prefix\n";
        assert_eq!(last_error(log).as_deref(), Some("Could not reach repo.steampowered.com."));
        assert_eq!(last_error("[umu.umu_run:1] ERROR: Not enough space.\n").as_deref(), Some("Not enough space."));
        assert_eq!(last_error("Proton: Upgrading prefix\nfsync: up and running.\n"), None);
        // umu-run 1.4.4 with no Proton and no network (the build machine, behind a dead proxy).
        let offline = "FileNotFoundError: Environment variable not set or is empty: PROTONPATH\nPossible reason: GE-Proton or UMU-Proton not found in '/home/b/.local/share/Steam/compatibilitytools.d' or network error\n";
        assert_eq!(last_error(offline).as_deref(), Some("Proton couldn't be downloaded. Check Wi-Fi."));
    }

    #[test]
    fn paths_under_home_read_with_a_tilde() {
        let home = Path::new("/home/bazzite");
        assert_eq!(shown(Path::new("/home/bazzite/.cache/rare/battlenet/install.log"), Some(home)), "~/.cache/rare/battlenet/install.log");
        assert_eq!(shown(Path::new("/var/tmp/install.log"), Some(home)), "/var/tmp/install.log");
    }
}
