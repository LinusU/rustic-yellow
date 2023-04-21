use std::sync::mpsc::{Receiver, SyncSender};

use crate::{
    gpu::Gpu,
    keypad::{Keypad, KeypadEvent},
    mbc5::MBC5,
    serial::Serial,
    sound::{AudioPlayer, Sound},
    sound2::Sound2,
    timer::Timer,
};

const WRAM_SIZE: usize = 0x8000;
const ZRAM_SIZE: usize = 0x7F;

#[derive(PartialEq)]
enum DMAType {
    NoDMA,
    Gdma,
    Hdma,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Single,
    Double,
}

pub struct Mmu {
    wram: [u8; WRAM_SIZE],
    zram: [u8; ZRAM_SIZE],
    hdma: [u8; 4],
    pub inte: u8,
    pub intf: u8,
    pub serial: Serial,
    pub timer: Timer,
    pub keypad: Keypad,
    pub gpu: Gpu,
    pub sound: Sound,
    pub sound2: Sound2,
    hdma_status: DMAType,
    hdma_src: u16,
    hdma_dst: u16,
    hdma_len: u8,
    wrambank: usize,
    pub mbc: MBC5,
    gbspeed: GbSpeed,
    speed_switch_req: bool,
}

fn fill_random(slice: &mut [u8], start: u32) {
    // Simple LCG to generate (non-cryptographic) random values
    // Each distinct invocation should use a different start value
    const A: u32 = 1103515245;
    const C: u32 = 12345;

    let mut x = start;
    for v in slice.iter_mut() {
        x = x.wrapping_mul(A).wrapping_add(C);
        *v = ((x >> 23) & 0xFF) as u8;
    }
}

impl Mmu {
    pub fn new(
        rom: Vec<u8>,
        player: Box<dyn AudioPlayer>,
        update_screen: SyncSender<Vec<u8>>,
        keypad_events: Receiver<KeypadEvent>,
    ) -> Mmu {
        let mut mmu = Mmu {
            wram: [0; WRAM_SIZE],
            zram: [0; ZRAM_SIZE],
            hdma: [0; 4],
            inte: 0,
            intf: 0,
            serial: Serial::new(),
            timer: Timer::new(),
            keypad: Keypad::new(keypad_events),
            gpu: Gpu::new(update_screen),
            sound: Sound::new(player),
            sound2: Sound2::new(),
            hdma_status: DMAType::NoDMA,
            hdma_src: 0,
            hdma_dst: 0,
            hdma_len: 0,
            wrambank: 1,
            mbc: MBC5::new(rom),
            gbspeed: GbSpeed::Single,
            speed_switch_req: false,
        };

        fill_random(&mut mmu.wram, 42);

        mmu.wb(0xFF05, 0);
        mmu.wb(0xFF06, 0);
        mmu.wb(0xFF07, 0);
        mmu.wb(0xFF10, 0x80);
        mmu.wb(0xFF11, 0xBF);
        mmu.wb(0xFF12, 0xF3);
        mmu.wb(0xFF14, 0xBF);
        mmu.wb(0xFF16, 0x3F);
        mmu.wb(0xFF16, 0x3F);
        mmu.wb(0xFF17, 0);
        mmu.wb(0xFF19, 0xBF);
        mmu.wb(0xFF1A, 0x7F);
        mmu.wb(0xFF1B, 0xFF);
        mmu.wb(0xFF1C, 0x9F);
        mmu.wb(0xFF1E, 0xFF);
        mmu.wb(0xFF20, 0xFF);
        mmu.wb(0xFF21, 0);
        mmu.wb(0xFF22, 0);
        mmu.wb(0xFF23, 0xBF);
        mmu.wb(0xFF24, 0x77);
        mmu.wb(0xFF25, 0xF3);
        mmu.wb(0xFF26, 0xF1);
        mmu.wb(0xFF40, 0x91);
        mmu.wb(0xFF42, 0);
        mmu.wb(0xFF43, 0);
        mmu.wb(0xFF45, 0);
        mmu.wb(0xFF47, 0xFC);
        mmu.wb(0xFF48, 0xFF);
        mmu.wb(0xFF49, 0xFF);
        mmu.wb(0xFF4A, 0);
        mmu.wb(0xFF4B, 0);

        mmu
    }

    pub fn do_cycle(&mut self, ticks: u32) -> u32 {
        let cpudivider = match self.gbspeed {
            GbSpeed::Single => 1,
            GbSpeed::Double => 2,
        };
        let vramticks = self.perform_vramdma();
        let gputicks = ticks / cpudivider + vramticks;
        let cputicks = ticks + vramticks * cpudivider;

        self.timer.do_cycle(cputicks);
        self.intf |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.gpu.do_cycle(gputicks);
        self.intf |= self.gpu.interrupt;
        self.gpu.interrupt = 0;

        self.sound.do_cycle(gputicks);

        self.intf |= self.serial.interrupt;
        self.serial.interrupt = 0;

        gputicks
    }

    #[rustfmt::skip]
    pub fn rb(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.readrom(address),
            0x8000..=0x9FFF => self.gpu.rb(address),
            0xA000..=0xBFFF => self.mbc.readram(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wrambank * 0x1000) | address as usize & 0x0FFF],
            0xFE00..=0xFE9F => self.gpu.rb(address),
            0xFF00 => self.keypad.rb(),
            0xFF01..=0xFF02 => self.serial.rb(address),
            0xFF04..=0xFF07 => self.timer.rb(address),
            0xFF0F => self.intf,
            0xFF10..=0xFF3F => self.sound.rb(address),
            0xFF4D => (if self.gbspeed == GbSpeed::Double { 0x80 } else { 0 }) | (if self.speed_switch_req { 1 } else { 0 }),
            0xFF40..=0xFF4F => self.gpu.rb(address),
            0xFF51..=0xFF55 => self.hdma_read(address),
            0xFF68..=0xFF6B => self.gpu.rb(address),
            0xFF70 => self.wrambank as u8,
            0xFF80..=0xFFFE => self.zram[address as usize & 0x007F],
            0xFFFF => self.inte,
            // _ => 0xFF,
            _ => panic!("Unimplemented read from {:04X}", address),
        }
    }

    pub fn rw(&mut self, address: u16) -> u16 {
        (self.rb(address) as u16) | ((self.rb(address + 1) as u16) << 8)
    }

    #[rustfmt::skip]
    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.writerom(address, value),
            0x8000..=0x9FFF => self.gpu.wb(address, value),
            0xA000..=0xBFFF => self.mbc.writeram(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF] = value,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wrambank * 0x1000) | (address as usize & 0x0FFF)] = value,
            0xFE00..=0xFE9F => self.gpu.wb(address, value),
            0xFF00 => self.keypad.wb(value),
            0xFF01..=0xFF02 => self.serial.wb(address, value),
            0xFF04..=0xFF07 => self.timer.wb(address, value),
            0xFF10..=0xFF3F => self.sound.wb(address, value),
            0xFF46 => self.oamdma(value),
            0xFF4D => if value & 0x1 == 0x1 { self.speed_switch_req = true; },
            0xFF40..=0xFF4F => self.gpu.wb(address, value),
            0xFF51..=0xFF55 => self.hdma_write(address, value),
            0xFF68..=0xFF6B => self.gpu.wb(address, value),
            0xFF0F => self.intf = value,
            0xFF70 => { self.wrambank = match value & 0x7 { 0 => 1, n => n as usize }; },
            0xFF80..=0xFFFE => self.zram[address as usize & 0x007F] = value,
            0xFFFF => self.inte = value,
            _ => panic!("Invalid write to address {:#06X}", address),
        };
    }

    pub fn ww(&mut self, address: u16, value: u16) {
        self.wb(address, (value & 0xFF) as u8);
        self.wb(address + 1, (value >> 8) as u8);
    }

    pub fn switch_speed(&mut self) {
        if self.speed_switch_req {
            if self.gbspeed == GbSpeed::Double {
                self.gbspeed = GbSpeed::Single;
            } else {
                self.gbspeed = GbSpeed::Double;
            }
        }
        self.speed_switch_req = false;
    }

    fn oamdma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let b = self.rb(base + i);
            self.wb(0xFE00 + i, b);
        }
    }

    #[rustfmt::skip]
    fn hdma_read(&self, a: u16) -> u8 {
        match a {
            0xFF51..=0xFF54 => { self.hdma[(a - 0xFF51) as usize] },
            0xFF55 => self.hdma_len | if self.hdma_status == DMAType::NoDMA { 0x80 } else { 0 },
            _ => panic!("The address {:04X} should not be handled by hdma_read", a),
        }
    }

    fn hdma_write(&mut self, a: u16, v: u8) {
        match a {
            0xFF51 => self.hdma[0] = v,
            0xFF52 => self.hdma[1] = v & 0xF0,
            0xFF53 => self.hdma[2] = v & 0x1F,
            0xFF54 => self.hdma[3] = v & 0xF0,
            0xFF55 => {
                if self.hdma_status == DMAType::Hdma {
                    if v & 0x80 == 0 {
                        self.hdma_status = DMAType::NoDMA;
                    };
                    return;
                }
                let src = ((self.hdma[0] as u16) << 8) | (self.hdma[1] as u16);
                let dst = ((self.hdma[2] as u16) << 8) | (self.hdma[3] as u16) | 0x8000;
                if !(src <= 0x7FF0 || (0xA000..=0xDFF0).contains(&src)) {
                    panic!("HDMA transfer with illegal start address {:04X}", src);
                }

                self.hdma_src = src;
                self.hdma_dst = dst;
                self.hdma_len = v & 0x7F;

                self.hdma_status = if v & 0x80 == 0x80 {
                    DMAType::Hdma
                } else {
                    DMAType::Gdma
                };
            }
            _ => panic!("The address {:04X} should not be handled by hdma_write", a),
        };
    }

    fn perform_vramdma(&mut self) -> u32 {
        match self.hdma_status {
            DMAType::NoDMA => 0,
            DMAType::Gdma => self.perform_gdma(),
            DMAType::Hdma => self.perform_hdma(),
        }
    }

    fn perform_hdma(&mut self) -> u32 {
        if !self.gpu.may_hdma() {
            return 0;
        }

        self.perform_vramdma_row();
        if self.hdma_len == 0x7F {
            self.hdma_status = DMAType::NoDMA;
        }

        8
    }

    fn perform_gdma(&mut self) -> u32 {
        let len = self.hdma_len as u32 + 1;
        for _i in 0..len {
            self.perform_vramdma_row();
        }

        self.hdma_status = DMAType::NoDMA;
        len * 8
    }

    fn perform_vramdma_row(&mut self) {
        let mmu_src = self.hdma_src;
        for j in 0..0x10 {
            let b: u8 = self.rb(mmu_src + j);
            self.gpu.wb(self.hdma_dst + j, b);
        }
        self.hdma_src += 0x10;
        self.hdma_dst += 0x10;

        if self.hdma_len == 0 {
            self.hdma_len = 0x7F;
        } else {
            self.hdma_len -= 1;
        }
    }
}
