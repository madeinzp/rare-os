//! First run: Epic Games and GOG, both optional. Each signs in in a web window (the same as
//! Settings > Accounts); their games join the Library. Continue moves on with either, both or none.

use rare_i18n::{tr, trf};
use super::kit::{done_line, step};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::ui::{button_el, panel, BSize, ButtonSpec, BVariant};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    /// "epic" or "gog"
    SignIn(&'static str),
    Continue,
    Back,
}

/// One store: its key, name, the signed-in name (None not signed in).
pub struct Row<'a> {
    pub key: &'static str,
    pub name: &'a str,
    pub account: Option<&'a str>,
}

pub fn view(rows: &[Row], online: bool) -> El<Msg> {
    let any = rows.iter().any(|r| r.account.is_some());
    let list = panel(
        "max-w-[40rem]",
        rows.iter()
            .map(|r| {
                let mut row = div("flex items-center gap-[1rem] border-t border-hair px-[1.4rem] py-[1rem] first:border-t-0").key(r.key);
                let info = span("min-w-0 flex-1").kid(span("block text-body font-semibold").text(r.name));
                match r.account {
                    Some(a) => row.kid(info).kid(span("shrink-0").kid(done_line(vec![span("").text(trf!(tr().firstrun.signed_in_as, name = a))]))),
                    None => {
                        row = row.kid(info.kid(span("block text-small text-muted").text(trf!(tr().firstrun.store_games_note, name = r.name))));
                        row.kid(button_el(ButtonSpec::new(tr().common.sign_in).v(BVariant::Secondary).size(BSize::Md).disabled(!online), Some(Msg::SignIn(r.key))))
                    }
                }
            })
            .collect(),
    );
    let mut content = vec![list.attr("data-nav-group", "firstrun-stores")];
    if !online {
        content.push(div("mt-[1rem] text-small text-muted").text(tr().firstrun.stores_offline));
    }
    let actions = vec![button_el(ButtonSpec::new(if any { tr().common.continue_ } else { tr().firstrun.skip_for_now }).v(if any { BVariant::Primary } else { BVariant::Secondary }).size(BSize::Lg).def(true).class("min-w-[11rem]"), Some(Msg::Continue))];
    step(text(tr().firstrun.stores_title), Some(text(tr().firstrun.stores_body)), content, actions, false).on_pad("b", Msg::Back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores() {
        let rows = [Row { key: "epic", name: "Epic Games", account: None }, Row { key: "gog", name: "GOG", account: None }];
        let v = view(&rows, true);
        assert_eq!(v.texts()[0], "Epic Games and GOG");
        assert_eq!(v.landing().as_deref(), Some("Skip for now"));
        let rows = [Row { key: "epic", name: "Epic Games", account: Some("player") }, Row { key: "gog", name: "GOG", account: None }];
        let v = view(&rows, true);
        assert!(v.contains_text("Signed in as "));
        assert_eq!(v.landing().as_deref(), Some("Continue"));
        assert!(view(&rows, false).contains_text("You are offline. Sign in later from Settings > Accounts."));
        assert_eq!(v.pad("b"), Some(&Msg::Back));
    }
}
