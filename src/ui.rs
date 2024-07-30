use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style, Stylize}, symbols::block, text::{Line, Span}, widgets::{block::Title, Block, Borders, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table}, Frame};

use crate::app::{App, Mode};

const KEY_STYLE:Style = Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD);
const DESC_STYLE:Style = Style::new().fg(Color::White);

pub fn ui(f: &mut Frame, app: &mut App) {
    
    let main_layout = Layout::new(Direction::Vertical, [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1)
    ]).split(f.size());
    
    f.render_widget(Paragraph::new("Saswatch")
        .bold()
        .centered()
        , 
        main_layout[0]);    

    let mode = app.get_mode();

    match mode{
        Mode::Generating => {
            let widths = [
            Constraint::Length(2),
            Constraint::Length(17),
            Constraint::Fill(1)
        ];
 
        let generator_layout = Layout::new(Direction::Horizontal, [
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).split(main_layout[1]);

        let colors = app.get_colors();

        let table = Table::new(colors, widths)
            .widths(widths)
            .highlight_style(Style::new().bold().fg(Color::Cyan));
        
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .thumb_symbol(block::FULL)
            .thumb_style(Style::new().gray())
            .track_symbol(Some(block::FULL))
            .track_style(Style::new().dark_gray());

        f.render_stateful_widget(scrollbar, 
            generator_layout[1].inner(Margin{
                vertical:0,
                horizontal:0 
            }),
            app.get_scrollbar_state()
        );

        f.render_stateful_widget(table, generator_layout[0], app.get_table_state());
        f.render_widget(Clear, main_layout[2]);
        f.render_widget(Clear, main_layout[3]);
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

        ])).on_black(), main_layout[2]);
        f.render_widget(Paragraph::new(Line::from(vec![
                    Span::styled("K/<S-\u{2191}>", KEY_STYLE),
                    Span::styled(": Move selected up  ", DESC_STYLE),
                    Span::styled("J/<S-\u{2193}>", KEY_STYLE),
                    Span::styled(": Move selected down  ", DESC_STYLE),
                    Span::styled("q", KEY_STYLE),
                    Span::styled(": Quit  ", DESC_STYLE),
                    Span::styled("?", KEY_STYLE),
                    Span::styled(": Help  ", DESC_STYLE),
        ])).on_black(), main_layout[3]);

        }

        Mode::Help => {
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
                Row::new(["e","Enter export mode for the current color palette, exports to .sh"]),
                Row::new(["Space","Reroll all unlocked colors"]),
            ], [
            Constraint::Length(16),
            Constraint::Fill(1)
            ])
                .block(Block::bordered()
                    .title(Title::from("Help").alignment(Alignment::Center))
                );

            f.render_widget(help_table, 
                main_layout[1]
            );

            f.render_widget(Clear, main_layout[2]);
            f.render_widget(Block::new().on_black(), main_layout[3]);
            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("q", KEY_STYLE),
                        Span::styled(": Back  ", DESC_STYLE),
                        Span::styled("?", KEY_STYLE),
                        Span::styled(": Toggle help", DESC_STYLE)
            ])).on_black(), main_layout[2]);

        }
        Mode::Exporting => {
            let widths = [
                Constraint::Length(2),
                Constraint::Length(17),
                Constraint::Fill(1)
            ];

            let generator_layout = Layout::new(Direction::Horizontal, [
                Constraint::Fill(1),
                Constraint::Length(2)
            ]).split(main_layout[1]);

            let colors = app.get_colors();
            //let colors_len = colors.len();

            let table = Table::new(colors, widths)
                .widths(widths)
                .highlight_style(Style::new().bold().fg(Color::Cyan));

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .thumb_symbol(block::FULL)
                .thumb_style(Style::new().gray())
                .track_symbol(Some(block::FULL))
                .track_style(Style::new().dark_gray());

            f.render_stateful_widget(scrollbar, 
                generator_layout[1].inner(Margin{
                    vertical:0,
                    horizontal:0 
                }),
                app.get_scrollbar_state()
            );

            f.render_stateful_widget(table, generator_layout[0], app.get_table_state());
            f.render_widget(Clear, main_layout[2]);
            f.render_widget(Clear, main_layout[3]);
            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("ESC", KEY_STYLE),
                        Span::styled(": Back  ", DESC_STYLE),
                        Span::styled("Enter", KEY_STYLE),
                        Span::styled(": Export", DESC_STYLE)
            ])).on_black(), main_layout[2]);

            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("Exporting to: ", Style::new().light_yellow()),
                        Span::raw(format!("{}.sh",app.input_buffer.clone()))
            ]))
                .on_black().white(),
                main_layout[3])}
    } 
}

