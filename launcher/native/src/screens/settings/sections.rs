//! The sections of Settings, in the order of the left list (screens/settings/sections.ts).

use rare_i18n::tr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SectionId {
    System,
    Display,
    Sound,
    Controls,
    Lighting,
    Power,
    Network,
    Accounts,
    Devices,
    Parental,
    Storage,
    Capture,
    About,
    Emulators,
    Notifications,
    Licences,
    Developer,
    Ask,
}

pub const SECTIONS: [SectionId; 18] = [
    SectionId::System,
    SectionId::Display,
    SectionId::Sound,
    SectionId::Controls,
    SectionId::Lighting,
    SectionId::Power,
    SectionId::Network,
    SectionId::Accounts,
    SectionId::Devices,
    SectionId::Parental,
    SectionId::Storage,
    SectionId::Capture,
    SectionId::Notifications,
    SectionId::Ask,
    SectionId::Emulators,
    SectionId::Developer,
    SectionId::About,
    SectionId::Licences,
];

impl SectionId {
    pub fn label(self) -> &'static str {
        let t = &tr().settings;
        match self {
            SectionId::System => t.sec_system,
            SectionId::Display => t.sec_display,
            SectionId::Sound => t.sec_sound,
            SectionId::Controls => t.sec_controls,
            SectionId::Lighting => tr().lights.section,
            SectionId::Power => t.sec_power,
            SectionId::Network => t.sec_network,
            SectionId::Accounts => t.sec_accounts,
            SectionId::Devices => t.sec_devices,
            SectionId::Parental => t.sec_parental,
            SectionId::Storage => t.sec_storage,
            SectionId::Capture => t.sec_capture,
            SectionId::About => t.sec_about,
            SectionId::Emulators => t.sec_emulators,
            SectionId::Notifications => t.sec_notifications,
            SectionId::Licences => t.sec_licences,
            SectionId::Developer => t.sec_developer,
            SectionId::Ask => t.sec_ask,
        }
    }

    /// The id the web launcher uses in `data-section` and the jump store.
    pub fn key(self) -> &'static str {
        match self {
            SectionId::System => "system",
            SectionId::Display => "display",
            SectionId::Sound => "sound",
            SectionId::Controls => "controls",
            SectionId::Lighting => "lighting",
            SectionId::Power => "power",
            SectionId::Network => "network",
            SectionId::Accounts => "accounts",
            SectionId::Devices => "devices",
            SectionId::Parental => "parental",
            SectionId::Storage => "storage",
            SectionId::Capture => "capture",
            SectionId::About => "about",
            SectionId::Emulators => "emulators",
            SectionId::Notifications => "notifications",
            SectionId::Licences => "licences",
            SectionId::Developer => "developer",
            SectionId::Ask => "ask",
        }
    }

    pub fn from_key(k: &str) -> Option<SectionId> {
        SECTIONS.iter().copied().find(|id| id.key() == k)
    }
}

/// LT / RT: the section `step` places away (wraps).
pub fn step(from: SectionId, forward: bool) -> SectionId {
    let i = SECTIONS.iter().position(|id| *id == from).unwrap_or(0);
    let n = SECTIONS.len();
    SECTIONS[(i + if forward { 1 } else { n - 1 }) % n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lt_rt_wrap_around_the_list() {
        assert_eq!(step(SectionId::System, false), SectionId::Licences);
        assert_eq!(step(SectionId::Licences, true), SectionId::System);
        assert_eq!(step(SectionId::Display, true), SectionId::Sound);
    }

    #[test]
    fn keys_round_trip() {
        for id in SECTIONS {
            assert_eq!(SectionId::from_key(id.key()), Some(id));
        }
    }
}
