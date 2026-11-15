//! An Epic or GOG game's first-start setup without the UI (prep.rs): its prefix, a Proton and
//! umu's runtime, as Rare does after an install or before the first start. Prints what is missing,
//! each line umu-run prints, the time and the log's path.
//!
//!   cargo run -p rare-stores --example store_setup -- epic:<app name>

use std::time::Instant;

fn main() {
    let id = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: store_setup epic:<app> | gog:<id>");
        std::process::exit(2)
    });
    rare_stores::tools::add_to_path();
    let u = &rare_library::umu::prefix(&id);
    println!("prefix: {} (drive_c: {})", u.display(), rare_library::umu::wine_root(u).join("drive_c").is_dir());
    println!("proton: {}", rare_library::umu::protonpath(&id).map(|p| p.display().to_string()).unwrap_or_else(|| "none yet (umu-run downloads UMU-Proton)".into()));
    let proton = rare_library::umu::protonpath(&id);
    let rt = proton.as_deref().and_then(rare_library::umu::runtime_of).unwrap_or("any");
    println!("umu runtime ({rt}): {}", if rare_library::umu::runtime_ready(proton.as_deref()) { "there" } else { "missing" });
    println!("setup needed: {}", rare_stores::prep::needed(&id));
    let t0 = Instant::now();
    let r = rare_stores::prep::prepare(&id, &mut |pid| println!("umu-run pid {pid}"), &mut |l| println!("{:>6.1} s  {l}", t0.elapsed().as_secs_f64()));
    println!("{:.1} s: {:?}", t0.elapsed().as_secs_f64(), r);
    println!("setup needed now: {}; log: {}", rare_stores::prep::needed(&id), rare_library::umu::log_path(&id).display());
    std::process::exit(if r.is_ok() { 0 } else { 1 });
}
