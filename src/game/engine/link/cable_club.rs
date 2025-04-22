use crate::{
    cpu::{Cpu, CpuFlag},
    game::{macros, ram::wram},
};

pub fn return_to_cable_club_room(cpu: &mut Cpu) {
    cpu.pc = 0x581e;

    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3dd8); // GBPalWhiteOutWithDelay3
        cpu.pc = pc;
    }

    // ld hl, wFontLoaded
    cpu.set_hl(0xcfc3);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // res 0, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << 0));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wd72d], a
    cpu.write_byte(wram::W_D72D, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wDestinationWarpID], a
    {
        let warp_id = cpu.a;
        cpu.borrow_wram_mut().set_destination_warp_id(warp_id);
        cpu.pc += 3;
        cpu.cycle(16);
    }

    // call LoadMapData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0ecb); // LoadMapData
        cpu.pc = pc;
    }

    // farcall ClearVariablesOnEnterMap
    macros::farcall::farcall(cpu, 0x03, 0x407c);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // call GBFadeInFromWhite
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        cpu.call(0x1ebd); // GBFadeInFromWhite
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
