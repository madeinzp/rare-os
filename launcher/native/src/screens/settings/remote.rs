//! Remote access (SSH) and network apps: the rows for Settings > Developer, and the short entry in
//! System. The state is `SshInfo` in the live snapshot (read through `rare_system::ssh`, which asks
//! the image's `os-admin ssh` helper). Off by default. See docs/SSH.md.

use super::rows::*;
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_i18n::{tr, trf, trn};
use rare_render::{Fit, Id, ImgReq, Node, Src};
use rare_system::ssh::{self, SshKey, SshStatus};

/// English fallback; the screen reads `tr().settings_more.rem_warning`.
pub const WARNING: &str = "Anyone on your network with the password can control this device.";

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SshInfo {
    /// None until the first reading, or when the helper isn't there (not a Rare OS image).
    pub status: Option<SshStatus>,
    pub host: String,
    pub ip: Option<String>,
    /// A change is under way.
    pub busy: bool,
    /// "Allow network apps" is on.
    pub netapi: bool,
    pub net_token: Option<String>,
    /// The capability switches that are on (`rare_localapi::CAPS` ids).
    pub caps: Vec<String>,
    /// The latest calls from network apps, newest first.
    #[serde(skip)]
    pub activity: Vec<rare_localapi::Activity>,
}

impl SshInfo {
    pub fn on(&self) -> bool {
        self.status.as_ref().is_some_and(|s| s.enabled)
    }
    pub fn user(&self) -> String {
        self.status.as_ref().map(|s| s.user.clone()).filter(|u| !u.is_empty()).unwrap_or_else(|| "bazzite".into())
    }
    pub fn command(&self) -> String {
        ssh::connect_command(&self.user(), &self.host)
    }
}

pub fn fixture_info() -> SshInfo {
    SshInfo { status: Some(SshStatus { user: "bazzite".into(), password_auth: true, ..Default::default() }), host: "suiplay".into(), ip: Some("192.168.1.92".into()), ..Default::default() }
}

/// What a fixture run (no device) shows: SSH off, or on with RR_FIXTURE_SSH=on (screenshots), or
/// on with GameOS's default password still set with RR_FIXTURE_SSH=default-password.
pub fn fixture_default() -> SshInfo {
    match std::env::var("RR_FIXTURE_SSH").as_deref() {
        Ok("on") => return fixture_on(),
        Ok("default-password") => {
            let mut i = fixture_on();
            if let Some(st) = i.status.as_mut() {
                st.user = "playtron".into();
                st.keys.clear();
                st.default_password = true;
                st.password_auth = false;
            }
            return i;
        }
        _ => {}
    }
    fixture_info()
}

/// A fixture with SSH on, a key and network apps allowed (screenshots).
pub fn fixture_on() -> SshInfo {
    let st = SshStatus {
        enabled: true,
        active: true,
        user: "bazzite".into(),
        has_password: true,
        keys: vec![SshKey { fingerprint: "SHA256:9fT2xQ0c1mZ7vYk4bN8aLr3uHw5eJd6sPqXoVtCiAg0".into(), kind: "ed25519".into(), comment: "me@laptop".into() }],
        ..Default::default()
    };
    let now = now_secs() * 1000;
    let act = |ago_s: u64, cap: &str, what: &str, ok: bool| rare_localapi::Activity { at_ms: now.saturating_sub(ago_s * 1000), cap: cap.into(), what: what.into(), ok, from: "192.168.0.50".into() };
    SshInfo {
        status: Some(st),
        netapi: true,
        net_token: Some("4f1d9c7a0b3e58126a9d0c4e7f2b6a83d15e90c7b4a2f6081e3d5c9a7b0f4e21".into()),
        caps: vec!["profile".into(), "library".into(), "input".into()],
        activity: vec![act(12, "input", "input 3 events", true), act(95, "library", "library", true), act(310, "launch", "launch steam:620 (switch is off)", false)],
        ..fixture_info()
    }
}

/// "42 minutes left" for the 1 hour timer.
pub fn left_text(at: u64, now: u64) -> String {
    let m = at.saturating_sub(now).div_ceil(60);
    if m <= 1 {
        tr().settings_more.rem_less_than_minute.into()
    } else {
        trn!(tr().settings_more.rem_minutes_left, m)
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub(super) fn small_button(cx: &mut Cx, nav: &mut Nav, parent: Id, label: &str, sid: &str, tag: u32) {
    let b = button(cx, parent, label, Variant::Secondary, Sz::Md, false).sid(sid).tag(tag);
    nav.add(b.group(11));
}

pub(super) fn qr_box(cx: &mut Cx, parent: Id, text: &str, rem: f32) {
    let r = cx.r();
    let px = (rem * r).round() as u32;
    let Some(png) = crate::screens::share::qr::png(text, px) else { return };
    let name: &'static str = Box::leak(format!("ssh-qr-{px}-{}.png", text.len()).into_boxed_str());
    let bytes: &'static [u8] = Box::leak(png.into_boxed_slice());
    let k = cx.img.request(ImgReq { src: Src::Static(name, bytes), w: px, h: px, fit: Fit::Contain });
    let frame = cx.scene.add(parent, s().wh(rem * r, rem * r).done(), Node::bg(rare_render::Color::WHITE).radius(0.4 * r).clip());
    cx.scene.add(frame, s().fill().done(), Node::default().image(k));
}

/// The SSH rows, built into Developer. Returns the keys listed (a Remove button's tag is its index).
pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, rows: &mut RowSet, info: &SshInfo, now_s: u64) -> Vec<SshKey> {
    let on = info.on();
    let st = info.status.clone().unwrap_or_default();
    let p = panel(cx, parent);
    let t = &tr().settings_more;
    let intro = if info.busy {
        t.rem_working
    } else if on {
        t.rem_intro_on
    } else {
        t.rem_intro_off
    };
    rows.switch(cx, nav, p, "ssh-toggle", t.rem_title, intro, on, true);
    setting_row(cx, p, t.rem_before, Some(t.rem_warning), false);
    if !on {
        return vec![];
    }
    // Where to connect.
    let cmd = info.command();
    let addr = match &info.ip {
        Some(ip) => trf!(t.rem_addr_ip, host = info.host.trim_end_matches(".local"), ip = ip, user = info.user()),
        None => trf!(t.rem_addr, host = info.host.trim_end_matches(".local"), user = info.user()),
    };
    let row_ = setting_row(cx, p, &cmd, Some(&addr), false);
    small_button(cx, nav, row_.right, tr().common.copy, "ssh-copy", 0);
    qr_box(cx, row_.right, &cmd, 9.0);
    if !st.active {
        setting_row(cx, p, t.rem_starting, Some(t.rem_starting_body), false);
    }
    // Password.
    let pw_body = if st.default_password {
        t.rem_pw_default
    } else if st.has_password {
        if st.password_auth { t.rem_pw_on } else { t.rem_pw_off_key }
    } else {
        t.rem_pw_none
    };
    let row_ = setting_row(cx, p, t.rem_password, Some(pw_body), false);
    small_button(cx, nav, row_.right, if st.default_password { t.rem_set_new_password } else if st.has_password { tr().common.change } else { t.rem_set_password }, "ssh-password", 0);
    // Keys.
    let keys_body = if st.keys.is_empty() { t.rem_keys_none.to_string() } else { trn!(t.rem_keys_n, st.keys.len()) };
    let row_ = setting_row(cx, p, t.rem_add_key, Some(&keys_body), false);
    small_button(cx, nav, row_.right, t.rem_paste_or_type, "ssh-key-paste", 0);
    small_button(cx, nav, row_.right, t.rem_usb_stick, "ssh-key-usb", 0);
    small_button(cx, nav, row_.right, "GitHub", "ssh-key-github", 0);
    for (i, k) in st.keys.iter().enumerate() {
        let title = if k.comment.is_empty() { trf!(t.rem_key_kind, kind = k.kind) } else { trf!(t.rem_key_comment, comment = k.comment, kind = k.kind) };
        let row_ = setting_row(cx, p, &title, Some(&k.fingerprint), false);
        small_button(cx, nav, row_.right, tr().common.remove, "ssh-remove", i as u32);
    }
    if !st.keys.is_empty() {
        rows.switch(cx, nav, p, "ssh-keep-password", t.rem_keep_pw, t.rem_keep_pw_body, st.keep_password, false);
    }
    let timer = match st.auto_off_at {
        Some(at) => trf!(t.rem_auto_off_in, left = left_text(at, now_secs().max(now_s))),
        None => t.rem_auto_off_idle.into(),
    };
    rows.switch(cx, nav, p, "ssh-autooff", t.rem_auto_off, &timer, st.auto_off_at.is_some(), false);
    st.keys
}

/// The developer tools panel: where the docs are, and "Allow network apps".
pub fn build_tools(cx: &mut Cx, nav: &mut Nav, parent: Id, rows: &mut RowSet, info: &SshInfo) {
    let t = &tr().settings_more;
    let p = panel(cx, parent);
    setting_row(cx, p, t.rem_dev_tools, Some(t.rem_dev_tools_body), true);
    let host = info.host.trim_end_matches(".local").to_string();
    let body = if info.netapi {
        trf!(t.rem_netapi_on, host = host, port = ssh::NETAPI_PORT)
    } else {
        t.rem_netapi_off.to_string()
    };
    rows.switch(cx, nav, p, "netapi-toggle", t.rem_netapi, &body, info.netapi, false);
    if info.netapi {
        if let Some(tok) = &info.net_token {
            let row_ = setting_row(cx, p, t.rem_token, Some(tok), false);
            small_button(cx, nav, row_.right, tr().common.copy, "netapi-copy", 0);
            small_button(cx, nav, row_.right, t.rem_new_token, "netapi-newtoken", 0);
        }
        // One switch per capability, all off until the owner turns them on.
        let p = panel(cx, parent);
        setting_row(cx, p, t.rem_caps, Some(t.rem_caps_body), true);
        for (id, title, about) in rare_localapi::CAPS {
            rows.switch(cx, nav, p, &format!("netapi-cap-{id}"), title, about, info.caps.iter().any(|c| c == id), false);
        }
        let p = panel(cx, parent);
        setting_row(cx, p, t.rem_activity, Some(t.rem_activity_body), true);
        if info.activity.is_empty() {
            setting_row(cx, p, t.rem_activity_none, Some(t.rem_activity_none_body), false);
        }
        let now = now_secs() * 1000;
        for a in info.activity.iter().take(8) {
            let cap = rare_localapi::CAPS.iter().find(|c| c.0 == a.cap).map(|c| c.1).unwrap_or(a.cap.as_str());
            let body = if a.ok { trf!(t.rem_call_allowed, cap = cap, from = a.from) } else { trf!(t.rem_call_refused, cap = cap, from = a.from) };
            setting_row(cx, p, &format!("{} - {}", ago_text(a.at_ms, now), a.what), Some(&body), false);
        }
    }
}

/// "just now", "5 min ago", "2 h ago".
pub fn ago_text(at_ms: u64, now_ms: u64) -> String {
    let s = now_ms.saturating_sub(at_ms) / 1000;
    match s {
        0..=9 => tr().settings_more.rem_just_now.to_string(),
        10..=59 => trf!(tr().settings_more.rem_s_ago, n = s),
        60..=3599 => trf!(tr().settings_more.rem_min_ago, n = s / 60),
        _ => trf!(tr().settings_more.rem_h_ago, n = s / 3600),
    }
}

/// The short entry in System.
pub fn build_system(cx: &mut Cx, nav: &mut Nav, parent: Id, rows: &mut RowSet, info: &SshInfo) {
    let p = panel(cx, parent);
    let t = &tr().settings_more;
    let body = if info.on() { trf!(t.rem_sys_on, command = info.command()) } else { t.rem_sys_off.to_string() };
    rows.switch(cx, nav, p, "ssh-system", t.rem_title, &body, info.on(), true);
}

/// A on a focused row of ours: the action for the app, or None when the row is not ours.
pub fn press(rows: &mut RowSet, keys: &[SshKey], cx: &mut Cx, nav: &mut Nav, f: &Focusable, now: f64) -> Option<super::SectionOut> {
    use super::SectionOut::Act;
    let sid = f.sid.as_str();
    let act = |a: &str| Some(Act(a.into()));
    if let Some(cap) = sid.strip_prefix("netapi-cap-") {
        let (_, v) = rows.press(cx, nav, now)?;
        return act(&format!("netapi:cap:{cap}:{}", matches!(v, Val::Bool(true))));
    }
    match sid {
        "ssh-toggle" | "ssh-system" | "ssh-keep-password" | "ssh-autooff" | "netapi-toggle" => {
            let (_, v) = rows.press(cx, nav, now)?;
            let on = matches!(v, Val::Bool(true));
            let a = match sid {
                "ssh-toggle" | "ssh-system" => "ssh:toggle",
                "ssh-keep-password" => "ssh:keep-password",
                "ssh-autooff" => "ssh:auto-off",
                _ => "netapi:toggle",
            };
            act(&format!("{a}:{on}"))
        }
        "ssh-copy" => act("ssh:copy"),
        "ssh-password" => act("ssh:ask-password"),
        "ssh-key-paste" => act("ssh:ask-key"),
        "ssh-key-usb" => act("ssh:usb"),
        "ssh-key-github" => act("ssh:ask-github"),
        "ssh-remove" => keys.get(f.tag as usize).and_then(|k| act(&format!("ssh:remove-ask:{}", k.fingerprint))),
        "netapi-copy" => act("netapi:copy"),
        "netapi-newtoken" => act("netapi:newtoken"),
        _ => None,
    }
}

/// Words for the OSK requests.
pub mod ask {
    use rare_i18n::tr;

    /// English fallbacks; the `*_title()` functions give the player's language.
    pub const PASSWORD_TITLE: &str = "Password for remote sign-in";
    pub const KEY_TITLE: &str = "Public key (starts with ssh-ed25519 or ssh-rsa)";
    pub const GITHUB_TITLE: &str = "GitHub user name";

    pub fn password_title() -> &'static str {
        tr().settings_more.rem_ask_password
    }
    pub fn key_title() -> &'static str {
        tr().settings_more.rem_ask_key
    }
    pub fn github_title() -> &'static str {
        tr().settings_more.rem_ask_github
    }
}

/// The confirm sheet for a pending key list: (title, sub, body).
pub fn github_confirm(user: &str, n: usize) -> (String, String, String) {
    (
        trn!(tr().settings_more.rem_gh_title, n),
        format!("github.com/{user}"),
        trf!(tr().settings_more.rem_gh_body, user = user),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_and_state() {
        let i = fixture_on();
        assert!(i.on());
        assert_eq!(i.command(), "ssh bazzite@suiplay.local");
        assert!(!fixture_info().on());
        assert!(!SshInfo::default().on());
    }

    #[test]
    fn ago_words() {
        assert_eq!(ago_text(1_000_000, 1_003_000), "just now");
        assert_eq!(ago_text(1_000_000, 1_030_000), "30 s ago");
        assert_eq!(ago_text(1_000_000, 1_300_000), "5 min ago");
        assert_eq!(ago_text(0, 7_300_000), "2 h ago");
    }

    #[test]
    fn every_capability_has_a_plain_line() {
        assert_eq!(rare_localapi::CAPS.len(), 8);
        for (_, title, about) in rare_localapi::CAPS {
            assert!(!title.is_empty() && about.ends_with('.') && !about.contains('\u{2014}'));
        }
    }

    #[test]
    fn timer_words() {
        assert_eq!(left_text(1000 + 2520, 1000), "42 minutes left");
        assert_eq!(left_text(1000, 1000), "less than a minute left");
    }

    #[test]
    fn github_words() {
        let (t, sub, body) = github_confirm("octocat", 2);
        assert_eq!(t, "Add 2 keys from GitHub?");
        assert_eq!(sub, "github.com/octocat");
        assert!(body.contains("octocat"));
        assert!(github_confirm("a", 1).0.contains("1 key from"));
    }
}

