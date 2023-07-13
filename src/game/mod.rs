use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, SyncSender},
};

use crate::{cpu::Cpu, keypad::KeyboardEvent, rom::ROM, PokemonSpecies};

pub mod audio;
pub mod constants;
pub mod data;
pub mod engine;
pub mod home;
pub mod macros;
pub mod ram;
pub mod scripts;

pub fn resources_root() -> Option<PathBuf> {
    if std::env::var_os("CARGO").is_some() {
        return Some(PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR")?));
    }

    // TODO: support for other platforms
    #[cfg(target_os = "macos")]
    {
        let bundle = core_foundation::bundle::CFBundle::main_bundle();
        let bundle_path = bundle.path()?;
        let resources_path = bundle.resources_path()?;
        Some(bundle_path.join(resources_path))
    }
    #[cfg(not(any(target_os = "macos")))]
    None
}

pub struct Game {
    cpu: Cpu,
}

impl Game {
    pub fn new(
        update_screen: SyncSender<Vec<u8>>,
        keyboard_events: Receiver<KeyboardEvent>,
        starter: PokemonSpecies,
    ) -> Self {
        assert_eq!(ROM[0x143], 0x80);
        assert_eq!(ROM[0x147], 0x1b);
        assert_eq!(ROM[0x149], 0x03);

        Self {
            cpu: Cpu::new(update_screen, keyboard_events, starter),
        }
    }

    pub fn boot(&mut self) {
        self.cpu.call(0x0100)
    }

    pub fn sync_audio(&mut self) {
        self.cpu.sync_audio()
    }
}
