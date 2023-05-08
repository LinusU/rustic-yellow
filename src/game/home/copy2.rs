use crate::{
    cpu::{Cpu, CpuFlag},
    game::{constants::gfx_constants, macros},
};

use super::palettes;

/// Clear tilemap area cxb at hl.
pub fn clear_screen_area(cpu: &mut Cpu) {
    cpu.pc = 0x1692;

    // ld a, \" \" ; blank tile
    cpu.a = 0x7f;
    cpu.pc += 1;
    cpu.cycle(8);

    // screen width
    // ld de 20
    cpu.set_de(20);
    cpu.pc += 3;
    cpu.cycle(12);

    clear_screen_area_y(cpu);
}

fn clear_screen_area_y(cpu: &mut Cpu) {
    cpu.pc = 0x1697;

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    clear_screen_area_x(cpu);
}

fn clear_screen_area_x(cpu: &mut Cpu) {
    cpu.pc = 0x1699;

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // dec c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x00);
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .x
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return clear_screen_area_x(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .y
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return clear_screen_area_y(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

/// Clear wTileMap, then wait for the bg map to update.
pub fn clear_screen(cpu: &mut Cpu) {
    for y in 0..=gfx_constants::SCREEN_HEIGHT {
        for x in 0..=gfx_constants::SCREEN_WIDTH {
            cpu.write_byte(macros::coords::coord!(x, y), 0x7f); // " "
        }
    }

    palettes::delay3(cpu);
}
