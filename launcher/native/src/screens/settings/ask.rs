//! Settings > Ask Rare: the model the owner brings (Claude, OpenAI, any OpenAI-compatible service,
//! Ollama), its model name and address, the API key (sealed in Rare Wallet's vault, never shown or
//! written to launcher.json), push to talk, RC's hold, and the default for pictures of the game.

use super::config::Config;
use super::rows::*;
use super::{SectionOut, SectionView};
use crate::screens::overlay::common::*;
use rare_agent::provider::Kind;
use rare_i18n::{tr, trf};
use rare_render::Id;

pub const PROVIDERS: [Option<Kind>; 5] = [None, Some(Kind::Claude), Some(Kind::OpenAi), Some(Kind::Compatible), Some(Kind::Ollama)];

fn provider_label(k: Option<Kind>) -> &'static str {
    match k {
        None => tr().common.off,
        Some(Kind::Claude) => "Claude",
        Some(Kind::OpenAi) => "OpenAI",
        Some(Kind::Compatible) => tr().settings.provider_other,
        Some(Kind::Ollama) => "Ollama",
    }
}

/// An address the owner typed: http(s) only, no trailing slash. Some(None) clears it.
pub fn clean_url(v: &str) -> Option<Option<String>> {
    let v = v.trim();
    if v.is_empty() {
        return Some(None);
    }
    let v = if !v.contains("://") { format!("http://{v}") } else { v.to_string() };
    (v.starts_with("http://") || v.starts_with("https://")).then(|| Some(v.trim_end_matches('/').to_string()))
}

pub struct AskSection {
    rows: RowSet,
}

fn kind_of(cfg: &Config) -> Option<Kind> {
    cfg.agent_provider.as_deref().and_then(Kind::from_id)
}

impl AskSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, _now: f64) -> AskSection {
        let mut rows = RowSet::default();
        let kind = kind_of(cfg);
        let p = panel(cx, parent);
        let t = &tr().settings;
        setting_row(cx, p, t.ask_title, Some(t.ask_intro), true);
        let idx = PROVIDERS.iter().position(|k| *k == kind).unwrap_or(0);
        let labels: Vec<&str> = PROVIDERS.iter().map(|k| provider_label(*k)).collect();
        let body = match kind {
            None => t.ask_pick,
            Some(Kind::Claude) => t.ask_claude,
            Some(Kind::OpenAi) => t.ask_openai,
            Some(Kind::Compatible) => t.ask_compat,
            Some(Kind::Ollama) => t.ask_ollama,
        };
        rows.choice(cx, nav, p, "ask-provider", t.row_provider, Some(body), &labels, Some(idx), false);
        if let Some(k) = kind {
            let model = cfg.agent_model.clone().filter(|m| !m.is_empty()).unwrap_or_else(|| if k.default_model().is_empty() { t.not_set.into() } else { trf!(t.default_value, value = k.default_model()) });
            let r = setting_row(cx, p, t.row_model, Some(&model), false);
            super::remote::small_button(cx, nav, r.right, tr().common.change, "ask-model", 0);
            if k.takes_url() {
                let url = cfg.agent_url.clone().filter(|u| !u.is_empty()).unwrap_or_else(|| if k.default_url().is_empty() { t.not_set.into() } else { trf!(t.default_value, value = k.default_url()) });
                let r = setting_row(cx, p, t.row_address, Some(&url), false);
                super::remote::small_button(cx, nav, r.right, tr().common.change, "ask-url", 0);
            }
            if k.takes_key() {
                let saved = crate::features::agent::setup().keys.iter().any(|x| x == k.id());
                let line = match (saved, k.needs_key()) {
                    (true, _) => t.key_saved,
                    (false, true) => t.not_set,
                    (false, false) => t.key_optional,
                };
                let r = setting_row(cx, p, t.row_api_key, Some(line), false);
                super::remote::small_button(cx, nav, r.right, if saved { t.replace } else { tr().common.add }, "ask-key", 0);
                if saved {
                    super::remote::small_button(cx, nav, r.right, tr().common.remove, "ask-key-remove", 0);
                }
            }
            let r = setting_row(cx, p, t.row_check, Some(t.check_body), false);
            super::remote::small_button(cx, nav, r.right, t.test, "ask-test", 0);
        }
        let p = panel(cx, parent);
        rows.switch(cx, nav, p, "ask-voice", t.voice_title, t.voice_body, cfg.agent_voice == Some(true), true);
        rows.switch(cx, nav, p, "ask-hold", t.hold_title, t.hold_body, cfg.agent_hold == Some(true), false);
        let p = panel(cx, parent);
        rows.switch(cx, nav, p, "ask-shots", t.shots_title, t.shots_body, cfg.agent_shots == Some(true), true);
        setting_row(cx, p, t.agents_title, Some(t.agents_body), false);
        AskSection { rows }
    }
}

impl SectionView for AskSection {
    fn press(&mut self, cx: &mut Cx, nav: &mut Nav, cfg: &mut Config, now: f64) -> SectionOut {
        let Some(f) = nav.current().cloned() else { return SectionOut::None };
        let act = |a: &str| SectionOut::Act(a.into());
        match f.sid.as_str() {
            "ask-provider" => {
                let Some((_, Val::Idx(i))) = self.rows.press(cx, nav, now) else { return SectionOut::None };
                let k = PROVIDERS.get(i).copied().flatten();
                cfg.agent_provider = k.map(|k| k.id().to_string());
                // Another provider: its own model and address defaults.
                cfg.agent_model = None;
                cfg.agent_url = None;
                // The rows under it change: draw the section again after the save.
                crate::screens::settings::live::touch();
                SectionOut::Changed
            }
            "ask-voice" | "ask-hold" | "ask-shots" => {
                let Some((sid, Val::Bool(on))) = self.rows.press(cx, nav, now) else { return SectionOut::None };
                match sid.as_str() {
                    "ask-voice" => cfg.agent_voice = Some(on),
                    "ask-hold" => cfg.agent_hold = Some(on),
                    _ => cfg.agent_shots = Some(on),
                }
                if sid == "ask-voice" && on {
                    // Fetch the speech model now, not on the first hold.
                    crate::features::agent::download_model();
                }
                SectionOut::Changed
            }
            "ask-model" => act("ask:model"),
            "ask-url" => act("ask:url"),
            "ask-key" => act("ask:key"),
            "ask-key-remove" => act("ask:key-remove"),
            "ask-test" => act("ask:test"),
            _ => SectionOut::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls() {
        assert_eq!(clean_url("pc.local:11434"), Some(Some("http://pc.local:11434".into())));
        assert_eq!(clean_url("https://api.example.com/v1/"), Some(Some("https://api.example.com/v1".into())));
        assert_eq!(clean_url(""), Some(None));
        assert_eq!(clean_url("ftp://x"), None);
    }
}
