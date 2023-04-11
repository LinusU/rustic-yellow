use crate::{
    cpu::{Cpu, CpuFlag},
    game::ram::wram::{W_SHADOW_OAM, W_SHADOW_OAM_END},
};

pub fn clear_sprites(cpu: &mut Cpu, cycles: &mut u64) {
    cpu.pc = 0x0082;

    // xor a
    // ld hl, wShadowOAM
    // ld b, wShadowOAMEnd - wShadowOAM

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
    cpu.pc = 0x0083;

    cpu.h = (W_SHADOW_OAM >> 8) as u8;
    cpu.l = (W_SHADOW_OAM & 0x00ff) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x0086;

    cpu.b = (W_SHADOW_OAM_END - W_SHADOW_OAM) as u8;
    *cycles += cpu.mmu.do_cycle(8) as u64;
    cpu.pc = 0x0088;

    // .loop
    //     ld [hli], a
    //     dec b
    //     jr nz, .loop
    //     ret

    loop {
        let addr = cpu.hl();
        cpu.sethl(addr + 1);
        cpu.mmu.wb(addr, cpu.a);
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x0089;

        cpu.b = {
            let a = cpu.b;
            let r = a.wrapping_sub(1);
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
            cpu.flag(CpuFlag::N, true);
            r
        };
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x008a;

        if !cpu.getflag(CpuFlag::Z) {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x0088;
            continue;
        } else {
            *cycles += cpu.mmu.do_cycle(8) as u64;
            cpu.pc = 0x008c;
            break;
        }
    }

    cpu.popstack();
    *cycles += cpu.mmu.do_cycle(16) as u64;
}
