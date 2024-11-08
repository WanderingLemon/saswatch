mod ui;
mod app;
mod color;
mod files;

use core::panic;
use std::{error::Error, io::{self}};

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture}, 
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    if app.is_err() {
        panic!("Encountered an error instantiating the application");
    }
    
    enable_raw_mode()?;
    let mut stdout = io::stderr();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app?.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    match result{
        Err(..) => {
            println!("There was an error!");
        },
        _ =>{}
    }

    Ok(())
}
