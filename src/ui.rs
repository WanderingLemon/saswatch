use ratatui::{layout::{Alignment, Constraint}, style::{Color, Style, Stylize}, widgets::{block::Title, Block, Row, Table, TableState}, Frame};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {

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
                Row::new(["Space","Reroll all unlocked colors"]),
            ], [
                Constraint::Length(16),
                Constraint::Fill(1)
            ])
            .block(Block::bordered()
                .title(Title::from("Help").alignment(Alignment::Center))
            ), 
            f.size()
        )
    } else {
        let widths = [
            Constraint::Length(2),
            Constraint::Length(17),
            Constraint::Fill(1)
        ];
        //let rows = app.get_colors();
        let table = Table::new(app.get_colors(), widths)
            .widths(widths)
            .highlight_style(Style::new().bold().fg(Color::Cyan));

        // Note: TableState should be stored in your application state (not constructed in your render
        // method) so that the selected row is preserved across renders
            let mut table_state = TableState::default();
            table_state.select(Some(app.get_offset())); // select the forth row (0-indexed)
            f.render_stateful_widget(table, f.size(), &mut table_state);

    }
}
