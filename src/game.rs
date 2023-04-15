use std::sync::mpsc::SyncSender;

use crate::{cpu::Cpu, AudioPlayer, KeypadKey};

pub struct Game {
    cpu: Cpu,
}

impl Game {
    pub fn new(player: Box<dyn AudioPlayer>, update_screen: SyncSender<Vec<u8>>) -> Self {
        let rom = include_bytes!("../rom_file.gb").to_vec();

        assert_eq!(rom[0x143], 0x80);
        assert_eq!(rom[0x147], 0x1b);
        assert_eq!(rom[0x149], 0x03);

        Self {
            cpu: Cpu::new(rom, player, update_screen),
        }
    }

    pub fn do_cycle(&mut self) -> u32 {
        self.cpu.do_cycle()
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        self.cpu.keyup(key)
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        self.cpu.keydown(key)
    }

    pub fn sync_audio(&mut self) {
        self.cpu.sync_audio()
    }
}
