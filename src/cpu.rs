use std::{cell::RefCell, rc::Rc, ops::Generator};

use crate::{mmu::Mmu, yield_from, op::run_rom_code};
use CpuFlag::{C, H, N, Z};

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

pub struct Cpu {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) pc: u16,
    pub(crate) sp: u16,

    pub(crate) halted: bool,
    pub(crate) ime: bool,
    pub(crate) setdi: u32,
    pub(crate) setei: u32,

    mmu: Rc<RefCell<Mmu>>,
}

impl Cpu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Self {
            a: 0x11,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,

            halted: false,
            ime: true,
            setdi: 0,
            setei: 0,

            mmu,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let res = self.read_byte(self.pc);
        self.pc += 1;
        res
    }

    pub fn fetch_word(&mut self) -> u16 {
        let res = self.read_word(self.pc);
        self.pc += 2;
        res
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.borrow_mut().rb(addr)
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        self.mmu.borrow_mut().rw(addr)
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.mmu.borrow_mut().wb(addr, byte);
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.mmu.borrow_mut().ww(addr, word);
    }

    pub fn stack_push(&mut self, value: u16) {
        self.sp -= 2;
        self.write_word(self.sp, value);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let res = self.read_word(self.sp);
        self.sp += 2;
        res
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn hld(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res - 1);
        res
    }

    pub fn hli(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res + 1);
        res
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn flag(&self, flags: CpuFlag) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }

    pub fn set_flag(&mut self, flags: CpuFlag, set: bool) {
        let mask = flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }

    pub fn cycle(mut self, n: u32) -> impl Generator<Yield = u32, Return = Self> {
        move || {
            loop {
                self.updateime();
                // yield_from!(self.handleinterrupt());

                let mut g = self.handleinterrupt();

                self = loop {
                    match std::pin::Pin::new(&mut g).resume(()) {
                        std::ops::GeneratorState::Yielded(n) => yield n,
                        std::ops::GeneratorState::Complete(cpu) => break cpu,
                    }
                };

                if self.halted {
                    self.mmu.borrow_mut().do_cycle(4);
                    yield 1;
                    continue;
                }

                self.mmu.borrow_mut().do_cycle(n * 4);
                yield n;
                break;
            }

            self
        }
    }

    fn updateime(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }

    fn handleinterrupt(mut self) -> impl Generator<Yield = u32, Return = Self> {
        move || {
            if !self.ime && !self.halted {
                return self;
            }

            let triggered = self.mmu.borrow().inte & self.mmu.borrow().intf;
            if triggered == 0 {
                return self;
            }

            self.halted = false;
            if !self.ime {
                return self;
            }
            self.ime = false;

            let n = triggered.trailing_zeros();
            if n >= 5 {
                panic!("Invalid interrupt triggered");
            }
            self.mmu.borrow_mut().intf &= !(1 << n);
            self.stack_push(self.pc);
            self.pc = 0x0040 | ((n as u16) << 3);

            if 4 < 3 {
                yield 0;
            }

            self

            // let mut g = run_rom_code(self);

            // loop {
            //     match std::pin::Pin::new(&mut g).resume(()) {
            //         std::ops::GeneratorState::Yielded(n) => yield n,
            //         std::ops::GeneratorState::Complete(cpu) => break cpu,
            //     }
            // }

            // yield 0;
            // todo!();
            // yield_from!(run_rom_code(self));
        }
    }





    pub fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.flag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.set_flag(N, false);
        self.set_flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.a = r;
    }

    pub fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.flag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.set_flag(N, true);
        self.set_flag(C, (a as u16) < (b as u16) + (c as u16));
        self.a = r;
    }

    pub fn alu_and(&mut self, b: u8) {
        let r = self.a & b;
        self.set_flag(Z, r == 0);
        self.set_flag(H, true);
        self.set_flag(C, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_or(&mut self, b: u8) {
        let r = self.a | b;
        self.set_flag(Z, r == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_xor(&mut self, b: u8) {
        let r = self.a ^ b;
        self.set_flag(Z, r == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_cp(&mut self, b: u8) {
        let r = self.a;
        self.alu_sub(b, false);
        self.a = r;
    }

    pub fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) + 1 > 0x0F);
        self.set_flag(N, false);
        r
    }

    pub fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) == 0);
        self.set_flag(N, true);
        r
    }

    pub fn alu_add16(&mut self, b: u16) {
        let a = self.hl();
        let r = a.wrapping_add(b);
        self.set_flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.set_flag(N, false);
        self.set_flag(C, a > 0xFFFF - b);
        self.set_hl(r);
    }

    pub fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.fetch_byte() as i8 as i16 as u16;
        self.set_flag(N, false);
        self.set_flag(Z, false);
        self.set_flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.set_flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        a.wrapping_add(b)
    }

    pub fn alu_swap(&mut self, a: u8) -> u8 {
        self.set_flag(Z, a == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        (a >> 4) | (a << 4)
    }

    pub fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.set_flag(Z, r == 0);
        self.set_flag(C, c);
    }

    pub fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.flag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.flag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.set_flag(N, false);
        self.set_flag(H, true);
        self.set_flag(Z, r);
    }

    pub fn alu_daa(&mut self) {
        let mut a = self.a;
        let mut adjust = if self.flag(C) { 0x60 } else { 0x00 };
        if self.flag(H) {
            adjust |= 0x06;
        };
        if !self.flag(N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.set_flag(C, adjust >= 0x60);
        self.set_flag(H, false);
        self.set_flag(Z, a == 0);
        self.a = a;
    }

    pub fn cpu_jr(&mut self) {
        let n = self.fetch_byte() as i8;
        self.pc = ((self.pc as u32 as i32) + (n as i32)) as u16;
    }
}
