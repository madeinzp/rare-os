//! The Lighting section: the lights around the analog sticks (features/lights.rs draws them
//! through rare-fand). Default leaves them to the controller, as it came; On shows the effect,
//! color and brightness below, live while they change; Off keeps them dark.

use super::{config::Config, rows::*, SectionOut, SectionView};
use crate::features::lights::{self, Effect, LightsMode};
use crate::screens::overlay::common::*;
use crate::screens::overlay::lay::s;
use rare_i18n::tr;
use rare_render::scene::LinearGradient;
use rare_render::theme::tok;
use rare_render::{Color, Id, Node};

/// The palette: white, Sui blue, cyan, green, yellow, orange, red, pink, purple.
pub const PALETTE: [[u8; 3]; 9] = [
    [0xff, 0xff, 0xff],
    [0x4d, 0xa2, 0xff],
    [0x00, 0xe0, 0xff],
    [0x2b, 0xff, 0x88],
    [0xff, 0xd6, 0x00],
    [0xff, 0x7a, 0x00],
    [0xff, 0x2a, 0x2a],
    [0xff, 0x3d, 0x9a],
    [0x9b, 0x5c, 0xff],
];

/// Which ring the color rows set: both, left, right. Not saved: Settings opens on both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ring {
    Both,
    Left,
    Right,
}

thread_local! {
    static RING: std::cell::Cell<Ring> = const { std::cell::Cell::new(Ring::Both) };
}

fn ring() -> Ring {
    RING.with(|r| r.get())
}

fn color(c: [u8; 3]) -> Color {
    Color::rgb(c[0], c[1], c[2])
}

/// The color the rows show for the ring being set; None for both rings in two colors.
fn edited(cfg: &Config) -> Option<[u8; 3]> {
    let (l, r) = lights::colors(cfg);
    match ring() {
        Ring::Left => Some(l),
        Ring::Right => Some(r),
        Ring::Both => (l == r).then_some(l),
    }
}

/// Sets the color of the ring being set.
fn set_color(cfg: &mut Config, c: [u8; 3]) {
    let h = Some(lights::hex(c));
    match ring() {
        Ring::Left => cfg.lights_left = h,
        Ring::Right => cfg.lights_right = h,
        Ring::Both => {
            cfg.lights_left = h.clone();
            cfg.lights_right = h;
        }
    }
}

pub struct LightsSection {
    rows: RowSet,
    /// The hue slider's knob, colored with the hue as it moves.
    hue_knob: Option<Id>,
}

const HUE_STEP: f32 = 5.0;

impl LightsSection {
    pub fn build(cx: &mut Cx, nav: &mut Nav, parent: Id, cfg: &Config, _now: f64) -> LightsSection {
        let r = cx.r();
        let t = &tr().lights;
        let mut rows = RowSet::default();
        let mut hue_knob = None;
        let mode = lights::mode(cfg);
        let available = lights::available() != Some(false);

        let p1 = panel(cx, parent);
        let body = if !available {
            t.none_body
        } else {
            match mode {
                LightsMode::Default => t.default_body,
                LightsMode::On => t.on_body,
                LightsMode::Off => t.off_body,
            }
        };
        if !available {
            setting_row(cx, p1, t.row_lights, Some(body), true);
            return LightsSection { rows, hue_knob };
        }
        let idx = match mode {
            LightsMode::Default => 0,
            LightsMode::On => 1,
            LightsMode::Off => 2,
        };
        rows.choice(cx, nav, p1, "lights-mode", t.row_lights, Some(body), &[t.mode_default, t.mode_on, t.mode_off], Some(idx), true);
        if mode != LightsMode::On {
            return LightsSection { rows, hue_knob };
        }

        let p2 = panel(cx, parent);
        let effect = lights::effect(cfg);
        let fx = match effect {
            Effect::Solid => 0,
            Effect::Breathe => 1,
            Effect::Cycle => 2,
        };
        let fx_body = (effect == Effect::Cycle).then_some(t.cycle_body);
        rows.choice(cx, nav, p2, "lights-effect", t.row_effect, fx_body, &[t.fx_solid, t.fx_breathe, t.fx_cycle], Some(fx), true);
        if effect != Effect::Cycle {
            let ri = match ring() {
                Ring::Both => 0,
                Ring::Left => 1,
                Ring::Right => 2,
            };
            rows.choice(cx, nav, p2, "lights-ring", t.row_rings, Some(t.rings_body), &[t.rings_both, t.rings_left, t.rings_right], Some(ri), false);
            // The swatches: a row of their own under the title, focusable one by one.
            let cur = edited(cfg);
            let row = setting_row(cx, p2, t.row_color, None, false);
            let sw = cx.scene.add(row.right, s().row().items_center().gap(0.7 * r).done(), Node::default());
            for (i, c) in PALETTE.iter().enumerate() {
                let on = cur == Some(*c);
                let d = 2.0 * r;
                let mut node = Node::bg(color(*c)).radius(d / 2.0);
                if on {
                    // The picked one: a white ring with a gap, as console color pickers do.
                    node = node.border(0.2 * r, tok::FG);
                }
                let id = cx.scene.add(sw, s().wh(d, d).done(), node);
                if on {
                    cx.scene.add(id, s().abs(Some(0.2 * r), Some(0.2 * r), Some(0.2 * r), Some(0.2 * r)).done(), Node::default().radius(d / 2.0).border(0.15 * r, tok::BG));
                }
                nav.add(Focusable { id, kind: Kind::Tile { scale: 1.15 }, group: 10, hint: tr().settings.choose.into(), tag: i as u32, sid: "lights-color".into() });
            }
            // Two colors on Both: the hue starts from the left ring's.
            let h = lights::hue_of(cur.unwrap_or(lights::colors(cfg).0));
            let h = (h / HUE_STEP).round() * HUE_STEP % 360.0;
            rows.slider(cx, nav, p2, "lights-hue", t.row_hue, None, h, 0.0, 355.0, HUE_STEP, "°", false);
            if let Some((_, sl)) = rows.sl.last() {
                hue_track(cx, sl.fill, sl.knob, h);
                hue_knob = Some(sl.knob);
            }
        }
        rows.slider(cx, nav, p2, "lights-brightness", t.row_brightness, None, lights::brightness(cfg) as f32, 10.0, 100.0, 10.0, "%", false);
        LightsSection { rows, hue_knob }
    }
}

/// The hue slider's track: the hue wheel left to right, no fill, a knob in the picked hue.
fn hue_track(cx: &mut Cx, fill: Id, knob: Id, h: f32) {
    let Some(track) = cx.scene.parent(fill) else { return };
    let mut g = LinearGradient::new(90.0).srgb();
    for i in 0..=6 {
        g = g.stop(color(lights::hue(i as f32 * 60.0)), i as f32 / 6.0);
    }
    cx.scene.n(track).bg.snap(Color::TRANSPARENT);
    cx.scene.n(track).lgrad = Some(g);
    cx.scene.n(fill).bg.snap(Color::TRANSPARENT);
    let n = cx.scene.n(knob);
    n.bg.snap(color(lights::hue(h)));
    n.border = 2.0;
    n.border_color = tok::FG;
}

impl SectionView for LightsSection {
    fn press(&mut self, cx: &mut Cx, nav: &mut Nav, cfg: &mut Config, now: f64) -> SectionOut {
        if let Some(f) = nav.current().filter(|f| f.sid == "lights-color").cloned() {
            if let Some(c) = PALETTE.get(f.tag as usize) {
                set_color(cfg, *c);
                return SectionOut::Changed;
            }
            return SectionOut::None;
        }
        match self.rows.press(cx, nav, now) {
            Some((sid, Val::Idx(i))) => {
                match sid.as_str() {
                    "lights-mode" => {
                        cfg.lights_mode = Some(["default", "on", "off"][i].into());
                        // First time on: start from the defaults so the rows show what the rings show.
                        if i == 1 && cfg.lights_left.is_none() {
                            let (l, r) = lights::colors(cfg);
                            cfg.lights_left = Some(lights::hex(l));
                            cfg.lights_right = Some(lights::hex(r));
                        }
                    }
                    "lights-effect" => cfg.lights_effect = Some(["solid", "breathe", "cycle"][i].into()),
                    "lights-ring" => {
                        RING.with(|r| r.set([Ring::Both, Ring::Left, Ring::Right][i]));
                        // Nothing saved changes, but the swatches and hue follow the ring.
                    }
                    _ => return SectionOut::None,
                }
                SectionOut::Changed
            }
            _ => SectionOut::None,
        }
    }

    fn nudge(&mut self, cx: &mut Cx, nav: &mut Nav, cfg: &mut Config, dir: i32, _now: f64) -> SectionOut {
        match self.rows.nudge(cx, nav, dir) {
            Some((sid, Val::Num(v))) => {
                match sid.as_str() {
                    "lights-hue" => {
                        let c = lights::hue(v);
                        set_color(cfg, c);
                        if let Some(k) = self.hue_knob {
                            cx.scene.n(k).bg.snap(color(c));
                        }
                    }
                    "lights-brightness" => cfg.lights_brightness = Some(v),
                    _ => return SectionOut::None,
                }
                SectionOut::Changed
            }
            Some(_) => SectionOut::None,
            None if nav.current().is_some_and(|f| self.rows.is_slider(&f.sid)) => SectionOut::None,
            None => SectionOut::Pass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_follow_the_ring_being_set() {
        let mut c = Config::default();
        RING.with(|r| r.set(Ring::Both));
        set_color(&mut c, PALETTE[6]);
        assert_eq!(c.lights_left.as_deref(), Some("ff2a2a"));
        assert_eq!(c.lights_right.as_deref(), Some("ff2a2a"));
        assert_eq!(edited(&c), Some(PALETTE[6]));
        RING.with(|r| r.set(Ring::Right));
        set_color(&mut c, PALETTE[1]);
        assert_eq!(c.lights_left.as_deref(), Some("ff2a2a"));
        assert_eq!(c.lights_right.as_deref(), Some("4da2ff"));
        RING.with(|r| r.set(Ring::Both));
        assert_eq!(edited(&c), None, "two colors: no swatch is both rings'");
    }
}
