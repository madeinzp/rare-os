//! A store game's first start, before the game: its Wine prefix, a Proton and umu's Steam Runtime.
//!
//! umu-run does all three on its own the first time it runs a game: it downloads UMU-Proton (about
//! 490 MB when Steam has no Valve Proton) and the runtime, then makes the prefix, with no window for
//! minutes (77 s on the build machine's wired link, 10 minutes and more from a slow GitHub). Inside a game's
//! start that read as "didn't start" after 45 s, and its Try again stopped the download, which umu
//! starts again from nothing, so the game never started. Rare does it here instead, as the game's
//! "Setting up" step in Downloads (after an install, or before the first start when that didn't
//! happen): `umu-run createprefix` with the game's prefix and Proton, everything it prints in the
//! game's log (rare-library umu `log_path`), and an error that says why and where the log is.

use crate::tools::{self, UMU};
use std::io::Write;
use std::time::Duration;

/// umu-run prints nothing while it downloads (about 490 MB), so a slow download and a dead one
/// look alike; it is stopped only when neither its download folder nor its output moved for this long.
const STALL: Duration = Duration::from_secs(10 * 60);
/// And never runs longer than this.
const LIMIT: Duration = Duration::from_secs(3 * 3600);

/// Bytes under umu-run's download folder (`~/.cache/umu`) and in its runtime and Proton folders.
fn umu_bytes() -> u64 {
    fn walk(d: &std::path::Path, depth: u32) -> u64 {
        let Ok(rd) = std::fs::read_dir(d) else { return 0 };
        rd.flatten()
            .map(|e| match e.file_type() {
                Ok(t) if t.is_dir() && depth > 0 => walk(&e.path(), depth - 1),
                Ok(t) if t.is_file() => e.metadata().map(|m| m.len()).unwrap_or(0),
                _ => 0,
            })
            .sum()
    }
    let cache = dirs::cache_dir().unwrap_or_default().join("umu");
    // The partial downloads sit one or two folders down; the unpacked trees are only counted by
    // their top files so this stays cheap every few seconds.
    walk(&cache, 2) + walk(&dirs::data_dir().unwrap_or_default().join("umu"), 1) + walk(&rare_library::umu::umu_compat_dir(), 1)
}

/// Watches a running umu-run: stops it (its process group) once nothing moved for `STALL`.
/// `moved` is bumped by the caller on each output line.
fn watchdog(pid: u32, moved: std::sync::Arc<std::sync::atomic::AtomicU64>, done: std::sync::Arc<std::sync::atomic::AtomicBool>, stalled: std::sync::Arc<std::sync::atomic::AtomicBool>) {
    use std::sync::atomic::Ordering;
    std::thread::spawn(move || {
        let mut last = (umu_bytes(), moved.load(Ordering::Relaxed));
        let mut since = std::time::Instant::now();
        while !done.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_secs(5));
            let now = (umu_bytes(), moved.load(Ordering::Relaxed));
            if now != last {
                last = now;
                since = std::time::Instant::now();
            } else if since.elapsed() >= STALL && !done.load(Ordering::Relaxed) {
                stalled.store(true, Ordering::Relaxed);
                tools::stop(pid);
                return;
            }
        }
    });
}

/// The game still needs its prefix, a Proton or umu's runtime before it can start.
pub fn needed(id: &str) -> bool {
    !rare_library::umu::ready(id)
}

/// "Details: ~/.cache/rare/epic/<app>.log"
pub fn log_note(id: &str) -> String {
    format!("Details: {}", rare_library::umu::show_path(&rare_library::umu::log_path(id)))
}

/// The error for a failed setup: umu's own reason when it gave one, and where the log is.
pub fn error_text(log: &str, fallback: &str, id: &str) -> String {
    let why = rare_library::umu::last_error(log).unwrap_or_else(|| fallback.to_string());
    format!("Couldn't set up Proton: {} {}", why, log_note(id))
}

/// Downloads what is missing and makes the prefix (`umu-run createprefix`). `line` sees each line
/// umu-run prints. Never runs twice at once (two games' first starts share the one download).
pub fn prepare(id: &str, started: &mut dyn FnMut(u32), line: &mut dyn FnMut(&str)) -> Result<(), String> {
    static ONE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _one = ONE.lock().unwrap_or_else(|e| e.into_inner());
    if !needed(id) {
        return Ok(());
    }
    if !tools::have(UMU) {
        return Err("umu-run is missing. Update Rare and try again.".into());
    }
    let prefix = rare_library::umu::prefix(id);
    let _ = std::fs::create_dir_all(&prefix);
    let env = rare_library::umu::env(id);
    let path = rare_library::umu::log_path(id);
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let mut log = std::fs::File::create(&path).ok();
    let mut text = String::new();
    let head = format!(
        "Rare: setting up {id} (umu-run createprefix)\nWINEPREFIX={}\nPROTONPATH={}\n",
        prefix.display(),
        env.iter().find(|(k, _)| k == "PROTONPATH").map(|(_, v)| v.as_str()).unwrap_or("(none: umu-run downloads UMU-Proton)")
    );
    if let Some(f) = log.as_mut() {
        let _ = f.write_all(head.as_bytes());
    }
    let t0 = std::time::Instant::now();
    let moved = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let stalled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let r = tools::run_with(
        UMU,
        &["createprefix".to_string()],
        &env,
        Some(LIMIT),
        &mut |pid| {
            watchdog(pid, moved.clone(), done.clone(), stalled.clone());
            started(pid)
        },
        &mut |l: &str| {
            moved.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Some(f) = log.as_mut() {
                let _ = writeln!(f, "{l}");
            }
            text.push_str(l);
            text.push('\n');
            line(l);
        },
    );
    done.store(true, std::sync::atomic::Ordering::Relaxed);
    if stalled.load(std::sync::atomic::Ordering::Relaxed) {
        text.push_str("ERROR: Nothing downloaded for 10 minutes. Check Wi-Fi and try again\n");
    }
    let done = format!("Rare: umu-run ended after {:.1} s\n", t0.elapsed().as_secs_f64());
    if let Some(f) = log.as_mut() {
        let _ = f.write_all(done.as_bytes());
    }
    log::info!("prep {id}: {}", done.trim());
    match r {
        Err(e) => Err(error_text(&text, &e, id)),
        Ok(out) if !out.ok => Err(error_text(&text, "umu-run stopped with an error.", id)),
        // umu-run can end well and still leave nothing behind (no Proton found offline).
        Ok(_) if needed(id) => Err(error_text(&text, "the prefix wasn't made.", id)),
        Ok(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn errors_name_the_reason_and_the_log() {
        let offline = "FileNotFoundError: Environment variable not set or is empty: PROTONPATH\n";
        let e = super::error_text(offline, "x", "epic:Fortnite");
        assert!(e.starts_with("Couldn't set up Proton: Proton couldn't be downloaded. Check Wi-Fi. Details: "), "{e}");
        assert!(e.ends_with("rare/epic/Fortnite.log"), "{e}");
        let e = super::error_text("INFO: fine\n", "umu-run took too long. Try again.", "gog:1");
        assert!(e.contains("umu-run took too long. Try again. Details: "), "{e}");
    }
}
