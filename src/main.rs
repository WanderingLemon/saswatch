mod ui;
mod app;
mod color;
mod files;
mod helptext;

use std::{
    io, 
    panic::{set_hook, take_hook}
};

use app::App;
use ratatui::{init, restore, try_restore};
use ui::ui;

fn main() -> io::Result<()> {
    init_panic_hook();
    let app = App::new();
    let mut terminal = init();

    let result = app?.run(&mut terminal);
    if let Err(err) = try_restore(){
        eprintln!("Failed to restore the terminal! You may need to restore it yourself!: {err}")
    }
    result
}

fn init_panic_hook(){
    let original_hook= take_hook();
    set_hook(Box::new(move |info|{
        restore(); 
        original_hook(info);
    }));
}
