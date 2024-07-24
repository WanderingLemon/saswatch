mod ui;
mod app;
mod color;

use std::{collections::hash_map::Keys, error::Error, io};

use app::App;
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stderr();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>{
    loop {
        if let Err(_res) = terminal.draw(|f| ui(f, app)) {
            return Ok(false)
        }

        if let Event::Key(key) = event::read()? {
            
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key{
                KeyEvent{code: KeyCode::Char('q'),..}=> {
                    return Ok(true);
                }
                KeyEvent{code: KeyCode::Down,modifiers: KeyModifiers::SHIFT,..} | KeyEvent{code: KeyCode::Char('J'),..}=> {
                    app.shift_down()
                }
                KeyEvent{code: KeyCode::Up,modifiers: KeyModifiers::SHIFT,..} | KeyEvent{code: KeyCode::Char('K'),..}=> {
                    app.shift_up()
                }
                KeyEvent{code: KeyCode::Up,..} | KeyEvent{code: KeyCode::Char('k'),..}=> {
                    app.dec_offset()
                }
                KeyEvent{code: KeyCode::Down,..} | KeyEvent{code: KeyCode::Char('j'),..}=> {
                    app.inc_offset()
                }
                KeyEvent{code: KeyCode::Char('a'),..}=> {
                    app.insert_color()
                }
                KeyEvent{code: KeyCode::Char('d'),..} => {
                    app.remove_color()
                }
                KeyEvent{code: KeyCode::Char('s'),..} => {
                    app.toggle_lock()
                }
                KeyEvent{code: KeyCode::Char(' '),..} => {
                    app.regen_unlocked()
                }
                _ => {}
            }
        }
    }
}
