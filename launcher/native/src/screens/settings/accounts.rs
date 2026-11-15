//! The Accounts section (screens/settings/AccountsStorage.tsx): username, Rare account, wallet,
//! Epic Games and GOG (features/stores.rs), setup.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::screens::overlay::common::*;
use rare_i18n::{tr, trf};
use rare_render::Id;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct ProfileInfo {
    /// None: signed out, no username row.
    pub address: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    /// What the wallet calls the signed-in account ("Demo wallet", a SuiNS name).
    pub label: Option<String>,
    /// The Steam account signed in on this device (its profile name).
    pub steam: Option<String>,
}

/// The Username row's line (UsernameRow).
pub fn username_body(p: &ProfileInfo) -> String {
    let t = &tr().settings;
    match (&p.username, &p.name) {
        (Some(u), Some(n)) => trf!(t.username_set_suins, username = u, name = n),
        (Some(u), None) => trf!(t.username_set, username = u),
        (None, _) => t.username_none.into(),
    }
}

/// The Rare account row's line: who is signed in, or what signing in gives.
pub fn rare_body(p: &ProfileInfo) -> String {
    match &p.address {
        Some(a) => {
            let who = p.username.clone().or_else(|| p.name.clone()).or_else(|| p.label.clone()).unwrap_or_else(|| short(a));
            trf!(tr().settings.rare_signed_in, who = who)
        }
        None => tr().settings.rare_signed_out.into(),
    }
}

/// The Steam row's line.
pub fn steam_body(p: &ProfileInfo) -> String {
    match &p.steam {
        Some(s) => trf!(tr().settings.steam_signed_in, name = s),
        None => tr().settings.steam_signed_out.into(),
    }
}

/// One wallet account's row: what it is and its rare.market sign-in on this device.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountLine {
    pub address: String,
    pub label: String,
    /// "Recovery phrase on this SuiPlay, signs with A", "Passkey", ...
    pub kind: String,
    /// Signed in to rare.market on this device: since (ms, 0 when not known).
    pub session: Option<i64>,
    pub main: bool,
    /// It can sign (not watch only).
    pub can_sign: bool,
    /// Its key is in the device vault (the send PIN applies to it).
    pub device: bool,
}

/// What kind of account it is and how it signs, in a few words.
pub fn kind_text(a: &crate::screens::wallet::types::RareAccount) -> String {
    let t = &tr().settings;
    match a.kind.as_str() {
        "native" if a.device && a.hd.is_some() => t.acct_recovery_device,
        "native" if a.device => t.acct_key_device,
        "native" if a.hd.is_some() => t.acct_recovery_pin,
        "native" => t.acct_key_pin,
        "passkey" if a.passkey.as_ref().is_some_and(|p| p.pin) => t.acct_pin_account,
        "passkey" => t.acct_passkey,
        "ledger" => t.acct_ledger,
        "keystone" => t.acct_keystone,
        "watch" => t.acct_watch,
        _ => t.acct_generic,
    }
    .into()
}

/// The device's accounts with their rare.market sign-ins (the live launcher).
pub fn account_lines() -> Vec<AccountLine> {
    let (list, selected) = crate::screens::wallet::page::load_accounts();
    let main = selected.clone().or_else(|| list.first().map(|a| a.address.clone()));
    let keys = rare_friends::storage::FileStorage::new(rare_core::paths::data_dir().join("friends"));
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0);
    list.iter()
        .map(|a| AccountLine {
            address: a.address.clone(),
            label: a.label.clone(),
            kind: kind_text(a),
            session: rare_friends::sign::session_of(&keys, &a.address).filter(|s| s.live(now)).map(|s| s.linked_at),
            main: main.as_deref() == Some(a.address.as_str()),
            can_sign: a.kind != "watch",
            device: a.device,
        })
        .collect()
}

/// Screenshots: three accounts (a phrase one signed in, a passkey one, a watch one).
pub fn fixture_lines() -> Vec<AccountLine> {
    vec![
        AccountLine { address: "0x7d20dcdb2bca4f50".into(), label: "Account 1".into(), kind: "Recovery phrase on this SuiPlay, signs with A".into(), session: Some(1_793_000_000_000), main: true, can_sign: true, device: true },
        AccountLine { address: "0x7bba19bc168f6774".into(), label: "trade".into(), kind: "Passkey, signs with your phone or PIN".into(), session: None, main: false, can_sign: true, device: false },
        AccountLine { address: "0x1111222233334444".into(), label: "A very long account name that keeps going on".into(), kind: "Watch only, cannot sign".into(), session: None, main: false, can_sign: false, device: false },
    ]
}

/// "Signed in to rare.market since Oct 27, 2026" or why not.
pub fn session_text(l: &AccountLine) -> String {
    let t = &tr().settings;
    let s = match l.session {
        Some(0) => trf!(t.session_signed_in, kind = l.kind),
        Some(at) => trf!(t.session_since, kind = l.kind, date = date_of(at)),
        None if l.can_sign => trf!(t.session_not, kind = l.kind),
        None => trf!(t.session_kind_only, kind = l.kind),
    };
    s.trim_end().to_string()
}

/// "Oct 27, 2026" from ms since the epoch (UTC).
pub fn date_of(ms: i64) -> String {
    let iso = crate::screens::wallet::service::iso_of(ms);
    let (y, m, d) = (&iso[0..4], iso[5..7].parse::<usize>().unwrap_or(1), iso[8..10].trim_start_matches('0'));
    let month = tr().settings.months_short.split_whitespace().nth((m - 1).min(11)).unwrap_or("");
    trf!(tr().settings.date_short, month = month, day = d, year = y)
}

/// The send PIN row's line.
pub fn send_pin_body(threshold: Option<f64>) -> String {
    match threshold {
        Some(t) => trf!(tr().settings.send_pin_on, amount = format!("{t:.0}")),
        None => trf!(tr().settings.send_pin_off, amount = format!("{:.0}", rare_wallet::send_pin::DEFAULT_THRESHOLD_USD)),
    }
}

fn short(a: &str) -> String {
    if a.len() > 12 {
        format!("{}...{}", &a[..6], &a[a.len() - 4..])
    } else {
        a.to_string()
    }
}

pub struct AccountsSection {
    /// The account rows' addresses, by row (tags 100 + 3 * row + {0 sign out, 1 make main, 2 sign in}).
    addrs: Vec<String>,
}

const T_ACCOUNT: u32 = 100;
const T_PIN_SET: u32 = 90;
const T_PIN_OFF: u32 = 91;
const T_DEV_PIN: u32 = 92;
const T_DEV_WORDS: u32 = 93;

impl AccountsSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, now: f64) -> AccountsSection {
        let mut prof = super::live::get().map(|s| s.profile).unwrap_or_default();
        if super::live::enabled() {
            // Read the account and Steam now: signing in or out elsewhere shows here at once.
            let fresh = super::live::read_profile();
            if fresh.address != prof.address {
                prof.username = None;
                prof.name = None;
            }
            prof.address = fresh.address;
            prof.label = fresh.label;
            prof.steam = fresh.steam;
        }
        AccountsSection::build_with(cx, nav, parent, cfg, &prof, now)
    }

    pub fn build_with(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, prof: &ProfileInfo, now: f64) -> AccountsSection {
        let (lines, pin) = if super::live::enabled() {
            let pin = crate::screens::wallet::service::wallet().and_then(|w| w.send_pin.threshold_usd());
            (account_lines(), pin)
        } else {
            (fixture_lines(), None)
        };
        Self::build_lines(cx, nav, parent, cfg, prof, &lines, pin, now)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build_lines(cx: &mut Cx, nav: &mut Nav, parent: Id, _cfg: &Config, prof: &ProfileInfo, lines: &[AccountLine], pin: Option<f64>, _now: f64) -> AccountsSection {
        let p = panel(cx, parent);
        let signed_in = prof.address.is_some();
        let t = &tr().settings;
        let rows: Vec<(&str, &str, String, &str, Variant, u32)> = vec![
            ("rare-account", t.row_rare_account, rare_body(prof), if signed_in { tr().common.sign_out } else { tr().common.sign_in }, if signed_in { Variant::Secondary } else { Variant::Primary }, if signed_in { 5 } else { 2 }),
            ("steam", t.row_steam, steam_body(prof), if prof.steam.is_some() { t.open_steam } else { tr().common.sign_in }, if prof.steam.is_some() { Variant::Secondary } else { Variant::Primary }, 6),
            ("wallet", t.row_rare_wallet, t.wallet_body.into(), tr().common.open, Variant::Secondary, 3),
            ("setup", t.row_setup, t.setup_body.into(), t.run_setup, Variant::Secondary, 4),
        ];
        let mut first = true;
        for (i, (sid, title, body, label, v, tag)) in rows.into_iter().enumerate() {
            let r = setting_row(cx, p, title, Some(&body), first);
            first = false;
            let b = button(cx, r.right, label, v, Sz::Md, false).sid(sid).tag(tag);
            nav.add(b.group(10));
            // Epic Games and GOG after Steam (features/stores.rs).
            if sid == "steam" {
                for (j, (store, line, signed)) in crate::features::stores::account_rows().into_iter().enumerate() {
                    let r = setting_row(cx, p, store.name(), Some(&line), false);
                    let b = button(cx, r.right, if signed { tr().common.sign_out } else { tr().common.sign_in }, if signed { Variant::Secondary } else { Variant::Primary }, Sz::Md, false).sid(store.key()).tag(7 + j as u32);
                    nav.add(b.group(10));
                }
            }
            // The username sits under the Rare account it belongs to.
            if i == 0 && signed_in {
                let r = setting_row(cx, p, t.row_username, Some(&username_body(prof)), false);
                let b = button(cx, r.right, if prof.username.is_some() { tr().common.change } else { t.pick_username }, if prof.username.is_some() { Variant::Secondary } else { Variant::Primary }, Sz::Md, false).sid("username").tag(1);
                nav.add(b.group(10));
            }
        }
        // Every account on this device: what it is, its rare.market sign-in, sign out, main.
        let mut addrs = Vec::new();
        if !lines.is_empty() {
            let p = panel(cx, parent);
            let mut first = true;
            for (i, l) in lines.iter().enumerate() {
                let title = if l.main { trf!(t.acct_main_title, label = l.label) } else { l.label.clone() };
                let r = setting_row(cx, p, &title, Some(&session_text(l)), first);
                first = false;
                let base = T_ACCOUNT + 3 * i as u32;
                if l.session.is_some() {
                    let b = button(cx, r.right, tr().common.sign_out, Variant::Secondary, Sz::Md, false).sid(&format!("acct-out-{i}")).tag(base);
                    nav.add(b.group(11));
                } else if l.main && l.can_sign {
                    let b = button(cx, r.right, tr().common.sign_in, Variant::Primary, Sz::Md, false).sid(&format!("acct-in-{i}")).tag(base + 2);
                    nav.add(b.group(11));
                }
                if !l.main {
                    let b = button(cx, r.right, t.make_main, Variant::Secondary, Sz::Md, false).sid(&format!("acct-main-{i}")).tag(base + 1);
                    nav.add(b.group(11));
                }
                addrs.push(l.address.clone());
            }
            let device_pin = if super::live::enabled() { crate::screens::wallet::service::has_device_pin() } else { true };
            if device_pin {
                let r = setting_row(cx, p, t.row_device_pin, Some(t.device_pin_body), false);
                let b = button(cx, r.right, t.change_pin, Variant::Secondary, Sz::Md, false).sid("device-pin").tag(T_DEV_PIN);
                nav.add(b.group(11));
                let b = button(cx, r.right, t.recovery_words, Variant::Secondary, Sz::Md, false).sid("device-words").tag(T_DEV_WORDS);
                nav.add(b.group(11));
            }
            if lines.iter().any(|l| l.device) || !super::live::enabled() {
                let r = setting_row(cx, p, t.row_big_sends, Some(&send_pin_body(pin)), false);
                let b = if pin.is_some() {
                    button(cx, r.right, tr().common.turn_off, Variant::Secondary, Sz::Md, false).sid("send-pin-off").tag(T_PIN_OFF)
                } else {
                    button(cx, r.right, t.set_pin, Variant::Secondary, Sz::Md, false).sid("send-pin-set").tag(T_PIN_SET)
                };
                nav.add(b.group(11));
            }
        }
        AccountsSection { addrs }
    }
}

impl SectionView for AccountsSection {
    fn press(&mut self, _cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, _now: f64) -> SectionOut {
        match nav.current().map(|f| f.tag) {
            Some(1) => SectionOut::Act("accounts:pick-username".into()),
            Some(2) => SectionOut::Act("open:signin".into()),
            Some(3) => SectionOut::Act("tab:wallet".into()),
            Some(4) => SectionOut::Act("open:firstrun".into()),
            Some(5) => SectionOut::Act("accounts:sign-out".into()),
            Some(6) => SectionOut::Act("accounts:steam".into()),
            Some(t @ (7 | 8)) => {
                let store = if t == 7 { rare_stores::Store::Epic } else { rare_stores::Store::Gog };
                SectionOut::Act(format!("stores:{}:{}", if store.signed_in() { "signout" } else { "signin" }, store.key()))
            }
            Some(T_PIN_SET) => SectionOut::Act("accounts:send-pin:set".into()),
            Some(T_PIN_OFF) => SectionOut::Act("accounts:send-pin:off".into()),
            Some(T_DEV_PIN) => SectionOut::Act("accounts:pin:change".into()),
            Some(T_DEV_WORDS) => SectionOut::Act("accounts:pin:words".into()),
            Some(t) if t >= T_ACCOUNT => {
                let (row, what) = (((t - T_ACCOUNT) / 3) as usize, (t - T_ACCOUNT) % 3);
                let Some(a) = self.addrs.get(row) else { return SectionOut::None };
                SectionOut::Act(match what {
                    0 => format!("accounts:signout:{a}"),
                    1 => format!("accounts:main:{a}"),
                    _ => "accounts:signin".into(),
                })
            }
            _ => SectionOut::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_username_line() {
        assert_eq!(username_body(&ProfileInfo::default()), "Friends find you by it. Free, and nobody needs a SuiNS name.");
        let p = ProfileInfo { address: Some("0x1".into()), username: Some("tyler".into()), ..Default::default() };
        assert_eq!(username_body(&p), "tyler. You can change it once every 30 days.");
        let p = ProfileInfo { name: Some("tyler.sui".into()), ..p };
        assert_eq!(username_body(&p), "tyler, SuiNS tyler.sui. You can change it once every 30 days.");
        assert_eq!(rare_body(&p), "Signed in as tyler. One sign-in for Rare Wallet and Rare Marketplace.");
        assert_eq!(rare_body(&ProfileInfo::default()), "One sign-in for Rare Wallet and Rare Marketplace. Your games work without an account.");
        let s = ProfileInfo { steam: Some("playerone".into()), ..Default::default() };
        assert_eq!(steam_body(&s), "Signed in as playerone. Sign out or switch accounts in Steam's settings.");
    }
}
