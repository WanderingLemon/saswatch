use std::usize;

use crate::color::Color;

pub struct App{
    help_screen: bool,
    entries: usize,
    offset: usize,
    colors: Vec<Color>
}

impl App {
    pub fn new() -> App {
        let colors = Vec::from([Color::random_new()]);
        App {
            help_screen: false,
            entries: 1,
            offset: 0,
            colors
        }
    }
    
    pub fn inc_offset(&mut self) {
        if self.offset < self.entries-1 {
            self.offset += 1;
        }else{
            self.offset = 0;
        }
    }
    
    pub fn dec_offset(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }else {
            self.offset = self.entries-1;
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn insert_color(&mut self) {
        self.colors.push(Color::random_new());
        self.entries += 1;
    }

    pub fn remove_color(&mut self) {
        let offset = self.offset;
        let entries = self.entries;
        if entries > 1{
            self.colors.remove(offset);
            if offset == entries-1{
                self.dec_offset();
            }
            self.entries -= 1;
        }
    }

    pub fn get_colors(&self) -> Vec<Color> {
        self.colors.to_owned()
    }

    pub fn shift_up(&mut self) {
        if self.entries <= 1{
            return
        }

        let offset = self.offset;
        if offset == 0 {
            self.colors.swap(self.offset, self.entries-1);
            self.offset = self.entries - 1;
        } else {
            self.colors.swap(self.offset, self.offset-1);
            self.offset -= 1;
        }
    }

    pub fn shift_down(&mut self) {
        if self.entries <= 1{
            return
        }
        
        let offset = self.offset;
        if offset < self.entries-1 {
            self.colors.swap(self.offset, self.offset+1);
            self.offset += 1;
        } else {
            self.colors.swap(self.offset, 0);
            self.offset = 0;
        }
    }

    pub fn toggle_lock(&mut self) {
        let color = self.colors.get_mut(self.offset).unwrap();
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
} 
