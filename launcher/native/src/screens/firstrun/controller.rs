//! First run: check the controller. Each button lights up here when it is pressed, so a button that
//! does not answer shows before a game needs it. A (Continue) and B (Back) are proven by using them;
//! the d-pad lights and still moves the highlight; the sticks move it too.

use rare_i18n::{tr, trf};
use super::kit::step;
use crate::screens::wallet::markup::*;
use crate::screens::wallet::ui::{button_el, BSize, ButtonSpec, BVariant};

/// The buttons the check lights (keys).
pub const BUTTONS: [&str; 12] = ["x", "y", "lb", "rb", "lt", "rt", "select", "start", "up", "down", "left", "right"];

/// The label a check button shows.
fn label(key: &str) -> &'static str {
    match key {
        "x" => "X",
        "y" => "Y",
        "lb" => "LB",
        "rb" => "RB",
        "lt" => "LT",
        "rt" => "RT",
        "select" => tr().common.view,
        "start" => tr().common.menu,
        "up" => tr().firstrun.pad_up,
        "down" => tr().firstrun.pad_down,
        "left" => tr().firstrun.pad_left,
        _ => tr().firstrun.pad_right,
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub lit: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Continue,
    Back,
}

/// A press reached the step: light it. True when it was one of the check's buttons.
pub fn press(st: &mut State, key: &str) -> bool {
    let Some(k) = BUTTONS.iter().find(|k| **k == key) else { return false };
    if !st.lit.contains(k) {
        st.lit.push(k);
    }
    true
}

fn chip<M>(label: &str, on: bool) -> El<M> {
    span(cn(&["flex h-[3rem] min-w-[4.5rem] items-center justify-center rounded-card border px-[1rem] text-body font-semibold transition-colors duration-150", if on { "border-fg bg-fg text-bg" } else { "border-hair bg-l1 text-fg/70" }])).key(label).text(label)
}

pub fn view(st: &State) -> El<Msg> {
    let n = st.lit.len();
    let all = n == BUTTONS.len();
    let row = |keys: &[&str]| div("flex flex-wrap gap-[0.75rem]").kids(BUTTONS.iter().filter(|k| keys.contains(k)).map(|k| chip(label(k), st.lit.contains(k))));
    let content = vec![
        div("flex flex-col gap-[0.75rem]").attr("aria-live", "polite").kid(row(&["x", "y", "lb", "rb", "lt", "rt"])).kid(row(&["select", "start", "up", "down", "left", "right"])),
        div("mt-[1.1rem] text-small text-muted").text(if all {
            tr().firstrun.every_button_works.to_string()
        } else {
            trf!(tr().firstrun.buttons_pressed, n = n, total = BUTTONS.len())
        }),
    ];
    let actions = vec![button_el(ButtonSpec::new(tr().common.continue_).v(BVariant::Primary).size(BSize::Lg).def(true).class("min-w-[11rem]"), Some(Msg::Continue))];
    step(text(tr().firstrun.check_controller_title), Some(text(tr().firstrun.check_controller_body)), content, actions, false).on_pad("b", Msg::Back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut st = State::default();
        assert!(press(&mut st, "x"));
        assert!(press(&mut st, "x"));
        assert!(!press(&mut st, "a"));
        assert_eq!(st.lit, ["x"]);
        let v = view(&st);
        assert_eq!(v.texts()[0], "Check your controller");
        assert!(v.contains_text("1 of 12 pressed. The sticks move the highlight, so smooth moves mean they work. A and B work if they got you here."));
        for k in BUTTONS {
            press(&mut st, k);
        }
        assert!(view(&st).contains_text("Every button works."));
        assert_eq!(view(&st).landing().as_deref(), Some("Continue"));
    }
}
