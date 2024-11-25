use lazy_static::lazy_static;
use ratatui::{style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Paragraph, Row}};

pub const KEY_STYLE:Style = Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD);
pub const DESC_STYLE:Style = Style::new().fg(Color::White);

lazy_static!{
    pub static ref HELPSCREEN_ENTRIES: Vec<Row<'static>> = vec![
        Row::new(["?","Toggle this help screen"]),
        Row::new(["q","Leaves sub-menu or quits the program when on main screen"]),
        Row::new(["k/Up","Move up by one color, looping at top"]),
        Row::new(["j/Down","Move down by one color, looping at bottom"]),
        Row::new(["K/Shift+Up","Move selected color up by one, looping at top"]),
        Row::new(["J/Shift+Down","Move selected color down by one, looping at bottom"]),
        Row::new(["a","Append a new, random color at the bottom of the list"]),
        Row::new(["d","Remove the selected color"]),
        Row::new(["s","Toggle the lock state of selected color"]),
        Row::new(["c","Copy the selected color's hex code to the system clipboard"]),
        Row::new(["e","Enter export mode for the current color palette, exports to .sh"]),
        Row::new(["Space","Reroll all unlocked colors"]),
    ];
}


pub const GENERATING_HELP_LINE_1: [(&str, &str); 7] = [
    ("k/\u{2191}", ": Up  "),
    ("j/\u{2193}", ": Down  "),
    ("s", ": Toggle color lock  "),
    ("a", ": Append new  "),
    ("c", ": Copy hex  "),
    ("e", ": Export  "),
    ("Space", ": Reroll  ")
];

pub const GENERATING_HELP_LINE_2: [(&str, &str); 4] = [
    ("K/<S-\u{2191}>", ": Move selected up  "),
    ("J/<S-\u{2193}>", ": Move selected down  "),
    ("q", ": Quit  "),
    ("?", ": Help  ")
];

pub const EXPORTING_HELP_LINE: [(&str, &str); 2] = [
    ("ESC", ": Back  "),
    ("Enter", ": Export")
];

pub const HELP_HELP_LINE_1: [(&str, &str); 2] = [
    ("q", ": Back  "),
    ("?", ": Toggle help ")
];

pub fn to_paragraph<'a>(v: Vec<(&'a str, &'a str)>) -> Paragraph<'a> {
    let mut pvec = Vec::new();   
    for entry in v {
        pvec.push(Span::styled(entry.0, KEY_STYLE));
        pvec.push(Span::styled(entry.1, DESC_STYLE));
    }
    Paragraph::new(Line::from(pvec))
}
