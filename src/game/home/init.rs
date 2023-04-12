use std::ops::Generator;

use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::hardware_constants::{
            HRAM_BEGIN, HRAM_END, MBC1_ROM_BANK, R_BGP, R_IE, R_IF, R_LCDC, R_LCDC_ENABLE_MASK,
            R_OBP0, R_OBP1, R_SB, R_SC, R_SCX, R_SCY, R_STAT, R_TAC, R_TMA, R_WX, R_WY, SERIAL,
            TIMER, VBLANK, VRAM_BEGIN, VRAM_END, WRAM0_BEGIN, WRAM1_END,
        },
        engine::gfx::oam_dma::{write_dma_code_to_hram, BANK_WRITE_DMA_CODE_TO_HRAM},
        ram::{
            hram::{H_LOADED_ROM_BANK, H_SCX, H_SCY, H_TILE_ANIMATIONS},
            wram::{W_C0F3, W_STACK},
        },
    },
    yield_from,
};

use super::{clear_sprites::clear_sprites, copy2::fill_memory, lcd::disable_lcd};

const R_LCDC_DEFAULT: u8 = 0b11100011;

pub fn init(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
    move || {
        cpu.pc = 0x1d10;

        // Init::

        // di
        cpu.setdi = 2;
        yield cpu.mmu.do_cycle(4);
        cpu.pc = 0x1d11;

        // xor a
        cpu.setdi = 1;
        cpu.pc = 0x1d12;
        cpu.a = 0;
        yield cpu.mmu.do_cycle(4);

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
        cpu.pc = 0x1d2c;
        yield cpu.mmu.do_cycle(12 * 13);

        cpu.a = R_LCDC_ENABLE_MASK;
        cpu.pc = 0x1d2e;
        yield cpu.mmu.do_cycle(8);

        cpu.mmu.wb(R_LCDC, R_LCDC_ENABLE_MASK);
        cpu.pc = 0x1d30;
        yield cpu.mmu.do_cycle(12);

        cpu.pushstack(0x1d33);
        yield cpu.mmu.do_cycle(24);
        yield_from!(disable_lcd(cpu));
        cpu.pc = 0x1d33;

        // ld sp, wStack

        cpu.sp = W_STACK;
        cpu.pc = 0x1d36;
        yield cpu.mmu.do_cycle(12);

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
        cpu.pc = 0x1d39;
        yield cpu.mmu.do_cycle(12);

        cpu.b = ((WRAM1_END - WRAM0_BEGIN) >> 8) as u8;
        cpu.c = ((WRAM1_END - WRAM0_BEGIN) & 0x00ff) as u8;
        cpu.pc = 0x1d3c;
        yield cpu.mmu.do_cycle(12);

        loop {
            cpu.mmu.wb(cpu.hl(), 0);
            yield cpu.mmu.do_cycle(12);
            cpu.pc = 0x1d3e;

            cpu.sethl(cpu.hl().wrapping_add(1));
            yield cpu.mmu.do_cycle(8);
            cpu.pc = 0x1d3f;

            cpu.setbc(cpu.bc().wrapping_sub(1));
            yield cpu.mmu.do_cycle(8);
            cpu.pc = 0x1d40;

            cpu.a = cpu.b;
            yield cpu.mmu.do_cycle(4);
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
            yield cpu.mmu.do_cycle(4);
            cpu.pc = 0x1d42;

            if !cpu.getflag(CpuFlag::Z) {
                yield cpu.mmu.do_cycle(12);
                cpu.pc = 0x1d3c;
                continue;
            } else {
                yield cpu.mmu.do_cycle(8);
                cpu.pc = 0x1d44;
                break;
            }
        }

        // call ClearVram

        cpu.pushstack(0x1d47);
        yield cpu.mmu.do_cycle(24);
        yield_from!(clear_vram(cpu));
        cpu.pc = 0x1d47;

        // ld hl, HRAM_Begin
        // ld bc, HRAM_End - HRAM_Begin - 1
        // call FillMemory

        cpu.h = (HRAM_BEGIN >> 8) as u8;
        cpu.l = (HRAM_BEGIN & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d4a;

        cpu.b = ((HRAM_END - HRAM_BEGIN - 1) >> 8) as u8;
        cpu.c = ((HRAM_END - HRAM_BEGIN - 1) & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d4d;

        cpu.pushstack(0x1d50);
        yield cpu.mmu.do_cycle(24);
        yield_from!(fill_memory(cpu));
        cpu.pc = 0x1d50;

        // call ClearSprites

        cpu.pushstack(0x1d53);
        yield cpu.mmu.do_cycle(24);
        yield_from!(clear_sprites(cpu));
        cpu.pc = 0x1d53;

        // ld a, BANK(WriteDMACodeToHRAM)
        // ldh [hLoadedROMBank], a
        // ld [MBC1RomBank], a
        // call WriteDMACodeToHRAM

        cpu.a = BANK_WRITE_DMA_CODE_TO_HRAM;
        yield cpu.mmu.do_cycle(8);
        cpu.pc = 0x1d55;

        cpu.mmu.wb(H_LOADED_ROM_BANK, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d57;

        cpu.mmu.wb(MBC1_ROM_BANK, cpu.a);
        yield cpu.mmu.do_cycle(16);
        cpu.pc = 0x1d5a;

        cpu.pushstack(0x1d5d);
        yield cpu.mmu.do_cycle(24);
        yield_from!(write_dma_code_to_hram(cpu));
        cpu.pc = 0x1d5d;

        // xor a
        // ldh [hTileAnimations], a
        // ldh [rSTAT], a
        // ldh [hSCX], a
        // ldh [hSCY], a
        // ldh [rIF], a
        // ld [wc0f3], a
        // ld [wc0f3 + 1], a
        // ld a, 1 << VBLANK + 1 << TIMER + 1 << SERIAL
        // ldh [rIE], a

        {
            let b = cpu.a;
            let r = cpu.a ^ b;
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::C, false);
            cpu.flag(CpuFlag::H, false);
            cpu.flag(CpuFlag::N, false);
            cpu.a = r;
        }
        yield cpu.mmu.do_cycle(4);
        cpu.pc = 0x1d5e;

        cpu.mmu.wb(H_TILE_ANIMATIONS, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d60;

        cpu.mmu.wb(R_STAT, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d62;

        cpu.mmu.wb(H_SCX, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d64;

        cpu.mmu.wb(H_SCY, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d66;

        cpu.mmu.wb(R_IF, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d68;

        cpu.mmu.wb(W_C0F3, cpu.a);
        yield cpu.mmu.do_cycle(16);
        cpu.pc = 0x1d6b;

        cpu.mmu.wb(W_C0F3 + 1, cpu.a);
        yield cpu.mmu.do_cycle(16);
        cpu.pc = 0x1d6e;

        cpu.a = (1 << VBLANK) | (1 << TIMER) | (1 << SERIAL);
        yield cpu.mmu.do_cycle(8);
        cpu.pc = 0x1d70;

        cpu.mmu.wb(R_IE, cpu.a);
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1d72;
    }
}

fn clear_vram(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
    move || {
        cpu.pc = 0x1dc6;

        // ld hl, VRAM_Begin
        // ld bc, VRAM_End - VRAM_Begin
        // xor a
        // jp FillMemory

        cpu.h = (VRAM_BEGIN >> 8) as u8;
        cpu.l = (VRAM_BEGIN & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x1dc9;

        cpu.b = ((VRAM_END - VRAM_BEGIN) >> 8) as u8;
        cpu.c = ((VRAM_END - VRAM_BEGIN) & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(12);
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
        yield cpu.mmu.do_cycle(4);
        cpu.pc = 0x1dcd;

        cpu.pc = 0x166e;
        yield cpu.mmu.do_cycle(16);
        yield_from!(fill_memory(cpu));
    }
}
