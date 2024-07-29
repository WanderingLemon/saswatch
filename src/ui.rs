use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{block::Title, Block, Paragraph, Row, Table, TableState}, Frame};

use crate::app::{App, Mode};

pub fn ui(f: &mut Frame, app: &App) {
    
    let main_layout = Layout::new(Direction::Vertical, [
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1)
    ]).split(f.size());

    f.render_widget(Block::default().on_dark_gray(), main_layout[1]);
    f.render_widget(Block::default().on_dark_gray(), main_layout[2]);
    
    if app.get_help_screen() {
        
        f.render_widget(Table::new([
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
            ), 
            main_layout[0]
        )
    } else {
        let widths = [
            Constraint::Length(2),
            Constraint::Length(17),
            Constraint::Fill(1)
        ];

        let table = Table::new(app.get_colors(), widths)
            .widths(widths)
            .highlight_style(Style::new().bold().fg(Color::Cyan));

        // Note: TableState should be stored in your application state (not constructed in your render
        // method) so that the selected row is preserved across renders
        let mut table_state = TableState::default();
        table_state.select(Some(app.get_offset())); // select the forth row (0-indexed)
        f.render_stateful_widget(table, main_layout[0], &mut table_state);
            
        if *app.get_mode() == Mode::Exporting {
            f.render_widget(Paragraph::new(Line::from(vec![
                        Span::styled("ESC", Style::new().light_red().bold()),
                        Span::raw(": Back  "),
                        Span::styled("Enter", Style::new().light_red().bold()),
                        Span::raw(": Export")
            ])), main_layout[1]);

            f.render_widget(Paragraph::new(Line::from(vec![
            Span::styled("Exporting to: ", Style::new().light_yellow()),
            Span::raw(format!("{}.sh",app.input_buffer.clone()))
        ]))
                .on_dark_gray()
                .white(), 
                main_layout[2])
        }
    }
}

