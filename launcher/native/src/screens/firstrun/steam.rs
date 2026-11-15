//! First run: Steam (launcher/src/onboarding/steps/Steam.tsx). Steam opens in its big-screen mode
//! to sign in; coming back (the Rare button) looks again.

use rare_i18n::{tr, trf, trn};
use super::kit::{done_line, step};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::ui::{button_el, BSize, ButtonSpec, BVariant};

/// Steam Guard is Steam's own check, not Rare's: where its code comes from, and the keyboard for it.
pub fn guard() -> &'static str {
    tr().firstrun.steam_guard
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Continue,
    Open,
    Back,
}

/// `account`: the signed-in Steam account (None not signed in); `steam_games` in the library.
pub fn view(account: Option<&str>, opened: bool, online: bool, steam_games: usize) -> El<Msg> {
    let (title, body) = if account.is_some() {
        (tr().firstrun.steam_ready_title, tr().firstrun.steam_ready_body)
    } else {
        (tr().firstrun.steam_sign_in_title, tr().firstrun.steam_sign_in_body)
    };
    let actions = if account.is_some() {
        vec![
            button_el(ButtonSpec::new(tr().common.continue_).v(BVariant::Primary).size(BSize::Lg).def(true), Some(Msg::Continue)),
            button_el(ButtonSpec::new(tr().firstrun.open_steam).v(BVariant::Ghost).size(BSize::Lg).icon("steam", ""), Some(Msg::Open)),
        ]
    } else {
        vec![
            button_el(ButtonSpec::new(if opened { tr().firstrun.open_steam_again } else { tr().firstrun.open_steam }).v(BVariant::Primary).size(BSize::Lg).def(true).icon("steam", ""), Some(Msg::Open)),
            button_el(ButtonSpec::new(tr().firstrun.skip_steam).v(BVariant::Ghost).size(BSize::Lg), Some(Msg::Continue)),
        ]
    };
    let mut c = div("flex flex-col gap-[0.9rem]");
    if let Some(a) = account {
        let line = if steam_games > 0 { trn!(tr().firstrun.steam_signed_in_games, steam_games, name = a) } else { trf!(tr().firstrun.signed_in_as, name = a) };
        c = c.kid(done_line(vec![span("").text(line)]));
    } else if opened {
        c = c.kid(div("text-body text-muted").text(tr().firstrun.steam_waiting));
    }
    if account.is_none() && !online {
        c = c.kid(div("text-small text-muted").text(tr().firstrun.steam_offline));
    }
    let mut tips = ul("flex max-w-[40rem] flex-col gap-[0.4rem] text-small leading-relaxed text-muted");
    if account.is_none() {
        tips = tips.kid(li("").text(guard()));
    }
    c = c.kid(
        tips.kid(li("").text(tr().firstrun.steam_tip_installed))
            .kid(li("").text(tr().firstrun.steam_tip_heroic)),
    );
    step(text(title), Some(text(body)), vec![c], actions, false).on_pad("b", Msg::Back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam() {
        let v = view(None, false, true, 0);
        assert_eq!(v.texts()[0], "Sign in to Steam");
        assert_eq!(v.landing().as_deref(), Some("Open Steam"));
        assert!(view(None, true, false, 0).contains_text("Waiting for you to sign in to Steam."));
        assert!(view(None, false, true, 0).contains_text(guard()));
        assert!(!view(Some("playerone"), true, true, 1).contains_text(guard()));
        assert!(view(None, true, false, 0).contains_text("You are offline, so Steam cannot sign in yet. Skip this and sign in to Steam later from Library."));
        let v = view(Some("playerone"), true, true, 42);
        assert_eq!(v.texts()[..3], ["Steam is ready", "Your Steam games show up in Library and on Home, and they start straight from Rare.", "Signed in as playerone, 42 games found"]);
        assert_eq!(v.landing().as_deref(), Some("Continue"));
    }
}
