//! Sign in with a passkey, or create an account (launcher/src/wallet/ui/pages/Passkey.tsx), and
//! AddPasskey.tsx: the one answer to a wallet that cannot sign (watch only or no account), a line
//! and a button that opens the passkey flow in a sheet over whatever is on screen.

use rare_i18n::tr;
use super::flow::h;
use super::passkey_flow::{self, Step};
use crate::screens::wallet::home::lib::Hint;
use crate::screens::wallet::kit::{btn, page, sheet, SheetSpec, Variant};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::ui::{button_el, BSize, BVariant, ButtonSpec};

/// The Passkey page around the flow.
pub fn page_view(pr: &passkey_flow::Props, st: &passkey_flow::State) -> El<passkey_flow::Msg> {
    let create = pr.create;
    let title = match st.step {
        Step::Pin | Step::Device => None,
        _ => Some(if create { tr().wallet_setup.create_account_short } else { tr().wallet_setup.sign_in_passkey }),
    };
    let sub = (st.step == Step::Choose).then(|| {
        text(if create {
            tr().wallet_setup.create_sub_long
        } else {
            tr().wallet_setup.signin_sub_long
        })
    });
    page(title, sub, None, true, "", vec![passkey_flow::view(pr, st)])
}

pub fn page_hints(st: &passkey_flow::State) -> Vec<Hint> {
    if st.step == Step::Pin {
        vec![h::select(), h::del(), h::ok(), h::back()]
    } else {
        vec![h::select(), h::back()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    Sell,
    Buy,
    Trade,
    Items,
}

fn line(n: Need, signed_out: bool) -> &'static str {
    let t = tr().wallet_setup;
    match (n, signed_out) {
        (Need::Sell, false) => t.need_sell,
        (Need::Buy, false) => t.need_buy,
        (Need::Trade, false) => t.need_trade,
        (Need::Sell, true) => t.need_sell_signed_out,
        (Need::Buy, true) => t.need_buy_signed_out,
        (Need::Trade, true) => t.need_trade_signed_out,
        (Need::Items, _) => t.need_items,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Open,
    Close,
    /// the sheet's footer: switch between signing in and creating
    Switch,
    Flow(passkey_flow::Msg),
}

/// `AddPasskey`: `signed_out` is no account at all (else a watch-only one).
pub fn add_passkey_view(need: Need, signed_out: bool, class: &str) -> El<Msg> {
    let l = line(need, signed_out);
    let label = if signed_out { tr().common.sign_in } else { tr().wallet_setup.add_passkey };
    div(cn(&["flex flex-col items-start gap-[0.75rem]", class]))
        .kid(p("text-small text-secondary").text(l))
        .kid(button_el(ButtonSpec::new(label).v(BVariant::Primary).size(BSize::Md).hint(label), Some(Msg::Open)))
}

/// `AddPasskeySheet`: the flow with "Create account" / "Use a passkey I already have" in the footer.
pub fn add_passkey_sheet(pr: &passkey_flow::Props, st: &passkey_flow::State) -> El<Msg> {
    let create = pr.create;
    sheet(
        SheetSpec {
            title: if create { tr().wallet_setup.create_account_short } else { tr().wallet_setup.add_passkey }.into(),
            subtitle: Some(if create { tr().wallet_setup.create_sub } else { tr().wallet_setup.signin_sub }.into()),
            icon: None,
            footer: Some(vec![btn(if create { tr().wallet_setup.use_existing_passkey } else { tr().wallet_setup.create_account_short }, Variant::Ghost, "", Some(Msg::Switch))]),
            close: Some(Msg::Close),
            width: "40rem",
        },
        vec![passkey_flow::view(pr, st).map(&Msg::Flow).key(if create { "create" } else { "signin" })],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::wallet::onboard::ceremony::{Ceremony, Via};

    #[test]
    fn passkey_page() {
        let pr = passkey_flow::Props { create: true, via: Via::Phone, ceremony: &Ceremony::Idle, qr_url: None, can_exit: false };
        let st = passkey_flow::State::default();
        let v = page_view(&pr, &st);
        assert_eq!(v.texts()[..3], ["Create account", "Make a new wallet protected by a passkey. There are no words to write down.", "Use your phone"]);
        let pr = passkey_flow::Props { create: false, ..pr };
        assert_eq!(page_view(&pr, &st).texts()[..2], ["Sign in with a passkey", "Use the passkey you made before to get your wallet back."]);
    }

    #[test]
    fn add_passkey() {
        let v = add_passkey_view(Need::Sell, true, "");
        assert_eq!(v.texts(), ["Sign in with a passkey to sell.", "Sign in"]);
        let v = add_passkey_view(Need::Sell, false, "");
        assert_eq!(v.texts(), ["Add a passkey to sell.", "Add a passkey"]);
        let v = add_passkey_view(Need::Items, true, "");
        assert_eq!(v.texts(), ["Sign in with a passkey to see your items.", "Sign in"]);
        let pr = passkey_flow::Props { create: false, via: Via::Usb, ceremony: &Ceremony::Idle, qr_url: None, can_exit: true };
        let s = add_passkey_sheet(&pr, &passkey_flow::State::default());
        assert_eq!(s.texts()[..2], ["Add a passkey", "Use the passkey you made before."]);
        assert!(s.contains_text("Create account"));
    }
}
