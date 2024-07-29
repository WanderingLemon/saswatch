use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{block::Title, Block, Clear, Paragraph, Row, Table}, Frame};

use crate::app::{App, Mode};

const KEY_STYLE:Style = Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD);
const DESC_STYLE:Style = Style::new().fg(Color::White);

pub fn ui(f: &mut Frame, app: &mut App) {
    
    let main_layout = Layout::new(Direction::Vertical, [
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1)
    ]).split(f.size());

    if app.get_help_screen() {
        let help_table = Table::new([
            Row::new(["?","Toggle this help screen"]),
            Row::new(["q","Leaves sub-menu or quits the program when on main screen"]),
            Row::new(["k/Up","Move up by one color, looping at top"]),
            Row::new(["j/Down","Move down by one color, looping at bottom"]),
            Row::new(["K/Shift+Up","Move selected color up by one, looping at top"]),
            Row::new(["J/Shift+Down","Move selected color down by one, looping at bottom"]),
            Row::new(["a","Append a new, random color at the bottom of the list"]),
            Row::new(["s","Toggle the lock state of selected color"]),
            Row::new(["c","Copy the selected color's hex code to the system clipboard"]),
            Row::new(["Space","Reroll all unlocked colors"]),
        ], [
        Constraint::Length(16),
        Constraint::Fill(1)
        ])
            .block(Block::bordered()
                .title(Title::from("Help").alignment(Alignment::Center))
            );


        f.render_widget(help_table, 
            main_layout[0]
        );
        f.render_widget(Clear, main_layout[1]);
        f.render_widget(Block::new().on_black(), main_layout[2]);
        f.render_widget(Paragraph::new(Line::from(vec![
                    Span::styled("q", KEY_STYLE),
                    Span::styled(": Back  ", DESC_STYLE),
                    Span::styled("?", KEY_STYLE),
                    Span::styled(": Toggle help", DESC_STYLE)
        ])).on_black(), main_layout[1]);

    } else {
        let widths = [
            Constraint::Length(2),
            Constraint::Length(17),
            Constraint::Fill(1)
        ];

        let table = Table::new(app.get_colors(), widths)
            .widths(widths)
            .highlight_style(Style::new().bold().fg(Color::Cyan));

        f.render_stateful_widget(table, main_layout[0], app.get_table_state());
        f.render_widget(Clear, main_layout[1]);
        f.render_widget(Clear, main_layout[2]);
        f.render_widget(Paragraph::new(Line::from(vec![
                    Span::styled("k/\u{2191}", KEY_STYLE),
                    Span::styled(": Up  ", DESC_STYLE),
                    Span::styled("j/\u{2193}", KEY_STYLE),
                    Span::styled(": Down  ", DESC_STYLE),
                    Span::styled("s", KEY_STYLE),
                    Span::styled(": Toggle color lock  ", DESC_STYLE),
                    Span::styled("a", KEY_STYLE),
                    Span::styled(": Append new  ", DESC_STYLE),
                    Span::styled("c", KEY_STYLE),
                    Span::styled(": Copy hex  ", DESC_STYLE),
                    Span::styled("e", KEY_STYLE),
                    Span::styled(": Export  ", DESC_STYLE),
                    Span::styled("Space", KEY_STYLE),
                    Span::styled(": Reroll  ", DESC_STYLE),

        ])).on_black(), main_layout[1]);
        f.render_widget(Paragraph::new(Line::from(vec![
                    Span::styled("<S-Up>", KEY_STYLE),
                    Span::styled(": Move selected up  ", DESC_STYLE),
                    Span::styled("<S-Down>", KEY_STYLE),
                    Span::styled(": Move selected down  ", DESC_STYLE),
                    Span::styled("q", KEY_STYLE),
                    Span::styled(": Quit  ", DESC_STYLE),
                    Span::styled("?", KEY_STYLE),
                    Span::styled(": Help  ", DESC_STYLE),
        ])).on_black(), main_layout[2]);



        if *app.get_mode() == Mode::Exporting {
            f.render_widget(Clear, main_layout[1]);
            f.render_widget(Clear, main_layout[2]);
            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("ESC", KEY_STYLE),
                        Span::styled(": Back  ", DESC_STYLE),
                        Span::styled("Enter", KEY_STYLE),
                        Span::styled(": Export", DESC_STYLE)
            ])).on_black(), main_layout[1]);

            f.render_widget(Paragraph::new(Line::from(vec![
            Span::styled("Exporting to: ", Style::new().light_yellow()),
            Span::raw(format!("{}.sh",app.input_buffer.clone()))
        ]))
                .on_black().white(),
                main_layout[2])
        }
    }
}

