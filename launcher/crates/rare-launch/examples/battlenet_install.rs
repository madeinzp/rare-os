//! Get Battle.net without the UI: downloads Blizzard's installer and runs it through umu-run in
//! Rare's prefix for the app, the way Apps > Battle.net > Get Battle.net does, printing each
//! `app-install` event and then the installer's log. A device or rig check (DISPLAY set; outside
//! game mode the installer's window gets the screen at once):
//!
//!   cargo run --manifest-path launcher/crates/Cargo.toml -p rare-launch --example battlenet_install
//!
//! It leaves what it installed in ~/.local/share/rare/prefixes/battlenet.

use rare_core::{Ctx, Event};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;

fn main() {
    rare_stores_path();
    let (tx, rx) = mpsc::channel::<Event>();
    let tx = Mutex::new(tx);
    let ctx = Ctx::new(
        Arc::new(move |e: Event| {
            let _ = tx.lock().unwrap().send(e);
        }),
        Arc::new(rare_core::host::NoHost),
    );
    let runner = rare_launch::launch::Runner::default();
    runner.watch(ctx.clone());
    let t0 = Instant::now();
    if let Err(e) = rare_launch::apps::battlenet::start(ctx.clone(), runner.clone()) {
        eprintln!("start: {e}");
        std::process::exit(1);
    }
    let mut code = 0;
    for e in rx {
        let Event::AppInstall(i) = e else { continue };
        let p = i.progress.map(|p| format!("{:.0}%", p * 100.0)).unwrap_or_else(|| "...".into());
        println!("{:>6.1} s  progress {p}  done {}  {}", t0.elapsed().as_secs_f32(), i.done, i.error.as_deref().unwrap_or(""));
        if i.done {
            code = i32::from(i.error.is_some());
            break;
        }
    }
    println!("running: {:?}", runner.running());
    println!("log: {}", rare_launch::apps::battlenet::log_path().display());
    std::process::exit(code);
}

/// umu-run where Rare ships it (the image's /usr/libexec/rare/stores, or RARE_STORES_BIN).
fn rare_stores_path() {
    let mut paths: Vec<std::path::PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    for d in [std::env::var_os("RARE_STORES_BIN").map(Into::into), Some("/usr/libexec/rare/stores".into())].into_iter().flatten() {
        if !paths.contains(&d) {
            paths.push(d);
        }
    }
    if let Ok(p) = std::env::join_paths(paths) {
        std::env::set_var("PATH", p);
    }
}
