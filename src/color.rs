use std::ops::Range;

use palette::{
    convert::{FromColorUnclamped, TryIntoColor},
    DarkenAssign, DesaturateAssign, Hsl, IntoColor, IsWithinBounds, LightenAssign, Okhsl, OklabHue,
    SaturateAssign, ShiftHueAssign, Srgb,
};
use rand::Rng;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Cell, Row},
};

#[derive(Clone)]
pub struct Constraints {
    hue: Range<f32>,
    saturation: Range<f32>,
    lightness: Range<f32>,
}

impl Default for Constraints {
    fn default() -> Self {
        Self::new()
    }
}

impl Constraints {
    pub fn new() -> Self {
        Self {
            hue: (0.0..360.0),
            saturation: (0.0..1.0),
            lightness: (0.0..1.0),
        }
    }
}

#[derive(Clone)]
pub struct Color {
    pub okhsl: Okhsl<f32>,
    pub locked: bool,
}

impl Color {
    /// Create a new instance of Color from another palette color format.
    /// If unable to safely convert within the colorspace, convert in a
    /// possibly lossy way.
    pub fn new<T>(color: T) -> Self
    where
        T: TryIntoColor<Okhsl> + IntoColor<Okhsl> + Copy,
    {
        if let Ok(okhsl) = color.try_into_color() {
            Self {
                okhsl,
                locked: false,
            }
        } else {
            // TODO: Emit a warning to the log if we get here

            Self {
                okhsl: color.into_color(),
                locked: false,
            }
        }
    }

    /// Create a new instance of Color with a random color within the passed constraints
    pub fn random_new(constraints: Constraints) -> Self {
        let mut rng = rand::thread_rng();
        let okhsl = Okhsl::new(
            OklabHue::new(rng.gen_range(constraints.hue)),
            rng.gen_range(constraints.saturation),
            rng.gen_range(constraints.lightness),
        );
        Self {
            okhsl,
            locked: false,
        }
    }

    /// Randomize the color within some given constraints
    pub fn regen(&mut self, constraints: Constraints) {
        let mut rng = rand::thread_rng();
        self.okhsl = Okhsl::new(
            OklabHue::new(rng.gen_range(constraints.hue)),
            rng.gen_range(constraints.saturation),
            rng.gen_range(constraints.lightness),
        );
    }

    /// Convert the current OkHSL value to a hex code for display purposes
    pub fn hex_string(&self) -> String {
        let rgb: Srgb<u8> = Srgb::from_color_unclamped(self.okhsl).into();
        format!("#{:x}", rgb)
    }

    /// Wrapper function for palette's saturate_fixed_assign
    pub fn saturate(&mut self, amount: f32) {
        self.okhsl.saturate_fixed_assign(amount);
    }

    /// Wrapper function for palette's desaturate_fixed_assign
    pub fn desaturate(&mut self, amount: f32) {
        self.okhsl.desaturate_fixed_assign(amount);
    }

    /// Wrapper function for palette's lighten_fixed_assign
    pub fn lighten(&mut self, amount: f32) {
        self.okhsl.lighten_fixed_assign(amount);
    }

    /// Wrapper function for palette's darken_fixed_assign
    pub fn darken(&mut self, amount: f32) {
        self.okhsl.darken_fixed_assign(amount);
    }

    /// Wrapper function for palette's hue_shift_assign
    pub fn rotate(&mut self, degrees: f32) {
        self.okhsl.shift_hue_assign(degrees);
    }
}

impl<'a> From<Color> for Row<'a> {
    fn from(val: Color) -> Self {
        let rgb: Srgb<u8> = Srgb::from_color_unclamped(val.okhsl).into();
        let hsl = val.okhsl;
        let lock_icon = if val.locked { "\u{1f512}" } else { "\u{1f513}" };
        let cell1 = Cell::from(lock_icon.yellow());
        let cell2 = Cell::from(format!(
            "hsl({:.0},{:.2},{:.2})\n#{:x}",
            hsl.hue.into_inner(),
            hsl.saturation,
            hsl.lightness,
            rgb
        ))
        .white();
        let cell3 = Cell::from("").style(Style::default().bg(rgb.into()));
        Row::new([cell1, cell2, cell3]).height(2)
    }
}
