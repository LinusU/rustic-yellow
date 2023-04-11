use crate::{
    cpu::Cpu,
    game::constants::hardware_constants::{
        R_BGP, R_IE, R_IF, R_OBP0, R_OBP1, R_SB, R_SC, R_SCX, R_SCY, R_TAC, R_TMA, R_WX, R_WY,
    },
};

const R_LCDC_DEFAULT: u8 = 0b11100011;

pub fn init(cpu: &mut Cpu, cycles: &mut u64) {
    cpu.pc = 0x1d10;

    // Init::

    // di
    cpu.setdi = 2;
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x1d11;

    // xor a
    cpu.setdi = 1;
    cpu.pc = 0x1d12;
    cpu.a = 0;
    *cycles += cpu.mmu.do_cycle(4) as u64;

    // multiple ldh
    cpu.ime = false;
    cpu.setdi = 0;
    cpu.pc = 0x1d14;
    cpu.mmu.wb(R_IF, 0);
    cpu.mmu.wb(R_IE, 0);
    cpu.mmu.wb(R_SCX, 0);
    cpu.mmu.wb(R_SCY, 0);
    cpu.mmu.wb(R_SB, 0);
    cpu.mmu.wb(R_SC, 0);
    cpu.mmu.wb(R_WX, 0);
    cpu.mmu.wb(R_WY, 0);
    cpu.mmu.wb(R_TMA, 0);
    cpu.mmu.wb(R_TAC, 0);
    cpu.mmu.wb(R_BGP, 0);
    cpu.mmu.wb(R_OBP0, 0);
    cpu.mmu.wb(R_OBP1, 0);
    *cycles += cpu.mmu.do_cycle(12 * 13) as u64;

    cpu.pc = 0x1d2c;
}
