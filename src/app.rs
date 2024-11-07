use std::{fs::{create_dir, create_dir_all, File}, io::{self, BufWriter, Result, Write}};

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use directories::ProjectDirs;
use ratatui::{prelude::Backend, widgets::{ScrollbarState, TableState}, Terminal};

use crate::{color::{Color, Constraints}, ui};
 
#[derive(PartialEq)]
pub enum Mode {
    Generating,
    Help,
    Exporting,
}

pub struct App {
    clipboard_ctx: ClipboardContext,
    app_directories: ProjectDirs,
    pub input_buffer: String,
    mode: Mode,
    colors: Vec<Color>,
    color_table_state: TableState,
    scrollbar_state: ScrollbarState,
    pub constraints: Constraints
}

impl App {
    pub fn new() -> Result<App> {
        let mut color_table_state = TableState::default();
        color_table_state.select(Some(0)); 
        let constraints = Constraints::new();
        let colors = Vec::from([Color::random_new(constraints.clone())]);
        let app_directories = ProjectDirs::from("dev", "Corax", "Saswatch").expect("Failed to get program directories");
        
        let data_dir = app_directories.data_dir();
        if !data_dir.exists() {
            create_dir_all(data_dir)?;
        }
        
        let palette_dir = data_dir.join("palettes");
        if !palette_dir.exists() {
            create_dir(data_dir.join("palettes"))?;
        }
        let scrollbar_state = ScrollbarState::new(1)
            .position(0);
        Ok(App {
            clipboard_ctx: ClipboardProvider::new().unwrap(),
            app_directories,
            input_buffer: String::new(),
            mode: Mode::Generating,
            colors,
            color_table_state,
            scrollbar_state,
            constraints
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>)-> io::Result<bool> {
        loop {
            if let Err(_res) = terminal.draw(|f| ui(f, self)){
                return Ok(false)
}

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
               }

                if let Some(result) = self.handle_input(key){
                    return result
                }
            
            }    
        }
    }
    
    pub fn handle_input(&mut self, key: KeyEvent) -> Option<io::Result<bool>> {
                match self.mode {
                    Mode::Generating => {
                        match key {
                            KeyEvent{code: KeyCode::Char('q'), ..} => { return Some(Ok(true))}

                        KeyEvent{code: KeyCode::Down,modifiers: KeyModifiers::SHIFT,..} | KeyEvent{code: KeyCode::Char('J'),..}=> {
                            self.shift_down()
                        }

                        KeyEvent{code: KeyCode::Up,modifiers: KeyModifiers::SHIFT,..} | KeyEvent{code: KeyCode::Char('K'),..}=> {
                            self.shift_up()
                        }
                    
                        KeyEvent{code: KeyCode::Up,..} | KeyEvent{code: KeyCode::Char('k'),..}=> {
                            self.dec_select()
                        }

                        KeyEvent{code: KeyCode::Down,..} | KeyEvent{code: KeyCode::Char('j'),..}=> {
                            self.inc_select()
                        }

                        KeyEvent{code: KeyCode::Char('a'),..}=> {
                            self.insert_color()
                        }

                        KeyEvent{code: KeyCode::Char('d'),..} => {
                            self.remove_color()
                        }
                        
                        KeyEvent{code: KeyCode::Char('s'),..} => {
                            self.toggle_lock()
                        }

                        KeyEvent{code: KeyCode::Char('c'),..} => {
                            self.copy_hex()
                        }

                        KeyEvent{code: KeyCode::Char('?'),..} => {
                            self.mode = Mode::Help
                        }

                        KeyEvent{code: KeyCode::Char('e'),..} => {
                            self.mode = Mode::Exporting
                        }

                        KeyEvent{code: KeyCode::Char(' '),..} => {
                            self.regen_unlocked()
                        }
                            _ => {}
                        }
                    }

                    Mode::Help => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('?') => {
                                self.mode = Mode::Generating
                            }

                            _ => {}
                        }
                    }

                    Mode::Exporting => {
                        match key.code {
                            KeyCode::Esc => {
                                self.input_buffer = String::new();
                                self.mode = Mode::Generating;
                            }

                            KeyCode::Backspace => {
                                self.input_buffer.pop();
                            }

                            KeyCode::Enter => {
                                let result = self.export_to_sh();
                                if result.is_err() {
                                    return Some(Ok(false))
                                }
                            }

                            KeyCode::Char(ch) => {
                                self.input_buffer.push(ch);
                            }

                            _ => {}
                        }
                    }
                }
        return None
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }
    
    pub fn inc_select(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if selected < entries-1 {
            self.color_table_state.select(Some(selected+1));
            self.scrollbar_state.next();
        }else{
            self.color_table_state.select(Some(0));
            self.scrollbar_state.first();
        }
    }
    
    pub fn dec_select(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if selected > 0 {
            self.color_table_state.select(Some(selected-1));
            self.scrollbar_state.prev();
        }else {
            self.color_table_state.select(Some(entries-1));
            self.scrollbar_state.last();
        }
    }
    
    pub fn insert_color(&mut self) {
        let constraints = Constraints::new();
        self.colors.push(Color::random_new(constraints));
        self.scrollbar_state = self.scrollbar_state.content_length(self.colors.len());
    }

    pub fn remove_color(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if entries > 1{
            self.colors.remove(selected);
            self.scrollbar_state = self.scrollbar_state.content_length(self.colors.len());
            if selected == entries-1{
                self.dec_select();
                self.scrollbar_state.prev();
            }
        }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        self.colors.to_owned()
    }

    pub fn shift_up(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if entries <= 1{
            return
        }

        if selected == 0 {
            self.colors.swap(selected, entries-1);
            self.color_table_state.select(Some(entries-1));
            self.scrollbar_state.last();
       } else {
            self.colors.swap(selected, selected-1);
            self.color_table_state.select(Some(selected-1));
            self.scrollbar_state.prev();
        }
    }

    pub fn shift_down(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if entries <= 1{
            return
        }
        
        if selected < entries-1 {
            self.colors.swap(selected, selected+1);
            self.color_table_state.select(Some(selected+1));
            self.scrollbar_state.next();
        } else {
            self.colors.swap(selected, 0);
            self.color_table_state.select(Some(0));
            self.scrollbar_state.first();
        }
    }

    pub fn toggle_lock(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let color = self.colors.get_mut(selected).unwrap();
        if !color.locked {
            color.locked = true;
        } else {
            color.locked = false; 
        }
    }

    pub fn regen_unlocked(&mut self) {
        let colors = self.colors.iter_mut();
        for color in colors {
            if !color.locked{
                color.regen(self.constraints.clone());
            }
        }
    }
    
    pub fn copy_hex(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let color = self.colors.get(selected).unwrap();
        let hex = color.hex_string();
        let _ = self.clipboard_ctx.set_contents(hex);
    }

    pub fn export_to_sh(&mut self) -> Result<()>{
        let pallets = self.app_directories.data_dir().join("palettes");
        let file = File::create(pallets.join(format!("{}.sh", self.input_buffer)))?;
        let mut writer = BufWriter::new(file);

        let mut counter = 0;
        for color in self.colors.to_owned() {
            writer.write_fmt(format_args!("color{}=\"{}\"\n", counter, color.hex_string()))?;
            counter += 1;
        }
        
        self.input_buffer = String::new();
        self.mode = Mode::Generating;

        Ok(())
    }

    pub fn get_table_state(&mut self) -> &mut TableState {
        &mut self.color_table_state
    }

    pub fn get_scrollbar_state(&mut self) -> &mut ScrollbarState {
        &mut self.scrollbar_state
    }
} 
