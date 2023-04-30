use std::sync::mpsc::{Receiver, SyncSender};

use crate::{cpu::Cpu, keypad::KeyboardEvent, rom::ROM};

pub mod constants;
pub mod engine;
pub mod home;
pub mod macros;
pub mod ram;

pub struct Game {
    cpu: Cpu,
}

impl Game {
    pub fn new(
        update_screen: SyncSender<Vec<u8>>,
        keyboard_events: Receiver<KeyboardEvent>,
    ) -> Self {
        assert_eq!(ROM[0x143], 0x80);
        assert_eq!(ROM[0x147], 0x1b);
        assert_eq!(ROM[0x149], 0x03);

        Self {
            cpu: Cpu::new(update_screen, keyboard_events),
        }
    }

    pub fn boot(&mut self) {
        self.cpu.call(0x0100)
    }

    pub fn sync_audio(&mut self) {
        self.cpu.sync_audio()
    }
}
