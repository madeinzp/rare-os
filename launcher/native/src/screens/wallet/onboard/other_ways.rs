//! "Other ways to sign in" (launcher/src/wallet/ui/onboard/OtherWays.tsx): the same list on
//! Welcome, the Add account page and the launcher's sign-in overlay.

use rare_i18n::tr;
use crate::screens::wallet::home::stack::{ImportWhat, WalletView};
use crate::screens::wallet::kit::{list_button, panel};
use crate::screens::wallet::markup::*;

pub struct Way {
    pub view: WalletView,
    pub title: &'static str,
    pub body: &'static str,
    pub icon: &'static str,
}

pub fn other_ways_list() -> Vec<Way> {
    vec![
        Way { view: WalletView::Import { what: ImportWhat::Phrase }, title: tr().wallet_setup.way_phrase_title, body: tr().wallet_setup.way_phrase_body, icon: "file-text" },
        Way { view: WalletView::Import { what: ImportWhat::Key }, title: tr().wallet_setup.way_key_title, body: tr().wallet_setup.way_key_body, icon: "key-round" },
        Way { view: WalletView::Ledger, title: "Ledger", body: tr().wallet_setup.way_ledger_body, icon: "usb" },
        Way { view: WalletView::Keystone, title: "Keystone", body: tr().wallet_setup.way_keystone_body, icon: "shield-check" },
        Way { view: WalletView::Watch, title: tr().wallet_setup.way_watch_title, body: tr().wallet_setup.way_watch_body, icon: "eye" },
    ]
}

/// The panel of ways; A sends `on(view)`.
pub fn other_ways<M>(on: impl Fn(WalletView) -> M, first_default: bool) -> El<M> {
    panel(
        None,
        None,
        "divide-y divide-border-primary p-1",
        other_ways_list()
            .into_iter()
            .enumerate()
            .map(|(i, w)| list_button(Some(icon(w.icon, "").attr("size", "1.4rem")), w.title, Some(text(w.body)), None, on(w.view), first_default && i == 0, "").key(w.title))
            .collect(),
    )
}
