mod ui;
mod app;
mod color;
mod files;
mod helptext;

use std::panic::{set_hook, take_hook};

use app::App;
use better_panic::Settings;
use ratatui::{init, restore, try_restore};
use ui::ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_panic_hook();
    let app = App::new();
    let mut terminal = init();

    let result = app?.run(&mut terminal);
    if let Err(err) = try_restore(){
        eprintln!("Failed to restore the terminal! You may need to restore it yourself!: {err}")
    }
    Ok(result?)
}

fn init_panic_hook(){
    let original_hook= take_hook();

    set_hook(Box::new(move |info|{
        restore(); 
        original_hook(info);
        Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(info);
    }));
}
