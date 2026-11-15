//! The Network section (screens/settings/NetworkSection.tsx): Wi-Fi (scan and join), Bluetooth.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_i18n::{tr, trf};
use rare_render::theme::tok;
use rare_render::{Id, Node};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NetStatus {
    pub online: bool,
    pub wifi: Option<String>,
    /// false when NetworkManager isn't available (only status can be shown).
    pub managed: bool,
}

#[derive(Clone, Debug)]
pub struct WifiNet {
    pub ssid: String,
    pub signal: u32,
    pub secure: bool,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    pub present: bool,
    pub powered: bool,
    pub blocked: bool,
}

/// What the section shows at one moment (the backend's answers plus the view's own busy state).
#[derive(Clone, Debug)]
pub struct NetInfo {
    pub net: Option<NetStatus>,
    pub nets: Option<Vec<WifiNet>>,
    pub scanning: bool,
    pub joining: Option<String>,
    pub error: Option<String>,
    /// None: still asking. Some(None): no adapter at all.
    pub bt: Option<Option<BluetoothAdapter>>,
    /// Devices BlueZ knows when the adapter is on.
    pub bt_devices: Vec<BtDev>,
    pub bt_scanning: bool,
    /// The address being paired or connected.
    pub bt_busy: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BtDev {
    pub addr: String,
    pub name: String,
    pub paired: bool,
    pub connected: bool,
}

pub fn fixture_info() -> NetInfo {
    let n = |ssid: &str, signal: u32, secure: bool, active: bool| WifiNet { ssid: ssid.into(), signal, secure, active };
    NetInfo {
        net: Some(NetStatus { online: true, wifi: Some("Home".into()), managed: true }),
        nets: Some(vec![n("Home", 88, true, true), n("Neighbour 5G", 61, true, false), n("Coffee Shop", 43, false, false), n("Guest", 20, true, false)]),
        scanning: false,
        joining: None,
        error: None,
        bt: Some(Some(BluetoothAdapter { present: true, powered: false, blocked: false })),
        bt_devices: vec![],
        bt_scanning: false,
        bt_busy: None,
    }
}

/// Signal strength as 1 to 4 bars (onboarding/steps/Wifi.tsx Bars).
pub fn bars_for(signal: u32) -> u32 {
    if signal >= 75 {
        4
    } else if signal >= 50 {
        3
    } else if signal >= 25 {
        2
    } else {
        1
    }
}

/// The Wi-Fi row's line.
pub fn wifi_body(net: Option<&NetStatus>) -> String {
    match net {
        None => tr().settings_more.net_checking.into(),
        Some(n) if !n.managed => tr().settings_more.net_managed_outside.into(),
        Some(n) => match &n.wifi {
            Some(w) if n.online => trf!(tr().settings_more.net_connected_to, name = w),
            Some(w) => trf!(tr().settings_more.net_connected_no_internet, name = w),
            None if n.online => tr().settings_more.net_connected_internet.into(),
            None => tr().settings_more.net_not_connected.into(),
        },
    }
}

/// The Bluetooth row's line.
pub fn bluetooth_body(bt: &Option<BluetoothAdapter>) -> &'static str {
    match bt {
        Some(b) if b.present => {
            if b.powered {
                tr().settings_more.net_bt_on_body
            } else if b.blocked {
                tr().settings_more.net_bt_blocked
            } else {
                tr().settings_more.net_bt_off
            }
        }
        _ => tr().settings_more.net_bt_none,
    }
}

fn signal_bars(cx: &mut Cx, parent: Id, signal: u32) {
    let r = cx.r();
    let n = bars_for(signal);
    let h = 0.95 * r;
    let w = cx.scene.add(parent, s().row().items_end().gap(2.0).h(h).no_shrink_().done(), Node::default());
    for i in 1..=4u32 {
        let col = if i <= n { tok::FG.alpha(0.85) } else { tok::FG.alpha(0.2) };
        cx.scene.add(w, s().w(3.0).h(h * i as f32 * 0.25).done(), Node::bg(col).radius(1.0));
    }
}

const WIFI_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h.01"/><path d="M2 8.82a15 15 0 0 1 20 0"/><path d="M5 12.859a10 10 0 0 1 14 0"/><path d="M8.5 16.429a5 5 0 0 1 7 0"/></svg>"#;

pub struct NetworkSection {
    info: NetInfo,
}

impl NetworkSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, now: f64) -> NetworkSection {
        let info = super::live::get().map(|s| s.net).unwrap_or_else(fixture_info);
        NetworkSection::build_with(cx, nav, parent, cfg, info, now)
    }

    pub fn build_with(cx: &mut Cx, nav: &mut Nav, parent: Id, _cfg: &Config, info: NetInfo, _now: f64) -> NetworkSection {
        let r = cx.r();
        let p1 = panel(cx, parent);
        let managed = info.net.as_ref().map(|n| n.managed).unwrap_or(false);
        let row_ = setting_row(cx, p1, "Wi-Fi", Some(&wifi_body(info.net.as_ref())), true);
        if managed {
            let b = button(cx, row_.right, if info.scanning { tr().settings_more.net_scanning } else { tr().settings_more.net_scan_again }, if info.scanning || info.joining.is_some() { Variant::Dimmed } else { Variant::Secondary }, Sz::Md, false).hint(tr().settings_more.net_scan).sid("wifi").tag(1);
            nav.add(b.group(10));
        }
        if let Some(e) = &info.error {
            let er = setting_row(cx, p1, "", None, false);
            cx.scene.set_text(cx.t, er.title, e);
            cx.scene.n(er.title).text.as_mut().unwrap().color.snap(tok::NEG);
        }
        // A hotel or airport network that wants a sign-in (features/portal.rs).
        if managed && crate::features::portal::needs_sign_in() {
            let pr = setting_row(cx, p1, tr().settings_more.net_needs_sign_in, Some(tr().settings_more.net_needs_sign_in_body), false);
            let b = button(cx, pr.right, tr().common.sign_in, Variant::Secondary, Sz::Md, false).hint(tr().common.sign_in).sid("wifi-portal").tag(5);
            nav.add(b.group(10));
        }
        let active = info.nets.as_ref().and_then(|n| n.iter().find(|x| x.active)).cloned();
        if let (true, Some(a)) = (managed, &active) {
            let fr = setting_row(cx, p1, tr().settings_more.net_forget_title, Some(&trf!(tr().settings_more.net_forget_body, name = a.ssid)), false);
            let b = button(cx, fr.right, tr().settings_more.net_forget, Variant::Secondary, Sz::Md, false).sid("wifi-forget").tag(2);
            nav.add(b.group(10));
        }
        // Bluetooth
        if let Some(bt) = &info.bt {
            let br = setting_row(cx, p1, "Bluetooth", Some(bluetooth_body(bt)), false);
            if let Some(b) = bt.as_ref().filter(|b| b.present) {
                if !b.powered {
                    let btn = button(cx, br.right, tr().common.turn_on, Variant::Secondary, Sz::Md, false).sid("bluetooth").tag(3);
                    nav.add(btn.group(10));
                } else {
                    let st = cx.scene.add(br.right, s().row().items_center().gap(0.5 * r).done(), Node::default());
                    dot(cx, st, tok::MUTED);
                    text(cx, st, ty::SMALL, 400, tok::MUTED, tr().common.on);
                }
            }
        }
        // Bluetooth devices: scan, pair, connect, forget.
        if let Some(Some(b)) = &info.bt {
            if b.present && b.powered {
                let p3 = panel(cx, parent);
                let row_ = setting_row(cx, p3, tr().settings_more.net_devices, Some(if info.bt_scanning { tr().settings_more.net_devices_looking } else { tr().settings_more.net_devices_idle }), true);
                let btn = button(cx, row_.right, if info.bt_scanning { tr().settings_more.net_looking } else { tr().settings_more.net_look_for_devices }, if info.bt_scanning || info.bt_busy.is_some() { Variant::Dimmed } else { Variant::Secondary }, Sz::Md, false).hint(tr().settings_more.net_look_for_devices).sid("bt-scan").tag(4);
                nav.add(btn.group(10));
                for (i, d) in info.bt_devices.iter().enumerate() {
                    let state = if info.bt_busy.as_deref() == Some(d.addr.as_str()) {
                        tr().settings_more.net_working
                    } else if d.connected {
                        tr().common.connected
                    } else if d.paired {
                        tr().settings_more.net_paired
                    } else {
                        tr().settings_more.net_not_paired
                    };
                    let r2 = setting_row(cx, p3, &d.name, Some(state), false);
                    let label = if d.connected { tr().common.disconnect } else if d.paired { tr().common.connect } else { tr().settings_more.net_pair };
                    let v = if d.connected || info.bt_busy.is_some() { Variant::Dimmed } else { Variant::Secondary };
                    let v = if d.connected { Variant::Secondary } else { v };
                    nav.add(button(cx, r2.right, label, v, Sz::Md, false).hint(label).sid(&format!("bt:{}", d.addr)).tag(200 + i as u32).group(10));
                    if d.paired {
                        nav.add(button(cx, r2.right, tr().settings_more.net_forget, Variant::Secondary, Sz::Md, false).hint(tr().settings_more.net_forget).sid(&format!("btf:{}", d.addr)).tag(300 + i as u32).group(10));
                    }
                }
            }
        }
        if managed {
            let p2 = panel(cx, parent);
            let nets = info.nets.clone().unwrap_or_default();
            if nets.is_empty() && !info.scanning && info.nets.is_some() {
                setting_row(cx, p2, tr().settings_more.net_none_found, Some(tr().settings_more.net_none_found_body), true);
            }
            for (i, n) in nets.iter().enumerate() {
                let row_ = cx.scene.add(p2, s().row().items_center().gap(1.0 * r).min_h(3.75 * r).wp(1.0).px(1.4 * r).py(0.7 * r).done(), Node::default());
                if i > 0 {
                    edge(cx, row_, 0, tok::HAIR);
                }
                icon(cx, row_, "wifi", WIFI_ICON, 1.15, tok::FG);
                cx.scene.add_text(cx.t, row_, s().grow().done(), &n.ssid, ts(r, ty::BODY.0, ty::BODY.1, 500).truncate(), tok::FG);
                let joining_this = info.joining.as_deref() == Some(n.ssid.as_str());
                if joining_this {
                    text(cx, row_, ty::SMALL, 400, tok::MUTED, tr().settings_more.net_joining);
                } else if n.active {
                    text(cx, row_, ty::SMALL, 400, tok::MUTED, tr().common.connected);
                } else if n.secure {
                    text(cx, row_, ty::SMALL, 400, tok::MUTED, tr().settings_more.net_password);
                }
                signal_bars(cx, row_, n.signal);
                // The network in use is only shown (Forget this network above acts on it).
                if !n.active {
                    let hint = if info.joining.is_some() { tr().settings_more.net_joining } else { tr().settings_more.net_join };
                    nav.add(Focusable { id: row_, kind: Kind::Row, group: 11, hint: hint.into(), tag: 100 + i as u32, sid: format!("net:{}", n.ssid) });
                }
            }
        }
        NetworkSection { info }
    }
}

impl SectionView for NetworkSection {
    fn press(&mut self, _cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, _now: f64) -> SectionOut {
        let Some(f) = nav.current() else { return SectionOut::None };
        match f.tag {
            1 if !self.info.scanning && self.info.joining.is_none() => SectionOut::Act("network:scan".into()),
            2 => {
                let ssid = self.info.nets.as_ref().and_then(|n| n.iter().find(|x| x.active)).map(|x| x.ssid.clone()).unwrap_or_default();
                SectionOut::Act(format!("network:forget:{ssid}"))
            }
            3 => SectionOut::Act("network:bluetooth-on".into()),
            5 => SectionOut::Act("network:portal".into()),
            4 if !self.info.bt_scanning && self.info.bt_busy.is_none() => SectionOut::Act("network:bt-scan".into()),
            t if (200..300).contains(&t) && self.info.bt_busy.is_none() => match self.info.bt_devices.get((t - 200) as usize) {
                Some(d) if d.connected => SectionOut::Act(format!("network:bt-disconnect:{}", d.addr)),
                Some(d) if d.paired => SectionOut::Act(format!("network:bt-connect:{}", d.addr)),
                Some(d) => SectionOut::Act(format!("network:bt-pair:{}", d.addr)),
                None => SectionOut::None,
            },
            t if (300..400).contains(&t) && self.info.bt_busy.is_none() => match self.info.bt_devices.get((t - 300) as usize) {
                Some(d) => SectionOut::Act(format!("network:bt-forget:{}", d.addr)),
                None => SectionOut::None,
            },
            t if t >= 100 => {
                let n = self.info.nets.as_ref().and_then(|n| n.get((t - 100) as usize));
                match n {
                    Some(n) if !n.active => SectionOut::Act(format!("network:join:{}:{}", n.secure as u8, n.ssid)),
                    _ => SectionOut::None,
                }
            }
            _ => SectionOut::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_maps_to_one_to_four_bars() {
        assert_eq!(bars_for(100), 4);
        assert_eq!(bars_for(75), 4);
        assert_eq!(bars_for(74), 3);
        assert_eq!(bars_for(50), 3);
        assert_eq!(bars_for(25), 2);
        assert_eq!(bars_for(24), 1);
        assert_eq!(bars_for(0), 1);
    }

    #[test]
    fn the_wifi_line() {
        assert_eq!(wifi_body(None), "Checking");
        assert_eq!(wifi_body(Some(&NetStatus { online: true, wifi: None, managed: false })), "Managed outside Rare OS. Change networks in desktop mode.");
        assert_eq!(wifi_body(Some(&NetStatus { online: true, wifi: Some("Home".into()), managed: true })), "Connected to Home.");
        assert_eq!(wifi_body(Some(&NetStatus { online: false, wifi: Some("Home".into()), managed: true })), "Connected to Home, no internet.");
        assert_eq!(wifi_body(Some(&NetStatus { online: true, wifi: None, managed: true })), "Connected to the internet.");
        assert_eq!(wifi_body(Some(&NetStatus { online: false, wifi: None, managed: true })), "Not connected.");
    }

    #[test]
    fn the_bluetooth_line() {
        assert_eq!(bluetooth_body(&None), "No Bluetooth adapter found on this device.");
        let b = |p, w, bl| Some(BluetoothAdapter { present: p, powered: w, blocked: bl });
        assert_eq!(bluetooth_body(&b(false, false, false)), "No Bluetooth adapter found on this device.");
        assert_eq!(bluetooth_body(&b(true, true, false)), tr().settings_more.net_bt_on_body);
        assert_eq!(bluetooth_body(&b(true, false, true)), tr().settings_more.net_bt_blocked);
        assert_eq!(bluetooth_body(&b(true, false, false)), "Off.");
    }
}
