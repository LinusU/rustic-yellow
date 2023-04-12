use std::ops::Generator;

use crate::cpu::{Cpu, CpuFlag};

pub const BANK_WRITE_DMA_CODE_TO_HRAM: u8 = 1;
pub const H_DMA_ROUTINE: u16 = 0x0080;

pub const DMA_ROUTINE: u16 = 0x4aa0;
pub const DMA_ROUTINE_END: u16 = 0x4aaa;

/// Since no other memory is available during OAM DMA,
/// DMARoutine is copied to HRAM and executed there.
pub fn write_dma_code_to_hram(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
    move || {
        cpu.pc = 0x4a92;

        // ld c, LOW(hDMARoutine)
        // ld b, DMARoutineEnd - DMARoutine
        // ld hl, DMARoutine

        cpu.c = (H_DMA_ROUTINE & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(8);
        cpu.pc = 0x4a94;

        cpu.b = (DMA_ROUTINE_END - DMA_ROUTINE) as u8;
        yield cpu.mmu.do_cycle(8);
        cpu.pc = 0x4a96;

        cpu.h = (DMA_ROUTINE >> 8) as u8;
        cpu.l = (DMA_ROUTINE & 0x00ff) as u8;
        yield cpu.mmu.do_cycle(12);
        cpu.pc = 0x4a99;

        // .copy
        //     ld a, [hli]
        //     ldh [c], a
        //     inc c
        //     dec b
        //     jr nz, .copy
        //     ret

        loop {
            {
                let addr = cpu.hl();
                cpu.sethl(addr + 1);
                cpu.a = cpu.mmu.rb(addr);
            }
            yield cpu.mmu.do_cycle(8);
            cpu.pc = 0x4a9a;

            cpu.mmu.wb(0xff00 | (cpu.c as u16), cpu.a);
            yield cpu.mmu.do_cycle(8);
            cpu.pc = 0x4a9b;

            cpu.c = {
                let a = cpu.c;
                let r = a.wrapping_add(1);
                cpu.flag(CpuFlag::Z, r == 0);
                cpu.flag(CpuFlag::H, (a & 0x0f) + 1 > 0x0f);
                cpu.flag(CpuFlag::N, false);
                r
            };
            yield cpu.mmu.do_cycle(4);
            cpu.pc = 0x4a9c;

            cpu.b = {
                let a = cpu.b;
                let r = a.wrapping_sub(1);
                cpu.flag(CpuFlag::Z, r == 0);
                cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
                cpu.flag(CpuFlag::N, true);
                r
            };
            yield cpu.mmu.do_cycle(4);
            cpu.pc = 0x4a9d;

            if !cpu.getflag(CpuFlag::Z) {
                yield cpu.mmu.do_cycle(12);
                cpu.pc = 0x4a99;
                continue;
            } else {
                yield cpu.mmu.do_cycle(8);
                cpu.pc = 0x4a9f;
                break;
            }
        }

        cpu.popstack();
        yield cpu.mmu.do_cycle(16);
    }
}
