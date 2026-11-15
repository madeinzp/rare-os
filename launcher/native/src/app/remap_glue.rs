//! Settings > Controls > Button layout and Key shortcut: the sheets over Settings
//! (screens/settings/remap.rs, shortcut.rs), and the Controls page drawn again when one closes so
//! its lines are current.

use super::*;

impl App {
    /// The acts of the layout sheet and its Controls row. False when it is not one of them.
    pub(super) fn remap_act(&mut self, a: &str) -> bool {
        match a {
            "controls:remap-open" => self.open_remap_sheet(),
            // controls:shortcut:<lc|rc>:<the row that picked Key shortcut, or empty>
            a if a.starts_with("controls:shortcut:") => {
                let mut it = a["controls:shortcut:".len()..].splitn(2, ':');
                let lc = it.next() == Some("lc");
                let row = it.next().filter(|r| !r.is_empty()).map(str::to_string);
                self.open_shortcut_sheet(lc, row);
            }
            // Saved: Settings reads the config again. Cancelled: the Controls page draws again,
            // so a Key shortcut chip picked without keys goes back.
            "controls:shortcut-saved" | "controls:shortcut-done" => {
                self.fx.fb(Feedback::Back);
                let now = clock::secs();
                let cmd = if a == "controls:shortcut-saved" { "reload" } else { "refresh" };
                if let Some((_, screen, _)) = self.legacy_page.as_mut() {
                    let mut cx = crate::screens::overlay::Cx { scene: &mut self.scene, t: &mut self.text, img: &mut self.images };
                    screen.command(&mut cx, cmd, "", now);
                }
            }
            "controls:remap-done" => {
                self.fx.fb(Feedback::Back);
                let now = clock::secs();
                if let Some((_, screen, _)) = self.legacy_page.as_mut() {
                    let mut cx = crate::screens::overlay::Cx { scene: &mut self.scene, t: &mut self.text, img: &mut self.images };
                    screen.command(&mut cx, "refresh", "", now);
                }
            }
            _ => return false,
        }
        true
    }

    fn open_shortcut_sheet(&mut self, lc: bool, row: Option<String>) {
        use crate::screens::settings::shortcut::ShortcutSheet;
        let now = clock::secs();
        let live = !self.fixture;
        let cfg = if live { crate::screens::settings::config::load() } else { crate::screens::settings::config::fixture() };
        let saved = if lc { cfg.lc_shortcut } else { cfg.rc_shortcut };
        let saved = saved.as_deref().and_then(rare_input::shortcut::Shortcut::parse);
        let root = self.scene.root;
        let before = self.scene.children(root);
        let screen: Box<dyn crate::screens::overlay::Screen> = {
            let mut cx = crate::screens::overlay::Cx { scene: &mut self.scene, t: &mut self.text, img: &mut self.images };
            Box::new(ShortcutSheet::open(&mut cx, lc, row, saved, live, now))
        };
        let nodes: Vec<Id> = self.scene.children(root).into_iter().filter(|n| !before.contains(n)).collect();
        for n in &nodes {
            self.scene.n(*n).z = layer::SHEET + 5;
        }
        self.fx.fb(Feedback::Open);
        self.legacy = Some((screen, nodes));
    }

    fn open_remap_sheet(&mut self) {
        use crate::screens::settings::remap::RemapSheet;
        let now = clock::secs();
        let live = !self.fixture;
        let layout = if live { rare_input::remap::load() } else { rare_input::remap::Layout::default() };
        let root = self.scene.root;
        let before = self.scene.children(root);
        let screen: Box<dyn crate::screens::overlay::Screen> = {
            let mut cx = crate::screens::overlay::Cx { scene: &mut self.scene, t: &mut self.text, img: &mut self.images };
            Box::new(RemapSheet::open(&mut cx, layout, live, now))
        };
        let nodes: Vec<Id> = self.scene.children(root).into_iter().filter(|n| !before.contains(n)).collect();
        for n in &nodes {
            self.scene.n(*n).z = layer::SHEET + 5;
        }
        self.fx.fb(Feedback::Open);
        self.legacy = Some((screen, nodes));
    }
}
