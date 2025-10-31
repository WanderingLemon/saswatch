use std::ops::Range;

use ratatui::{style::Style, text::Line};

pub struct Slider<'a> {
    position: usize,
    range: Range<usize>,
    label: Option<Line<'a>>,
    show_value: bool,
    thumb_style: Style,
    track_style: Style,
    value_style: Style,
}

impl<'a> Slider<'a> {
    pub fn new(position: usize, range: Range<usize>) -> Self {
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
    pub fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// A fluent setter for the range of the slider.
    pub fn range(mut self, range: Range<usize>) -> Self {
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
