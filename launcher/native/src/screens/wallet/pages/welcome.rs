//! Rare Wallet with no accounts yet (launcher/src/wallet/ui/pages/Welcome.tsx): "Create account"
//! makes a recovery-phrase account kept on this device (it signs with A, no PIN); a passkey is the
//! second way in; everything else sits under "Other ways to sign in". Also the Add account page
//! (AddAccount.tsx): the same ways, plus a new account from a recovery phrase this device holds.

use crate::screens::wallet::home::lib::Hint;
use crate::screens::wallet::home::stack::{PasskeyMode, WalletView};
use crate::screens::wallet::kit::{btn, btn_full, list_button, page, panel, Size, Variant};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::onboard::flow::h;
use crate::screens::wallet::onboard::other_ways::other_ways;
use rare_i18n::tr;

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Push(WalletView),
    /// "Add an account from your recovery phrase": unlock, derive the next account, finish
    Derive,
}

pub fn welcome_view() -> El<Msg> {
    page(
        Some(tr().wallet.rare_wallet),
        None,
        None,
        true,
        "",
        vec![
            div("flex flex-col gap-3")
                .kid(btn(tr().wallet.create_account, Variant::Primary, "w-full", Some(Msg::Push(WalletView::CreatePhrase))).def(true))
                .kid(btn(tr().wallet.sign_in_passkey, Variant::Secondary, "w-full", Some(Msg::Push(WalletView::Passkey { mode: PasskeyMode::Signin }))))
                .kid(p("text-center text-sm text-secondary").text(create_line())),
            div("mt-6").kid(passkey_create_way()),
            h2("mb-2 mt-9 text-base font-semibold").text(tr().wallet.other_ways_sign_in),
            other_ways(Msg::Push, false),
            div("mt-6 flex justify-center").kid(btn_full(tr().wallet.move_assets, Variant::Ghost, Size::Md, false, "", Some(Msg::Push(WalletView::Playtron)))),
        ],
    )
}

/// What "Create account" makes, in one line.
pub fn create_line() -> &'static str {
    tr().wallet.create_line
}

/// A PIN account: signs here with a PIN, a phone or a security key is the backup.
pub fn pin_way() -> &'static str {
    tr().wallet.pin_way
}
/// A PIN account from another SuiPlay, back on this one.
pub fn restore_way() -> &'static str {
    tr().wallet.restore_way
}

/// The PIN account and the passkey-only account, for people who want one.
fn passkey_create_way() -> El<Msg> {
    panel(
        None,
        None,
        "p-1",
        vec![
            list_button(Some(icon("lock", "").attr("size", "1.4rem")), tr().wallet.create_pin_account, Some(text(pin_way())), None, Msg::Push(WalletView::PinAccount), false, ""),
            list_button(Some(icon("rotate-ccw", "").attr("size", "1.4rem")), tr().wallet.restore_pin_account, Some(text(restore_way())), None, Msg::Push(WalletView::Import { what: crate::screens::wallet::home::stack::ImportWhat::Restore }), false, ""),
            list_button(Some(icon("smartphone", "").attr("size", "1.4rem")), tr().wallet.create_passkey_account, Some(text(tr().wallet.passkey_way)), None, Msg::Push(WalletView::Passkey { mode: PasskeyMode::Create }), false, ""),
        ],
    )
}

pub fn welcome_hints() -> Vec<Hint> {
    vec![h::select()]
}

/// Add account. `none`: there are no accounts yet (the page is "Sign in").
pub struct AddProps {
    pub none: bool,
    /// some account on this device came from a recovery phrase it still holds
    pub has_phrase: bool,
    pub busy: bool,
    pub error: Option<String>,
}

pub fn add_view(p_: &AddProps) -> El<Msg> {
    let mut ways = div("flex flex-col gap-3");
    if p_.has_phrase {
        ways = ways.kid(panel(
            None,
            None,
            "p-1",
            vec![list_button(
                Some(icon("file-text", "").attr("size", "1.4rem")),
                tr().wallet.add_from_phrase,
                Some(text(tr().wallet.add_from_phrase_body)),
                p_.busy.then(|| text(tr().wallet.adding)),
                Msg::Derive,
                false,
                "",
            )],
        ));
    }
    ways = ways.kid(other_ways(Msg::Push, false));
    let mut body = vec![
        div("flex flex-col gap-3")
            .kid(btn(tr().wallet.create_account, Variant::Primary, "w-full", Some(Msg::Push(WalletView::CreatePhrase))).def(true))
            .kid(btn(tr().wallet.sign_in_passkey, Variant::Secondary, "w-full", Some(Msg::Push(WalletView::Passkey { mode: PasskeyMode::Signin }))))
            .kid(p("text-center text-sm text-secondary").text(create_line())),
        div("mt-6").kid(passkey_create_way()),
        h2("mb-2 mt-8 text-base font-semibold").text(if p_.none { tr().wallet.other_ways_sign_in } else { tr().wallet.other_ways_add }),
        ways,
    ];
    if let Some(e) = &p_.error {
        body.push(p("mt-4 text-sm text-negative").text(e));
    }
    if p_.none {
        body.push(div("mt-6 flex justify-center").kid(btn_full(tr().wallet.move_assets, Variant::Ghost, Size::Md, false, "", Some(Msg::Push(WalletView::Playtron)))));
    }
    page(Some(if p_.none { tr().common.sign_in } else { tr().wallet.add_an_account }), (!p_.none).then(|| text(tr().wallet.pick_how_to_add)), None, true, "", body)
}

pub fn add_hints() -> Vec<Hint> {
    vec![h::select(), h::back()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome() {
        let v = welcome_view();
        assert_eq!(
            v.texts(),
            [
                "Rare Wallet",
                "Create account",
                "Sign in with a passkey",
                create_line(),
                "Create account with PIN",
                pin_way(),
                "Restore an account with PIN",
                restore_way(),
                "Create a passkey account",
                "Signs with your phone or a security key. No words to write down",
                "Other ways to sign in",
                "Recovery phrase",
                "Use the 12 or 24 words from another wallet",
                "Private key",
                "Paste a suiprivkey... key",
                "Ledger",
                "Nano S Plus, Nano X, Stax and Flex over USB or Bluetooth",
                "Keystone",
                "Keystone 3 Pro over a USB-C cable",
                "Watch an address",
                "Follow any wallet. You cannot send from it",
                "Move your assets from your old SuiPlay wallet",
            ]
        );
        assert_eq!(v.landing().as_deref(), Some("Create account"));
    }

    #[test]
    fn add() {
        let v = add_view(&AddProps { none: false, has_phrase: true, busy: true, error: Some("Could not add the account".into()) });
        let t = v.texts();
        assert_eq!(&t[..2], ["Add an account", "Pick how you want to add it."]);
        assert!(t.contains(&"Other ways to add".to_string()));
        assert!(t.contains(&"Adding".to_string()));
        assert!(t.contains(&"Could not add the account".to_string()));
        assert!(!t.contains(&"Move your assets from your old SuiPlay wallet".to_string()));
        let v = add_view(&AddProps { none: true, has_phrase: false, busy: false, error: None });
        let t = v.texts();
        assert_eq!(t[0], "Sign in");
        assert!(!t.contains(&"Add an account from your recovery phrase".to_string()));
        assert!(t.contains(&"Move your assets from your old SuiPlay wallet".to_string()));
    }
}
