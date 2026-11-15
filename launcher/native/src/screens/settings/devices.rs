//! The Devices section: pair a phone so rare.market/devices on it controls this device (features/companion.rs),
//! with a QR code to scan, the pairing state, what the phone did lately, and Unpair.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::features::companion::{self, View};
use crate::screens::overlay::common::*;
use rare_i18n::{tr, trf, trn};
use rare_render::Id;

pub struct DevicesSection;

/// Fixture runs (screenshots): RR_FIXTURE_DEVICES=signedout | code | paired | error, else the start.
pub fn fixture_view() -> View {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
    let base = View { live: false, signed_in: true, key: true, paired: Some(false), account: Some("0x7a3f0c1e9b2d4c6a8e0f1a3b5c7d9e1f2a4b6c8d0e1f3a5b7c9d1e3f5a7b9c1d".into()), ..Default::default() };
    match std::env::var("RR_FIXTURE_DEVICES").as_deref() {
        Ok("signedout") => View { signed_in: false, key: false, paired: None, account: None, ..base },
        Ok("code") => View { code: Some(("K7QF-M2XP".into(), "https://rare.market/devices/pair?code=K7QFM2XP".into(), now + 540_000)), ..base },
        Ok("error") => View { error: Some("rare.market did not answer. Check the connection and try again.".into()), ..base },
        Ok("paired") => View {
            paired: Some(true),
            recent: vec![
                (now - 40_000, "Typed into Search".into(), true),
                (now - 300_000, "Hollow Knight: Silksong Collector's Edition with a very long name is in the downloads".into(), true),
                (now - 900_000, "Nothing on the device is waiting for text. Open a text field first.".into(), false),
                (now - 3_600_000, "Saved holiday-photo-from-the-beach-2026-10-27.jpg to Share".into(), true),
                (now - 7_200_000, "Paired with your phone".into(), true),
            ],
            ..base
        },
        _ => base,
    }
}

fn short(a: &str) -> String {
    if a.len() > 12 {
        format!("{}...{}", &a[..6], &a[a.len() - 4..])
    } else {
        a.to_string()
    }
}

fn ago(at: i64, now: i64) -> String {
    let s = (now - at).max(0) / 1000;
    match s {
        0..=59 => tr().settings.ago_now.into(),
        60..=3599 => trf!(tr().settings.ago_min, n = s / 60),
        3600..=86_399 => trf!(tr().settings.ago_hour, n = s / 3600),
        _ => trf!(tr().settings.ago_day, n = s / 86_400),
    }
}

fn small_button(cx: &mut Cx, nav: &mut Nav, parent: Id, label: &str, sid: &str, primary: bool) {
    let b = button(cx, parent, label, if primary { Variant::Primary } else { Variant::Secondary }, Sz::Md, false).sid(sid);
    nav.add(b.group(11));
}

impl DevicesSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, _cfg: &Config, _now: f64) -> DevicesSection {
        let v = companion::view();
        let v = if v.live { v } else { fixture_view() };
        Self::build_with(cx, nav, parent, &v);
        DevicesSection
    }

    pub fn build_with(cx: &mut Cx, nav: &mut Nav, parent: Id, v: &View) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
        let t = &tr().settings;
        let p = panel(cx, parent);
        setting_row(cx, p, t.row_phone_control, Some(t.phone_control_body), true);
        if !v.signed_in {
            let r = setting_row(cx, p, t.row_sign_in_first, Some(t.sign_in_first_body), false);
            small_button(cx, nav, r.right, tr().common.sign_in, "devices-signin", true);
            return;
        }
        match (v.paired, &v.code) {
            (Some(true), _) => {
                let who = v.account.as_deref().map(short).unwrap_or_default();
                let r = setting_row(cx, p, t.row_paired, Some(&trf!(t.paired_body_phone, who = who)), false);
                small_button(cx, nav, r.right, t.unpair, "devices-unpair", false);
                let r = setting_row(cx, p, t.row_add_phone, Some(t.add_phone_body), false);
                let _ = r;
            }
            (_, Some((code, url, exp))) if *exp > now => {
                let left = ((*exp - now) / 60_000).max(1);
                let body = trn!(t.pair_scan_body, left, code = code);
                let r = setting_row(cx, p, &trf!(t.pair_code_title, code = code), Some(&body), false);
                super::remote::qr_box(cx, r.right, url, 11.0);
                let r = setting_row(cx, p, t.row_waiting_phone, Some(t.waiting_phone_body), false);
                small_button(cx, nav, r.right, t.new_code, "devices-pair", false);
            }
            _ => {
                let body = if v.busy { t.pair_working } else { t.pair_intro };
                let r = setting_row(cx, p, t.row_pair_phone, Some(body), false);
                small_button(cx, nav, r.right, t.row_pair_phone, "devices-pair", true);
            }
        }
        if let Some(e) = &v.error {
            setting_row(cx, p, t.row_didnt_work, Some(e), false);
        }
        if v.paired == Some(true) {
            let p = panel(cx, parent);
            setting_row(cx, p, t.row_recent_phone, None, true);
            if v.recent.is_empty() {
                setting_row(cx, p, t.row_nothing_yet, Some(t.nothing_yet_body), false);
            }
            for (at, what, ok) in v.recent.iter().take(5) {
                let body = if *ok { ago(*at, now) } else { trf!(t.recent_failed, when = ago(*at, now)) };
                setting_row(cx, p, what, Some(&body), false);
            }
        }
    }
}

impl SectionView for DevicesSection {
    fn press(&mut self, _cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, _now: f64) -> SectionOut {
        let Some(f) = nav.current().cloned() else { return SectionOut::None };
        match f.sid.as_str() {
            "devices-pair" => SectionOut::Act("devices:pair".into()),
            "devices-unpair" => SectionOut::Act("devices:unpair".into()),
            "devices-signin" => SectionOut::Act("open:signin".into()),
            _ => SectionOut::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words() {
        assert_eq!(short("0x7a3f0c1e9b2d4c6a8e0f"), "0x7a3f...8e0f");
        assert_eq!(ago(0, 30_000), "just now");
        assert_eq!(ago(0, 7_200_000), "2 h ago");
    }
}
