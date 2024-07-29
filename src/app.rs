use std::{fs::{create_dir, create_dir_all, File}, io::{BufWriter, Result, Write}};

use clipboard::{ClipboardContext, ClipboardProvider};
use directories::ProjectDirs;
use ratatui::widgets::TableState;

use crate::color::Color;

#[derive(PartialEq)]
pub enum Mode {
    Generating,
    Exporting
}

pub struct App {
    clipboard_ctx: ClipboardContext,
    app_directories: ProjectDirs,
    pub input_buffer: String,
    mode: Mode,
    help_screen: bool,
    colors: Vec<Color>,
    color_table_state: TableState
}

impl App {
    pub fn new() -> Result<App> {
        let mut color_table_state = TableState::default();
        color_table_state.select(Some(0)); 

        let colors = Vec::from([Color::random_new()]);
        let app_directories = ProjectDirs::from("dev", "Corax", "Saswatch").expect("Failed to get program directories");
        
        let data_dir = app_directories.data_dir();
        if !data_dir.exists() {
            create_dir_all(data_dir)?;
        }
        
        let palette_dir = data_dir.join("palettes");
        if !palette_dir.exists() {
            create_dir(data_dir.join("palettes"))?;
        }

        Ok(App {
            clipboard_ctx: ClipboardProvider::new().unwrap(),
            app_directories,
            input_buffer: String::new(),
            mode: Mode::Generating,
            help_screen: false,
            colors,
            color_table_state
        })
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }
    
    pub fn inc_select(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if selected < entries-1 {
            self.color_table_state.select(Some(selected+1));
        }else{
            self.color_table_state.select(Some(0));
        }
    }
    
    pub fn dec_select(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if selected > 0 {
            self.color_table_state.select(Some(selected-1));
        }else {
            self.color_table_state.select(Some(entries-1));
        }
    }

    pub fn insert_color(&mut self) {
        self.colors.push(Color::random_new());
    }

    pub fn remove_color(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let entries = self.colors.len();
        if entries > 1{
            self.colors.remove(selected);
            if selected == entries-1{
                self.dec_select();
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
            self.color_table_state.select(Some(entries-1))
        } else {
            self.colors.swap(selected, selected-1);
            self.color_table_state.select(Some(selected-1))
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
            self.color_table_state.select(Some(selected+1))
        } else {
            self.colors.swap(selected, 0);
            self.color_table_state.select(Some(0))
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
                *color = Color::random_new();
            }
        }
    }
    
    pub fn get_help_screen(&self) -> bool {
        self.help_screen
    }

    pub fn toggle_help(&mut self) {
        if !self.help_screen{
            self.help_screen = true
        } else {
            self.help_screen = false
        }
    }

    pub fn copy_hex(&mut self) {
        let selected = self.color_table_state.selected().unwrap();
        let color = self.colors.get(selected).unwrap();
        let hex = color.hex_string();
        let _ = self.clipboard_ctx.set_contents(hex);
    }

    pub fn toggle_export_menu(&mut self) {
        if self.mode == Mode::Exporting {
            self.mode = Mode::Generating;
        } else {
            self.mode = Mode::Exporting;
        }
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
} 
