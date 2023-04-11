use crate::cpu::{Cpu, CpuFlag};

pub fn fill_memory(cpu: &mut Cpu, cycles: &mut u64) {
    cpu.pc = 0x166e;

    //     push af
    //     ld a, b
    //     and a
    //     jr z, .eightbitcopyamount
    //     ld a, c
    //     and a
    //     jr z, .mulitpleof0x100
    // .eightbitcopyamount
    //     inc b
    // .mulitpleof0x100
    //     pop af

    cpu.pushstack(cpu.af());
    *cycles += cpu.mmu.do_cycle(16) as u64;
    cpu.pc = 0x166f;

    cpu.a = cpu.b;
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x1670;

    {
        let b = cpu.a;
        let r = cpu.a & b;
        cpu.flag(CpuFlag::Z, r == 0);
        cpu.flag(CpuFlag::H, true);
        cpu.flag(CpuFlag::C, false);
        cpu.flag(CpuFlag::N, false);
        cpu.a = r;
    }
    *cycles += cpu.mmu.do_cycle(4) as u64;
    cpu.pc = 0x1671;

    if !cpu.getflag(CpuFlag::Z) {
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x1673;

        cpu.a = cpu.c;
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x1674;

        {
            let b = cpu.a;
            let r = cpu.a & b;
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, true);
            cpu.flag(CpuFlag::C, false);
            cpu.flag(CpuFlag::N, false);
            cpu.a = r;
        }
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x1675;

        if !cpu.getflag(CpuFlag::Z) {
            todo!()
        } else {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x1678;
        }
    } else {
        *cycles += cpu.mmu.do_cycle(12) as u64;
        cpu.pc = 0x1677;

        cpu.b = {
            let a = cpu.b;
            let r = a.wrapping_add(1);
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
            cpu.flag(CpuFlag::N, false);
            r
        };
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x1678;
    }

    let v = cpu.popstack() & 0xfff0;
    cpu.a = (v >> 8) as u8;
    cpu.f = (v & 0x00f0) as u8;
    *cycles += cpu.mmu.do_cycle(12) as u64;
    cpu.pc = 0x1679;

    // .loop
    //     ld [hli], a
    //     dec c
    //     jr nz, .loop
    //     dec b
    //     jr nz, .loop
    //     ret

    loop {
        let addr = cpu.hl();
        cpu.sethl(addr + 1);
        cpu.mmu.wb(addr, cpu.a);
        *cycles += cpu.mmu.do_cycle(8) as u64;
        cpu.pc = 0x167a;

        cpu.c = {
            let a = cpu.c;
            let r = a.wrapping_sub(1);
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
            cpu.flag(CpuFlag::N, true);
            r
        };
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x167b;

        if !cpu.getflag(CpuFlag::Z) {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x1679;
            continue;
        } else {
            *cycles += cpu.mmu.do_cycle(8) as u64;
            cpu.pc = 0x167d;
        }

        cpu.b = {
            let a = cpu.b;
            let r = a.wrapping_sub(1);
            cpu.flag(CpuFlag::Z, r == 0);
            cpu.flag(CpuFlag::H, (a & 0x0f) == 0);
            cpu.flag(CpuFlag::N, true);
            r
        };
        *cycles += cpu.mmu.do_cycle(4) as u64;
        cpu.pc = 0x167e;

        if !cpu.getflag(CpuFlag::Z) {
            *cycles += cpu.mmu.do_cycle(12) as u64;
            cpu.pc = 0x1679;
            continue;
        } else {
            *cycles += cpu.mmu.do_cycle(8) as u64;
            cpu.pc = 0x1680;
            break;
        }
    }

    cpu.popstack();
    *cycles += cpu.mmu.do_cycle(16) as u64;
}
