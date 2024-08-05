use std::ops::Range;

use palette::{convert::FromColorUnclamped, Okhsl, OklabHue, Srgb};
use rand::Rng;
use ratatui::{style::{Style, Stylize}, widgets::{Cell, Row}};

#[derive(Clone)]
pub struct Constraints {
    hue: Range<f32>,
    saturation: Range<f32>,
    lightness: Range<f32>
}

impl Constraints {
    pub fn new() -> Self {
        Self {
            hue: (0.0..360.0),
            saturation: (0.0..1.0),
            lightness: (0.0..1.0)
        }
    }
}

#[derive(Clone)]
pub struct Color {
    pub okhsl: Okhsl<f32>,
    pub locked: bool,
}

impl Color {
    pub fn random_new(constraints: Constraints) -> Self {
        let mut rng = rand::thread_rng();
        let okhsl = Okhsl::new(OklabHue::new(rng.gen_range(constraints.hue)), rng.gen_range(constraints.saturation), rng.gen_range(constraints.lightness));
        Self {
            okhsl,
            locked: false,
        }
    }

    pub fn regen(&mut self, constraints: Constraints){
        let mut rng = rand::thread_rng();
        self.okhsl = Okhsl::new(
            OklabHue::new(rng.gen_range(constraints.hue)),
            rng.gen_range(constraints.saturation),
            rng.gen_range(constraints.lightness)
        );
    }

    pub fn hex_string(&self) -> String {
        let rgb: Srgb<u8> = Srgb::from_color_unclamped(self.okhsl).into();
        format!("#{:x}",rgb)
    }
}

impl Into<Row<'_>> for Color {
    fn into(self) -> Row<'static> {
        let rgb: Srgb<u8> = Srgb::from_color_unclamped(self.okhsl).into();
        let hsl = self.okhsl;
        let lock_icon = if self.locked {"\u{1f512}"} else {"\u{1f513}"};
        let cell1 = Cell::from(lock_icon.yellow());
        let cell2 = Cell::from(format!("hsl({:.0},{:.2},{:.2})\n#{:x}", hsl.hue.into_inner(), hsl.saturation, hsl.lightness, rgb)).white();
        let cell3 = Cell::from("").style(Style::default().bg(rgb.into()));
        Row::new([cell1,cell2,cell3]).height(2)
    }
}
