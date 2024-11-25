use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect}, 
    style::{Color, Style, Stylize}, 
    symbols::block, 
    text::{Line, Span}, 
    widgets::{Block, Clear, Paragraph, Scrollbar, ScrollbarOrientation, Table},
    Frame
};

use crate::{
    app::{App, Mode},
    helptext::{to_paragraph, EXPORTING_HELP_LINE, GENERATING_HELP_LINE_1, GENERATING_HELP_LINE_2, HELPSCREEN_ENTRIES, HELP_HELP_LINE_1}
};

pub fn ui(f: &mut Frame, app: &mut App) {

    let main_layout = Layout::new(Direction::Vertical, [
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1)
    ]).split(f.area());

    let mode = app.get_mode();

    match mode{
        Mode::Generating => {
            render_main_section(f, app, main_layout[0]);
            f.render_widget(Clear, main_layout[1]);
            f.render_widget(Clear, main_layout[2]);
            f.render_widget(to_paragraph(GENERATING_HELP_LINE_1.to_vec()).on_black(), main_layout[1]);
            f.render_widget(to_paragraph(GENERATING_HELP_LINE_2.to_vec()).on_black(), main_layout[2]);
        }

        Mode::Help => {
            let help_table = Table::new(HELPSCREEN_ENTRIES.clone(), [
            Constraint::Length(16),
            Constraint::Fill(1)
            ])
                .block(Block::bordered()
                    .title_top(Line::from("Help").centered())
                );

            f.render_widget(help_table, 
                main_layout[0]
            );

            f.render_widget(Clear, main_layout[1]);
            f.render_widget(to_paragraph(HELP_HELP_LINE_1.to_vec()).on_black(), main_layout[1]);
            f.render_widget(Block::new().on_black(), main_layout[2]);

        }
        Mode::Exporting => {
            render_main_section(f, app, main_layout[0]);
            f.render_widget(Clear, main_layout[1]);
            f.render_widget(Clear, main_layout[2]);
            f.render_widget(to_paragraph(EXPORTING_HELP_LINE.to_vec()).on_black(), main_layout[1]);

            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("Exporting to: ", Style::new().light_yellow()),
                        Span::raw(format!("{}.sh",app.input_buffer.clone()))
            ]))
                .on_black().white(),
                main_layout[2])
        }
    }
}

fn render_main_section(f: &mut Frame, app: &mut App, area: Rect) {
    let blk = Block::default().title_top(Line::from("Saswatch").centered());
    let widths = [
        Constraint::Length(2),
        Constraint::Length(20),
        Constraint::Fill(1)
    ];

    let generator_layout = Layout::new(Direction::Horizontal, [
        Constraint::Fill(1),
        Constraint::Length(2)
    ]).split(blk.inner(area));

    let colors = app.get_colors();

    let table = Table::new(colors, widths)
        .widths(widths)
        .cell_highlight_style(Style::new().bold().fg(Color::Cyan));

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
    f.render_widget(blk, area);
}
