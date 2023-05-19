use std::path;

use crate::{rom::ROM, save_state::SaveState};

pub struct MBC5 {
    ram: SaveState,
    pub(crate) rombank: usize,
    rambank: usize,
    ram_on: bool,
    save_path: Option<path::PathBuf>,
}

impl MBC5 {
    pub fn new() -> MBC5 {
        MBC5 {
            ram: SaveState::new(),
            rombank: 1,
            rambank: 0,
            ram_on: false,
            save_path: None,
        }
    }

    pub fn borrow_sram(&self) -> &SaveState {
        &self.ram
    }

    pub fn borrow_sram_mut(&mut self) -> &mut SaveState {
        &mut self.ram
    }

    pub fn replace_ram(&mut self, ram: SaveState) {
        self.ram = ram;
    }

    pub fn set_save_path(&mut self, save_path: path::PathBuf) {
        self.save_path = Some(save_path);
    }

    pub fn save_to_disk(&mut self) {
        if let Some(ref save_path) = self.save_path {
            self.ram.write_to_file(save_path).unwrap();
        }
    }

    pub fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 {
            a as usize
        } else {
            (self.rombank * 0x4000) | ((a as usize) & 0x3FFF)
        };
        *ROM.get(idx).unwrap_or(&0)
    }

    pub fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0;
        }
        self.ram
            .byte((self.rambank * 0x2000) | ((a as usize) & 0x1FFF))
    }

    pub fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => self.ram_on = v == 0x0A,
            0x2000..=0x2FFF => self.rombank = (self.rombank & 0x100) | (v as usize),
            0x3000..=0x3FFF => self.rombank = (self.rombank & 0x0FF) | (((v & 0x1) as usize) << 8),
            0x4000..=0x5FFF => self.rambank = (v & 0x0F) as usize,
            0x6000..=0x7FFF => { /* ? */ }
            _ => panic!("Could not write to {:04X} (MBC5)", a),
        }
    }

    pub fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }
        self.ram
            .set_byte((self.rambank * 0x2000) | ((a as usize) & 0x1FFF), v);
    }
}
