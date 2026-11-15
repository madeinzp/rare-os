//! Settings > System > Support: the time zone and automatic time, logs for a bug report (saved to
//! Documents, copied to a USB drive, or uploaded for a link) and the factory reset. The rows live
//! here; the sheets are in `supportsheets.rs`; the work runs on workers through `run`, which
//! `live::run` hands the `time:`, `logs:` and `reset:` acts to. The device side is
//! `rare_system::support`.

use super::live::{self, Ran};
use super::{rows::*, SectionOut};
use crate::screens::overlay::common::*;
use rare_i18n::{tr, trf};
use rare_render::Id;
use rare_system::support as sup;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// What the rows show, read on a worker.
#[derive(Clone, Default)]
pub struct State {
    /// None until read.
    pub time: Option<sup::TimeInfo>,
    pub zones: Vec<String>,
    /// The system can do a factory reset (None until read).
    pub reset_ok: Option<bool>,
    /// "save" or "link" while the logs are being made or uploaded.
    pub logs_busy: Option<&'static str>,
    pub last_bundle: Option<PathBuf>,
}

/// A sheet a worker wants the app to open.
#[derive(Clone, Debug, PartialEq)]
pub enum SheetReq {
    /// The logs' link (shown with a QR code).
    Link(String),
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static SHEETS: Mutex<Vec<SheetReq>> = Mutex::new(Vec::new());
/// Acts a sheet finished with after a keyboard answer (the app runs them).
static REQUESTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn get() -> State {
    STATE.lock().ok().and_then(|s| s.clone()).unwrap_or_default()
}

fn edit(f: impl FnOnce(&mut State)) {
    if let Ok(mut s) = STATE.lock() {
        f(s.get_or_insert_with(State::default));
    }
    live::bump_pub();
}

pub fn take_sheets() -> Vec<SheetReq> {
    SHEETS.lock().map(|mut s| std::mem::take(&mut *s)).unwrap_or_default()
}

pub fn request(act: String) {
    if let Ok(mut r) = REQUESTS.lock() {
        r.push(act);
    }
}

pub fn take_requests() -> Vec<String> {
    REQUESTS.lock().map(|mut r| std::mem::take(&mut *r)).unwrap_or_default()
}

/// The first reading, with the rest of Settings' (`live::load_all`).
pub fn load(waker: Arc<dyn Fn() + Send + Sync>) {
    std::thread::spawn(move || {
        let t = sup::time_info();
        let ok = sup::reset_supported();
        edit(|s| {
            s.time = Some(t);
            s.reset_ok = Some(ok);
        });
        waker();
        let z = sup::zones();
        edit(|s| s.zones = z);
        waker();
    });
}

fn fixture_time() -> sup::TimeInfo {
    sup::TimeInfo { zone: Some("America/New_York".into()), ntp: Some(true) }
}

/// "New York, America. 3:42 PM"
pub fn zone_line(zone: Option<&str>) -> String {
    match zone {
        Some("UTC") => trf!(tr().settings_more.sup_zone_utc, time = crate::screens::shell::clock_text()),
        Some(z) => trf!(tr().settings_more.sup_zone, city = sup::city_name(z), region = sup::region_of(z), time = crate::screens::shell::clock_text()),
        None => tr().settings_more.sup_not_set.into(),
    }
}

pub struct SupportRows {
    rows: RowSet,
    zone_btn: Id,
    save_btn: Id,
    link_btn: Id,
    reset_btn: Id,
}

impl SupportRows {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id) -> SupportRows {
        let (time, busy, reset_ok) = if live::enabled() {
            let s = get();
            (s.time.unwrap_or_default(), s.logs_busy, s.reset_ok.unwrap_or(true))
        } else {
            (fixture_time(), None, true)
        };
        let t = &tr().settings_more;
        let p = panel(cx, parent);
        let row_ = setting_row(cx, p, t.sup_time_zone, Some(&zone_line(time.zone.as_deref())), true);
        let b = button(cx, row_.right, tr().common.change, Variant::Secondary, Sz::Md, false).sid("time-zone").tag(30);
        let zone_btn = nav.add(b.group(10));
        let mut rows = RowSet::default();
        rows.switch(cx, nav, p, "time-auto", t.sup_auto_time, t.sup_auto_time_body, time.ntp.unwrap_or(true), false);
        let row_ = setting_row(cx, p, t.sup_logs, Some(t.sup_logs_body), false);
        let v = |on: bool| if on { Variant::Dimmed } else { Variant::Secondary };
        let b = button(cx, row_.right, if busy == Some("save") { t.sup_saving } else { tr().common.save }, v(busy.is_some()), Sz::Md, false).sid("logs-save").tag(31);
        let save_btn = nav.add(b.group(10));
        let b = button(cx, row_.right, if busy == Some("link") { t.sup_uploading } else { t.sup_get_link }, v(busy.is_some()), Sz::Md, false).sid("logs-link").tag(32);
        let link_btn = nav.add(b.group(10));
        let body = if reset_ok { t.sup_reset_body } else { t.sup_needs_update };
        let row_ = setting_row(cx, p, t.sup_reset, Some(body), false);
        let b = button(cx, row_.right, t.sup_reset_btn, if reset_ok { Variant::Secondary } else { Variant::Dimmed }, Sz::Md, false).sid("factory-reset").tag(33);
        let reset_btn = nav.add(b.group(10));
        SupportRows { rows, zone_btn, save_btn, link_btn, reset_btn }
    }

    /// A on one of the rows.
    pub fn press(&mut self, cx: &mut Cx, nav: &mut Nav, now: f64) -> Option<SectionOut> {
        let cur = nav.cur?;
        let busy = get().logs_busy.is_some();
        let act = |a: &str| Some(SectionOut::Act(a.into()));
        if cur == self.zone_btn {
            return act("support:tz-open");
        }
        if cur == self.save_btn || cur == self.link_btn {
            if busy {
                return Some(SectionOut::None);
            }
            return act(if cur == self.save_btn { "logs:save" } else { "logs:link" });
        }
        if cur == self.reset_btn {
            return act("support:reset-open");
        }
        if nav.current().is_some_and(|f| f.sid == "time-auto") {
            let (_, v) = self.rows.press(cx, nav, now)?;
            return act(&format!("time:ntp:{}", matches!(v, Val::Bool(true))));
        }
        None
    }
}

fn device_text() -> String {
    serde_json::to_string_pretty(&rare_system::system::device_info()).unwrap_or_default()
}

fn saved_where(p: &std::path::Path) -> String {
    let home = rare_core::paths::home();
    p.strip_prefix(&home).map(|r| r.display().to_string()).unwrap_or_else(|_| p.display().to_string())
}

fn drive_name(p: &std::path::Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| tr().settings_more.sup_the_drive.into())
}

/// The Support acts: `time:zone:<zone>`, `time:ntp:<bool>`, `logs:save`, `logs:link`,
/// `logs:usb:<mount>`, `reset:keep-games|erase-all`.
pub fn run(act: &str, waker: Arc<dyn Fn() + Send + Sync>) -> Ran {
    let parts: Vec<&str> = act.splitn(3, ':').collect();
    let (a, b, rest) = (parts.first().copied().unwrap_or(""), parts.get(1).copied().unwrap_or(""), parts.get(2).copied().unwrap_or("").to_string());
    let spawn = move |f: Box<dyn FnOnce() + Send>| {
        let w = waker.clone();
        std::thread::spawn(move || {
            f();
            w();
        });
    };
    match (a, b) {
        ("time", "zone") => spawn(Box::new(move || {
            match sup::set_zone(&rest) {
                Ok(()) => {
                    live::toast_pub(trf!(tr().settings_more.sup_zone_set, zone = if rest == "UTC" { "UTC".into() } else { sup::city_name(&rest) }), false);
                    // The top bar's clock reads the new zone now, not at the next minute.
                    request("support:clock".into());
                }
                Err(e) => live::toast_pub(live::failed(tr().settings_more.sup_err_zone, &e, None), true),
            }
            let t = sup::time_info();
            edit(|s| s.time = Some(t));
        })),
        ("time", "ntp") => {
            let on = rest == "true";
            spawn(Box::new(move || {
                if let Err(e) = sup::set_ntp(on) {
                    live::toast_pub(live::failed(tr().settings_more.sup_err_ntp, &e, None), true);
                }
                let t = sup::time_info();
                edit(|s| s.time = Some(t));
            }))
        }
        ("logs", "save") => {
            edit(|s| s.logs_busy = Some("save"));
            spawn(Box::new(move || {
                let r = sup::save_logs(&device_text());
                edit(|s| {
                    s.logs_busy = None;
                    if let Ok(p) = &r {
                        s.last_bundle = Some(p.clone());
                    }
                });
                match r {
                    Ok(p) => {
                        let drive = sup::usb_drives().into_iter().next();
                        let t = &tr().settings_more;
                        let msg = if drive.is_some() { trf!(t.sup_saved_to, path = saved_where(&p)) } else { trf!(t.sup_saved_to_usb_hint, path = saved_where(&p)) };
                        live::toast_pub(msg, false);
                        if let Some(d) = drive {
                            live::ask_pub(live::Ask {
                                title: tr().settings_more.sup_copy_title.into(),
                                sub: drive_name(&d),
                                body: tr().settings_more.sup_copy_body.into(),
                                yes: tr().common.copy.into(),
                                act: format!("logs:usb:{}", d.display()),
                            });
                        }
                    }
                    Err(e) => live::toast_pub(live::failed(tr().settings_more.sup_err_save, &e, None), true),
                }
            }))
        }
        ("logs", "usb") => {
            let drive = PathBuf::from(rest);
            spawn(Box::new(move || {
                let Some(bundle) = get().last_bundle else { return };
                match sup::copy_to(&bundle, &drive) {
                    Ok(_) => live::toast_pub(trf!(tr().settings_more.sup_copied, name = drive_name(&drive)), false),
                    Err(e) => live::toast_pub(live::failed(tr().settings_more.sup_err_copy, &e, None), true),
                }
            }))
        }
        ("logs", "link") => {
            edit(|s| s.logs_busy = Some("link"));
            spawn(Box::new(move || {
                let r = sup::save_logs(&device_text()).and_then(|p| {
                    let bytes = std::fs::read(&p).map_err(|e| e.to_string())?;
                    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "rare-logs.tar.gz".into());
                    edit(|s| s.last_bundle = Some(p.clone()));
                    crate::app::share_up::upload_file(&name, "application/gzip", bytes)
                });
                edit(|s| s.logs_busy = None);
                match r {
                    Ok(link) => {
                        crate::app::share_up::copy_link(&link);
                        if let Ok(mut q) = SHEETS.lock() {
                            q.push(SheetReq::Link(link));
                        }
                    }
                    Err(e) => live::toast_pub(live::failed(tr().settings_more.sup_err_upload, &e, Some(tr().settings_more.sup_err_upload_hint)), true),
                }
            }))
        }
        ("reset", mode) => {
            let Some(mode) = sup::ResetMode::parse(mode) else { return Ran::NotMine };
            spawn(Box::new(move || {
                if !sup::reset_supported() {
                    live::toast_pub(tr().settings_more.sup_needs_update, true);
                    return;
                }
                // With parental controls on, the parent's PIN first (answers at once without one).
                let pin = crate::features::parental::ask(tr().settings_more.sup_reset, tr().settings_more.sup_reset_pin, rare_render::clock::secs());
                if pin.recv().ok() != Some(true) {
                    live::toast_pub(tr().settings_more.sup_not_reset, false);
                    return;
                }
                // The reset is booked first, so a refusal leaves everything (wallets included) as it was.
                if let Err(e) = sup::factory_reset(mode) {
                    live::toast_pub(live::failed(tr().settings_more.sup_err_reset, &e, None), true);
                    return;
                }
                // Then the wallets: the keyring entry can only be removed from the session.
                let vault = rare_wallet::vault::Vault::new(rare_wallet::vault::default_dir(), true, rare_wallet::vault::Kdf::DEFAULT);
                if let Err(e) = vault.reset() {
                    log::warn!("factory reset: wallet vault: {e}");
                }
                live::toast_pub(tr().settings_more.sup_restarting, false);
                std::thread::sleep(std::time::Duration::from_millis(1200));
                if let Err(e) = rare_system::system::power("reboot") {
                    live::toast_pub(live::failed(tr().settings_more.sup_err_reboot, &e, None), true);
                }
            }))
        }
        _ => return Ran::NotMine,
    }
    Ran::Started
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_queue_once() {
        request("reset:keep-games".into());
        assert_eq!(take_requests(), vec!["reset:keep-games".to_string()]);
        assert!(take_requests().is_empty());
    }
}
