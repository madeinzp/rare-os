//! The Parental controls section: set, change or turn off the parent's PIN, and the current
//! profile's rules (daily play time, allowed hours, age rating). The rules live in
//! `features::parental`, not the launcher config; a change there moves its version and Settings
//! redraws the section.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::features::parental::{self as pc, Rules};
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use crate::state::{State, Tone};
use rare_i18n::{tr, trf};
use rare_render::theme::tok;
use rare_render::{Id, Node};

const SID_PIN: &str = "parental-pin";

pub struct ParentalSection {
    rows: RowSet,
    rules: Rules,
}

impl ParentalSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, _cfg: &Config, _now: f64) -> ParentalSection {
        // Shots: RR_PARENTAL=1 shows the section with a PIN and rules.
        if std::env::var("RR_FIXTURE").is_ok() && std::env::var("RR_PARENTAL").is_ok() {
            return ParentalSection::build_with(cx, nav, parent, true, "rare.sui", Rules { daily_min: Some(60), hours: Some((8, 20)), max_age: Some(12) }, 45 * 60);
        }
        let pin = pc::pin_set();
        let (_, who) = pc::profile();
        ParentalSection::build_with(cx, nav, parent, pin, &who, pc::rules(), pc::played_today())
    }

    pub fn build_with(cx: &mut Cx, nav: &mut Nav, parent: Id, pin: bool, who: &str, rules: Rules, played: u64) -> ParentalSection {
        let r = cx.r();
        let t = &tr().settings_more;
        let mut rows = RowSet::default();
        let p = panel(cx, parent);
        if !pin {
            let row = setting_row(cx, p, t.pc_title, Some(t.pc_body), true);
            nav.add(button(cx, row.right, t.pc_set_pin, Variant::Primary, Sz::Md, false).sid(SID_PIN).tag(1).group(10));
            return ParentalSection { rows, rules };
        }
        let row = setting_row(cx, p, t.pc_pin, Some(t.pc_pin_body), true);
        nav.add(button(cx, row.right, t.pc_change_pin, Variant::Secondary, Sz::Md, false).sid(SID_PIN).tag(2).group(10));
        nav.add(button(cx, row.right, tr().common.turn_off, Variant::Secondary, Sz::Md, false).sid(SID_PIN).tag(3).group(10));

        let lab = cx.scene.add(parent, s().pad(0.25 * r, 0.0, 0.0, 0.5 * r).done(), Node::default());
        text(cx, lab, ty::LABEL, 500, tok::MUTED, &trf!(t.pc_rules_for, name = who));
        let p = panel(cx, parent);
        let row = setting_row(cx, p, t.pc_played_today, None, true);
        text(cx, row.right, ty::BODY, 500, tok::FG, &pc::played_text(played));
        let daily = pc::DAILY.iter().position(|d| *d == rules.daily_min);
        rows.choice(cx, nav, p, "parental-daily", t.pc_daily, Some(t.pc_daily_body), &pc::daily_labels().iter().map(|s| s.as_str()).collect::<Vec<_>>(), daily, false);
        rows.choice(cx, nav, p, "parental-hours", t.pc_hours, Some(t.pc_hours_body), &[t.pc_any_time, t.pc_set_hours], Some(rules.hours.is_some() as usize), false);
        if let Some((from, to)) = rules.hours {
            rows.slider(cx, nav, p, "parental-from", t.pc_from, None, from as f32, 0.0, 23.0, 1.0, ":00", false);
            rows.slider(cx, nav, p, "parental-to", t.pc_to, None, to as f32, 1.0, 24.0, 1.0, ":00", false);
        }
        let age = pc::AGES.iter().position(|a| *a == rules.max_age);
        rows.choice(cx, nav, p, "parental-age", t.pc_age, Some(t.pc_age_body), &pc::age_labels().iter().map(|s| s.as_str()).collect::<Vec<_>>(), age, false);
        ParentalSection { rows, rules }
    }

    fn locked(now: f64) -> Option<SectionOut> {
        pc::section_locked("parental", now).then(|| SectionOut::Act("parental:unlock:parental".into()))
    }
}

impl SectionView for ParentalSection {
    fn press(&mut self, cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, now: f64) -> SectionOut {
        if let Some(o) = Self::locked(now) {
            return o;
        }
        let Some(f) = nav.current().cloned() else { return SectionOut::None };
        if f.sid == SID_PIN {
            return SectionOut::Act(if f.tag == 3 { "parental:off" } else { "parental:set" }.into());
        }
        let Some((sid, Val::Idx(i))) = self.rows.press(cx, nav, now) else { return SectionOut::None };
        let mut r = self.rules.clone();
        match sid.as_str() {
            "parental-daily" => r.daily_min = pc::DAILY.get(i).copied().flatten(),
            "parental-hours" => r.hours = if i == 0 { None } else { Some(self.rules.hours.unwrap_or((8, 20))) },
            "parental-age" => r.max_age = pc::AGES.get(i).copied().flatten(),
            _ => return SectionOut::None,
        }
        self.rules = r.clone();
        pc::set_rules(r);
        SectionOut::None
    }

    fn nudge(&mut self, cx: &mut Cx, nav: &mut Nav, _cfg: &mut Config, dir: i32, now: f64) -> SectionOut {
        let on_slider = nav.current().is_some_and(|f| self.rows.is_slider(&f.sid));
        if !on_slider {
            return SectionOut::Pass;
        }
        if let Some(o) = Self::locked(now) {
            return o;
        }
        let Some((sid, Val::Num(v))) = self.rows.nudge(cx, nav, dir) else { return SectionOut::None };
        let (mut from, mut to) = self.rules.hours.unwrap_or((8, 20));
        match sid.as_str() {
            "parental-from" => from = v as u8,
            _ => to = v as u8,
        }
        self.rules.hours = Some((from, to));
        pc::set_rules(self.rules.clone());
        SectionOut::None
    }
}

/// What the section's buttons and the lock ask of the app (`parental:*`).
pub fn act(st: &mut State, a: &str, now: f64) {
    match a {
        "parental:set" => {
            let _ = pc::ask_set();
        }
        "parental:off" => {
            pc::clear_pin();
            st.toast(tr().settings_more.pc_off_toast, Tone::Info, now);
        }
        _ => {
            if let Some(key) = a.strip_prefix("parental:unlock:") {
                let label = super::sections::SectionId::from_key(key).map(|s| s.label()).unwrap_or("these");
                let _ = pc::ask(tr().settings_more.pc_parent_pin, &trf!(tr().settings_more.pc_enter_pin_to_open, label = label), now);
            }
        }
    }
}
