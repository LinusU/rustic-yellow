use crate::{
    cpu::{Cpu, CpuFlag},
    game::ram::wram,
};

pub fn load_wild_data(cpu: &mut Cpu) {
    log::debug!("load_wild_data()");

    cpu.pc = 0x4b62;

    // ld hl, WildDataPointers
    cpu.set_hl(0x4b95); // WildDataPointers
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [wCurMap]
    cpu.a = cpu.borrow_wram().cur_map();
    cpu.pc += 3;
    cpu.cycle(16);

    // get wild data for current map
    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, 0
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld h, [hl]
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // hl now points to wild data for current map
    // ld l, a
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wGrassRate], a
    cpu.write_byte(wram::W_GRASS_RATE, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // if no grass data, skip to surfing data
    // jr z, .NoGrassData
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_wild_data_no_grass_data(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // otherwise, load grass data
    // ld de, wGrassMons
    cpu.set_de(wram::W_GRASS_MONS);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, $14
    cpu.set_bc(0x14);
    cpu.pc += 3;
    cpu.cycle(12);

    cpu.call(0x00b1); // CopyData

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld bc, $14
    cpu.set_bc(0x14);
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    load_wild_data_no_grass_data(cpu);
}

fn load_wild_data_no_grass_data(cpu: &mut Cpu) {
    cpu.pc = 0x4b86;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wWaterRate], a
    cpu.write_byte(wram::W_WATER_RATE, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // if no water data, we're done
    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // otherwise, load surfing data
    // ld de, wWaterMons
    cpu.set_de(wram::W_WATER_MONS);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, $14
    cpu.set_bc(0x14);
    cpu.pc += 3;
    cpu.cycle(12);

    cpu.call(0x00b1); // CopyData

    cpu.pc = cpu.stack_pop(); // ret
}
