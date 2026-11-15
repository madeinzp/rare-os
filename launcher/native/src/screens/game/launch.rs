//! "Hades II didn't start." (components/LaunchFail.tsx): one sentence, what happened and the next
//! step, Try again and Close. Try again starts it again the way the player did.

use crate::act::Act;
use rare_i18n::{tr, trf};
use crate::kit::{self, s, tok, Btn as B, Ty, Ui, T};
use crate::page::{hash, Acts, Page};
use crate::state::{Overlay, State};
use rare_render::view::El;

pub struct LaunchFail;

/// The sheet's second line (store.ts launchFailText).
pub fn fail_text(reason: &str, code: Option<i32>) -> String {
    let code = match code {
        Some(c) if c != -1 => trf!(tr().game.exit_code, code = c),
        _ => String::new(),
    };
    match reason {
        "steam" => tr().game.fail_steam.into(),
        "steamui" => tr().game.fail_steamui.into(),
        "never" => tr().game.fail_never.into(),
        "stuck" => tr().game.fail_stuck.into(),
        "quick" => trf!(tr().game.fail_quick, code = code),
        _ => trf!(tr().game.fail_closed, code = code),
    }
}

impl Page for LaunchFail {
    fn stamp(&self, st: &State) -> u64 {
        hash((format!("{:?}", st.overlay), st.ver.library))
    }

    fn view(&mut self, ui: &mut Ui, st: &State, acts: &mut Acts) -> El {
        let Some(Overlay::LaunchFail { id, reason, code }) = &st.overlay else { return ui.bx(s()) };
        let name = st.run_name(id);
        // Apps start from the Apps screen (not wired natively yet), so Try again is for games.
        let retry = st.game(id).is_some() || id.starts_with("app:");
        let mut footer = Vec::new();
        if retry {
            footer.push(kit::button(ui, B::new(tr().common.try_again, acts.tag(Act::Retry(id.clone()))).primary().def(true).hint(tr().common.try_again)));
        }
        footer.push(kit::button(ui, B::new(tr().common.close, acts.tag(Act::Close)).def(!retry).hint(tr().common.close)));
        let body = ui.text_s(s().wp(1.0), fail_text(reason, *code), T::new(Ty::Body).c(tok::MUTED).wrap());
        kit::sheet(ui, &trf!(tr().game.fail_title, name = name), None, vec![body], footer, false, 1)
    }
}
