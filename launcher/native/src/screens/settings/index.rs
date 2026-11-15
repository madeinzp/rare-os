//! Every row of Settings, once (screens/settings/index.ts). Search reads this list.

use super::search::SettingEntry;
use super::sections::SectionId;
use rare_i18n::tr;

/// The kinds of notice that have a switch in Settings, in the order shown (notify/kinds.ts).
pub const KINDS: [&str; 18] = ["sold", "bought", "offer_received", "coin_received", "nft_received", "airdrop", "perp_liquidation", "price_alert", "followed_trade", "friend_request", "friend_online", "party_invite", "direct_message", "party_message", "os_update", "download_done", "emu_update", "trophy"];

/// A notice kind's title and what it is for, in the current language.
pub fn kind_text(kind: &str) -> (&'static str, &'static str) {
    let t = &tr().settings;
    match kind {
        "sold" => (t.nk_sold, t.nkb_sold),
        "bought" => (t.nk_bought, t.nkb_bought),
        "offer_received" => (t.nk_offer_received, t.nkb_offer_received),
        "coin_received" => (t.nk_coin_received, t.nkb_coin_received),
        "nft_received" => (t.nk_nft_received, t.nkb_nft_received),
        "airdrop" => (t.nk_airdrop, t.nkb_airdrop),
        "perp_liquidation" => (t.nk_perp_liquidation, t.nkb_perp_liquidation),
        "price_alert" => (t.nk_price_alert, t.nkb_price_alert),
        "followed_trade" => (t.nk_followed_trade, t.nkb_followed_trade),
        "friend_request" => (t.nk_friend_request, t.nkb_friend_request),
        "friend_online" => (t.nk_friend_online, t.nkb_friend_online),
        "party_invite" => (t.nk_party_invite, t.nkb_party_invite),
        "direct_message" => (t.nk_direct_message, t.nkb_direct_message),
        "party_message" => (t.nk_party_message, t.nkb_party_message),
        "os_update" => (t.nk_os_update, t.nkb_os_update),
        "download_done" => (t.nk_download_done, t.nkb_download_done),
        "emu_update" => (t.nk_emu_update, t.nkb_emu_update),
        "trophy" => (t.nk_trophy, t.nkb_trophy),
        _ => ("", ""),
    }
}

/// `keywords` is one catalog string of space-separated search words in the current language.
fn e(id: &str, section: SectionId, title: &str, keywords: &str) -> SettingEntry {
    SettingEntry { id: id.into(), section, title: title.into(), keywords: keywords.split_whitespace().map(String::from).collect() }
}

fn words_over3(body: &str) -> Vec<String> {
    body.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '_').filter(|w| w.len() > 3).map(String::from).collect()
}

pub fn settings_index() -> Vec<SettingEntry> {
    let t = &tr().settings;
    let mut v = vec![
        e("devices-pair", SectionId::Devices, t.ix_devices_pair, t.ixk_devices_pair),
        e("ssh-system", SectionId::System, t.ix_ssh_system, t.ixk_ssh_system),
        e("language", SectionId::System, t.ix_language, t.ixk_language),
        e("updates", SectionId::System, t.ix_updates, t.ixk_updates),
        e("restore-system", SectionId::System, t.ix_restore_system, t.ixk_restore_system),
        e("time-zone", SectionId::System, t.ix_time_zone, t.ixk_time_zone),
        e("time-auto", SectionId::System, t.ix_time_auto, t.ixk_time_auto),
        e("logs-save", SectionId::System, t.ix_logs_save, t.ixk_logs_save),
        e("factory-reset", SectionId::System, t.ix_factory_reset, t.ixk_factory_reset),
        e("boot-menu", SectionId::System, t.ix_boot_menu, t.ixk_boot_menu),
        e("desktop-mode", SectionId::System, t.ix_desktop_mode, t.ixk_desktop_mode),
        e("docked", SectionId::Display, t.ix_docked, t.ixk_docked),
        e("ui-size", SectionId::Display, t.ix_ui_size, t.ixk_ui_size),
        e("brightness", SectionId::Display, t.ix_brightness, t.ixk_brightness),
        e("dim-idle", SectionId::Display, t.ix_dim_idle, t.ixk_dim_idle),
        e("night-light", SectionId::Display, t.ix_night_light, t.ixk_night_light),
        e("tv-size", SectionId::Display, t.ix_tv_size, t.ixk_tv_size),
        e("tv-refresh", SectionId::Display, t.ix_tv_refresh, t.ixk_tv_refresh),
        e("volume", SectionId::Sound, t.ix_volume, t.ixk_volume),
        e("output", SectionId::Sound, t.ix_output, t.ixk_output),
        e("mic", SectionId::Sound, t.ix_mic, t.ixk_mic),
        e("mic-level", SectionId::Sound, t.ix_mic_level, t.ixk_mic_level),
        e("sounds", SectionId::Sound, t.ix_sounds, t.ixk_sounds),
        e("buttons", SectionId::Controls, t.ix_buttons, t.ixk_buttons),
        e("btn-lc", SectionId::Controls, t.ix_btn_lc, t.ixk_btn_lc),
        e("btn-rc", SectionId::Controls, t.ix_btn_rc, t.ixk_btn_rc),
        e("remap", SectionId::Controls, t.ix_remap, t.ixk_remap),
        e("deadzone-left", SectionId::Controls, t.ix_deadzone_left, t.ixk_deadzone_left),
        e("deadzone-right", SectionId::Controls, t.ix_deadzone_right, t.ixk_deadzone_right),
        e("invert-left", SectionId::Controls, t.ix_invert_left, t.ixk_invert_left),
        e("invert-right", SectionId::Controls, t.ix_invert_right, t.ixk_invert_right),
        e("gyro-on", SectionId::Controls, t.ix_gyro_on, t.ixk_gyro_on),
        e("gyro-sens", SectionId::Controls, t.ix_gyro_sens, t.ixk_gyro_sens),
        e("rumble-strength", SectionId::Controls, t.ix_rumble_strength, t.ixk_rumble_strength),
        e("rumble", SectionId::Controls, t.ix_rumble, t.ixk_rumble),
        e("gyro", SectionId::Controls, t.ix_gyro, t.ixk_gyro),
        e("lights-mode", SectionId::Lighting, tr().lights.ix_lights, tr().lights.ixk_lights),
        e("lights-color", SectionId::Lighting, tr().lights.ix_light_color, tr().lights.ixk_light_color),
        e("lights-effect", SectionId::Lighting, tr().lights.ix_light_effect, tr().lights.ixk_light_effect),
        e("profile", SectionId::Power, t.ix_profile, t.ixk_profile),
        e("tdp", SectionId::Power, t.ix_tdp, t.ixk_tdp),
        e("cpu-boost", SectionId::Power, t.ix_cpu_boost, t.ixk_cpu_boost),
        e("fan", SectionId::Power, t.ix_fan, t.ixk_fan),
        e("sleep-idle", SectionId::Power, t.ix_sleep_idle, t.ixk_sleep_idle),
        e("battery-care", SectionId::Power, t.ix_battery_care, t.ixk_battery_care),
        e("battery-health", SectionId::Power, t.ix_battery_health, t.ixk_battery_health),
        e("wifi", SectionId::Network, t.ix_wifi, t.ixk_wifi),
        e("wifi-forget", SectionId::Network, t.ix_wifi_forget, t.ixk_wifi_forget),
        e("bluetooth", SectionId::Network, t.ix_bluetooth, t.ixk_bluetooth),
        e("username", SectionId::Accounts, t.ix_username, t.ixk_username),
        e("rare-account", SectionId::Accounts, t.ix_rare_account, t.ixk_rare_account),
        e("setup", SectionId::Accounts, t.ix_setup, t.ixk_setup),
        e("wallet", SectionId::Accounts, t.ix_wallet, t.ixk_wallet),
        e("parental-pin", SectionId::Parental, t.ix_parental_pin, t.ixk_parental_pin),
        e("parental-daily", SectionId::Parental, t.ix_parental_daily, t.ixk_parental_daily),
        e("parental-hours", SectionId::Parental, t.ix_parental_hours, t.ixk_parental_hours),
        e("parental-age", SectionId::Parental, t.ix_parental_age, t.ixk_parental_age),
        e("scan", SectionId::Storage, t.ix_scan, t.ixk_scan),
        e("games-folder", SectionId::Storage, t.ix_games_folder, t.ixk_games_folder),
        e("steam-start", SectionId::Storage, t.ix_steam_start, t.ixk_steam_start),
        e("steamgriddb", SectionId::Storage, t.ix_steamgriddb, t.ixk_steamgriddb),
        e("share", SectionId::Storage, t.ix_share, t.ixk_share),
        e("replay-toggle", SectionId::Capture, t.ix_replay_toggle, t.ixk_replay_toggle),
        e("replay-length", SectionId::Capture, t.ix_replay_length, t.ixk_replay_length),
        e("replay-cap", SectionId::Capture, t.ix_replay_cap, t.ixk_replay_cap),
        e("discord-presence", SectionId::Capture, t.ix_discord_presence, t.ixk_discord_presence),
        e("discord-mute", SectionId::Capture, t.ix_discord_mute, t.ixk_discord_mute),
        e("host-toggle", SectionId::Capture, t.ix_host_toggle, t.ixk_host_toggle),
        e("nearby-scan", SectionId::Capture, t.ix_nearby_scan, t.ixk_nearby_scan),
        e("notif-dnd", SectionId::Notifications, t.ix_notif_dnd, t.ixk_notif_dnd),
        e("notif-toasts", SectionId::Notifications, t.ix_notif_toasts, t.ixk_notif_toasts),
        e("notif-desktop", SectionId::Notifications, t.ix_notif_desktop, t.ixk_notif_desktop),
        e("notif-alerts", SectionId::Notifications, t.ix_notif_alerts, t.ixk_notif_alerts),
        e("dev-ssh", SectionId::Developer, t.ix_dev_ssh, t.ixk_dev_ssh),
        e("ask-provider", SectionId::Ask, t.ix_ask_provider, t.ixk_ask_provider),
        e("ask-voice", SectionId::Ask, t.ix_ask_voice, t.ixk_ask_voice),
        e("ask-hold", SectionId::Ask, t.ix_ask_hold, t.ixk_ask_hold),
        e("ask-shots", SectionId::Ask, t.ix_ask_shots, t.ixk_ask_shots),
        e("dev-netapi", SectionId::Developer, t.ix_dev_netapi, t.ixk_dev_netapi),
        e("dev-netapi-caps", SectionId::Developer, t.ix_dev_netapi_caps, t.ixk_dev_netapi_caps),
        e("dev-netapi-activity", SectionId::Developer, t.ix_dev_netapi_activity, t.ixk_dev_netapi_activity),
        e("dev-network", SectionId::Developer, t.ix_dev_network, t.ixk_dev_network),
        e("dev-testnet-rpc", SectionId::Developer, t.ix_dev_testnet_rpc, t.ixk_dev_testnet_rpc),
        e("dev-testnet-api", SectionId::Developer, t.ix_dev_testnet_api, t.ixk_dev_testnet_api),
        e("dev-testnet-sponsor", SectionId::Developer, t.ix_dev_testnet_sponsor, t.ixk_dev_testnet_sponsor),
        e("dev-game", SectionId::Developer, t.ix_dev_game, t.ixk_dev_game),
        e("emulators", SectionId::Emulators, t.ix_emulators, t.ixk_emulators),
        e("about", SectionId::About, t.ix_about, t.ixk_about),
        e("licences", SectionId::Licences, t.ix_licences, t.ixk_licences),
    ];
    // The notification kinds sit between "notif-desktop" and "notif-alerts".
    let at = v.iter().position(|x| x.id == "notif-alerts").unwrap_or(v.len());
    let kinds: Vec<SettingEntry> = KINDS
        .iter()
        .map(|kind| {
            let (title, body) = kind_text(kind);
            let mut kw: Vec<String> = t.ixk_notif.split_whitespace().map(String::from).collect();
            kw.extend(words_over3(body));
            SettingEntry { id: format!("notif-{kind}"), section: SectionId::Notifications, title: title.into(), keywords: kw }
        })
        .collect();
    v.splice(at..at, kinds);
    // Controls rows InputPlumber can't apply are not shown (controls.rs), so search skips them.
    let off: Vec<&str> = rare_input::ip::controls_support()
        .iter()
        .filter(|c| !c.applies)
        .filter_map(|c| match c.key {
            "deadzoneLeft" => Some("deadzone-left"),
            "deadzoneRight" => Some("deadzone-right"),
            "invertLeftY" => Some("invert-left"),
            "invertRightY" => Some("invert-right"),
            "gyroSensitivity" => Some("gyro-sens"),
            _ => None,
        })
        .collect();
    v.retain(|x| !off.contains(&x.id.as_str()));
    v
}
