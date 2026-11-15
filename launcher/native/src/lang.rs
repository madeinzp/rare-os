//! The UI's language (rare-i18n): every user-visible string is `tr().<area>.<key>`, with `trf!` for
//! placeholders and `trn!` for plurals. Set once at start (the saved language, else the system's,
//! else English) and switched live from Settings > System > Language or first run: the app rebuilds
//! in the new language, the way a UI size change does.

pub use rare_i18n::{tr, trf, trn, Lang};

/// At start: the saved language, else the system's (`LANG`), else English. `RR_LANG` overrides.
pub fn init() -> Lang {
    rare_i18n::init(crate::screens::settings::config::load().language.as_deref())
}

/// Puts the language in use on what is not a catalog string: the CJK faces text shaping picks
/// (Chinese, Japanese and Korean draw some Han characters differently) and the hint bar's own words.
pub fn apply(text: &mut rare_render::TextSys) {
    text.set_locale(rare_i18n::lang().font_locale());
    let c = &tr().common;
    rare_render::focus::hints::set_words(rare_render::focus::hints::Words { select: c.select, back: c.back, options: c.options, menu: c.menu, hold_view_menu: c.hold_view_menu });
}

static SWITCHED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A language picked in Settings or first run (the caller saves it): the app is built again in it
/// on its next update, on the tab it was on.
pub fn switch(l: Lang) {
    if l != rare_i18n::lang() {
        rare_i18n::set(l);
        SWITCHED.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// True once after `switch`: the app rebuilds.
pub fn take_switched() -> bool {
    SWITCHED.swap(false, std::sync::atomic::Ordering::Relaxed)
}
