use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::hardware_constants::{
            R_BGP, R_IE, R_IF, R_LCDC, R_LCDC_ENABLE_MASK, R_OBP0, R_OBP1, R_SB, R_SC, R_SCX,
            R_SCY, R_TAC, R_TMA, R_WX, R_WY, VRAM_BEGIN, VRAM_END, WRAM0_BEGIN, WRAM1_END,
        },
        ram::wram::W_STACK,
    },
};

use super::{copy2::fill_memory, lcd::disable_lcd};

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

    cpu.a = R_LCDC_ENABLE_MASK;
    *cycles += cpu.mmu.do_cycle(8) as u64;
    cpu.pc = 0x1d2e;

    cpu.mmu.wb(R_LCDC, R_LCDC_ENABLE_MASK);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1d30;

    cpu.pushstack(0x1d33);
    *cycles += cpu.mmu.do_cycle(24) as u64;
    disable_lcd(cpu, cycles);
    cpu.pc = 0x1d33;

    // ld sp, wStack

    cpu.sp = W_STACK;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1d36;

    //     ld hl, WRAM0_Begin
    //     ld bc, WRAM1_End - WRAM0_Begin
    // .loop
    //     ld [hl], 0
    //     inc hl
    //     dec bc
    //     ld a, b
    //     or c
    //     jr nz, .loop

    cpu.h = (WRAM0_BEGIN >> 8) as u8;
    cpu.l = (WRAM0_BEGIN & 0x00ff) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1d39;

    cpu.b = ((WRAM1_END - WRAM0_BEGIN) >> 8) as u8;
    cpu.c = ((WRAM1_END - WRAM0_BEGIN) & 0x00ff) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1d3c;

    loop {
        cpu.mmu.wb(cpu.hl(), 0);
        *cycles += cpu.mmu.do_cycle(12) as u64;
        cpu.pc = 0x1d3e;

        cpu.sethl(cpu.hl().wrapping_add(1));
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x1d3f;

        cpu.setbc(cpu.bc().wrapping_sub(1));
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x1d40;

        cpu.a = cpu.b;
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x1d41;

        {
            let b = cpu.c;
            let r = cpu.a | b;
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::C, false);
            cpu.flag(CpuFlag::H, false);
            cpu.flag(CpuFlag::N, false);
            cpu.a = r;
        }
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x1d42;

        if !cpu.getflag(CpuFlag::Z) {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x1d3c;
            continue;
        } else {
            *cycles += cpu.mmu.do_cycle(8) as u64;
            cpu.pc = 0x1d44;
            break;
        }
    }

    // call ClearVram

    cpu.pushstack(0x1d47);
    *cycles += cpu.mmu.do_cycle(24) as u64;
    clear_vram(cpu, cycles);
    cpu.pc = 0x1d47;
}

fn clear_vram(cpu: &mut Cpu, cycles: &mut u64) {
    cpu.pc = 0x1dc6;

    // ld hl, VRAM_Begin
    // ld bc, VRAM_End - VRAM_Begin
    // xor a
    // jp FillMemory

    cpu.h = (VRAM_BEGIN >> 8) as u8;
    cpu.l = (VRAM_BEGIN & 0x00ff) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1dc9;

    cpu.b = ((VRAM_END - VRAM_BEGIN) >> 8) as u8;
    cpu.c = ((VRAM_END - VRAM_BEGIN) & 0x00ff) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1dcc;

    {
        let b = cpu.a;
        let r = cpu.a ^ b;
        cpu.flag(CpuFlag::Z, r == 0);
        cpu.flag(CpuFlag::C, false);
        cpu.flag(CpuFlag::H, false);
        cpu.flag(CpuFlag::N, false);
        cpu.a = r;
    }
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x1dcd;

    cpu.pc = 0x166e;
    *cycles += cpu.mmu.do_cycle(16) as u64;
    fill_memory(cpu, cycles);
}
