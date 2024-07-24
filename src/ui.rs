use ratatui::{layout::Constraint, style::{Color, Style, Stylize}, widgets::{Table, TableState}, Frame};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
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
