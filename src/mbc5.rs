pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    pub(crate) rombank: usize,
    rambank: usize,
    ram_on: bool,
    // savepath: Option<path::PathBuf>,
}

impl MBC5 {
    pub fn new(rom: Vec<u8>) -> MBC5 {
        // TODO: Load `savepath` if it exists, and populate `ram`

        MBC5 {
            rom,
            ram: vec![0; 0x8000],
            rombank: 1,
            rambank: 0,
            ram_on: false,
            // savepath: None,
        }
    }

    pub fn replace_ram(&mut self, ram: Vec<u8>) {
        assert_eq!(ram.len(), 0x8000);
        self.ram = ram;
    }

    pub fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 {
            a as usize
        } else {
            (self.rombank * 0x4000) | ((a as usize) & 0x3FFF)
        };
        *self.rom.get(idx).unwrap_or(&0)
    }

    pub fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0;
        }
        self.ram[(self.rambank * 0x2000) | ((a as usize) & 0x1FFF)]
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
        self.ram[(self.rambank * 0x2000) | ((a as usize) & 0x1FFF)] = v;
    }
}

impl Drop for MBC5 {
    fn drop(&mut self) {
        // TODO: Save `ram` to `savepath`
    }
}
