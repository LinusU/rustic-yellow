use std::ops::Generator;

use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::hardware_constants::{
        LY_VBLANK, R_IE, R_IF, R_LCDC, R_LCDC_ENABLE_MASK, R_LY,
    },
};

// pub fn disable_lcd(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
//     move || {
//         cpu.pc = 0x0061;

//         // xor a
//         // ldh [rIF], a
//         // ldh a, [rIE]
//         // ld b, a
//         // res 0, a
//         // ldh [rIE], a

//         cpu.a = 0;
//         yield cpu.mmu.do_cycle(4);
//         cpu.pc = 0x0062;

//         cpu.mmu.wb(R_IF, 0);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x0064;

//         cpu.a = cpu.mmu.rb(R_IE);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x0066;

//         cpu.b = cpu.a;
//         yield cpu.mmu.do_cycle(4);
//         cpu.pc = 0x0067;

//         cpu.a &= 0b11111110;
//         yield cpu.mmu.do_cycle(8);
//         cpu.pc = 0x0069;

//         cpu.mmu.wb(R_IE, cpu.a);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x006b;

//         // .wait
//         //     ldh a, [rLY]
//         //     cp LY_VBLANK
//         //     jr nz, .wait

//         loop {
//             cpu.a = cpu.mmu.rb(R_LY);
//             yield cpu.mmu.do_cycle(12);
//             cpu.pc = 0x006d;

//             let r = cpu.a;
//             {
//                 let c = 0;
//                 let a = cpu.a;
//                 let r = a.wrapping_sub(LY_VBLANK).wrapping_sub(c);
//                 cpu.flag(CpuFlag::Z, r == 0);
//                 cpu.flag(CpuFlag::H, (a & 0x0f) < (LY_VBLANK & 0x0f) + c);
//                 cpu.flag(CpuFlag::N, true);
//                 cpu.flag(CpuFlag::C, (a as u16) < (LY_VBLANK as u16) + (c as u16));
//                 cpu.a = r;
//             }
//             cpu.a = r;
//             yield cpu.mmu.do_cycle(8);
//             cpu.pc = 0x006f;

//             if !cpu.getflag(CpuFlag::Z) {
//                 yield cpu.mmu.do_cycle(12);
//                 cpu.pc = 0x006b;
//                 continue;
//             } else {
//                 yield cpu.mmu.do_cycle(8);
//                 cpu.pc = 0x0071;
//                 break;
//             }
//         }

//         // ldh a, [rLCDC]
//         // and ~rLCDC_ENABLE_MASK
//         // ldh [rLCDC], a
//         // ld a, b
//         // ldh [rIE], a
//         // ret

//         cpu.a = cpu.mmu.rb(R_LCDC);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x0073;

//         let r = cpu.a & !R_LCDC_ENABLE_MASK;
//         cpu.flag(CpuFlag::Z, r == 0);
//         cpu.flag(CpuFlag::H, true);
//         cpu.flag(CpuFlag::C, false);
//         cpu.flag(CpuFlag::N, false);
//         cpu.a = r;
//         yield cpu.mmu.do_cycle(8);
//         cpu.pc = 0x0075;

//         cpu.mmu.wb(R_LCDC, cpu.a);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x0077;

//         cpu.a = cpu.b;
//         yield cpu.mmu.do_cycle(4);
//         cpu.pc = 0x0078;

//         cpu.mmu.wb(R_IE, cpu.a);
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x007a;

//         cpu.popstack();
//         yield cpu.mmu.do_cycle(16);
//     }
// }
