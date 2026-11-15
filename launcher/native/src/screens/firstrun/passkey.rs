//! First run: sign in (launcher/src/wallet/ui/onboard/firstrun.tsx, the passkey step Rare Wallet
//! registers): the start page with the three ways, the passkey flow, "Other ways" as the wallet's
//! own pages inside setup, and "You're @name" with a free username suggestion.

use rare_i18n::{tr, trf};
use crate::screens::wallet::kit::account_avatar;
use crate::screens::wallet::markup::*;
use crate::screens::wallet::onboard::passkey_flow::Step as FlowStep;
use crate::screens::wallet::ui::{button_el, panel, BSize, ButtonSpec, BVariant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Start,
    Signin,
    Create,
    Other,
    Named,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Mode(Mode),
    /// "Create account": a recovery-phrase account on this device (the wallet's Create page)
    Phrase,
    /// open the wallet's Add page inside setup
    Other,
    Next,
    /// "Connect to Wi-Fi" when offline: back a step
    Back,
    /// "You're @name": save it with one signature, then next
    Continue,
    /// pick another username (friends' picker)
    Change,
}

fn copy(title: &str, body: Option<&str>, kids: Vec<El<Msg>>) -> El<Msg> {
    div("flex flex-col gap-[1rem]")
        .kid(h1("text-[2.5rem] font-semibold leading-[1.1] tracking-[-0.03em]").text(title))
        .kid_opt(body.map(|b| p("text-row font-normal leading-relaxed text-muted").text(b)))
        .kids(kids)
}

fn way(ic: &'static str, title: &str, body: &str) -> El<Msg> {
    div("flex items-center gap-[1rem] border-t border-hair px-[1.4rem] py-[1rem] first:border-t-0")
        .key(title)
        .kid(span("flex h-[2.75rem] w-[2.75rem] shrink-0 items-center justify-center rounded-full bg-l3 text-fg").kid(icon(ic, "").attr("size", "1.25rem").attr("stroke-width", "1.75")))
        .kid(span("").kid(span("block text-body font-semibold").text(title)).kid(span("block text-small text-muted").text(body)))
}

/// The step. `flow` is the passkey flow's own markup (signin / create), `wallet` the wallet's page
/// stack for Other ways; `flow_step` is where the flow is (for the body line).
pub fn view(mode: Mode, flow_step: FlowStep, online: bool, flow: Option<El<Msg>>, wallet: Option<El<Msg>>) -> El<Msg> {
    match mode {
        Mode::Other => return div("mx-auto h-full w-full max-w-[56rem]").kid_opt(wallet),
        Mode::Named => return div("h-full"),
        _ => {}
    }
    let create = mode == Mode::Create;
    let title = if mode == Mode::Start {
        tr().firstrun.pk_create_your_account
    } else if create {
        tr().firstrun.pk_create_account
    } else {
        tr().firstrun.pk_sign_in_passkey
    };
    let body = if mode == Mode::Start {
        Some(tr().firstrun.pk_start_body)
    } else if flow_step == FlowStep::Pin {
        Some(tr().firstrun.pk_pin_body)
    } else if flow_step == FlowStep::Run {
        None
    } else if create {
        Some(tr().firstrun.pk_create_body)
    } else {
        Some(tr().firstrun.pk_signin_body)
    };
    let mut kids = Vec::new();
    if mode == Mode::Start && !online {
        kids.push(
            div("flex max-w-[24rem] flex-col items-start gap-[0.6rem] text-small text-muted")
                .kid(text(tr().firstrun.pk_no_internet))
                .kid(button_el(ButtonSpec::new(tr().firstrun.connect_wifi).size(BSize::Lg).v(BVariant::Secondary), Some(Msg::Back))),
        );
    }
    if mode == Mode::Start {
        kids.push(
            div("mt-[0.75rem] flex flex-col items-start gap-[0.75rem]")
                .attr("data-nav-group", "firstrun-actions")
                .attr("data-nav-remember", "")
                .kid(
                    div("flex w-full max-w-[22rem] flex-col gap-[0.75rem]")
                        .kid(button_el(ButtonSpec::new(tr().firstrun.pk_create_account).v(BVariant::Primary).size(BSize::Lg).def(true).class("w-full"), Some(Msg::Phrase)))
                        .kid(button_el(ButtonSpec::new(tr().firstrun.pk_sign_in_passkey).size(BSize::Lg).icon("key-round", "").class("w-full"), Some(Msg::Mode(Mode::Signin)))),
                )
                .kid(
                    div("-ml-[1.75rem] flex flex-wrap gap-[0.25rem]")
                        .kid(button_el(ButtonSpec::new(tr().firstrun.pk_other_ways).size(BSize::Lg).v(BVariant::Ghost), Some(Msg::Other)))
                        .kid(button_el(ButtonSpec::new(tr().firstrun.not_now).size(BSize::Lg).v(BVariant::Ghost), Some(Msg::Next))),
                ),
        );
    }
    let right = if mode == Mode::Start {
        panel(
            "max-w-[30rem]",
            vec![
                way("gamepad-2", tr().firstrun.pk_way_device, tr().firstrun.pk_way_device_body),
                way("smartphone", tr().firstrun.pk_way_phone, tr().firstrun.pk_way_phone_body),
                way("usb", tr().firstrun.pk_way_key, tr().firstrun.pk_way_key_body),
            ],
        )
    } else {
        div("max-w-[34rem]").key(if create { "create" } else { "signin" }).kid_opt(flow)
    };
    div("grid h-full grid-cols-[minmax(0,26rem)_minmax(0,1fr)] items-center gap-[4rem]").kid(copy(title, body, kids)).kid(div("min-w-0").kid(right))
}

/// "You're @name" (`Named`): `name` is the saved username or a free suggestion.
pub fn named_view(name: Option<&str>, address: Option<&str>, busy: bool) -> El<Msg> {
    let mut actions = div("mt-[0.75rem] flex flex-wrap gap-[0.75rem]")
        .attr("data-nav-group", "firstrun-actions")
        .attr("data-nav-remember", "")
        .kid(button_el(ButtonSpec::new(tr().common.continue_).v(BVariant::Primary).size(BSize::Lg).def(true).disabled(busy).class("min-w-[10rem]"), Some(Msg::Continue)));
    if address.is_some() {
        actions = actions.kid(button_el(ButtonSpec::new(tr().common.change).size(BSize::Lg).v(BVariant::Ghost).disabled(busy), Some(Msg::Change)));
    }
    let title = name.map(|n| trf!(tr().firstrun.pk_youre_name, name = n)).unwrap_or_else(|| tr().firstrun.pk_youre_signed_in.into());
    let mut right = div("min-w-0");
    if let Some(a) = address {
        right = right.kid(panel(
            "flex max-w-[30rem] items-center gap-[1.25rem] px-[1.6rem] py-[1.5rem]",
            vec![
                account_avatar(a, "4.5rem"),
                span("min-w-0").kid(span("block text-label text-muted").text(tr().firstrun.pk_how_friends_see)).kid(span("block truncate text-title font-semibold").text(name.map(|n| format!("@{n}")).unwrap_or_else(|| tr().firstrun.pk_your_username.into()))),
            ],
        ));
    }
    div("grid h-full grid-cols-[minmax(0,26rem)_minmax(0,1fr)] items-center gap-[4rem]")
        .kid(copy(&title, Some(tr().firstrun.pk_username_body), vec![actions]))
        .kid(right)
        // signed in already: nothing to go back to
        .on_pad("b", Msg::Mode(Mode::Named))
}

/// `suggest`: player + four hex digits of the address, the first of four tries that is free.
pub fn suggestions(address: &str) -> Vec<String> {
    let hex = address.trim_start_matches("0x").to_lowercase();
    (0..4).filter_map(|k| hex.get(k * 4..k * 4 + 4).map(|s| format!("player{s}"))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::wallet::home::fixtures::ME;

    #[test]
    fn start() {
        let v = view(Mode::Start, FlowStep::Choose, true, None, None);
        let t = v.texts();
        assert_eq!(&t[..2], ["Create your account", "One account for Rare Wallet, Friends and Rare Marketplace. It is kept on this SuiPlay and signs with A. You write down 12 words once as a backup."]);
        assert!(t.contains(&"Signs with A. Back up 12 words once".to_string()));
        assert_eq!(v.landing().as_deref(), Some("Create account"));
        let v = view(Mode::Start, FlowStep::Choose, false, None, None);
        assert!(v.contains_text("No internet. You can create your account now; Friends and the Marketplace need it later."));
        let v = view(Mode::Create, FlowStep::Pin, true, Some(div("").text("flow")), None);
        assert_eq!(v.texts()[..3], ["Create account", "This device signs everyday transactions with the PIN. Your passkey can always sign too.", "flow"]);
    }

    #[test]
    fn named() {
        let v = named_view(Some("player7d20"), Some(ME), false);
        assert_eq!(v.texts(), ["You're @player7d20", "Friends find you by your username. It is free, and you can change it from your profile.", "Continue", "Change", "How friends see you", "@player7d20"]);
        assert_eq!(suggestions(ME), ["player7d20", "playerdcdb", "player2bca", "player4f50"]);
    }
}
