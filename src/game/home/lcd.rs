use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::hardware_constants::{
        LY_VBLANK, R_IE, R_IF, R_LCDC, R_LCDC_ENABLE_MASK, R_LY,
    },
};

pub fn disable_lcd(cpu: &mut Cpu, cycles: &mut u64) {
    cpu.pc = 0x0061;

    // xor a
    // ldh [rIF], a
    // ldh a, [rIE]
    // ld b, a
    // res 0, a
    // ldh [rIE], a

    cpu.a = 0;
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x0062;

    cpu.mmu.wb(R_IF, 0);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x0064;

    cpu.a = cpu.mmu.rb(R_IE);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x0066;

    cpu.b = cpu.a;
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x0067;

    cpu.a &= 0b11111110;
    *cycles += cpu.mmu.do_cycle(8) as u64;
    cpu.pc = 0x0069;

    cpu.mmu.wb(R_IE, cpu.a);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x006b;

    // .wait
    //     ldh a, [rLY]
    //     cp LY_VBLANK
    //     jr nz, .wait

    loop {
        cpu.a = cpu.mmu.rb(R_LY);
        *cycles += cpu.mmu.do_cycle(12) as u64;
        cpu.pc = 0x006d;

        let r = cpu.a;
        {
            let c = 0;
            let a = cpu.a;
            let r = a.wrapping_sub(LY_VBLANK).wrapping_sub(c);
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, (a & 0x0f) < (LY_VBLANK & 0x0f) + c);
            cpu.flag(CpuFlag::N, true);
            cpu.flag(CpuFlag::C, (a as u16) < (LY_VBLANK as u16) + (c as u16));
            cpu.a = r;
        }
        cpu.a = r;
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x006f;

        if !cpu.getflag(CpuFlag::Z) {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x006b;
            continue;
        } else {
            *cycles += cpu.mmu.do_cycle(8) as u64;
            cpu.pc = 0x0071;
            break;
        }
    }

    // ldh a, [rLCDC]
    // and ~rLCDC_ENABLE_MASK
    // ldh [rLCDC], a
    // ld a, b
    // ldh [rIE], a
    // ret

    cpu.a = cpu.mmu.rb(R_LCDC);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x0073;

    let r = cpu.a & !R_LCDC_ENABLE_MASK;
    cpu.flag(CpuFlag::Z, r == 0);
    cpu.flag(CpuFlag::H, true);
    cpu.flag(CpuFlag::C, false);
    cpu.flag(CpuFlag::N, false);
    cpu.a = r;
    *cycles += cpu.mmu.do_cycle(8) as u64;
    cpu.pc = 0x0075;

    cpu.mmu.wb(R_LCDC, cpu.a);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x0077;

    cpu.a = cpu.b;
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x0078;

    cpu.mmu.wb(R_IE, cpu.a);
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x007a;

    cpu.popstack();
    *cycles += cpu.mmu.do_cycle(16) as u64;
}
