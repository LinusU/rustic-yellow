use std::ops::Generator;

use crate::cpu::{Cpu, CpuFlag};

// pub fn fill_memory(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
//     move || {
//         cpu.pc = 0x166e;

//         //     push af
//         //     ld a, b
//         //     and a
//         //     jr z, .eightbitcopyamount
//         //     ld a, c
//         //     and a
//         //     jr z, .mulitpleof0x100
//         // .eightbitcopyamount
//         //     inc b
//         // .mulitpleof0x100
//         //     pop af

//         cpu.pushstack(cpu.af());
//         yield cpu.mmu.do_cycle(16);
//         cpu.pc = 0x166f;

//         cpu.a = cpu.b;
//         yield cpu.mmu.do_cycle(4);
//         cpu.pc = 0x1670;

//         {
//             let b = cpu.a;
//             let r = cpu.a & b;
//             cpu.flag(CpuFlag::Z, r == 0);
//             cpu.flag(CpuFlag::H, true);
//             cpu.flag(CpuFlag::C, false);
//             cpu.flag(CpuFlag::N, false);
//             cpu.a = r;
//         }
//         yield cpu.mmu.do_cycle(4);
//         cpu.pc = 0x1671;

//         if !cpu.getflag(CpuFlag::Z) {
//             yield cpu.mmu.do_cycle(8);
//             cpu.pc = 0x1673;

//             cpu.a = cpu.c;
//             yield cpu.mmu.do_cycle(4);
//             cpu.pc = 0x1674;

//             {
//                 let b = cpu.a;
//                 let r = cpu.a & b;
//                 cpu.flag(CpuFlag::Z, r == 0);
//                 cpu.flag(CpuFlag::H, true);
//                 cpu.flag(CpuFlag::C, false);
//                 cpu.flag(CpuFlag::N, false);
//                 cpu.a = r;
//             }
//             yield cpu.mmu.do_cycle(4);
//             cpu.pc = 0x1675;

//             if !cpu.getflag(CpuFlag::Z) {
//                 todo!()
//             } else {
//                 yield cpu.mmu.do_cycle(12);
//                 cpu.pc = 0x1678;
//             }
//         } else {
//             yield cpu.mmu.do_cycle(12);
//             cpu.pc = 0x1677;

//             cpu.b = {
//                 let a = cpu.b;
//                 let r = a.wrapping_add(1);
//                 cpu.flag(CpuFlag::Z, r == 0);
//                 cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
//                 cpu.flag(CpuFlag::N, false);
//                 r
//             };
//             yield cpu.mmu.do_cycle(4);
//             cpu.pc = 0x1678;
//         }

//         let v = cpu.popstack() & 0xfff0;
//         cpu.a = (v >> 8) as u8;
//         cpu.f = (v & 0x00f0) as u8;
//         yield cpu.mmu.do_cycle(12);
//         cpu.pc = 0x1679;

//         // .loop
//         //     ld [hli], a
//         //     dec c
//         //     jr nz, .loop
//         //     dec b
//         //     jr nz, .loop
//         //     ret

//         loop {
//             let addr = cpu.hl();
//             cpu.sethl(addr + 1);
//             cpu.mmu.wb(addr, cpu.a);
//             yield cpu.mmu.do_cycle(8);
//             cpu.pc = 0x167a;

//             cpu.c = {
//                 let a = cpu.c;
//                 let r = a.wrapping_sub(1);
//                 cpu.flag(CpuFlag::Z, r == 0);
//                 cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
//                 cpu.flag(CpuFlag::N, true);
//                 r
//             };
//             yield cpu.mmu.do_cycle(4);
//             cpu.pc = 0x167b;

//             if !cpu.getflag(CpuFlag::Z) {
//                 yield cpu.mmu.do_cycle(12);
//                 cpu.pc = 0x1679;
//                 continue;
//             } else {
//                 yield cpu.mmu.do_cycle(8);
//                 cpu.pc = 0x167d;
//             }

//             cpu.b = {
//                 let a = cpu.b;
//                 let r = a.wrapping_sub(1);
//                 cpu.flag(CpuFlag::Z, r == 0);
//                 cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
//                 cpu.flag(CpuFlag::N, true);
//                 r
//             };
//             yield cpu.mmu.do_cycle(4);
//             cpu.pc = 0x167e;

//             if !cpu.getflag(CpuFlag::Z) {
//                 yield cpu.mmu.do_cycle(12);
//                 cpu.pc = 0x1679;
//                 continue;
//             } else {
//                 yield cpu.mmu.do_cycle(8);
//                 cpu.pc = 0x1680;
//                 break;
//             }
//         }

//         cpu.popstack();
//         yield cpu.mmu.do_cycle(16);
//     }
// }
