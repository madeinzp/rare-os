//! The Wishlist page (overlay `wishlist`): games added in Rare and the Steam account's wishlist,
//! each with the lowest price across stores right now (rare.market, from CheapShark) and the
//! lowest ever. A opens the best deal in the browser, X removes, Y adds a game (the keyboard, then
//! a search). Prices refresh every 6 hours; a drop of 5 % or more raises a notice.

use crate::act::Act;
use crate::data::knowledge::{self, Hit, Price};
use crate::data::wishlist::{self, money, Item};
use crate::kit::{self, s, tok, Btn as B, BtnIcon, Track, Ty, Ui, T};
use crate::page::{hash, Acts, Btn, Feedback, Fx, Page};
use crate::screens::game::parts::{self, Variant};
use crate::state::{State, Tone};
use rare_render::focus::hints::{hint, Hint, Key};
use rare_render::focus::{Focusable, Group};
use rare_render::view::{key, El, Motion};
use std::sync::{Arc, Mutex};
use rare_i18n::{tr, trf, trn};

pub const KIND: &str = "wishlist";
const ASK: &str = "wishlist:search";

const ACT_ADD: u32 = 1;
const ACT_ROW: u32 = 1000;
const ACT_HIT: u32 = 5000;

#[derive(Default)]
struct Found {
    q: String,
    hits: Option<Result<Vec<Hit>, String>>,
}

#[derive(Default)]
pub struct WishlistPage {
    /// The search the keyboard asked for (None: the list).
    found: Option<Arc<Mutex<Found>>>,
    ver: u64,
    /// Rows by tag order this build (id), for X.
    ids: Vec<String>,
}

const COLS: [Track; 4] = [Track::Rem(8.0), Track::Fr(1.0), Track::Rem(11.0), Track::Rem(8.5)];

/// A wishlist game's art: Steam's header for Steam apps, else CheapShark's capsule.
fn art_srcs(steam: Option<u32>, thumb: Option<&str>) -> Vec<String> {
    let mut v = Vec::new();
    if let Some(a) = steam {
        v.push(format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{a}/header.jpg"));
    }
    if let Some(t) = thumb {
        v.push(t.to_string());
    }
    v
}

impl WishlistPage {
    fn search(&mut self, fx: &mut Fx, q: String) {
        let f = Arc::new(Mutex::new(Found { q: q.clone(), hits: None }));
        self.found = Some(f.clone());
        self.ver += 1;
        fx.spawn(move |_| {
            let r = knowledge::search(&q);
            f.lock().unwrap().hits = Some(r);
            Box::new(|_, _| {})
        });
    }

    fn row(&self, ui: &mut Ui, st: &State, acts: &mut Acts, i: usize, it: &Item, p: Option<&Price>) -> El {
        let srcs = art_srcs(it.steam.or(p.and_then(|p| p.steam_app_id)), p.and_then(|p| p.thumb.as_deref()));
        let pic = parts::art(ui, &srcs, &it.title, Variant::Hero, 8.0, 3.75, 0.5);
        let art = ui.bx(s().wh(8.0, 3.75)).bg(tok::L3).radius(ui.r(0.35)).clip().child(pic);
        let owned = st.games().iter().any(|g| g.id == it.id || g.title.eq_ignore_ascii_case(&it.title));
        let mut sub: Vec<String> = Vec::new();
        if owned {
            sub.push(tr().features_more.wishlist_in_library.into());
        }
        sub.push(if it.steam_list && it.rare { tr().features_more.wishlist_steam_and_rare.into() } else if it.steam_list { tr().features_more.wishlist_from_steam.into() } else { tr().features_more.wishlist_added_in_rare.into() });
        if it.added > 0 {
            sub.push(trf!(tr().features_more.wishlist_added, when = knowledge::day_label(it.added, false)));
        }
        let title = ui.bx(s().col().min0().gap(0.2)).child(ui.text(it.title.clone(), T::new(Ty::Body).w(600).trunc())).child(ui.text(sub.join(" \u{b7} "), T::new(Ty::Meta).c(tok::MUTED).trunc()));
        let (store, low) = match p {
            Some(p) if p.price.is_some() => (
                p.store.clone().map(|s| trf!(tr().features_more.wishlist_at_store, store = s)).unwrap_or_default(),
                p.lowest.as_ref().map(|l| trf!(tr().features_more.wishlist_lowest_ever, price = money(l.price))).unwrap_or_default(),
            ),
            Some(_) => (tr().features_more.wishlist_no_store.to_string(), String::new()),
            None => (tr().features_more.wishlist_checking_prices.to_string(), String::new()),
        };
        let where_ = ui.bx(s().col().min0().gap(0.2)).child(ui.text(store, T::new(Ty::Small).c(tok::fg(0.8)).trunc())).child(ui.text(low, T::new(Ty::Meta).c(tok::MUTED).trunc()));
        let mut price = ui.bx(s().col().items_end().min0().gap(0.2));
        match p.and_then(|p| p.price.map(|v| (v, p))) {
            Some((v, p)) => {
                price = price.child(ui.text(money(v), T::new(Ty::Row).w(700).tab()));
                let line = if p.cut > 0 { trf!(tr().features_more.wishlist_cut_was, cut = p.cut, price = money(p.retail.unwrap_or(0.0))) } else { String::new() };
                price = price.child(ui.text(line, T::new(Ty::Meta).c(tok::MUTED).tab().trunc()));
            }
            None => price = price.child(ui.text("--", T::new(Ty::Row).c(tok::MUTED))),
        }
        let el = ui.bx(s().grid(&COLS).items_center().gap_xy(1.0, 0.0).h(5.0).px(0.75).wp(1.0)).radius(ui.r(0.5)).child(art).child(title).child(where_).child(kit::cell(ui, true).child(price));
        let has_deal = p.and_then(|p| p.url.as_ref()).is_some();
        let tag = acts.tag(Act::Local(ACT_ROW + i as u32));
        kit::nav(kit::row_look(el), tag, Some(if has_deal { tr().features_more.wishlist_open_deal } else { tr().common.details }), Some(tr().common.remove), i == 0).key(key(&it.id))
    }
}

impl Page for WishlistPage {
    fn stamp(&self, st: &State) -> u64 {
        let found = self.found.as_ref().map(|f| {
            let f = f.lock().unwrap();
            (f.q.clone(), f.hits.as_ref().map(|h| h.as_ref().map(|v| v.len()).unwrap_or(usize::MAX)))
        });
        hash((self.ver, wishlist::ver(), found, st.ver.library))
    }

    fn tick(&mut self, st: &mut State, fx: &mut Fx, now: f64) -> Option<f64> {
        if let Some(a) = st.take_answer(ASK) {
            match a.map(|q| q.trim().to_string()).filter(|q| q.len() >= 2) {
                Some(q) => self.search(fx, q),
                None => {}
            }
        }
        // Waiting on a search.
        self.found.as_ref().filter(|f| f.lock().unwrap().hits.is_none()).map(|_| now + 0.2)
    }

    fn view(&mut self, ui: &mut Ui, st: &State, acts: &mut Acts) -> El {
        let (shown, prices, sale, steam_ok, prices_at): (Vec<Item>, Vec<Option<Price>>, usize, bool, i64) = wishlist::with(|w| {
            let v: Vec<Item> = w.shown().into_iter().cloned().collect();
            let p = v.iter().map(|i| w.prices.get(&i.id).cloned()).collect();
            (v, p, w.on_sale(), w.steam_ok, w.prices_at)
        });
        let note = if shown.is_empty() { String::new() } else if sale > 0 { trf!(tr().features_more.wishlist_games_on_sale, games = trn!(tr().common.games_n, shown.len()), sale = sale) } else { trn!(tr().common.games_n, shown.len()) };
        let head = ui
            .bx(s().row().items_center().gap(0.75).h(2.5).mb(1.0))
            .child(kit::icon(ui, "heart", 1.5, tok::ICON2))
            .child(ui.text(tr().features_more.wishlist_title, T::new(Ty::Title).w(700)))
            .child(ui.text_s(s().ml_auto(), note, T::new(Ty::Small).c(tok::MUTED).tab()));
        let mut page = ui.bx(s().col().fill().px(2.5).pt(2.0).pb(5.0)).bg(tok::BG).enter(Motion::FADE).scrolls_y(key("wishlist-page")).child(head);
        // A search the keyboard asked for.
        if let Some(f) = self.found.clone() {
            let f = f.lock().unwrap();
            page = page.child(ui.text_s(s().mb(0.75), trf!(tr().features_more.wishlist_results_for, query = f.q), T::new(Ty::Row).w(600).trunc()));
            match &f.hits {
                None => page = page.child(ui.text(tr().features_more.wishlist_searching, T::new(Ty::Small).c(tok::MUTED))),
                Some(Err(_)) => {
                    let again = kit::button(ui, B::new(tr().features_more.wishlist_search_again, acts.tag(Act::Local(ACT_ADD))).primary().def(true));
                    page = page.child(kit::empty(ui, s(), tr().features_more.wishlist_search_failed, Some(tr().features_more.wishlist_search_failed_body), vec![again]));
                }
                Some(Ok(hits)) if hits.is_empty() => {
                    let again = kit::button(ui, B::new(tr().features_more.wishlist_search_again, acts.tag(Act::Local(ACT_ADD))).primary().def(true));
                    page = page.child(kit::empty(ui, s(), tr().features_more.wishlist_none_found, Some(tr().features_more.wishlist_none_found_body), vec![again]));
                }
                Some(Ok(hits)) => {
                    let rows: Vec<El> = hits
                        .iter()
                        .take(20)
                        .enumerate()
                        .map(|(i, h)| {
                            let srcs = art_srcs(h.steam_app_id, h.thumb.as_deref());
                            let pic = parts::art(ui, &srcs, &h.title, Variant::Hero, 8.0, 3.75, 0.5);
                            let art = ui.bx(s().wh(8.0, 3.75)).bg(tok::L3).radius(ui.r(0.35)).clip().child(pic);
                            let on = wishlist::with(|w| w.has(&h.id));
                            let el = ui
                                .bx(s().row().items_center().gap(1.0).h(5.0).px(0.75).wp(1.0))
                                .radius(ui.r(0.5))
                                .child(art)
                                .child(ui.bx(s().col().grow().gap(0.2)).child(ui.text(h.title.clone(), T::new(Ty::Body).w(600).trunc())).child(ui.text(if on { tr().features_more.wishlist_on_it } else { "" }, T::new(Ty::Meta).c(tok::MUTED))))
                                .child(ui.text_s(s().w(7.0), h.price.map(money).unwrap_or_default(), T::new(Ty::Row).w(700).tab().right()));
                            let tag = acts.tag(Act::Local(ACT_HIT + i as u32));
                            kit::nav(kit::row_look(el), tag, Some(if on { tr().features_more.wishlist_on_it } else { tr().common.add }), None, i == 0).key(key(&h.id))
                        })
                        .collect();
                    page = page.child(ui.bx(s().col().gap(0.25)).group(Group::new("wishlist-hits", [0.0; 4])).kids(rows));
                }
            }
            return page;
        }
        let add = kit::button(ui, B::new(tr().features_more.wishlist_add_game, acts.tag(Act::Local(ACT_ADD))).icon(BtnIcon::Lucide("plus", 1.0, false)).hint(tr().features_more.wishlist_add_game).def(shown.is_empty()));
        if shown.is_empty() {
            self.ids.clear();
            let body = if steam_ok { tr().features_more.wishlist_empty_body } else { tr().features_more.wishlist_empty_body_steam };
            return page.child(kit::empty(ui, s(), tr().features_more.wishlist_empty, Some(body), vec![add]));
        }
        page = page.child(ui.bx(s().row().items_center().gap(1.0).mb(1.0)).child(add).child(ui.text_s(s().ml_auto().shrink().min0(), if prices_at > 0 { trf!(tr().features_more.wishlist_prices_checked, when = crate::data::model::fmt_ago(Some(prices_at)).trim_start_matches("Played ").to_lowercase()) } else { tr().features_more.wishlist_prices.to_string() }, T::new(Ty::Small).c(tok::MUTED).trunc())));
        self.ids = shown.iter().map(|i| i.id.clone()).collect();
        let rows: Vec<El> = shown.iter().zip(prices.iter()).enumerate().map(|(i, (it, p))| self.row(ui, st, acts, i, it, p.as_ref())).collect();
        page = page.child(ui.bx(s().col().gap(0.25)).group(Group::new("wishlist-rows", [0.0; 4]).remember()).kids(rows));
        if !steam_ok {
            page = page.child(ui.text_s(s().mt(1.25).max_w(52.0), tr().features_more.wishlist_steam_note, T::new(Ty::Small).c(tok::MUTED).wrap()));
        }
        page
    }

    fn act(&mut self, st: &mut State, fx: &mut Fx, code: u32) {
        let now = rare_render::clock::secs();
        if code == ACT_ADD {
            st.ask_text(ASK, tr().features_more.wishlist_ask_title, "", Some(tr().features_more.wishlist_ask_placeholder));
            return;
        }
        if let Some(i) = code.checked_sub(ACT_HIT) {
            let Some(h) = self.found.as_ref().and_then(|f| f.lock().unwrap().hits.as_ref().and_then(|r| r.as_ref().ok()).and_then(|v| v.get(i as usize).cloned())) else { return };
            if wishlist::with(|w| w.has(&h.id)) {
                return;
            }
            wishlist::add(&h.id, &h.title, h.steam_app_id);
            fx.fb(Feedback::Toggle);
            st.toast(trf!(tr().features_more.wishlist_added_toast, title = h.title), Tone::Info, now);
            self.found = None;
            self.ver += 1;
            return;
        }
        if let Some(i) = code.checked_sub(ACT_ROW) {
            let Some(id) = self.ids.get(i as usize).cloned() else { return };
            let url = wishlist::with(|w| w.prices.get(&id).and_then(|p| p.url.clone()));
            let steam = id.strip_prefix("steam:").and_then(|a| a.parse::<u32>().ok());
            match (url, steam) {
                (Some(u), _) => {
                    st.toast(tr().features_more.wishlist_opening_deal, Tone::Info, now);
                    fx.run(Act::OpenWeb(u));
                }
                (None, Some(a)) => fx.run(Act::SteamStore(a)),
                _ => st.toast(tr().features_more.wishlist_no_store_toast, Tone::Info, now),
            }
        }
    }

    fn pad(&mut self, st: &mut State, fx: &mut Fx, b: Btn, cur: Option<(&Focusable, Option<&Act>)>) -> bool {
        let now = rare_render::clock::secs();
        match b {
            Btn::B if self.found.is_some() => {
                self.found = None;
                self.ver += 1;
                fx.fb(Feedback::Back);
                true
            }
            Btn::Y if self.found.is_none() => {
                self.act(st, fx, ACT_ADD);
                true
            }
            Btn::X if self.found.is_none() => {
                let Some((_, Some(Act::Local(c)))) = cur else { return false };
                let Some(id) = c.checked_sub(ACT_ROW).and_then(|i| self.ids.get(i as usize).cloned()) else { return false };
                let title = wishlist::with(|w| w.items.iter().find(|i| i.id == id).map(|i| i.title.clone())).unwrap_or_default();
                wishlist::remove(&id);
                fx.fb(Feedback::Toggle);
                st.toast(trf!(tr().features_more.wishlist_removed, title = title), Tone::Info, now);
                true
            }
            _ => false,
        }
    }

    fn hints(&self, _st: &State) -> Vec<Hint> {
        if self.found.is_some() {
            return vec![hint(Key::B, tr().features_more.wishlist_back)];
        }
        vec![hint(Key::Y, tr().features_more.wishlist_add_game), hint(Key::B, tr().common.back)]
    }
}
