//! The System section (screens/settings/SystemSections.tsx SystemSection): update rows, language,
//! Desktop mode.

use super::update::{fixture_info, UpdateRows, FIXTURE_NOW_MS};
use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::screens::overlay::common::*;
use rare_render::Id;
use rare_i18n::tr;

/// The name of the saved language in itself ("Русский"); the language in use when none is saved.
pub fn language_name(code: Option<&str>) -> &'static str {
    code.and_then(rare_i18n::Lang::from_code).unwrap_or_else(rare_i18n::lang).name()
}

pub struct SystemSection {
    update: UpdateRows,
    language_btn: Option<Id>,
    desktop_btn: Id,
    rows: RowSet,
    support: super::support::SupportRows,
}

impl SystemSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, _now: f64) -> SystemSection {
        let (info, now_ms) = match super::live::get() {
            Some(s) => (s.system, crate::app::wall_ms()),
            None => (fixture_info(), FIXTURE_NOW_MS),
        };
        let update = UpdateRows::build(cx, nav, parent, info, now_ms);
        let p = panel(cx, parent);
        let t = &tr().settings_more;
        let row_ = setting_row(cx, p, t.language, Some(t.language_body), true);
        let name = language_name(cfg.language.as_deref());
        let b = button(cx, row_.right, name, Variant::Secondary, Sz::Md, false).sid("language").tag(10);
        let language_btn = Some(nav.add(b.group(10)));
        let row_ = setting_row(cx, p, t.desktop_mode, Some(t.desktop_mode_body), false);
        let b = button(cx, row_.right, t.switch, Variant::Secondary, Sz::Md, false).sid("desktop-mode").tag(11);
        let desktop_btn = nav.add(b.group(10));
        let info = super::live::get().map(|s| s.ssh).unwrap_or_else(super::remote::fixture_default);
        let mut rows = RowSet::default();
        super::remote::build_system(cx, nav, parent, &mut rows, &info);
        let support = super::support::SupportRows::build(cx, nav, parent);
        SystemSection { update, language_btn, desktop_btn, rows, support }
    }
}

impl SectionView for SystemSection {
    fn press(&mut self, cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, now: f64) -> SectionOut {
        if self.update.press_details(cx, nav) {
            return SectionOut::None;
        }
        if let Some(a) = self.update.press(nav) {
            return SectionOut::Act(a.into());
        }
        if let Some(f) = nav.current().cloned() {
            if let Some(out) = super::remote::press(&mut self.rows, &[], cx, nav, &f, now) {
                return out;
            }
        }
        if let Some(out) = self.support.press(cx, nav, now) {
            return out;
        }
        let Some(cur) = nav.cur else { return SectionOut::None };
        if Some(cur) == self.language_btn {
            // The language list (supportsheets::LangSheet); picking one switches the whole UI.
            return SectionOut::Act("support:lang-open".into());
        }
        if cur == self.desktop_btn {
            return SectionOut::Act("power:desktop".into());
        }
        SectionOut::None
    }
}
