//! The parent's PIN sheet (`features::parental`), in the wallet's sheet style with the kit's number
//! pad: enter the PIN (a launch, an approval, a locked setting), or pick a new one twice. Drawn by
//! the review host (`screens::wallet::sign::host`), which sits over every sheet.

use super::parental::{self as pc, Ask};
use crate::screens::wallet::kit::{btn, pin_pad, pin_press, sheet, PinPadSpec, SheetSpec, Variant};
use crate::screens::wallet::markup::*;
use crate::screens::wallet::sign::unlock::wait_text;
use crate::state::{State, Tone};
use rare_i18n::{tr, trf, trn};

fn pin_title(had_pin: bool) -> String {
    if had_pin { tr().features_more.pin_change_title } else { tr().features_more.pin_set_title }.to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Key(&'static str),
    Cancel,
}

enum Step {
    Check,
    New,
    Confirm(String),
}

pub struct Sheet {
    ask: Ask,
    step: Step,
    value: String,
    error: Option<String>,
    wait_until: f64,
    had_pin: bool,
}

impl Sheet {
    pub fn new(ask: Ask) -> Sheet {
        let step = if matches!(ask, Ask::Set { .. }) { Step::New } else { Step::Check };
        Sheet { ask, step, value: String::new(), error: None, wait_until: 0.0, had_pin: pc::pin_set() }
    }

    /// True while a wait counts down (the view redraws every second).
    pub fn waiting(&self, now: f64) -> bool {
        now < self.wait_until
    }

    pub fn view(&self, now: f64) -> El<Msg> {
        let (title, reason) = match (&self.ask, &self.step) {
            (Ask::Check { title, reason, .. }, _) => (title.clone(), reason.clone()),
            (_, Step::Confirm(_)) => (pin_title(self.had_pin), tr().features_more.pin_enter_again.to_string()),
            _ => (pin_title(self.had_pin), tr().features_more.pin_pick_digits.to_string()),
        };
        let waiting = self.waiting(now);
        let pad = pin_pad(PinPadSpec { value: &self.value, min_length: pc::PIN_MIN, disabled: waiting, ..Default::default() }, Msg::Key).on_pad("x", Msg::Key("back")).on_pad("start", Msg::Key("ok"));
        let status = if waiting {
            Some(span("text-secondary").text(trf!(tr().features_more.pin_too_many, wait = wait_text(((self.wait_until - now) * 1000.0) as u64))))
        } else {
            self.error.as_ref().map(|e| span("text-negative").text(e))
        };
        sheet(
            SheetSpec { title, subtitle: None, icon: None, footer: Some(vec![btn(tr().common.cancel, Variant::Secondary, "flex-1", Some(Msg::Cancel))]), close: Some(Msg::Cancel), width: "28rem" },
            vec![div("flex flex-col items-center gap-5").kid(p("text-center text-base text-secondary").text(&reason)).kid(pad).kid(p("h-5 text-center text-sm").role("status").kid_opt(status))],
        )
    }

    fn answer(&self, ok: bool) {
        match &self.ask {
            Ask::Check { tx, .. } | Ask::Set { tx } => {
                let _ = tx.send(ok);
            }
        }
    }

    /// Applies a press. True when the sheet is done (answered).
    pub fn update(&mut self, st: &mut State, m: Msg, now: f64) -> bool {
        let k = match m {
            Msg::Cancel => {
                self.answer(false);
                return true;
            }
            Msg::Key(k) => k,
        };
        if self.waiting(now) {
            return false;
        }
        let (next, go) = pin_press(&self.value, k, None, pc::PIN_MIN, pc::PIN_MAX);
        self.value = next;
        if !go {
            return false;
        }
        let pin = std::mem::take(&mut self.value);
        self.error = None;
        match &self.step {
            Step::Check => match pc::try_pin(&pin, now) {
                Ok(true) => {
                    self.answer(true);
                    return true;
                }
                Ok(false) => {
                    let n = pc::tries_left();
                    self.error = Some(trn!(tr().features_more.pin_wrong, n));
                }
                Err(secs) => self.wait_until = now + secs,
            },
            Step::New => {
                if pc::valid_pin(&pin) {
                    self.step = Step::Confirm(pin);
                } else {
                    self.error = Some(tr().features_more.pin_use_digits.into());
                }
            }
            Step::Confirm(first) => {
                if *first == pin {
                    pc::set_pin(&pin, now);
                    st.toast(if self.had_pin { tr().features_more.pin_changed } else { tr().features_more.pin_set }, Tone::Info, now);
                    self.answer(true);
                    return true;
                }
                self.step = Step::New;
                self.error = Some(tr().features_more.pin_mismatch.into());
            }
        }
        false
    }
}
