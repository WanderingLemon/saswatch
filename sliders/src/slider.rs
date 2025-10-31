use std::ops::Range;

use ratatui::{
    style::Style,
    text::Line,
    widgets::{Widget, WidgetRef},
};

pub struct Slider<'a> {
    position: u16,
    range: Range<u16>,
    label: Option<Line<'a>>,
    show_value: bool,
    thumb_style: Style,
    track_style: Style,
    value_style: Style,
}

impl<'a> Slider<'a> {
    pub fn new(position: u16, range: Range<u16>) -> Self {
        Slider {
            position,
            range,
            label: None,
            show_value: false,
            thumb_style: Style::default(),
            track_style: Style::default(),
            value_style: Style::default(),
        }
    }
    /// Increment the position in range. Does nothing if it hits the end value of the range
    /// (exclusive).
    pub fn increment(&mut self) {
        let current = self.position;
        if current < (self.range.end - 1) {
            self.position = current.saturating_add(1);
        }
    }

    /// Decrement the position in range. Does nothing if it hits the start value of the range.
    pub fn decrement(&mut self) {
        let current = self.position;
        if current > self.range.start {
            self.position = current.saturating_sub(1);
        }
    }

    /// A fluent setter for the label of the slider.
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    /// A fluent setter for the position of the slider.
    pub fn position(mut self, position: u16) -> Self {
        self.position = position;
        self
    }

    /// A fluent setter for the range of the slider.
    pub fn range(mut self, range: Range<u16>) -> Self {
        self.range = range;
        self
    }

    /// A fluent setter for whether or not to show the value of the slider.
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// A fluent setter for the  of the slider.
    pub fn thumb_style(mut self, style: Style) -> Self {
        self.thumb_style = style;
        self
    }

    /// A fluent setter for the track style of the slider.
    pub fn track_style(mut self, style: Style) -> Self {
        self.track_style = style;
        self
    }

    /// A fluent setter for the value style of the slider.
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }
}

impl Widget for Slider<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        WidgetRef::render_ref(&self, area, buf);
    }
}

impl WidgetRef for Slider<'_> {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let width = area.width;
        let position = self.position;

        // Render the label
        let default_label = Line::from("");
        let label = self.label.as_ref().unwrap_or(&default_label);
        let (mut col, mut row) = buf.set_line(0, 0, label, width);

        if self.show_value {
            (col, row) = buf.set_line(
                col + 1,
                row,
                &Line::from(format!("{:1$}", position, self.range.end.ilog10() as usize)),
                width,
            )
        }

        let range_len = self.range.len() as u16;
        let delta: f32 = f32::from(width - (col + 1)) / range_len as f32;

        let location = if width > range_len {
            f32::from((position as f32) * delta)
        } else {
            f32::from((position as f32) * delta)
        };

        let start = col + 1;
        let track_style = self.track_style;
        let thumb_style = self.thumb_style;
        for col in start..width {
            buf[(col, 0)].set_symbol("\u{2550}").set_style(track_style);
        }
        buf[(location as u16, 0)]
            .set_symbol("\u{21d2}")
            .set_style(thumb_style);
    }
}
