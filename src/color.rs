use palette::Srgb;
use rand::random;
use ratatui::{style::Style, widgets::{Cell, Row}};

#[derive(Clone)]
pub struct Color {
    pub rgb: Srgb<u8>,
    pub locked: bool
}

impl Color {
    pub fn random_new() -> Self {
         Self {
            rgb: Srgb::new(random::<u8>(), random::<u8>(), random::<u8>()),
            locked: false
        }
    }
}

impl Into<Row<'_>> for Color {
    fn into(self) -> Row<'static> {
        let rgb = self.rgb;
        let lock_icon = if self.locked {"\u{1f512}"} else {"\u{1f513}"};
        let cell1 = Cell::from(lock_icon);
        let cell2 = Cell::from(format!("rgb({},{},{})\n#{:x}",rgb.red,rgb.green,rgb.blue, rgb));
        let cell3 = Cell::from("").style(Style::default().bg(rgb.into()));
        Row::new([cell1,cell2,cell3]).height(2)
    }
}