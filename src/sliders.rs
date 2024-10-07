use ratatui::{buffer::Buffer, layout::Rect, style::{Style, Styled}, text::Line, widgets::{StatefulWidget, StatefulWidgetRef, Widget, WidgetRef}};


#[derive(Debug, PartialEq, Clone, Default)]
pub struct SliderState {
    position: u16,
    range: (u16, u16)
}

impl SliderState {
    pub fn new(position: u16, range: (u16, u16)) -> Self {
        Self { position, range }
    }

    pub fn increment(&mut self){
        let current = self.position;
        if current < (self.range.1-1){
            self.position = current.saturating_add(1) ;
        }
    }

    pub fn decrement(&mut self){
        self.position = self.position.saturating_sub(1);
    }
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Slider {
    position: u16,      
    range: (u16, u16),
    label: Option<String>,
    label_style: Style,
    value_style: Style,
    show_value: bool,
    track_style: Style,
    thumb_style: Style
}


impl Slider {
    pub fn new(position: u16, range: (u16, u16)) -> Self {
        Slider {
            position,
            range,
            label: None,
            label_style: Style::default(),
            value_style: Style::default(),
            show_value: true,
            track_style: Style::default(),
            thumb_style: Style::default()
        }
    }

    pub fn position(&self) -> u16 {
        self.position
    }

    pub fn show_value(&self) -> bool {
        self.show_value
    }

    pub fn range(&self) -> (u16, u16) {
        self.range
    }

    pub fn label(&self) -> &Option<String> {
        &self.label
    }

    pub fn set_position(&mut self, position: u16) {
        self.position = position;
    }

    pub fn set_range(&mut self, range: (u16,u16)) {
        self.range = range;
    }

    pub fn set_show_value(&mut self, value: bool) {
        self.show_value = value;
    }

    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn label_style(&mut self, style: Style) {
        self.label_style = style;
    }
 
    pub fn value_style(&mut self, style: Style) {
        self.value_style = style;
    }
 
    pub fn track_style(&mut self, style: Style) {
        self.track_style = style;
    }
 
    pub fn thumb_style(&mut self, style: Style) {
        self.thumb_style = style;
    }

}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized {
        WidgetRef::render_ref(&self, area, buf);
    }
}

impl WidgetRef for Slider{
    fn render_ref(&self,area:Rect,buf: &mut Buffer) {
        if self.position != 0{
            StatefulWidgetRef::render_ref(self, area, buf, &mut SliderState::new(self.position, self.range));
        } else {
            StatefulWidgetRef::render_ref(self, area, buf, &mut SliderState::new(0, self.range));
        }
    }
}

impl StatefulWidget for Slider{
    type State = SliderState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
       StatefulWidgetRef::render_ref(&self, area, buf, state); 
    }
}

impl StatefulWidgetRef for Slider{
    type State = SliderState;
    fn render_ref(&self,area:Rect,buf: &mut Buffer,state: &mut Self::State) {
        let width = area.width;
        let mut row = 0;

        let range_len = state.range.1 - state.range.0;
        
        let position = state.position;
        
        if let Some(string) = &self.label {
           buf.set_line(0, row, &Line::from(string.as_str()).set_style(self.label_style), width);
           row += 1;
        }

        let delta: f32 = width as f32/range_len as f32; 
        let mut location = position as f32 * delta;
        
        if width > range_len {
            location = location.ceil();
        } else {
            location = location.floor();
        }
        
        for ch in 0..width {
            buf[(ch, row)].set_style(self.track_style);
            buf[(ch, row)].set_symbol("\u{2501}"); 
        }

        buf[(location as u16, row)].set_style(self.thumb_style);
        buf[(location as u16, row)].set_symbol("\u{254b}");

        row += 1;
        buf.set_line(0, row, &Line::from(format!("{}", state.position).set_style(self.value_style)), 10);
    }
}
