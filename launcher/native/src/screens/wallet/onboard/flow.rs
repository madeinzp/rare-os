//! Small helpers the onboarding pages share (launcher/src/wallet/ui/onboard/flow.ts): the hints,
//! plain error text, the udev fix line, and how a finished flow lands.

use rare_i18n::tr;
use crate::screens::wallet::home::lib::{hint, Hint};

/// `H`: the hints the onboarding pages show.
pub mod h {
    use super::*;
    pub fn select() -> Hint {
        hint("a", tr().common.select)
    }
    pub fn back() -> Hint {
        hint("b", tr().common.back)
    }
    pub fn type_() -> Hint {
        hint("a", tr().wallet_setup.hint_type)
    }
    pub fn del() -> Hint {
        hint("x", tr().common.delete)
    }
    pub fn ok() -> Hint {
        hint("start", tr().common.confirm)
    }
}

/// An error from the keystore or the Rust side (`NativeError` code and message, or a plain Error).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Failure {
    pub code: Option<String>,
    pub message: Option<String>,
    /// `data.fix` on a "udev" error: the one-line USB permission fix
    pub fix: Option<String>,
}

impl Failure {
    pub fn code(code: &str) -> Failure {
        Failure { code: Some(code.into()), ..Default::default() }
    }
    pub fn msg(m: &str) -> Failure {
        Failure { message: Some(m.into()), ..Default::default() }
    }
}

/// `plainError`: plain words, never a secret.
pub fn plain_error(e: &Failure, fallback: &str) -> String {
    if let Some(c) = e.code.as_deref() {
        let known = match c {
            "cancelled" => Some(tr().wallet_setup.err_cancelled),
            "timeout" => Some(tr().wallet_setup.err_timeout),
            "ble-off" => Some(tr().wallet_setup.err_ble_off),
            "notfound" => Some(tr().wallet_setup.err_notfound),
            "busy" => Some(tr().wallet_setup.err_busy),
            "network" => Some(tr().wallet_setup.err_network),
            "wrong-pin" => Some(tr().wallet_setup.err_wrong_pin),
            "unsupported" => Some(tr().wallet_setup.err_unsupported),
            _ => None,
        };
        if let Some(k) = known {
            return k.into();
        }
    }
    match e.message.as_deref() {
        Some(m) if !m.is_empty() && m.chars().count() < 140 => m.into(),
        _ => fallback.into(),
    }
}

/// The fallback error line.
pub fn default_error() -> &'static str {
    tr().wallet_setup.err_default
}

/// `udevFix`.
pub fn udev_fix(e: &Failure) -> Option<String> {
    if e.code.as_deref() == Some("udev") {
        e.fix.clone()
    } else {
        None
    }
}

/// `vaultGate`'s answer: the vault is open, there is none yet (ask for a new PIN), or unlock was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    Ready,
    Pin,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors() {
        assert_eq!(plain_error(&Failure::code("ble-off"), default_error()), "Bluetooth is off. Turn it on and try again.");
        assert_eq!(plain_error(&Failure::msg("Name not found"), default_error()), "Name not found");
        assert_eq!(plain_error(&Failure::msg(&"x".repeat(200)), "Could not add that address"), "Could not add that address");
        assert_eq!(plain_error(&Failure::default(), default_error()), default_error());
        let u = Failure { code: Some("udev".into()), message: None, fix: Some("pkexec x".into()) };
        assert_eq!(udev_fix(&u).as_deref(), Some("pkexec x"));
    }
}
