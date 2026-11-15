//! "Move everything to this account": pick the account to move from, see what it holds and what
//! stays behind, then one review sheet signed the way that account signs. Effects:
//! `flows/move_all.rs`; the transaction: `rare_wallet::move_all`.

use crate::screens::wallet::fmt::{addr_short, format_amount};
use crate::screens::wallet::home::lib::{hint, Hint};
use crate::screens::wallet::kit::{btn, list_button, page, panel, Variant};
use crate::screens::wallet::markup::*;
use rare_i18n::{tr, trf, trn};

/// An account that can move its things here.
#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    pub address: String,
    pub label: String,
    /// how it signs, in a few words
    pub method: String,
}

/// What the scan found, in the page's words.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Summary {
    pub sui: u128,
    /// (symbol, raw, decimals)
    pub coins: Vec<(String, u128, u32)>,
    pub nfts: usize,
    pub kiosks: usize,
    pub kiosk_items: usize,
    pub staked: usize,
    pub names: usize,
    /// (what, why)
    pub stuck: Vec<(String, String)>,
    pub txs: usize,
}

impl Summary {
    pub fn empty(&self) -> bool {
        self.txs == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Pick,
    Reading(String),
    Plan(String, Summary),
    Sending(String),
    Done { from: String, digests: Vec<String> },
    Failed(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub phase: Phase,
}

impl Default for State {
    fn default() -> Self {
        State { phase: Phase::Pick }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Pick(String),
    Move,
    Again,
    Done,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    Scan(String),
    Send(String),
    Close,
}

pub struct Props<'a> {
    pub to_label: &'a str,
    pub to: &'a str,
    pub froms: &'a [Source],
}

fn label_of<'a>(pr: &'a Props, a: &'a str) -> &'a str {
    pr.froms.iter().find(|f| f.address == a).map(|f| f.label.as_str()).unwrap_or(a)
}

fn row(left: &str, right: String) -> El<Msg> {
    div("flex items-center justify-between gap-4 px-5 py-3").kid(span("text-base text-secondary").text(left)).kid(span("min-w-0 truncate text-right text-base font-medium num").text(right))
}

pub fn view(pr: &Props, st: &State) -> El<Msg> {
    let sub = trf!(tr().wallet.move_to, name = pr.to_label, addr = addr_short(pr.to));
    let body: Vec<El<Msg>> = match &st.phase {
        Phase::Pick if pr.froms.is_empty() => vec![p("text-secondary").text(tr().wallet.move_no_source)],
        Phase::Pick => {
            let mut list = div("flex flex-col gap-1");
            for (i, f) in pr.froms.iter().enumerate() {
                list = list.kid(list_button(Some(icon("wallet", "").attr("size", "1.4rem")), &f.label, Some(text(format!("{}  {}", addr_short(&f.address), f.method))), None, Msg::Pick(f.address.clone()), i == 0, ""));
            }
            vec![p("mb-4 text-secondary").text(tr().wallet.move_pick), panel(None, None, "p-1", vec![list])]
        }
        Phase::Reading(a) => vec![div("flex flex-col items-center gap-3 py-12 text-center").kid(div("pointer-events-none absolute left-0 right-0 top-0 z-30 h-[2px] load-bar")).kid(p("text-lg font-semibold").text(trf!(tr().wallet.move_reading, name = label_of(pr, a))))],
        Phase::Plan(a, s) if s.empty() => vec![
            div("flex flex-col items-center gap-3 py-10 text-center").kid(p("text-lg font-semibold").text(trf!(tr().wallet.move_empty, name = label_of(pr, a)))).kid(p("text-secondary").text(tr().wallet.move_nothing)),
            btn(tr().common.done, Variant::Primary, "w-full", Some(Msg::Done)).def(true),
        ],
        Phase::Plan(a, s) => {
            let mut rows = div("flex flex-col divide-y divide-border-primary");
            if s.sui > 0 {
                rows = rows.kid(row("SUI", trf!(tr().wallet.move_sui_less_fee, amount = format_amount(s.sui.min(i128::MAX as u128) as i128, 9))));
            }
            for (sym, raw, dec) in s.coins.iter().take(6) {
                rows = rows.kid(row(tr().wallet.col_coin, format!("{} {sym}", format_amount((*raw).min(i128::MAX as u128) as i128, *dec))));
            }
            if s.coins.len() > 6 {
                rows = rows.kid(row(tr().wallet.coins, trf!(tr().wallet.n_more, n = s.coins.len() - 6)));
            }
            if s.nfts > 0 {
                rows = rows.kid(row(tr().wallet.collectibles, s.nfts.to_string()));
            }
            if s.kiosks > 0 {
                rows = rows.kid(row(tr().wallet.kiosks, trf!(tr().wallet.kiosks_items, kiosks = s.kiosks, items = trn!(tr().wallet.items_n, s.kiosk_items))));
            } else if s.kiosk_items > 0 {
                rows = rows.kid(row(tr().wallet.kiosk_items, s.kiosk_items.to_string()));
            }
            if s.staked > 0 {
                rows = rows.kid(row(tr().wallet.staked_sui, trn!(tr().wallet.stakes_n, s.staked)));
            }
            if s.names > 0 {
                rows = rows.kid(row(tr().wallet.suins_names, s.names.to_string()));
            }
            let mut v = vec![p("mb-4 text-secondary").text(trf!(tr().wallet.move_from_note, name = label_of(pr, a), addr = addr_short(a))), panel(None, None, "", vec![rows])];
            if !s.stuck.is_empty() {
                let mut st_rows = div("flex flex-col divide-y divide-border-primary");
                for (what, why) in s.stuck.iter().take(5) {
                    st_rows = st_rows.kid(div("flex flex-col gap-0.5 px-5 py-3").kid(span("truncate text-base font-medium").text(what)).kid(span("text-sm text-secondary").text(why)));
                }
                if s.stuck.len() > 5 {
                    st_rows = st_rows.kid(div("px-5 py-3 text-sm text-secondary").text(trf!(tr().wallet.n_more, n = s.stuck.len() - 5)));
                }
                v.push(h2("mb-2 mt-6 text-base font-semibold").text(tr().wallet.stays_behind));
                v.push(panel(None, None, "", vec![st_rows]));
            }
            let what = if s.txs > 1 { trf!(tr().wallet.move_everything_approvals, n = s.txs) } else { tr().wallet.move_everything.to_string() };
            v.push(div("mt-6").kid(btn(&what, Variant::Primary, "w-full", Some(Msg::Move)).def(true)));
            v
        }
        Phase::Sending(a) => vec![div("flex flex-col items-center gap-3 py-12 text-center").kid(p("text-lg font-semibold").text(trf!(tr().wallet.approve_with, name = label_of(pr, a)))).kid(p("text-secondary").text(tr().wallet.move_check_sheet))],
        Phase::Done { from, digests } => vec![
            div("flex flex-col items-center gap-3 py-8 text-center")
                .kid(icon("circle-check", "text-icon-primary").attr("size", "2.5rem").attr("stroke-width", "1.5"))
                .kid(p("text-lg font-semibold").text(tr().wallet.move_on_its_way))
                .kid(p("max-w-[32rem] text-secondary").text(trf!(tr().wallet.move_sent_all, from = label_of(pr, from), to = pr.to_label, count = trn!(tr().wallet.transactions_n, digests.len())))),
            btn(tr().common.done, Variant::Primary, "w-full", Some(Msg::Done)).def(true),
        ],
        Phase::Failed(m) => vec![p("mb-4 text-negative").text(m), btn(tr().common.try_again, Variant::Primary, "w-full", Some(Msg::Again)).def(true)],
    };
    page(Some(tr().wallet.move_everything_here), Some(text(sub)), None, true, "", body)
}

pub fn update(st: &mut State, msg: Msg) -> Option<Effect> {
    match msg {
        Msg::Pick(a) => {
            st.phase = Phase::Reading(a.clone());
            Some(Effect::Scan(a))
        }
        Msg::Move => match &st.phase {
            Phase::Plan(a, s) if !s.empty() => {
                let a = a.clone();
                st.phase = Phase::Sending(a.clone());
                Some(Effect::Send(a))
            }
            _ => None,
        },
        Msg::Again => {
            st.phase = Phase::Pick;
            None
        }
        Msg::Done => Some(Effect::Close),
    }
}

pub fn hints() -> Vec<Hint> {
    vec![hint("a", tr().common.select), hint("b", tr().common.back)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_plan_send_done() {
        let froms = vec![Source { address: format!("0x{}", "a".repeat(64)), label: "Old account".into(), method: "Signs with A".into() }];
        let pr = Props { to_label: "Rare account", to: "0x2250ac61c912b3eec61d00989acdc1057ee3fbd688cdc577f5a5224d1163d34e", froms: &froms };
        let mut st = State::default();
        let v = view(&pr, &st);
        assert!(v.contains_text("Move everything to this account") && v.contains_text("Old account"));
        assert_eq!(update(&mut st, Msg::Pick(froms[0].address.clone())), Some(Effect::Scan(froms[0].address.clone())));
        assert!(view(&pr, &st).contains_text("Reading what Old account holds"));
        st.phase = Phase::Plan(froms[0].address.clone(), Summary { sui: 8_940_560_728, kiosks: 1, kiosk_items: 1, staked: 1, txs: 1, stuck: vec![("Badge".into(), "It is bound to this account".into())], ..Default::default() });
        let v = view(&pr, &st);
        assert!(v.contains_text("8.9405 SUI, less the fee") || v.texts().iter().any(|t| t.ends_with("SUI, less the fee")));
        assert!(v.contains_text("Stays behind") && v.contains_text("1, 1 item"));
        assert_eq!(update(&mut st, Msg::Move), Some(Effect::Send(froms[0].address.clone())));
        st.phase = Phase::Done { from: froms[0].address.clone(), digests: vec!["x".into()] };
        assert!(view(&pr, &st).contains_text("Everything is on its way"));
        // nobody to move from
        let pr = Props { froms: &[], ..pr };
        assert!(view(&pr, &State::default()).texts().iter().any(|t| t.starts_with("There is no other account")));
    }
}
