use crate::{
    cpu::{Cpu, CpuFlag},
    game::home::{overworld::load_destination_warp_position, predef::get_predef_registers},
};

pub fn load_tileset_header(cpu: &mut Cpu) {
    log::debug!("load_tileset_header()");

    cpu.pc = 0x44f4;

    // call GetPredefRegisters
    cpu.stack_push(0x0001);
    get_predef_registers(cpu);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld d, 0
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld a, [wCurMapTileset]
    cpu.a = cpu.borrow_wram().cur_map_tileset();
    cpu.pc += 3;
    cpu.cycle(16);

    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    cpu.set_hl(0x4558); // Tilesets
    cpu.pc += 3;
    cpu.cycle(12);

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

    // ld de, wTilesetBank
    cpu.set_de(0xd52a); // wTilesetBank
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, $b
    cpu.set_bc(0xb);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyData
    cpu.call(0x00b1);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ldh [hTileAnimations], a
    let tile_animations = cpu.a;
    cpu.borrow_wram_mut().set_tile_animations(tile_animations);
    cpu.pc += 2;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hMovingBGTilesCounter1], a
    let moving_bg_tiles_counter1 = cpu.a;
    cpu.borrow_wram_mut()
        .set_moving_bg_tiles_counter1(moving_bg_tiles_counter1);
    cpu.pc += 2;
    cpu.cycle(12);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, [wCurMapTileset]
    cpu.a = cpu.borrow_wram().cur_map_tileset();
    cpu.pc += 3;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, DungeonTilesets
    cpu.set_hl(0x454c); // DungeonTilesets
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, $1
    cpu.set_de(0x1);
    cpu.pc += 3;
    cpu.cycle(12);

    // call IsInArray
    cpu.call(0x3da7); // IsInArray

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
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

    // jr c, .dungeon
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return load_tileset_header_dungeon(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wCurMapTileset]
    cpu.a = cpu.borrow_wram().cur_map_tileset();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh a, [hPreviousTileset]
    cpu.a = cpu.borrow_wram().previous_tileset();
    cpu.pc += 2;
    cpu.cycle(12);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_tileset_header_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    load_tileset_header_dungeon(cpu);
}

fn load_tileset_header_dungeon(cpu: &mut Cpu) {
    cpu.pc = 0x4531;

    // ld a, [wDestinationWarpID]
    cpu.a = cpu.borrow_wram().destination_warp_id();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp $ff
    cpu.set_flag(CpuFlag::Z, cpu.a == 0xff);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0xff & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0xff);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_tileset_header_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call LoadDestinationWarpPosition
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        load_destination_warp_position(cpu);
        cpu.pc = pc;
    }

    // ld a, [wYCoord]
    cpu.a = cpu.borrow_wram().y_coord();
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, $1
    cpu.a &= 0x1;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wYBlockCoord], a
    let y_block_coord = cpu.a;
    cpu.borrow_wram_mut().set_y_block_coord(y_block_coord);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wXCoord]
    cpu.a = cpu.borrow_wram().x_coord();
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, $1
    cpu.a &= 0x1;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wXBlockCoord], a
    let x_block_coord = cpu.a;
    cpu.borrow_wram_mut().set_x_block_coord(x_block_coord);
    cpu.pc += 3;
    cpu.cycle(16);

    load_tileset_header_done(cpu);
}

fn load_tileset_header_done(cpu: &mut Cpu) {
    cpu.pc = 0x454b;

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
