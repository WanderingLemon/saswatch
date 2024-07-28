mod ui;
mod app;
mod color;
mod files;

use core::panic;
use std::{error::Error, io::{self}};

use app::{App, Mode};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
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

    let result = run_app(&mut terminal, &mut app.unwrap());

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>{
    loop {
        if let Err(_res) = terminal.draw(|f| ui(f, app)) {
            return Ok(false)
        }

        if let Event::Key(key) = event::read()? {
            
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.get_mode() {
                Mode::Generating => {
                    match key{
                        KeyEvent{code: KeyCode::Char('q'),..}=> {
                            if app.get_help_screen() {
                                app.toggle_help()
                            } else {
                                return Ok(true);
                            }
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
                        KeyEvent{code: KeyCode::Char('c'),..} => {
                            app.copy_hex()
                        }
                        KeyEvent{code: KeyCode::Char('?'),..} => {
                            app.toggle_help()
                        }
                        KeyEvent{code: KeyCode::Char('e'),..} => {
                            app.toggle_export_menu()
                        }
                        KeyEvent{code: KeyCode::Char(' '),..} => {
                            app.regen_unlocked()
                        }
                        _ => {}
                    }
                }
                Mode::Exporting => {
                    match key.code {
                        KeyCode::Esc => {
                            app.toggle_export_menu();
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            app.export_to_sh()?;
                        }
                        KeyCode::Char(ch) => {
                            app.input_buffer.push(ch)
                        }
                        _ =>{}
                    }
                }   
            }
       }
    }
}
