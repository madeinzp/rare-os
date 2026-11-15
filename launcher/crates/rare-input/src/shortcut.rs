//! A key shortcut LC or RC holds down (Settings > Controls > Key shortcut): the keys go down when
//! the button goes down and up when it comes up, so push-to-talk apps (dictation) see the key held
//! for as long as the button is. Saved as `lcShortcut` / `rcShortcut` in launcher.json, as key
//! names joined by `+` (`ctrl+alt+space`), modifiers first.
//!
//! The keys are Linux input key codes (linux/input-event-codes.h), sent on a virtual keyboard
//! (rare_launch::apps::pointer::hold_keys) in Rare's session and in Desktop mode.

/// (code, saved name, label). Modifiers first, then the keys a shortcut can end on.
pub const KEYS: &[(u16, &str, &str)] = &[
    (29, "ctrl", "Ctrl"),
    (56, "alt", "Alt"),
    (42, "shift", "Shift"),
    (125, "super", "Super"),
    (97, "rctrl", "Right Ctrl"),
    (100, "ralt", "Right Alt"),
    (54, "rshift", "Right Shift"),
    (126, "rsuper", "Right Super"),
    (57, "space", "Space"),
    (28, "enter", "Enter"),
    (15, "tab", "Tab"),
    (1, "esc", "Esc"),
    (14, "backspace", "Backspace"),
    (110, "insert", "Insert"),
    (111, "delete", "Delete"),
    (102, "home", "Home"),
    (107, "end", "End"),
    (104, "pageup", "Page Up"),
    (109, "pagedown", "Page Down"),
    (119, "pause", "Pause"),
    (70, "scrolllock", "Scroll Lock"),
    (99, "print", "Print Screen"),
    (127, "menu", "Menu"),
    (58, "capslock", "Caps Lock"),
    (103, "up", "Up"),
    (108, "down", "Down"),
    (105, "left", "Left"),
    (106, "right", "Right"),
    (59, "f1", "F1"),
    (60, "f2", "F2"),
    (61, "f3", "F3"),
    (62, "f4", "F4"),
    (63, "f5", "F5"),
    (64, "f6", "F6"),
    (65, "f7", "F7"),
    (66, "f8", "F8"),
    (67, "f9", "F9"),
    (68, "f10", "F10"),
    (87, "f11", "F11"),
    (88, "f12", "F12"),
    (183, "f13", "F13"),
    (184, "f14", "F14"),
    (185, "f15", "F15"),
    (186, "f16", "F16"),
    (187, "f17", "F17"),
    (188, "f18", "F18"),
    (189, "f19", "F19"),
    (190, "f20", "F20"),
    (191, "f21", "F21"),
    (192, "f22", "F22"),
    (193, "f23", "F23"),
    (194, "f24", "F24"),
    (30, "a", "A"),
    (48, "b", "B"),
    (46, "c", "C"),
    (32, "d", "D"),
    (18, "e", "E"),
    (33, "f", "F"),
    (34, "g", "G"),
    (35, "h", "H"),
    (23, "i", "I"),
    (36, "j", "J"),
    (37, "k", "K"),
    (38, "l", "L"),
    (50, "m", "M"),
    (49, "n", "N"),
    (24, "o", "O"),
    (25, "p", "P"),
    (16, "q", "Q"),
    (19, "r", "R"),
    (31, "s", "S"),
    (20, "t", "T"),
    (22, "u", "U"),
    (47, "v", "V"),
    (17, "w", "W"),
    (45, "x", "X"),
    (21, "y", "Y"),
    (44, "z", "Z"),
    (11, "0", "0"),
    (2, "1", "1"),
    (3, "2", "2"),
    (4, "3", "3"),
    (5, "4", "4"),
    (6, "5", "5"),
    (7, "6", "6"),
    (8, "7", "7"),
    (9, "8", "8"),
    (10, "9", "9"),
    (12, "minus", "-"),
    (13, "equal", "="),
    (41, "grave", "`"),
    (26, "leftbrace", "["),
    (27, "rightbrace", "]"),
    (43, "backslash", "\\"),
    (39, "semicolon", ";"),
    (40, "apostrophe", "'"),
    (51, "comma", ","),
    (52, "dot", "."),
    (53, "slash", "/"),
];

/// The four modifiers the picker toggles (left side keys).
pub const MODIFIERS: [u16; 4] = [29, 56, 42, 125];

/// Every key a shortcut can hold: what the virtual keyboard must be able to send.
pub fn all_codes() -> Vec<u16> {
    KEYS.iter().map(|k| k.0).collect()
}

pub fn is_modifier(code: u16) -> bool {
    matches!(code, 29 | 56 | 42 | 125 | 97 | 100 | 54 | 126)
}

pub fn name(code: u16) -> Option<&'static str> {
    KEYS.iter().find(|k| k.0 == code).map(|k| k.1)
}

pub fn label_of(code: u16) -> Option<&'static str> {
    KEYS.iter().find(|k| k.0 == code).map(|k| k.2)
}

/// The keys of a shortcut, in the order they go down (modifiers first). Never empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shortcut(Vec<u16>);

impl Shortcut {
    /// Modifiers (in MODIFIERS order, then right-side ones as given) and at most one other key.
    /// None when nothing known is in it.
    pub fn new(codes: &[u16]) -> Option<Shortcut> {
        let mut mods: Vec<u16> = Vec::new();
        let mut key = None;
        for &c in codes {
            if label_of(c).is_none() {
                continue;
            }
            if is_modifier(c) {
                if !mods.contains(&c) {
                    mods.push(c);
                }
            } else {
                key = Some(c);
            }
        }
        mods.sort_by_key(|m| MODIFIERS.iter().position(|x| x == m).unwrap_or(MODIFIERS.len()));
        mods.extend(key);
        (!mods.is_empty()).then_some(Shortcut(mods))
    }

    /// `ctrl+alt+space`; None for an empty or unknown one.
    pub fn parse(s: &str) -> Option<Shortcut> {
        let codes: Vec<u16> = s.split('+').filter_map(|p| KEYS.iter().find(|k| k.1 == p.trim().to_ascii_lowercase()).map(|k| k.0)).collect();
        Shortcut::new(&codes)
    }

    pub fn keys(&self) -> &[u16] {
        &self.0
    }

    /// The modifiers in it.
    pub fn mods(&self) -> Vec<u16> {
        self.0.iter().copied().filter(|c| is_modifier(*c)).collect()
    }

    /// The one key that is not a modifier, if any.
    pub fn key(&self) -> Option<u16> {
        self.0.iter().copied().find(|c| !is_modifier(*c))
    }

    /// What is saved: `ctrl+alt+space`.
    pub fn to_saved(&self) -> String {
        self.0.iter().filter_map(|c| name(*c)).collect::<Vec<_>>().join("+")
    }

    /// What people read: `Ctrl + Alt + Space`.
    pub fn label(&self) -> String {
        self.0.iter().filter_map(|c| label_of(*c)).collect::<Vec<_>>().join(" + ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_form_round_trips() {
        let s = Shortcut::parse("ctrl+alt+space").unwrap();
        assert_eq!(s.keys(), &[29, 56, 57]);
        assert_eq!(s.to_saved(), "ctrl+alt+space");
        assert_eq!(s.label(), "Ctrl + Alt + Space");
        assert_eq!(Shortcut::parse(&s.to_saved()), Some(s));
        // A lone key, a lone modifier, case and spaces.
        assert_eq!(Shortcut::parse("F13").unwrap().keys(), &[183]);
        assert_eq!(Shortcut::parse(" rctrl ").unwrap().label(), "Right Ctrl");
        assert_eq!(Shortcut::parse(""), None);
        assert_eq!(Shortcut::parse("hyper+nothing"), None);
    }

    #[test]
    fn modifiers_go_first_in_a_fixed_order_and_one_key_wins() {
        // Pressed in any order: Shift, Space, Ctrl. Saved Ctrl, Shift, then the key.
        let s = Shortcut::new(&[42, 57, 29]).unwrap();
        assert_eq!(s.keys(), &[29, 42, 57]);
        assert_eq!(s.mods(), vec![29, 42]);
        assert_eq!(s.key(), Some(57));
        // A second plain key replaces the first.
        assert_eq!(Shortcut::new(&[29, 30, 31]).unwrap().keys(), &[29, 31]);
        // Unknown codes are dropped.
        assert_eq!(Shortcut::new(&[29, 999]).unwrap().keys(), &[29]);
        assert_eq!(Shortcut::new(&[999]), None);
    }

    #[test]
    fn every_key_has_a_unique_name_and_code() {
        let mut names: Vec<_> = KEYS.iter().map(|k| k.1).collect();
        let mut codes: Vec<_> = KEYS.iter().map(|k| k.0).collect();
        names.sort();
        names.dedup();
        codes.sort();
        codes.dedup();
        assert_eq!(names.len(), KEYS.len());
        assert_eq!(codes.len(), KEYS.len());
        assert!(KEYS.iter().all(|k| !k.1.contains('+')));
    }
}
