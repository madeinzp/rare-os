//! What the game page's lower sections show, read off the UI thread and kept: a Steam game's news
//! (Steam's public ISteamNews, no key) or a Sui game's (rare.market's catalog), its DLC (the store's
//! list and art, what the appmanifest says is installed or turned off), its manual (Steam's
//! /manual/<appid> link when it leads to a file), its store screenshots (Steam's, also for an Epic or
//! GOG game found on Steam by its exact name), and Rare's own screenshots of it.
//!
//! The network answers are kept on disk (`keep::save`) so a revisit or a restart shows the last
//! ones at once; they refresh in the background after `FRESH_S`.

use crate::data::keep;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

/// Seconds a kept answer counts as fresh.
pub const FRESH_S: u64 = 30 * 60;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String,
    pub title: String,
    /// Unix seconds.
    pub date: i64,
    pub url: String,
    /// "Patch notes", "Announcement", or the outlet's name.
    pub feed: String,
    /// The text as paragraphs (`Para`).
    pub body: Vec<Para>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParaKind {
    Text,
    Head,
    Bullet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Para {
    pub kind: ParaKind,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Dlc {
    pub appid: u32,
    pub name: String,
    pub art: Vec<String>,
    /// "$9.99", "Free"; None when the store shows no price.
    pub price: Option<String>,
}

/// One screenshot from the game's store page.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StoreShot {
    /// 600 px wide, for the rail.
    pub thumb: String,
    /// Full size, for the viewer.
    pub full: String,
}

/// The network part, kept per game.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Remote {
    /// Unix seconds it was read.
    pub at: u64,
    pub news: Vec<NewsItem>,
    pub dlc: Vec<Dlc>,
    pub manual: Option<String>,
    #[serde(default)]
    pub store_shots: Vec<StoreShot>,
    /// An Epic or GOG game's Steam app, once the store search found it: a refresh reads its
    /// screenshots without searching again.
    #[serde(default)]
    pub store_app: Option<u32>,
}

fn now_s() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(10))).build().into()
}

fn get_json(url: &str) -> Option<serde_json::Value> {
    agent().get(url).header("User-Agent", "RareLauncher/0.1").call().ok()?.body_mut().read_json().ok()
}

static MEM: Mutex<Option<HashMap<String, Remote>>> = Mutex::new(None);

/// The last answer for `id` (memory, else disk). Fixture runs never touch the disk copy.
pub fn cached(id: &str) -> Option<Remote> {
    if let Some(r) = MEM.lock().unwrap().as_ref().and_then(|m| m.get(id)).cloned() {
        return Some(r);
    }
    let r: Remote = keep::load(&format!("game-extras:{id}"))?;
    MEM.lock().unwrap().get_or_insert_with(HashMap::new).insert(id.to_string(), r.clone());
    Some(r)
}

pub fn fresh(r: &Remote) -> bool {
    now_s().saturating_sub(r.at) < FRESH_S
}

fn store(id: &str, r: &Remote) {
    MEM.lock().unwrap().get_or_insert_with(HashMap::new).insert(id.to_string(), r.clone());
    keep::save(&format!("game-extras:{id}"), r);
}

/// What to read for a game.
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Steam(u32),
    Sui(String),
    /// An Epic or GOG game: its store screenshots from the Steam page with exactly its name, if any.
    /// Only those: news and DLC of a wrongly matched page would mislead.
    Title(String),
}

/// Reads everything for `src` (blocking, on a worker) and keeps it. A part that fails keeps the
/// last value it had.
pub fn fetch(id: &str, src: &Source) -> Remote {
    let last = cached(id).unwrap_or_default();
    let r = match src {
        Source::Steam(appid) => {
            // One appdetails call for the DLC list and the screenshots, as many store calls as the
            // page made before it showed screenshots.
            let details = app_details(*appid, "basic,screenshots");
            Remote {
                at: now_s(),
                news: steam_news(*appid).unwrap_or(last.news),
                dlc: details.as_ref().map(steam_dlc).unwrap_or(last.dlc),
                manual: steam_manual(*appid).unwrap_or(last.manual),
                store_shots: details.as_ref().map(parse_shots).unwrap_or(last.store_shots),
                store_app: None,
            }
        }
        Source::Sui(slug) => Remote { at: now_s(), news: sui_news(slug).unwrap_or(last.news), ..Remote::default() },
        Source::Title(title) => {
            let app = match last.store_app {
                Some(a) => Some(Some(a)),
                None => steam_app_named(title),
            };
            let shots = match app {
                Some(Some(appid)) => app_details(appid, "screenshots").as_ref().map(parse_shots),
                Some(None) => Some(Vec::new()),
                None => None,
            };
            Remote { at: now_s(), store_shots: shots.unwrap_or(last.store_shots), store_app: app.flatten().or(last.store_app), ..Remote::default() }
        }
    };
    store(id, &r);
    r
}

// ---------------------------------------------------------------- store screenshots

/// The store's appdetails entry for `appid` with `filters` (None when the store did not answer).
fn app_details(appid: u32, filters: &str) -> Option<serde_json::Value> {
    let mut v = get_json(&format!("https://store.steampowered.com/api/appdetails?appids={appid}&filters={filters}"))?;
    Some(v.get_mut(appid.to_string())?.take())
}

fn parse_shots(data: &serde_json::Value) -> Vec<StoreShot> {
    if data.get("success").and_then(|s| s.as_bool()) != Some(true) {
        return Vec::new();
    }
    let list = data.get("data").and_then(|d| d.get("screenshots")).and_then(|s| s.as_array()).cloned().unwrap_or_default();
    list.iter()
        .filter_map(|it| {
            let full = it.get("path_full")?.as_str()?.to_string();
            let thumb = it.get("path_thumbnail").and_then(|t| t.as_str()).map(String::from).unwrap_or_else(|| full.clone());
            Some(StoreShot { thumb, full })
        })
        .take(30)
        .collect()
}

/// The Steam app whose name is exactly `title` (case, punctuation and marks aside), else exactly its
/// name without an edition ("BioShock Infinite: Complete Edition" is "BioShock Infinite" on Steam):
/// Some(None) when the search answered with no such app, None when it did not answer.
///
/// The search is the one the art picker uses (`gamemgmt::art::steam_search`). Offline, the first
/// search fails and the name without its edition is not tried.
fn steam_app_named(title: &str) -> Option<Option<u32>> {
    for name in std::iter::once(title.to_string()).chain(base_name(title)) {
        let hits = crate::features::gamemgmt::art::steam_search(&name).ok()?;
        if let Some((id, _)) = hits.iter().find(|(_, n)| same_name(n, &name)) {
            return u32::try_from(*id).ok().map(Some);
        }
    }
    Some(None)
}

/// A store name without its edition: "Batman™ Arkham City - Game of the Year Edition" is "Batman
/// Arkham City". None when there is no edition to drop. Only a trailing "... Edition" (or
/// "Remastered", "Director's Cut") after a dash or colon, or a whole known one, goes: the rest must
/// still match a Steam name exactly.
fn base_name(title: &str) -> Option<String> {
    let t = title.replace(['\u{2122}', '\u{00ae}', '\u{00a9}'], "");
    let t = t.trim();
    let lower = t.to_lowercase();
    // Byte offsets in `lower` slice `t`: only while lowercasing kept every length.
    if lower.len() != t.len() {
        return None;
    }
    let ends = ["edition", "remastered", "director's cut", "directors cut"];
    // The last dash or colon whose tail is an edition: "Game of the Year Edition", "Trials of Fear Edition".
    let cut = [" - ", ": ", " \u{2013} ", " \u{2014} "]
        .iter()
        .filter_map(|sep| lower.rfind(sep).map(|i| (i, i + sep.len())))
        .filter(|(_, tail)| ends.iter().any(|e| lower[*tail..].ends_with(e)))
        .map(|(i, _)| i)
        .max();
    let base = match cut {
        Some(i) => t[..i].trim().to_string(),
        // "Darksiders II Deathinitive Edition": a known edition word before " Edition", no dash.
        None => {
            let known = ["game of the year edition", "goty edition", "complete edition", "definitive edition", "deluxe edition", "enhanced edition", "ultimate edition", "gold edition", "special edition", "anniversary edition", "deathinitive edition", "warmastered edition"];
            let k = known.iter().find(|k| lower.ends_with(&format!(" {k}")))?;
            t[..t.len() - k.len()].trim().to_string()
        }
    };
    (!base.is_empty() && base != t).then_some(base)
}

/// Two store names for one game: equal once lowercased with everything but letters and digits
/// dropped ("APE OUT" and "Ape Out", "Observer_" and ">observer_").
fn same_name(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect::<String>();
    let (a, b) = (norm(a), norm(b));
    !a.is_empty() && a == b
}

// ---------------------------------------------------------------- Steam news

fn feed_label(it: &serde_json::Value) -> String {
    let name = it.get("feedname").and_then(|v| v.as_str()).unwrap_or("");
    let label = it.get("feedlabel").and_then(|v| v.as_str()).unwrap_or("");
    let tags: Vec<&str> = it.get("tags").and_then(|t| t.as_array()).map(|a| a.iter().filter_map(|v| v.as_str()).collect()).unwrap_or_default();
    if tags.iter().any(|t| *t == "patchnotes") {
        return "Patch notes".into();
    }
    if name == "steam_community_announcements" || label == "Community Announcements" {
        return "Announcement".into();
    }
    if label.is_empty() {
        "News".into()
    } else {
        label.to_string()
    }
}

fn parse_steam_news(v: &serde_json::Value) -> Option<Vec<NewsItem>> {
    let items = v.get("appnews")?.get("newsitems")?.as_array()?;
    Some(
        items
            .iter()
            .filter_map(|it| {
                let title = it.get("title")?.as_str()?.trim().to_string();
                let contents = it.get("contents").and_then(|c| c.as_str()).unwrap_or("");
                Some(NewsItem {
                    id: it.get("gid").and_then(|g| g.as_str()).unwrap_or(&title).to_string(),
                    date: it.get("date").and_then(|d| d.as_i64()).unwrap_or(0),
                    url: it.get("url").and_then(|u| u.as_str()).unwrap_or("").replace(' ', "%20"),
                    feed: feed_label(it),
                    body: paragraphs(contents),
                    title,
                })
            })
            .collect(),
    )
}

/// The game's own announcements (patch notes), else every feed Steam lists for it.
fn steam_news(appid: u32) -> Option<Vec<NewsItem>> {
    let base = format!("https://api.steampowered.com/ISteamNews/GetNewsForApp/v2/?appid={appid}&count=12&maxlength=0&format=json");
    let own = get_json(&format!("{base}&feeds=steam_community_announcements")).and_then(|v| parse_steam_news(&v));
    match own {
        Some(v) if !v.is_empty() => Some(v),
        _ => get_json(&base).and_then(|v| parse_steam_news(&v)).or(own),
    }
}

// ---------------------------------------------------------------- Sui news

/// rare.market's catalog entry for the game: its `news` (or `updates`) list, when it has one.
fn sui_news(slug: &str) -> Option<Vec<NewsItem>> {
    let v = get_json(&format!("https://rare.market/v1/games/{slug}"))?;
    let v = v.get("data").unwrap_or(&v);
    let list = v.get("news").or_else(|| v.get("updates")).and_then(|n| n.as_array());
    let Some(list) = list else { return Some(Vec::new()) };
    Some(
        list.iter()
            .filter_map(|it| {
                let title = it.get("title")?.as_str()?.trim().to_string();
                let date = it
                    .get("date")
                    .or_else(|| it.get("publishedAt"))
                    .or_else(|| it.get("createdAt"))
                    .and_then(|d| d.as_i64().map(|n| if n > 10_000_000_000 { n / 1000 } else { n }).or_else(|| d.as_str().and_then(parse_iso)))
                    .unwrap_or(0);
                let body = it.get("body").or_else(|| it.get("summary")).or_else(|| it.get("text")).and_then(|b| b.as_str()).unwrap_or("");
                let feed = if it.get("kind").and_then(|k| k.as_str()) == Some("patch") { "Patch notes" } else { "Update" };
                Some(NewsItem {
                    id: it.get("id").and_then(|i| i.as_str()).map(String::from).unwrap_or_else(|| format!("{slug}:{date}:{title}")),
                    title,
                    date,
                    url: it.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string(),
                    feed: feed.into(),
                    body: paragraphs(body),
                })
            })
            .collect(),
    )
}

/// "2026-10-27T16:18:20.082Z" to unix seconds (UTC, date and time only).
fn parse_iso(s: &str) -> Option<i64> {
    let (d, t) = s.split_once('T').unwrap_or((s, "00:00:00"));
    let mut p = d.split('-').map(|x| x.parse::<i64>().ok());
    let (y, m, dd) = (p.next()??, p.next()??, p.next()??);
    let mut q = t.trim_end_matches('Z').split(':').map(|x| x.split('.').next().and_then(|x| x.parse::<i64>().ok()));
    let (h, mi) = (q.next().flatten().unwrap_or(0), q.next().flatten().unwrap_or(0));
    // days_from_civil (Howard Hinnant)
    let y2 = if m <= 2 { y - 1 } else { y };
    let era = if y2 >= 0 { y2 } else { y2 - 399 } / 400;
    let yoe = y2 - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + dd - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some((era * 146097 + doe - 719468) * 86400 + h * 3600 + mi * 60)
}

// ---------------------------------------------------------------- Steam DLC

/// The store's DLC list for the game (from its appdetails entry), with names, art and prices (one
/// batched call).
fn steam_dlc(data: &serde_json::Value) -> Vec<Dlc> {
    if data.get("success").and_then(|s| s.as_bool()) != Some(true) {
        return Vec::new();
    }
    let ids: Vec<u32> = data.get("data").and_then(|d| d.get("dlc")).and_then(|d| d.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64().map(|x| x as u32)).collect()).unwrap_or_default();
    if ids.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for chunk in ids.chunks(50) {
        let input = serde_json::json!({
            "ids": chunk.iter().map(|a| serde_json::json!({"appid": a})).collect::<Vec<_>>(),
            "context": {"language": "english", "country_code": "US"},
            "data_request": {"include_assets": true}
        });
        let url = format!("https://api.steampowered.com/IStoreBrowseService/GetItems/v1/?input_json={}", urlencode(&input.to_string()));
        let items = get_json(&url).and_then(|v| v.get("response")?.get("store_items")?.as_array().cloned()).unwrap_or_default();
        for id in chunk {
            let it = items.iter().find(|i| i.get("appid").and_then(|a| a.as_u64()) == Some(*id as u64));
            out.push(dlc_from(*id, it));
        }
    }
    out
}

fn dlc_from(id: u32, it: Option<&serde_json::Value>) -> Dlc {
    let name = it.and_then(|i| i.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
    let mut art = Vec::new();
    if let Some(a) = it.and_then(|i| i.get("assets")) {
        let fmt = a.get("asset_url_format").and_then(|f| f.as_str()).unwrap_or("");
        for k in ["header", "main_capsule", "small_capsule"] {
            if let Some(f) = a.get(k).and_then(|f| f.as_str()).filter(|_| !fmt.is_empty()) {
                art.push(format!("https://shared.fastly.steamstatic.com/store_item_assets/{}", fmt.replace("${FILENAME}", f)));
            }
        }
    }
    art.push(format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{id}/header.jpg"));
    let price = it.and_then(|i| {
        if i.get("is_free").and_then(|f| f.as_bool()) == Some(true) {
            return Some("Free".to_string());
        }
        i.get("best_purchase_option")?.get("formatted_final_price")?.as_str().map(String::from)
    });
    Dlc { appid: id, name: if name.is_empty() { format!("DLC {id}") } else { name }, art, price }
}

fn urlencode(s: &str) -> String {
    let mut o = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            o.push(b as char);
        } else {
            o.push_str(&format!("%{b:02X}"));
        }
    }
    o
}

// ---------------------------------------------------------------- Steam manual

/// Steam's /manual/<appid> answers with a redirect to the manual's file, or to the store's front
/// page when there is none. Some(None): checked, no manual. None: not checked (offline).
fn steam_manual(appid: u32) -> Option<Option<String>> {
    let a: ureq::Agent = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(8))).max_redirects(0).http_status_as_error(false).build().into();
    let r = a.get(&format!("https://store.steampowered.com/manual/{appid}/")).header("User-Agent", "RareLauncher/0.1").call().ok()?;
    let loc = r.headers().get("location").and_then(|l| l.to_str().ok()).unwrap_or("");
    Some(manual_link(loc))
}

fn manual_link(loc: &str) -> Option<String> {
    (loc.contains("/manuals/") || loc.contains("/manual/") && !loc.trim_end_matches('/').ends_with("store.steampowered.com")).then(|| loc.to_string())
}

// ---------------------------------------------------------------- text

/// Steam's BBCode (and the HTML some outlets send) as plain paragraphs: headings, bullets and
/// text; pictures, videos and markup dropped, links kept as their words.
pub fn paragraphs(src: &str) -> Vec<Para> {
    let mut out: Vec<Para> = Vec::new();
    let mut cur = String::new();
    let mut kind = ParaKind::Text;
    let flush = |cur: &mut String, kind: &mut ParaKind, out: &mut Vec<Para>| {
        let t = cur.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            out.push(Para { kind: *kind, text: t });
        }
        cur.clear();
        *kind = ParaKind::Text;
    };
    let b = src.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == b'[' || c == b'<' {
            let close = if c == b'[' { b']' } else { b'>' };
            if let Some(j) = b[i + 1..].iter().take(400).position(|x| *x == close).map(|j| i + 1 + j) {
                let inner = &src[i + 1..j];
                let end = inner.starts_with('/');
                let name: String = inner.trim_start_matches('/').chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '*').collect::<String>().to_ascii_lowercase();
                let known = matches!(
                    name.as_str(),
                    "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "b" | "i" | "u" | "s" | "strike" | "strong" | "em" | "list" | "olist" | "ul" | "ol" | "li" | "*" | "url" | "a" | "img" | "previewyoutube" | "video" | "table" | "tr" | "td" | "th" | "quote" | "code" | "spoiler" | "hr" | "br" | "noparse" | "carousel" | "expand" | "dynamiclink" | "emoticon" | "div" | "span" | "blockquote" | "figure" | "iframe"
                );
                if known {
                    match name.as_str() {
                        "img" | "previewyoutube" | "video" | "dynamiclink" | "iframe" if !end => {
                            // Skip to the closing tag when there is one.
                            let tail = &src[j + 1..];
                            let closing = if c == b'[' { format!("[/{name}]") } else { format!("</{name}>") };
                            i = match tail.to_ascii_lowercase().find(&closing) {
                                Some(k) if c == b'[' || !inner.ends_with('/') => j + 1 + k + closing.len(),
                                _ => j + 1,
                            };
                            continue;
                        }
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                            flush(&mut cur, &mut kind, &mut out);
                            if !end {
                                kind = ParaKind::Head;
                            }
                        }
                        "*" | "li" if !end => {
                            flush(&mut cur, &mut kind, &mut out);
                            kind = ParaKind::Bullet;
                        }
                        "p" | "br" | "list" | "olist" | "ul" | "ol" | "li" | "hr" | "tr" | "table" | "quote" | "div" | "blockquote" => flush(&mut cur, &mut kind, &mut out),
                        "td" | "th" => cur.push(' '),
                        _ => {}
                    }
                    i = j + 1;
                    continue;
                }
            }
        }
        if c == b'{' && src[i..].starts_with("{STEAM_CLAN_IMAGE}") {
            // A picture link: up to the next space or bracket.
            let k = src[i..].find(|ch: char| ch.is_whitespace() || ch == '[').map(|k| i + k).unwrap_or(b.len());
            i = k;
            continue;
        }
        if c == b'\n' {
            if b.get(i + 1) == Some(&b'\n') || kind != ParaKind::Text {
                flush(&mut cur, &mut kind, &mut out);
            } else {
                cur.push(' ');
            }
            i += 1;
            continue;
        }
        let ch = src[i..].chars().next().unwrap_or(' ');
        cur.push(ch);
        i += ch.len_utf8();
    }
    flush(&mut cur, &mut kind, &mut out);
    for p in &mut out {
        p.text = entities(&p.text);
    }
    out.retain(|p| !p.text.trim().is_empty());
    out
}

fn entities(s: &str) -> String {
    s.replace("&nbsp;", " ").replace("&quot;", "\"").replace("&#39;", "'").replace("&apos;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

/// A kept feed label for the screen: the labels Rare itself makes are English words in the kept
/// answer, shown in the player's language; a feed's own label stays as the feed gave it.
pub fn feed_text(feed: &str) -> String {
    match feed {
        "Patch notes" => rare_i18n::tr().game.feed_patch_notes.into(),
        "Announcement" => rare_i18n::tr().game.feed_announcement.into(),
        "News" => rare_i18n::tr().game.news.into(),
        "Update" => rare_i18n::tr().game.feed_update.into(),
        _ => feed.to_string(),
    }
}

/// "Oct 21, 2026".
pub fn date_label(t: i64) -> String {
    if t <= 0 {
        return String::new();
    }
    let (y, m, d) = crate::screens::wallet::fmt::civil(t.div_euclid(86400));
    rare_i18n::fmt::date_ymd(y as i32, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbcode_to_paragraphs() {
        let p = paragraphs("[h2]Patch 1.2[/h2][p]Fixed a crash.[/p][list][*]Faster [b]loading[/b][*]New [url=https://x.y]map[/url][/list][img]{STEAM_CLAN_IMAGE}/1/a.png[/img]\n\nThanks &amp; see you");
        let kinds: Vec<ParaKind> = p.iter().map(|x| x.kind).collect();
        assert_eq!(kinds, vec![ParaKind::Head, ParaKind::Text, ParaKind::Bullet, ParaKind::Bullet, ParaKind::Text]);
        assert_eq!(p[3].text, "New map");
        assert_eq!(p[4].text, "Thanks & see you");
        // Square brackets that are not markup stay.
        assert_eq!(paragraphs("[Fixed] the jump")[0].text, "[Fixed] the jump");
    }

    #[test]
    fn manual_links_and_dates() {
        assert!(manual_link("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/292030/manuals/W3.pdf?t=1").is_some());
        assert!(manual_link("https://store.steampowered.com/").is_none());
        assert_eq!(parse_iso("2026-10-27T16:18:20.082Z"), Some(1_793_117_880));
        assert_eq!(date_label(1_793_117_880), "Oct 27, 2026");
    }

    #[test]
    fn store_shots_and_names() {
        let ok = serde_json::json!({"success": true, "data": {"screenshots": [
            {"id": 0, "path_thumbnail": "https://x/ss_1.600x338.jpg", "path_full": "https://x/ss_1.1920x1080.jpg"},
            {"id": 1, "path_full": "https://x/ss_2.jpg"},
            {"id": 2}
        ]}});
        let shots = parse_shots(&ok);
        assert_eq!(shots.len(), 2);
        assert_eq!(shots[0], StoreShot { thumb: "https://x/ss_1.600x338.jpg".into(), full: "https://x/ss_1.1920x1080.jpg".into() });
        assert_eq!(shots[1].thumb, "https://x/ss_2.jpg");
        assert!(parse_shots(&serde_json::json!({"success": false})).is_empty());
        assert!(same_name("APE OUT", "Ape Out"));
        assert!(same_name(">observer_", "Observer"));
        assert!(!same_name("Batman: Arkham City", "Batman: Arkham Knight"));
        assert!(!same_name("", ""));
        assert_eq!(base_name("Batman\u{2122} Arkham City - Game of the Year Edition").as_deref(), Some("Batman Arkham City"));
        assert_eq!(base_name("BioShock Infinite: Complete Edition").as_deref(), Some("BioShock Infinite"));
        assert_eq!(base_name("Darksiders II Deathinitive Edition").as_deref(), Some("Darksiders II"));
        assert_eq!(base_name("Dandara: Trials of Fear Edition").as_deref(), Some("Dandara"));
        assert_eq!(base_name("The Outer Worlds: Spacer's Choice Edition").as_deref(), Some("The Outer Worlds"));
        // No edition, or a subtitle that is part of the name: nothing to drop.
        assert_eq!(base_name("DREDGE"), None);
        assert_eq!(base_name("A Plague Tale: Innocence"), None);
        assert_eq!(base_name("Death Stranding Director's Cut").as_deref(), None);
    }

    #[test]
    fn kept_answers_without_store_shots_still_read() {
        let old = serde_json::json!({"at": 1, "news": [], "dlc": [], "manual": null});
        let r: Remote = serde_json::from_value(old).unwrap();
        assert!(r.store_shots.is_empty());
    }

    #[test]
    fn steam_news_parses() {
        let v = serde_json::json!({"appnews": {"newsitems": [{"gid": "1", "title": "Update 3", "url": "https://x", "contents": "[p]Hi[/p]", "feedname": "steam_community_announcements", "feedlabel": "Community Announcements", "date": 1, "tags": ["patchnotes"]}]}});
        let n = parse_steam_news(&v).unwrap();
        assert_eq!(n[0].feed, "Patch notes");
        assert_eq!(n[0].body[0].text, "Hi");
    }
}
