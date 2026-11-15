//! First run: the time zone. The installer leaves every new device on UTC, so the clock is wrong
//! for nearly everyone until a zone is picked. One press opens Settings' own picker (region, then
//! city); picking one lands back here on Continue.

use rare_i18n::{tr, trf};
use super::kit::{done_line, step};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::ui::{button_el, BSize, ButtonSpec, BVariant};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Pick,
    Continue,
    Back,
}

/// "America/New_York" -> "New York (America)", "UTC" stays.
pub fn zone_label(zone: Option<&str>) -> String {
    match zone {
        None | Some("UTC") | Some("Etc/UTC") => "UTC".into(),
        Some(z) => {
            let region = rare_system::support::region_of(z);
            format!("{} ({region})", rare_system::support::city_name(z))
        }
    }
}

/// `zone`: the system's zone now; `clock`: the time it shows; `picked`: a zone was picked here.
pub fn view(zone: Option<&str>, clock: &str, picked: bool) -> El<Msg> {
    let utc = matches!(zone, None | Some("UTC") | Some("Etc/UTC"));
    let line = if picked || !utc {
        done_line(vec![span("").text(trf!(tr().firstrun.zone_now, zone = zone_label(zone), clock = clock))])
    } else {
        div("text-body text-muted").text(trf!(tr().firstrun.utc_for_now, clock = clock))
    };
    let pick_first = utc && !picked;
    let actions = vec![
        button_el(ButtonSpec::new(if pick_first { tr().firstrun.pick_time_zone } else { tr().common.continue_ }).v(BVariant::Primary).size(BSize::Lg).def(true).class("min-w-[11rem]"), Some(if pick_first { Msg::Pick } else { Msg::Continue })),
        button_el(ButtonSpec::new(if pick_first { tr().firstrun.keep_utc } else { tr().common.change }).v(BVariant::Ghost).size(BSize::Lg), Some(if pick_first { Msg::Continue } else { Msg::Pick })),
    ];
    step(text(tr().firstrun.time_zone_title), Some(text(tr().firstrun.time_zone_body)), vec![line], actions, false).on_pad("b", Msg::Back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time() {
        let v = view(Some("UTC"), "15:30", false);
        assert_eq!(v.texts()[..3], ["Time zone", "So the clock, sleep timers and store sale ends show your time.", "Set to UTC for now, so the clock says 15:30."]);
        assert_eq!(v.landing().as_deref(), Some("Pick time zone"));
        let v = view(Some("America/New_York"), "11:30", true);
        assert!(v.contains_text("New York (America)"));
        assert_eq!(v.landing().as_deref(), Some("Continue"));
        assert_eq!(v.pad("b"), Some(&Msg::Back));
        assert_eq!(zone_label(Some("America/Argentina/Buenos_Aires")), "Buenos Aires, Argentina (America)");
    }
}
