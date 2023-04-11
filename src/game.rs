use crate::{cpu::Cpu, AudioPlayer, KeypadKey};

pub struct Game {
    cpu: Cpu,
}

impl Game {
    pub fn new(player: Box<dyn AudioPlayer>) -> Self {
        let rom = include_bytes!("../rom_file.gb").to_vec();

        assert_eq!(rom[0x143], 0x80);
        assert_eq!(rom[0x147], 0x1b);
        assert_eq!(rom[0x149], 0x03);

        Self {
            cpu: Cpu::new(rom, player),
        }
    }

    pub fn do_cycle(&mut self) -> u32 {
        self.cpu.do_cycle()
    }

    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        self.cpu.check_and_reset_gpu_updated()
    }

    pub fn get_gpu_data(&self) -> &[u8] {
        self.cpu.get_gpu_data()
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
