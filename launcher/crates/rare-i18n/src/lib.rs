//! Rare's UI strings, in every language Rare ships.
//!
//! Catalogs are Rust consts compiled into the binary, so a lookup is a field read with nothing
//! parsed or hashed at run time: `tr().home.play`.
//!
//! - `src/en/<area>.rs` defines an area's keys and their English text with `strings!`.
//! - `src/<lang>/<area>.rs` gives that language's text with `translate!`. A key it leaves out shows
//!   in English; the completeness test names every such key.
//! - Placeholders are `{name}`: `trf!(tr().library.in_library, n = count)`.
//! - Plurals are forms joined by `|`, in the language's CLDR order: English, Spanish and Portuguese
//!   `one|other`, Russian `one|few|many`, Chinese, Japanese and Korean one form.
//!   `trn!(tr().library.games, count)` picks the form and fills `{n}` (and any other names given).
//!
//! The language is the saved one (Settings > System > Language), else the system's (`LANG`), else
//! English. Until `init`/`set` is called it is English, so tests read English.

pub mod fmt;

use std::fmt::{Display, Write as _};
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

/// A language Rare ships, in the order the language picker lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Lang {
    En,
    Zh,
    Es,
    Ja,
    Ru,
    PtBr,
    Ko,
}

impl Lang {
    pub const ALL: [Lang; 7] = [Lang::En, Lang::Zh, Lang::Es, Lang::Ja, Lang::Ru, Lang::PtBr, Lang::Ko];

    /// The code saved in the config (`language`): BCP 47.
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Zh => "zh",
            Lang::Es => "es",
            Lang::Ja => "ja",
            Lang::Ru => "ru",
            Lang::PtBr => "pt-BR",
            Lang::Ko => "ko",
        }
    }

    /// The language's name in itself, as the picker shows it.
    pub fn name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Zh => "简体中文",
            Lang::Es => "Español",
            Lang::Ja => "日本語",
            Lang::Ru => "Русский",
            Lang::PtBr => "Português (Brasil)",
            Lang::Ko => "한국어",
        }
    }

    /// The POSIX locale for apps (`LANG`).
    pub fn posix(self) -> &'static str {
        match self {
            Lang::En => "en_US.UTF-8",
            Lang::Zh => "zh_CN.UTF-8",
            Lang::Es => "es_ES.UTF-8",
            Lang::Ja => "ja_JP.UTF-8",
            Lang::Ru => "ru_RU.UTF-8",
            Lang::PtBr => "pt_BR.UTF-8",
            Lang::Ko => "ko_KR.UTF-8",
        }
    }

    /// The locale text shaping picks CJK faces by (Han characters differ between Chinese,
    /// Japanese and Korean fonts).
    pub fn font_locale(self) -> &'static str {
        match self {
            Lang::En => "en-US",
            Lang::Zh => "zh-CN",
            Lang::Es => "es",
            Lang::Ja => "ja",
            Lang::Ru => "ru",
            Lang::PtBr => "pt-BR",
            Lang::Ko => "ko",
        }
    }

    /// `ru`, `pt-BR`, `pt_BR.UTF-8`, `zh_CN`, `zh-Hans`, `ja_JP.utf8@euro`... `C` and `POSIX` are
    /// no language.
    pub fn from_code(s: &str) -> Option<Lang> {
        let s = s.trim().split(['.', '@']).next()?.replace('_', "-").to_ascii_lowercase();
        let lang = s.split('-').next()?;
        Some(match lang {
            "en" => Lang::En,
            "zh" => Lang::Zh,
            "es" => Lang::Es,
            "ja" => Lang::Ja,
            "ru" => Lang::Ru,
            "pt" => Lang::PtBr,
            "ko" => Lang::Ko,
            _ => return None,
        })
    }

    /// The system's language: `LANGUAGE` (a list), then `LC_ALL`, `LC_MESSAGES`, `LANG`, the way
    /// gettext reads them.
    pub fn from_env() -> Option<Lang> {
        let get = |k: &str| std::env::var(k).ok().filter(|v| !v.trim().is_empty());
        let locale = get("LC_ALL").or_else(|| get("LC_MESSAGES")).or_else(|| get("LANG"));
        let is_c = locale.as_deref().map_or(true, |l| matches!(l.split('.').next(), Some("C" | "POSIX")));
        if !is_c {
            if let Some(list) = get("LANGUAGE") {
                if let Some(l) = list.split(':').find_map(Lang::from_code) {
                    return Some(l);
                }
            }
        }
        locale.as_deref().and_then(Lang::from_code)
    }

    /// How many plural forms the catalogs give for this language.
    pub fn plural_forms(self) -> usize {
        match self {
            Lang::En | Lang::Es | Lang::PtBr => 2,
            Lang::Ru => 3,
            Lang::Zh | Lang::Ja | Lang::Ko => 1,
        }
    }

    /// Which plural form `n` takes (CLDR cardinal rules, integers; fractions take the form
    /// fractions use).
    pub fn plural_index(self, n: f64) -> usize {
        let whole = n.fract() == 0.0;
        let i = n.abs().trunc() as u64;
        match self {
            // Portuguese too: Brazilian UIs say "0 jogos" (CLDR's pt puts 0 with "one").
            Lang::En | Lang::Es | Lang::PtBr => usize::from(!(whole && i == 1)),
            Lang::Ru => {
                if !whole {
                    return 1;
                }
                let (m10, m100) = (i % 10, i % 100);
                if m10 == 1 && m100 != 11 {
                    0
                } else if (2..=4).contains(&m10) && !(12..=14).contains(&m100) {
                    1
                } else {
                    2
                }
            }
            Lang::Zh | Lang::Ja | Lang::Ko => 0,
        }
    }
}

static CUR: AtomicU8 = AtomicU8::new(0);
static GEN: AtomicU32 = AtomicU32::new(0);

/// The strings of the language in use.
#[inline]
pub fn tr() -> &'static Strings {
    TABLES[CUR.load(Ordering::Relaxed) as usize]
}

/// The strings of `l`.
#[inline]
pub fn tr_in(l: Lang) -> &'static Strings {
    TABLES[l as usize]
}

/// The language in use.
pub fn lang() -> Lang {
    Lang::ALL[CUR.load(Ordering::Relaxed) as usize]
}

/// Switches the language. Text already built stays as it was: the UI rebuilds after this.
pub fn set(l: Lang) {
    if CUR.swap(l as u8, Ordering::Relaxed) != l as u8 {
        GEN.fetch_add(1, Ordering::Relaxed);
    }
}

/// Bumps on every language change (for caches of translated text).
pub fn generation() -> u32 {
    GEN.load(Ordering::Relaxed)
}

/// The language to use: the saved one, else the system's, else English. `RR_LANG` overrides
/// both (screenshots).
pub fn resolve(saved: Option<&str>) -> Lang {
    std::env::var("RR_LANG")
        .ok()
        .and_then(|v| Lang::from_code(&v))
        .or_else(|| saved.and_then(Lang::from_code))
        .or_else(Lang::from_env)
        .unwrap_or(Lang::En)
}

/// `resolve` and `set`.
pub fn init(saved: Option<&str>) -> Lang {
    let l = resolve(saved);
    set(l);
    l
}

/// `{name}` placeholders filled from `args`; unknown names stay as written.
pub fn fill(s: &str, args: &[(&str, &dyn Display)]) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    let mut rest = s;
    while let Some(i) = rest.find('{') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        if let Some(j) = after.find('}') {
            let name = &after[..j];
            if let Some((_, v)) = args.iter().find(|(k, _)| *k == name) {
                let _ = write!(out, "{v}");
                rest = &after[j + 1..];
                continue;
            }
        }
        out.push('{');
        rest = after;
    }
    out.push_str(rest);
    out
}

/// The form of a plural string (`one|other`...) that `n` takes in the language in use.
pub fn plural(s: &'static str, n: f64) -> &'static str {
    plural_in(lang(), s, n)
}

pub fn plural_in(l: Lang, s: &'static str, n: f64) -> &'static str {
    if !s.contains('|') {
        return s;
    }
    let forms: Vec<&'static str> = s.split('|').collect();
    // A key the language does not give yet falls back to English, which may have fewer forms.
    let i = if forms.len() == l.plural_forms() { l.plural_index(n) } else { Lang::En.plural_index(n) };
    forms[i.min(forms.len() - 1)]
}

/// `trf!(tr().area.key, name = value, ...)`: the string with its `{name}` placeholders filled.
#[macro_export]
macro_rules! trf {
    ($s:expr $(, $k:ident = $v:expr)* $(,)?) => {
        $crate::fill($s, &[$((stringify!($k), &$v as &dyn ::std::fmt::Display)),*])
    };
}

/// `trn!(tr().area.key, count, name = value, ...)`: the plural form `count` takes, with `{n}` set
/// to `count` (give `n = ...` to show it formatted) and the other placeholders filled.
#[macro_export]
macro_rules! trn {
    ($s:expr, $n:expr $(, $k:ident = $v:expr)* $(,)?) => {{
        let n_ = $n;
        $crate::fill($crate::plural($s, n_ as f64), &[$((stringify!($k), &$v as &dyn ::std::fmt::Display),)* ("n", &n_ as &dyn ::std::fmt::Display)])
    }};
}

/// An area's keys and English text: `strings! { /// context for translators\n key: "Text", ... }`.
macro_rules! strings {
    ($($(#[$m:meta])* $key:ident: $val:literal),* $(,)?) => {
        #[derive(Clone, Copy, Debug)]
        pub struct S {
            $($(#[$m])* pub $key: &'static str,)*
        }
        impl S {
            pub const EN: S = S { $($key: $val,)* };
            pub const KEYS: &'static [&'static str] = &[$(stringify!($key),)*];
            pub fn values(&self) -> Vec<&'static str> {
                vec![$(self.$key,)*]
            }
        }
    };
}

/// One language's text for an area: `translate! { area { key: "Text", ... } }`. Keys left out
/// show in English.
macro_rules! translate {
    ($area:ident { $($key:ident: $val:literal),* $(,)? }) => {
        pub const T: crate::en::$area::S = crate::en::$area::S { $($key: $val,)* ..crate::en::$area::S::EN };
        pub const KEYS: &[&str] = &[$(stringify!($key),)*];
    };
}

/// An area's strings in one language, for the tests.
pub struct Area {
    pub name: &'static str,
    /// Every key, in English's order.
    pub keys: &'static [&'static str],
    /// The text of each key in this language (English where it gives none).
    pub values: Vec<&'static str>,
    /// The keys this language gives.
    pub given: &'static [&'static str],
}

macro_rules! catalog {
    ($($area:ident),* $(,)?) => {
        pub mod en { $(pub mod $area;)* }
        pub mod zh { $(pub mod $area;)* }
        pub mod es { $(pub mod $area;)* }
        pub mod ja { $(pub mod $area;)* }
        pub mod ru { $(pub mod $area;)* }
        pub mod pt_br { $(pub mod $area;)* }
        pub mod ko { $(pub mod $area;)* }

        /// Every UI string, one field per area of the UI.
        pub struct Strings {
            $(pub $area: en::$area::S,)*
        }
        pub static EN: Strings = Strings { $($area: en::$area::S::EN,)* };
        pub static ZH: Strings = Strings { $($area: zh::$area::T,)* };
        pub static ES: Strings = Strings { $($area: es::$area::T,)* };
        pub static JA: Strings = Strings { $($area: ja::$area::T,)* };
        pub static RU: Strings = Strings { $($area: ru::$area::T,)* };
        pub static PT_BR: Strings = Strings { $($area: pt_br::$area::T,)* };
        pub static KO: Strings = Strings { $($area: ko::$area::T,)* };

        /// Every area in `l`.
        pub fn areas(l: Lang) -> Vec<Area> {
            let s = tr_in(l);
            vec![$(Area {
                name: stringify!($area),
                keys: en::$area::S::KEYS,
                values: s.$area.values(),
                given: match l {
                    Lang::En => en::$area::S::KEYS,
                    Lang::Zh => zh::$area::KEYS,
                    Lang::Es => es::$area::KEYS,
                    Lang::Ja => ja::$area::KEYS,
                    Lang::Ru => ru::$area::KEYS,
                    Lang::PtBr => pt_br::$area::KEYS,
                    Lang::Ko => ko::$area::KEYS,
                },
            },)*]
        }
    };
}

static TABLES: [&Strings; 7] = [&EN, &ZH, &ES, &JA, &RU, &PT_BR, &KO];

// One area per part of the UI. A new area: add it here and add `src/<lang>/<area>.rs` for every
// language (`translate! { area {} }` until it is translated).
catalog! {
    common,
    app,
    data,
    kit,
    home,
    library,
    search,
    osk,
    quick,
    shell,
    firstrun,
    perks,
    game,
    emulators,
    signin,
    apps,
    store,
    share,
    friends,
    stream,
    sui,
    market,
    trade,
    overlay,
    trophies,
    inventory,
    notify,
    settings,
    settings_more,
    features,
    features_more,
    wallet,
    wallet_setup,
    wallet_sign,
}

#[cfg(test)]
mod tests;
