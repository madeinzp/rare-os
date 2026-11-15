//! The keyboard both of Rare's native keyboards are: the launcher's own (screens/osk.rs) and the
//! one over games and apps (features/typekb.rs). Pure: the layout, where focus goes and what a
//! press does, so both behave the same and the tests here cover both. webgame.js (the keyboard in
//! web games and apps in Rare's window) carries the same layout as JSON; a test keeps them equal.
//!
//! Layers:
//! - letters: Shift gives capitals (in plain text fields it also turns the number row into
//!   ! ? # $ % & * ( ) /, as it always did);
//! - ?123: the digits and the symbols people type most;
//! - #+=: the rest (^ and `) with brackets, maths and punctuation again.
//!
//! Between ?123 and #+= every ASCII symbol has a key. Web address and email fields put . - / :
//! (or @ . - _) on the letters and add a .com key.
//!
//! Pad: the d-pad moves (rows wrap sideways, and from the bottom row to the top), A types, X
//! deletes, Y is Shift (#+= / 123 on the symbol pages), LT switches letters and symbols, Menu
//! finishes. Held X, and a held A on a key that types, repeat (`repeats`).

use rare_input::Pad;

pub const COLS: usize = 10;
pub const ROWS: usize = 4;
/// The special row's index (Shift, ?123, Space...).
pub const SPECIAL_ROW: usize = 4;

pub type Grid = [[&'static str; COLS]; ROWS];

pub const LOWER: Grid = [
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
    ["a", "s", "d", "f", "g", "h", "j", "k", "l", "@"],
    ["z", "x", "c", "v", "b", "n", "m", ".", "-", "_"],
];
pub const UPPER: Grid = [
    ["!", "?", "#", "$", "%", "&", "*", "(", ")", "/"],
    ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
    ["A", "S", "D", "F", "G", "H", "J", "K", "L", ":"],
    ["Z", "X", "C", "V", "B", "N", "M", ",", "'", "+"],
];
/// ?123.
pub const SYMBOLS: Grid = [
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    ["@", "#", "$", "_", "&", "-", "+", "(", ")", "/"],
    ["*", "\"", "'", ":", ";", "!", "?", ",", ".", "="],
    ["%", "~", "<", ">", "[", "]", "{", "}", "\\", "|"],
];
/// #+=.
pub const MORE: Grid = [
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    ["[", "]", "{", "}", "#", "%", "^", "*", "+", "="],
    ["_", "\\", "|", "~", "<", ">", "`", "$", "&", "@"],
    [".", ",", "?", "!", "'", "\"", "-", "/", ":", ";"],
];
/// The letters' last keys in a web address field (row 2's last, row 3's last three).
const URL_KEYS: [&str; 4] = [":", ".", "-", "/"];
/// The quick key web address and email fields add.
pub const DOT_COM: &str = ".com";

/// What the text is for: web address and email fields get their own quick keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Field {
    #[default]
    Text,
    Url,
    Email,
}

impl Field {
    /// A best guess from a prompt's title and placeholder ("Web game address", "https://...").
    pub fn guess(title: &str, placeholder: Option<&str>) -> Field {
        let t = title.to_lowercase();
        let p = placeholder.unwrap_or_default().to_lowercase();
        if t.contains("email")
            || p.contains("email")
            || (p.contains('@') && p.contains('.') && !p.ends_with(".sui"))
        {
            Field::Email
        } else if p.starts_with("http")
            || t.contains("web address")
            || t.contains(" url")
            || t.starts_with("url")
            || t.contains("endpoint")
        {
            Field::Url
        } else {
            Field::Text
        }
    }
}

/// Which keyboard: the launcher's (Paste, Done) or the one over apps (Enter, Close).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Host {
    Launcher,
    Over,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Layer {
    #[default]
    Letters,
    /// ?123.
    Symbols,
    /// #+=.
    More,
}

/// A key of the special row (row 4), or a character key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// Types this.
    Char(&'static str),
    /// Letters: Shift. Symbol pages: #+= / 123.
    Shift,
    /// ?123 / abc.
    Layer,
    Space,
    Paste,
    Delete,
    /// Done (the launcher) or Enter (over an app).
    Done,
    Close,
}

/// What a press asks of the keyboard's owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Out {
    Nothing,
    /// Focus, Shift or the layer changed: draw it again.
    Redraw,
    /// Type this text.
    Type(&'static str),
    Delete,
    Paste,
    Done,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Kb {
    pub layer: Layer,
    pub shift: bool,
    /// [row, col]; row 4 is the special row.
    pub pos: (usize, usize),
    pub field: Field,
    pub host: Host,
}

impl Kb {
    pub const fn new(field: Field, host: Host) -> Kb {
        Kb {
            layer: Layer::Letters,
            shift: false,
            pos: (1, 0),
            field,
            host,
        }
    }

    /// The four character rows as they are now.
    pub fn rows(&self) -> Grid {
        match self.layer {
            Layer::Symbols => SYMBOLS,
            Layer::More => MORE,
            Layer::Letters => {
                // Plain text keeps the shifted number row; addresses keep digits and their keys.
                let mut g = if self.shift && self.field == Field::Text {
                    UPPER
                } else {
                    LOWER
                };
                if self.shift && self.field != Field::Text {
                    for r in 1..ROWS {
                        g[r] = LOWER[r].map(|k| {
                            if k.len() == 1 && k.as_bytes()[0].is_ascii_lowercase() {
                                upper(k)
                            } else {
                                k
                            }
                        });
                    }
                }
                if self.field == Field::Url {
                    g[2][9] = URL_KEYS[0];
                    g[3][7] = URL_KEYS[1];
                    g[3][8] = URL_KEYS[2];
                    g[3][9] = URL_KEYS[3];
                }
                g
            }
        }
    }

    /// The special row, left to right. The same keys on every layer (only labels change), so
    /// focus stays put when the layer does.
    pub fn special(&self) -> Vec<Key> {
        let mut v = vec![Key::Shift, Key::Layer];
        if self.field != Field::Text {
            v.push(Key::Char(DOT_COM));
        }
        v.push(Key::Space);
        if self.host == Host::Launcher {
            v.push(Key::Paste);
        }
        v.push(Key::Delete);
        v.push(Key::Done);
        if self.host == Host::Over {
            v.push(Key::Close);
        }
        v
    }

    /// Flex weights of the special row (Space is wide).
    pub fn weights(&self) -> Vec<f32> {
        let narrow = self.field != Field::Text;
        self.special()
            .iter()
            .map(|k| {
                if *k == Key::Space {
                    if narrow {
                        3.0
                    } else {
                        4.0
                    }
                } else {
                    1.4
                }
            })
            .collect()
    }

    pub fn key_at(&self, pos: (usize, usize)) -> Key {
        if pos.0 >= SPECIAL_ROW {
            let sp = self.special();
            sp[pos.1.min(sp.len() - 1)]
        } else {
            Key::Char(self.rows()[pos.0][pos.1.min(COLS - 1)])
        }
    }

    pub fn label(&self, k: Key) -> &'static str {
        match k {
            Key::Char(c) => c,
            Key::Shift => match self.layer {
                Layer::Letters => "Shift",
                Layer::Symbols => "#+=",
                Layer::More => "123",
            },
            Key::Layer => {
                if self.layer == Layer::Letters {
                    "?123"
                } else {
                    "abc"
                }
            }
            Key::Space => "Space",
            Key::Paste => "Paste",
            Key::Delete => "Delete",
            Key::Done => {
                if self.host == Host::Launcher {
                    "Done"
                } else {
                    "Enter"
                }
            }
            Key::Close => "Close",
        }
    }

    /// The button that does a key's job, shown on it.
    pub fn button(k: Key) -> Option<Pad> {
        Some(match k {
            Key::Shift => Pad::Y,
            Key::Layer => Pad::Lt,
            Key::Delete => Pad::X,
            Key::Done => Pad::Start,
            _ => return None,
        })
    }

    /// The Shift key is drawn lit: Shift on, or the #+= page.
    pub fn lit(&self) -> bool {
        (self.layer == Layer::Letters && self.shift) || self.layer == Layer::More
    }

    /// Whether a held `p` repeats on the key in focus (the owner times the repeat).
    pub fn repeats(&self, p: Pad) -> bool {
        match p {
            Pad::X => true,
            Pad::A => matches!(
                self.key_at(self.pos),
                Key::Char(_) | Key::Space | Key::Delete
            ),
            _ => false,
        }
    }

    fn special_len(&self) -> usize {
        self.special().len()
    }

    /// The special key under a character column (by the keys' centres).
    fn col_to_special(&self, c: usize) -> usize {
        let w = self.weights();
        let total: f32 = w.iter().sum();
        let x = (c as f32 + 0.5) / COLS as f32 * total;
        let mut acc = 0.0;
        for (i, wi) in w.iter().enumerate() {
            acc += wi;
            if x < acc {
                return i;
            }
        }
        w.len() - 1
    }

    /// The character column over a special key's centre.
    fn special_to_col(&self, i: usize) -> usize {
        let w = self.weights();
        let total: f32 = w.iter().sum();
        let centre = w.iter().take(i).sum::<f32>() + w[i.min(w.len() - 1)] / 2.0;
        ((centre / total * COLS as f32) as usize).min(COLS - 1)
    }

    /// Where a direction goes from `pos`.
    pub fn next(&self, d: Pad) -> Option<(usize, usize)> {
        let (r, c) = self.pos;
        let last = SPECIAL_ROW;
        let n = self.special_len();
        let cols = |row: usize| if row == last { n } else { COLS };
        Some(match d {
            Pad::Up if r == 0 => (last, self.col_to_special(c)),
            Pad::Up if r == last => (r - 1, self.special_to_col(c)),
            Pad::Up => (r - 1, c),
            Pad::Down if r + 1 == last => (last, self.col_to_special(c)),
            Pad::Down if r == last => (0, self.special_to_col(c)),
            Pad::Down => (r + 1, c),
            Pad::Left => (r, (c + cols(r) - 1) % cols(r)),
            Pad::Right => (r, (c + 1) % cols(r)),
            _ => return None,
        })
    }

    /// A key pressed (A on it, or a touch). `repeat`: a held A.
    pub fn hit(&mut self, k: Key, repeat: bool) -> Out {
        if repeat && !matches!(k, Key::Char(_) | Key::Space | Key::Delete) {
            return Out::Nothing;
        }
        match k {
            Key::Char(c) => {
                // A capital ends Shift (one capital, as on a phone); symbols keep it.
                if self.layer == Layer::Letters
                    && self.shift
                    && c.len() == 1
                    && c.as_bytes()[0].is_ascii_uppercase()
                {
                    self.shift = false;
                }
                Out::Type(c)
            }
            Key::Shift => {
                match self.layer {
                    Layer::Letters => self.shift = !self.shift,
                    Layer::Symbols => self.layer = Layer::More,
                    Layer::More => self.layer = Layer::Symbols,
                }
                Out::Redraw
            }
            Key::Layer => {
                self.layer = if self.layer == Layer::Letters {
                    Layer::Symbols
                } else {
                    Layer::Letters
                };
                self.shift = false;
                Out::Redraw
            }
            Key::Space => Out::Type(" "),
            Key::Paste => Out::Paste,
            Key::Delete => Out::Delete,
            Key::Done => Out::Done,
            Key::Close => Out::Close,
        }
    }

    /// A pad press (or the repeat of a held one). B and the owner's own buttons (LB, RB, View
    /// over an app) are the owner's: Nothing here.
    pub fn press(&mut self, p: Pad, repeat: bool) -> Out {
        if let Some(to) = self.next(p) {
            if to == self.pos {
                return Out::Nothing;
            }
            self.pos = to;
            return Out::Redraw;
        }
        if repeat && !self.repeats(p) {
            return Out::Nothing;
        }
        match p {
            Pad::A => self.hit(self.key_at(self.pos), repeat),
            Pad::X => Out::Delete,
            Pad::Y => self.hit(Key::Shift, false),
            Pad::Lt => self.hit(Key::Layer, false),
            Pad::Start => Out::Done,
            _ => Out::Nothing,
        }
    }

    /// The controller line under the keys.
    pub fn hint(&self) -> String {
        let shift = match self.layer {
            Layer::Letters => "Shift",
            Layer::Symbols => "#+=",
            Layer::More => "123",
        };
        let layer = if self.layer == Layer::Letters {
            "?123"
        } else {
            "abc"
        };
        match self.host {
            Host::Launcher => format!("A Type    X Delete    Y {shift}    LT {layer}    Menu Done    B Cancel"),
            Host::Over => format!("A Type    X Delete    Y {shift}    LT {layer}    Menu Enter    View Tab    LB RB Cursor    B Close"),
        }
    }
}

fn upper(k: &'static str) -> &'static str {
    // Only single ASCII letters come here; LOWER's letters are all in UPPER.
    for (lr, ur) in LOWER.iter().zip(UPPER.iter()) {
        for (l, u) in lr.iter().zip(ur.iter()) {
            if *l == k {
                return u;
            }
        }
    }
    k
}

/// The layout as webgame.js carries it (`const KB = {...}`), for the test that keeps them equal.
pub fn layout_json() -> serde_json::Value {
    let g = |g: &Grid| g.iter().map(|r| r.concat()).collect::<Vec<_>>();
    serde_json::json!({
        "lower": g(&LOWER),
        "upper": g(&UPPER),
        "sym": g(&SYMBOLS),
        "more": g(&MORE),
        "url": URL_KEYS.concat(),
        "com": DOT_COM,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASCII_SYMBOLS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    fn all_chars(g: &Grid) -> String {
        g.iter().flat_map(|r| r.iter()).copied().collect()
    }

    #[test]
    fn every_ascii_symbol_has_a_key() {
        let both = all_chars(&SYMBOLS) + &all_chars(&MORE);
        for ch in ASCII_SYMBOLS.chars() {
            assert!(both.contains(ch), "no key for {ch}");
        }
        // Digits on both symbol pages; every key one character.
        for g in [&LOWER, &UPPER, &SYMBOLS, &MORE] {
            assert!(g
                .iter()
                .flat_map(|r| r.iter())
                .all(|k| k.chars().count() == 1));
        }
        assert_eq!(SYMBOLS[0], LOWER[0]);
        assert_eq!(MORE[0], LOWER[0]);
        // ?123 has all but two.
        let missing: String = ASCII_SYMBOLS
            .chars()
            .filter(|c| !all_chars(&SYMBOLS).contains(*c))
            .collect();
        assert_eq!(missing, "^`");
        // No key twice on one page.
        for g in [&SYMBOLS, &MORE] {
            let mut keys: Vec<&str> = g.iter().flat_map(|r| r.iter()).copied().collect();
            keys.sort();
            let n = keys.len();
            keys.dedup();
            assert_eq!(keys.len(), n);
        }
    }

    #[test]
    fn lt_and_y_reach_both_symbol_pages_and_back() {
        let mut k = Kb::new(Field::Text, Host::Launcher);
        assert_eq!(k.press(Pad::Lt, false), Out::Redraw);
        assert_eq!(k.layer, Layer::Symbols);
        assert_eq!(k.label(Key::Layer), "abc");
        assert_eq!(k.label(Key::Shift), "#+=");
        // Starts on row 1: ?123's first symbol.
        assert_eq!(k.press(Pad::A, false), Out::Type("@"));
        assert_eq!(k.press(Pad::Y, false), Out::Redraw);
        assert_eq!(k.layer, Layer::More);
        assert!(k.lit());
        assert_eq!(k.label(Key::Shift), "123");
        k.pos = (1, 6);
        assert_eq!(k.press(Pad::A, false), Out::Type("^"));
        k.pos = (2, 6);
        assert_eq!(k.press(Pad::A, false), Out::Type("`"));
        assert_eq!(k.press(Pad::Y, false), Out::Redraw);
        assert_eq!(k.layer, Layer::Symbols);
        assert_eq!(k.press(Pad::Lt, false), Out::Redraw);
        assert_eq!(k.layer, Layer::Letters);
        // The keys on screen do the same: ?123, #+=, abc.
        k.pos = (SPECIAL_ROW, 1);
        assert_eq!(k.key_at(k.pos), Key::Layer);
        k.press(Pad::A, false);
        assert_eq!(k.layer, Layer::Symbols);
        k.pos = (SPECIAL_ROW, 0);
        k.press(Pad::A, false);
        assert_eq!(k.layer, Layer::More);
        k.pos = (SPECIAL_ROW, 1);
        k.press(Pad::A, false);
        assert_eq!(k.layer, Layer::Letters);
        assert!(!k.shift);
    }

    #[test]
    fn shift_gives_one_capital_and_keeps_the_old_symbols() {
        let mut k = Kb::new(Field::Text, Host::Over);
        k.press(Pad::Y, false);
        assert!(k.shift && k.lit());
        assert_eq!(k.press(Pad::A, false), Out::Type("Q"));
        assert!(!k.shift);
        k.press(Pad::Y, false);
        k.pos = (0, 0);
        assert_eq!(k.press(Pad::A, false), Out::Type("!"));
        assert!(k.shift, "symbols keep Shift");
    }

    #[test]
    fn url_and_email_fields_get_their_keys() {
        let k = Kb::new(Field::Url, Host::Launcher);
        let r = k.rows();
        assert_eq!((r[2][9], r[3][7], r[3][8], r[3][9]), (":", ".", "-", "/"));
        assert!(k.special().contains(&Key::Char(".com")));
        assert_eq!(
            k.special(),
            vec![
                Key::Shift,
                Key::Layer,
                Key::Char(".com"),
                Key::Space,
                Key::Paste,
                Key::Delete,
                Key::Done
            ]
        );
        let e = Kb::new(Field::Email, Host::Over);
        assert_eq!(e.rows()[2][9], "@");
        assert_eq!(
            e.special(),
            vec![
                Key::Shift,
                Key::Layer,
                Key::Char(".com"),
                Key::Space,
                Key::Delete,
                Key::Done,
                Key::Close
            ]
        );
        // Shift in an address field: capitals only, the address keys stay.
        let mut u = Kb::new(Field::Url, Host::Launcher);
        u.shift = true;
        assert_eq!(u.rows()[0], LOWER[0]);
        assert_eq!(u.rows()[1][0], "Q");
        assert_eq!(u.rows()[3][9], "/");
        // .com types in one go.
        let mut u = Kb::new(Field::Url, Host::Launcher);
        u.pos = (SPECIAL_ROW, 2);
        assert_eq!(u.press(Pad::A, false), Out::Type(".com"));
        assert_eq!(
            Kb::new(Field::Text, Host::Launcher).special(),
            vec![
                Key::Shift,
                Key::Layer,
                Key::Space,
                Key::Paste,
                Key::Delete,
                Key::Done
            ]
        );
    }

    #[test]
    fn guesses_the_field() {
        assert_eq!(
            Field::guess("Web game address", Some("https://game.example")),
            Field::Url
        );
        assert_eq!(Field::guess("Email", None), Field::Email);
        assert_eq!(
            Field::guess("Send to", Some("0x address or name.sui")),
            Field::Text
        );
        assert_eq!(Field::guess("Search", None), Field::Text);
        assert_eq!(
            Field::guess("Contact", Some("you@example.com")),
            Field::Email
        );
    }

    #[test]
    fn moves_wrap_and_line_up_with_the_special_row() {
        for host in [Host::Launcher, Host::Over] {
            for field in [Field::Text, Field::Url, Field::Email] {
                let mut k = Kb::new(field, host);
                let n = k.special().len();
                // Down from row 3 lands on a special key; up from there comes back near the column.
                for c in 0..COLS {
                    k.pos = (3, c);
                    k.press(Pad::Down, false);
                    assert_eq!(k.pos.0, SPECIAL_ROW);
                    assert!(k.pos.1 < n);
                    let back = k.next(Pad::Up).unwrap();
                    assert_eq!(back.0, 3);
                    assert!(
                        (back.1 as i32 - c as i32).abs() <= 3,
                        "{field:?} {host:?} col {c} -> {:?} -> {back:?}",
                        k.pos
                    );
                }
                // The middle of the keyboard is Space.
                k.pos = (3, 5);
                k.press(Pad::Down, false);
                assert_eq!(k.key_at(k.pos), Key::Space, "{field:?} {host:?}");
                // Sideways wraps, bottom to top and top to bottom wrap.
                k.pos = (1, 0);
                k.press(Pad::Left, false);
                assert_eq!(k.pos, (1, 9));
                k.pos = (SPECIAL_ROW, n - 1);
                k.press(Pad::Right, false);
                assert_eq!(k.pos, (SPECIAL_ROW, 0));
                k.press(Pad::Down, false);
                assert_eq!(k.pos.0, 0);
                k.press(Pad::Up, false);
                assert_eq!(k.pos.0, SPECIAL_ROW);
                // Every position on every layer is a key.
                for layer in [Layer::Letters, Layer::Symbols, Layer::More] {
                    k.layer = layer;
                    for r in 0..=SPECIAL_ROW {
                        for c in 0..if r == SPECIAL_ROW { n } else { COLS } {
                            assert!(!k.label(k.key_at((r, c))).is_empty());
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn held_keys_repeat_where_it_makes_sense() {
        let mut k = Kb::new(Field::Text, Host::Launcher);
        // Held X deletes again and again.
        assert!(k.repeats(Pad::X));
        assert_eq!(k.press(Pad::X, true), Out::Delete);
        // Held A on a letter types it again; on Space, a space; on Delete, deletes.
        assert_eq!(k.press(Pad::A, true), Out::Type("q"));
        k.pos = (SPECIAL_ROW, 2);
        assert_eq!(k.key_at(k.pos), Key::Space);
        assert_eq!(k.press(Pad::A, true), Out::Type(" "));
        k.pos = (SPECIAL_ROW, 4);
        assert_eq!(k.key_at(k.pos), Key::Delete);
        assert_eq!(k.press(Pad::A, true), Out::Delete);
        // Not on Shift, ?123 or Done: a hold must not flicker the layer or finish twice.
        for c in [0, 1, 5] {
            k.pos = (SPECIAL_ROW, c);
            assert!(!k.repeats(Pad::A), "{:?}", k.key_at(k.pos));
            assert_eq!(k.press(Pad::A, true), Out::Nothing);
        }
        assert_eq!(k.press(Pad::Y, true), Out::Nothing);
        assert_eq!(k.press(Pad::Lt, true), Out::Nothing);
        assert_eq!(k.press(Pad::Start, true), Out::Nothing);
        // Directions repeat (the input thread times those).
        k.pos = (1, 0);
        assert_eq!(k.press(Pad::Right, true), Out::Redraw);
    }

    #[test]
    fn buttons_shown_on_keys() {
        assert_eq!(Kb::button(Key::Shift), Some(Pad::Y));
        assert_eq!(Kb::button(Key::Layer), Some(Pad::Lt));
        assert_eq!(Kb::button(Key::Delete), Some(Pad::X));
        assert_eq!(Kb::button(Key::Done), Some(Pad::Start));
        assert_eq!(Kb::button(Key::Space), None);
        let k = Kb::new(Field::Text, Host::Over);
        assert!(k.hint().contains("LT ?123"));
        assert_eq!(k.label(Key::Done), "Enter");
    }

    #[test]
    fn webgame_js_has_the_same_layout() {
        let js = include_str!("../../../crates/rare-webgame/src/webgame.js");
        let start = js.find("const KB = ").expect("webgame.js: const KB") + "const KB = ".len();
        let end = start + js[start..].find(";\n").unwrap();
        let theirs: serde_json::Value =
            serde_json::from_str(&js[start..end]).expect("webgame.js KB is JSON");
        assert_eq!(theirs, layout_json());
    }
}
