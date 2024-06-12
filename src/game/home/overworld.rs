use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            gfx_constants,
            hardware_constants::MBC1_ROM_BANK,
            input_constants::{A_BUTTON, B_BUTTON, D_UP, SELECT, START},
            map_data_constants::{EAST_F, NORTH_F, SOUTH_F, WEST_F},
            palette_constants,
            sprite_data_constants::{
                PLAYER_DIR_DOWN, PLAYER_DIR_LEFT, PLAYER_DIR_RIGHT, PLAYER_DIR_UP,
                SPRITE_FACING_DOWN, SPRITE_FACING_LEFT, SPRITE_FACING_RIGHT, SPRITE_FACING_UP,
            },
        },
        home, macros,
        ram::{hram, vram, wram},
    },
    rom::ROM,
};

/// Load a new map
pub fn enter_map(cpu: &mut Cpu) {
    log::debug!("EnterMap");

    cpu.borrow_wram_mut().set_joy_ignore(0xff);

    cpu.call(0x0ecb); // LoadMapData
    macros::farcall::farcall(cpu, 0x03, 0x407c); // ClearVariablesOnEnterMap

    // has the player already made 3 steps since the last battle?
    if (cpu.read_byte(wram::W_D72C) & 1) != 0 {
        // minimum number of steps between battles
        cpu.borrow_wram_mut()
            .set_number_of_no_random_battle_steps_left(3);
    }

    // did a battle happen immediately before this?
    let w_d72e = cpu.read_byte(wram::W_D72E);

    // unset the "battle just happened" flag
    cpu.write_byte(wram::W_D72E, w_d72e & !(1 << 5));

    if (w_d72e & (1 << 5)) == 0 {
        cpu.borrow_wram_mut()
            .set_using_strength_out_of_battle(false);
    } else {
        cpu.call(0x0750); // MapEntryAfterBattle
    }

    let w_d732 = cpu.read_byte(wram::W_D732);

    // fly warp or dungeon warp
    if (w_d732 & 24) != 0 {
        macros::farcall::farcall(cpu, 0x1c, 0x4567); // EnterMapAnim
        cpu.call(0x231c); // UpdateSprites

        // reset "used warp pad" flag
        let w_d732 = cpu.read_byte(wram::W_D732);
        cpu.write_byte(wram::W_D732, w_d732 & !(1 << 3));

        // reset "disable battles" flag
        let w_d72e = cpu.read_byte(wram::W_D72E);
        cpu.write_byte(wram::W_D72E, w_d72e & !(1 << 4));
    }

    cpu.call(0x342a); // IsSurfingPikachuInParty

    // handle currents in SF islands and forced bike riding in cycling road
    macros::farcall::farcall(cpu, 0x03, 0x40d2); // CheckForceBikeOrSurf

    // reset "dungeon warp" flag
    let w_d732 = cpu.read_byte(wram::W_D732);
    cpu.write_byte(wram::W_D732, w_d732 & !(1 << 4));

    // reset "NPCs don't face the player" flag
    let value = cpu.read_byte(wram::W_D72D);
    cpu.write_byte(wram::W_D72D, value & !(1 << 5));

    cpu.call(0x231c); // UpdateSprites

    let current_map_script_flags = cpu.read_byte(wram::W_CURRENT_MAP_SCRIPT_FLAGS);

    // set bit 5 & 6 of wCurrentMapScriptFlags
    cpu.write_byte(
        wram::W_CURRENT_MAP_SCRIPT_FLAGS,
        current_map_script_flags | (1 << 5) | (1 << 6),
    );

    cpu.borrow_wram_mut().set_joy_ignore(0);

    // Fallthrough to OverworldLoop
    cpu.pc = 0x0242;
}

/// function to check if there is a sign or sprite in front of the player \
/// if so, carry is set. otherwise, carry is cleared
pub fn is_sprite_or_sign_in_front_of_player(cpu: &mut Cpu) {
    cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, 0);
    cpu.a = cpu.borrow_wram().num_signs();

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);

    // if there are signs
    if cpu.a > 0 {
        // get the coordinates in front of the player in de
        macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

        if sign_loop(cpu, cpu.d, cpu.e) {
            cpu.pc = cpu.stack_pop(); // ret
            log::debug!("is_sprite_or_sign_in_front_of_player() == true (sign in front of player)");
            return;
        }
    }

    // check if the player is front of a counter in a pokemon center, pokemart, etc. and if so, extend the range at which he can talk to the NPC
    // get the tile in front of the player in c
    macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);
    let tile_in_front_of_player = cpu.c;

    // list of tiles that extend talking range (counter tiles)
    let talking_over_tiles = cpu.borrow_wram().tileset_talking_over_tiles();

    if talking_over_tiles.contains(&tile_in_front_of_player) {
        // talking range in pixels (long range)
        cpu.d = 0x20;

        log::debug!("is_sprite_or_sign_in_front_of_player() (counter tile in front of player)");
        is_sprite_in_front_of_player2(cpu)
    } else {
        is_sprite_in_front_of_player(cpu)
    }
}

/// sets carry flag if a sprite is in front of the player, resets if not
pub fn is_sprite_in_front_of_player(cpu: &mut Cpu) {
    // talking range in pixels (normal range)
    cpu.d = 0x10;

    // fallthrough
    is_sprite_in_front_of_player2(cpu)
}

pub fn is_sprite_in_front_of_player2(cpu: &mut Cpu) {
    log::trace!("is_sprite_in_front_of_player2(range = {})", cpu.d);

    // Y and X position of player sprite
    cpu.b = 0x3c;
    cpu.c = 0x40;

    let direction = match cpu.read_byte(wram::W_SPRITE_PLAYER_STATE_DATA1_FACING_DIRECTION) {
        SPRITE_FACING_UP => {
            cpu.b -= cpu.d;
            PLAYER_DIR_UP
        }
        SPRITE_FACING_DOWN => {
            cpu.b += cpu.d;
            PLAYER_DIR_DOWN
        }
        SPRITE_FACING_LEFT => {
            cpu.c -= cpu.d;
            PLAYER_DIR_LEFT
        }
        SPRITE_FACING_RIGHT => {
            cpu.c += cpu.d;
            PLAYER_DIR_RIGHT
        }
        fd => unreachable!("Unknown player sprite direction: {}", fd),
    };

    cpu.borrow_wram_mut().set_player_direction(direction);

    cpu.set_hl(wram::W_SPRITE01_STATE_DATA1);

    // yellow does not have the "if sprites are existant" check
    cpu.e = 0x01;
    cpu.d = 0x0f;

    is_sprite_in_front_of_player2_sprite_loop(cpu);
}

fn is_sprite_in_front_of_player2_sprite_loop(cpu: &mut Cpu) {
    // image (0 if no sprite)
    if cpu.read_byte(cpu.hl()) == 0 {
        return is_sprite_in_front_of_player2_next_sprite(cpu);
    }

    // sprite visibility
    if cpu.read_byte(cpu.hl() + 2) == 0xff {
        return is_sprite_in_front_of_player2_next_sprite(cpu);
    }

    // Y location
    if cpu.read_byte(cpu.hl() + 4) != cpu.b {
        return is_sprite_in_front_of_player2_next_sprite(cpu);
    }

    // X location
    if cpu.read_byte(cpu.hl() + 6) != cpu.c {
        return is_sprite_in_front_of_player2_next_sprite(cpu);
    }

    // hl + 1 = x#SPRITESTATEDATA1_MOVEMENTSTATUS
    // set flag to make the sprite face the player
    {
        let value = cpu.read_byte(cpu.hl() + 1);
        cpu.write_byte(cpu.hl() + 1, value | (1 << 7));
    }

    cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, cpu.e);

    if cpu.e == 0xf {
        cpu.write_byte(wram::W_D436, 0xff);
    }

    cpu.set_flag(CpuFlag::Z, cpu.e == 0xf);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, true);

    log::debug!("is_sprite_in_front_of_player2() == {}", true);
    cpu.pc = cpu.stack_pop(); // ret
}

fn is_sprite_in_front_of_player2_next_sprite(cpu: &mut Cpu) {
    cpu.l += 0x10;
    cpu.e += 1;
    cpu.d -= 1;

    if cpu.d > 0 {
        return is_sprite_in_front_of_player2_sprite_loop(cpu);
    }

    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);

    log::trace!("is_sprite_in_front_of_player2() == {}", false);
    cpu.pc = cpu.stack_pop(); // ret
}

/// search if a player is facing a sign
fn sign_loop(cpu: &mut Cpu, y: u8, x: u8) -> bool {
    let num_signs = cpu.borrow_wram().num_signs() as u16;

    for idx in 0..num_signs {
        let sign_y = cpu.read_byte(wram::W_SIGN_COORDS + (idx * 2));
        let sign_x = cpu.read_byte(wram::W_SIGN_COORDS + (idx * 2) + 1);

        if sign_y == y && sign_x == x {
            // store sign text ID
            let text_id = cpu.read_byte(wram::W_SIGN_TEXT_IDS + idx);
            cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, text_id);

            return true;
        }
    }

    false
}

// Load data from the map header
pub fn load_map_header(cpu: &mut Cpu) {
    log::debug!("load_map_header()");

    cpu.pc = 0x0dab;

    // MarkTownVisitedAndLoadMissableObjects
    macros::farcall::farcall(cpu, 0x03, 0x6f93);

    // ld a, [wCurMapTileset]
    cpu.a = cpu.read_byte(wram::W_CUR_MAP_TILESET);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wUnusedD119], a
    cpu.write_byte(wram::W_UNUSED_D119, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wCurMap]
    cpu.a = cpu.read_byte(wram::W_CUR_MAP);
    cpu.pc += 3;
    cpu.cycle(16);

    // call SwitchToMapRomBank
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        switch_to_map_rom_bank(cpu);
        cpu.pc = pc;
    }

    // ld a, [wCurMapTileset]
    cpu.a = cpu.read_byte(wram::W_CUR_MAP_TILESET);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // res 7, a
    cpu.a &= !(1 << 7);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wCurMapTileset], a
    cpu.write_byte(wram::W_CUR_MAP_TILESET, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hPreviousTileset], a
    let previous_tileset = cpu.a;
    cpu.borrow_wram_mut().set_previous_tileset(previous_tileset);
    cpu.pc += 2;
    cpu.cycle(12);

    // bit 7, b
    cpu.set_flag(CpuFlag::Z, (cpu.b & (1 << 7)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // call GetMapHeaderPointer
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        get_map_header_pointer(cpu);
        cpu.pc = pc;
    }

    // ld de, wCurMapHeader
    cpu.set_de(wram::W_CUR_MAP_HEADER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld c, wCurMapHeaderEnd - wCurMapHeader
    cpu.c = (wram::W_CUR_MAP_HEADER_END - wram::W_CUR_MAP_HEADER) as u8;

    load_map_header_copy_fixed_header_loop(cpu);
}

fn load_map_header_copy_fixed_header_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0ddf;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // dec c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x00);
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .copyFixedHeaderLoop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_copy_fixed_header_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // initialize all the connected maps to disabled at first, before loading the actual values
    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wNorthConnectedMap], a
    cpu.write_byte(wram::W_NORTH_CONNECTED_MAP, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wSouthConnectedMap], a
    cpu.write_byte(wram::W_SOUTH_CONNECTED_MAP, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wWestConnectedMap], a
    cpu.write_byte(wram::W_WEST_CONNECTED_MAP, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wEastConnectedMap], a
    cpu.write_byte(wram::W_EAST_CONNECTED_MAP, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // copy connection data (if any) to WRAM
    // ld a, [wCurMapConnections]
    cpu.a = cpu.read_byte(wram::W_CUR_MAP_CONNECTIONS);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    load_map_header_check_north(cpu);
}

fn load_map_header_check_north(cpu: &mut Cpu) {
    cpu.pc = 0x0df7;

    // bit NORTH_F, b
    cpu.set_flag(CpuFlag::Z, (cpu.b & (1 << NORTH_F)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .checkSouth
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_check_south(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld de, wNorthConnectionHeader
    cpu.set_de(wram::W_NORTH_CONNECTION_HEADER);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyMapConnectionHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        copy_map_connection_header(cpu);
        cpu.pc = pc;
    }

    load_map_header_check_south(cpu);
}

fn load_map_header_check_south(cpu: &mut Cpu) {
    cpu.pc = 0x0e01;

    // bit SOUTH_F, b
    cpu.set_flag(CpuFlag::Z, (cpu.b & (1 << SOUTH_F)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .checkWest
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_check_west(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld de, wSouthConnectionHeader
    cpu.set_de(wram::W_SOUTH_CONNECTION_HEADER);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyMapConnectionHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        copy_map_connection_header(cpu);
        cpu.pc = pc;
    }

    load_map_header_check_west(cpu);
}

fn load_map_header_check_west(cpu: &mut Cpu) {
    cpu.pc = 0x0e0b;

    // bit WEST_F, b
    cpu.set_flag(CpuFlag::Z, (cpu.b & (1 << WEST_F)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .checkEast
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_check_east(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld de, wWestConnectionHeader
    cpu.set_de(wram::W_WEST_CONNECTION_HEADER);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyMapConnectionHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        copy_map_connection_header(cpu);
        cpu.pc = pc;
    }

    load_map_header_check_east(cpu);
}

fn load_map_header_check_east(cpu: &mut Cpu) {
    cpu.pc = 0x0e15;

    // bit EAST_F, b
    cpu.set_flag(CpuFlag::Z, (cpu.b & (1 << EAST_F)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .getObjectDataPointer
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_get_object_data_pointer(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld de, wEastConnectionHeader
    cpu.set_de(wram::W_EAST_CONNECTION_HEADER);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyMapConnectionHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        copy_map_connection_header(cpu);
        cpu.pc = pc;
    }

    load_map_header_get_object_data_pointer(cpu);
}

fn load_map_header_get_object_data_pointer(cpu: &mut Cpu) {
    cpu.pc = 0x0e1f;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wObjectDataPointerTemp], a
    let w_object_data_pointer_temp0 = cpu.a;
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wObjectDataPointerTemp + 1], a
    let w_object_data_pointer_temp1 = cpu.a;
    cpu.pc += 3;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wObjectDataPointerTemp]
    cpu.a = w_object_data_pointer_temp0;
    cpu.pc += 3;
    cpu.cycle(16);

    // ld l, a
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wObjectDataPointerTemp + 1]
    cpu.a = w_object_data_pointer_temp1;
    cpu.pc += 3;
    cpu.cycle(16);

    // hl = base of object data
    // ld h, a
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld de, wMapBackgroundTile
    cpu.set_de(0xd3ac); // only used to set map_background_tile to a?
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    let map_background_tile = cpu.a;
    cpu.borrow_wram_mut()
        .set_map_background_tile(map_background_tile);
    cpu.pc += 1;
    cpu.cycle(8);

    load_map_header_load_warp_data(cpu);
}

fn load_map_header_load_warp_data(cpu: &mut Cpu) {
    cpu.pc = 0x0e35;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wNumberOfWarps], a
    let number_of_warps = cpu.a;
    cpu.borrow_wram_mut().set_number_of_warps(number_of_warps);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .loadSignData
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_load_sign_data(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld de, wWarpEntries
    cpu.set_de(wram::W_WARP_ENTRIES);
    cpu.pc += 3;
    cpu.cycle(12);

    load_map_header_warp_loop(cpu);
}

// one warp per loop iteration
fn load_map_header_warp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0e40;

    // ld b, 4
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);

    load_map_header_warp_inner_loop(cpu);
}

fn load_map_header_warp_inner_loop(cpu: &mut Cpu) {
    cpu.pc = 0x0e42;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .warpInnerLoop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_warp_inner_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // dec c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x00);
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .warpLoop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_warp_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    load_map_header_load_sign_data(cpu);
}

fn load_map_header_load_sign_data(cpu: &mut Cpu) {
    cpu.pc = 0x0e4b;

    // number of signs
    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wNumSigns], a
    let num_signs = cpu.a;
    cpu.borrow_wram_mut().set_num_signs(num_signs);
    cpu.pc += 3;
    cpu.cycle(16);

    // are there any signs?
    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // if not, skip this
    // jr z, .loadSpriteData
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_load_sprite_data(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call CopySignData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        copy_sign_data(cpu);
        cpu.pc = pc;
    }

    load_map_header_load_sprite_data(cpu);
}

fn load_map_header_load_sprite_data(cpu: &mut Cpu) {
    cpu.pc = 0x0e55;

    // ld a, [wd72e]
    cpu.a = cpu.read_byte(wram::W_D72E);
    cpu.pc += 3;
    cpu.cycle(16);

    // did a battle happen immediately before this?
    // bit 5, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << 5)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // if so, skip this because battles don't destroy this data
    // jr nz, .finishUp
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_finish_up(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call InitSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        init_sprites(cpu);
        cpu.pc = pc;
    }

    load_map_header_finish_up(cpu);
}

fn load_map_header_finish_up(cpu: &mut Cpu) {
    cpu.pc = 0x0e5f;

    // predef LoadTilesetHeader
    macros::predef::predef_call!(cpu, LoadTilesetHeader);

    // ld a, [wd72e]
    cpu.a = cpu.read_byte(wram::W_D72E);
    cpu.pc += 3;
    cpu.cycle(16);

    // did a battle happen immediately before this?
    // bit 5, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << 5)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .skip_pika_spawn
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return load_map_header_skip_pika_spawn(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // callfar SchedulePikachuSpawnForAfterText
    macros::farcall::callfar(cpu, 0x3f, 0x44fa);

    load_map_header_skip_pika_spawn(cpu);
}

fn load_map_header_skip_pika_spawn(cpu: &mut Cpu) {
    cpu.pc = 0x0e73;

    // callfar LoadWildData
    macros::farcall::callfar(cpu, 0x03, 0x4b62);

    // restore hl from before going to the warp/sign/sprite data (this value was saved for seemingly no purpose)
    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // map height in 4x4 tile blocks
    // ld a, [wCurMapHeight]
    cpu.a = cpu.borrow_wram().cur_map_height();
    cpu.pc += 3;
    cpu.cycle(16);

    // double it
    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // store map height in 2x2 tile blocks
    // ld [wCurrentMapHeight2], a
    let current_map_height_2 = cpu.a;
    cpu.borrow_wram_mut()
        .set_current_map_height_2(current_map_height_2);
    cpu.pc += 3;
    cpu.cycle(16);

    // map width in 4x4 tile blocks
    // ld a, [wCurMapWidth]
    cpu.a = cpu.borrow_wram().cur_map_width();
    cpu.pc += 3;
    cpu.cycle(16);

    // double it
    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // map width in 2x2 tile blocks
    // ld [wCurrentMapWidth2], a
    let current_map_width_2 = cpu.a;
    cpu.borrow_wram_mut()
        .set_current_map_width_2(current_map_width_2);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wCurMap]
    cpu.a = cpu.read_byte(wram::W_CUR_MAP);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, $00
    cpu.b = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh a, [hLoadedROMBank]
    cpu.a = cpu.borrow_wram().loaded_rom_bank();
    cpu.pc += 2;
    cpu.cycle(12);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, BANK(MapSongBanks)
    cpu.a = 0x3f;
    cpu.pc += 2;
    cpu.cycle(8);

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // ld hl, MapSongBanks
    cpu.set_hl(0x4000); // MapSongBanks
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

    // music 1
    // ld [wMapMusicSoundID], a
    let map_music_sound_id = cpu.a;
    cpu.borrow_wram_mut()
        .set_map_music_sound_id(map_music_sound_id);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // music 2
    // ld [wMapMusicROMBank], a
    let map_music_rom_bank = cpu.a;
    cpu.borrow_wram_mut()
        .set_map_music_rom_bank(map_music_rom_bank);
    cpu.pc += 3;
    cpu.cycle(16);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

/// Copy map connection data from ROM to WRAM.
///
/// Input: hl = source, de = destination \
/// Output: hl = end of source
pub fn copy_map_connection_header(cpu: &mut Cpu) {
    const MAP_CONNECTION_HEADER_SIZE: u16 = 0x0b;

    log::debug!("copy_map_connection_header()");

    for i in 0..MAP_CONNECTION_HEADER_SIZE {
        let byte = cpu.read_byte(cpu.hl() + i);
        cpu.write_byte(cpu.de() + i, byte);
    }

    cpu.set_hl(cpu.hl() + MAP_CONNECTION_HEADER_SIZE);

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input: hl = pointer to bg_event list to load
/// Output: hl = pointer to just after the bg_event list
pub fn copy_sign_data(cpu: &mut Cpu) {
    log::debug!("copy_sign_data()");

    cpu.set_de(wram::W_SIGN_COORDS);
    cpu.set_bc(wram::W_SIGN_TEXT_IDS);

    let num_signs = cpu.borrow_wram().num_signs();

    for _ in 0..num_signs {
        let y = cpu.read_byte(cpu.hl());
        let x = cpu.read_byte(cpu.hl() + 1);
        let txt_id = cpu.read_byte(cpu.hl() + 2);
        cpu.set_hl(cpu.hl() + 3);

        cpu.write_byte(cpu.de(), y);
        cpu.write_byte(cpu.de() + 1, x);
        cpu.set_de(cpu.de() + 2);

        cpu.write_byte(cpu.bc(), txt_id);
        cpu.set_bc(cpu.bc() + 1);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn load_map_data(cpu: &mut Cpu) {
    log::debug!("load_map_data()");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.call(0x0061); // DisableLCD

    reset_map_variables(cpu);

    cpu.call(0x36a3); // LoadTextBoxTilePatterns
    cpu.call(0x0dab); // LoadMapHeader

    // load tile pattern data for sprites
    cpu.call(0x3dba); // InitMapSprites

    load_screen_related_data(cpu);
    copy_map_view_to_vram(cpu, vram::V_BG_MAP0);
    cpu.borrow_wram_mut().set_update_sprites_enabled(0x01);

    cpu.call(0x007b); // EnableLCD

    cpu.b = palette_constants::SET_PAL_OVERWORLD;
    cpu.call(0x3e05); // RunPaletteCommand

    cpu.call(0x07d7); // LoadPlayerSpriteGraphics

    // fly warp or dungeon warp
    let w_d732 = cpu.read_byte(wram::W_D732);

    if (w_d732 & (1 << 4 | 1 << 3)) == 0 {
        let w_d733 = cpu.read_byte(wram::W_FLAGS_D733);

        if (w_d733 & (1 << 1)) == 0 {
            cpu.call(0x21e3); // UpdateMusic6Times
            cpu.call(0x2176); // PlayDefaultMusicFadeOutCurrent
        }
    }

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon
    cpu.pc = cpu.stack_pop(); // ret
}

pub fn load_screen_related_data(cpu: &mut Cpu) {
    log::trace!("load_screen_related_data()");

    cpu.call(0x083c); // LoadTileBlockMap
    cpu.call(0x0828); // LoadTilesetTilePatternData
    cpu.call(0x0b06); // LoadCurrentMapView
}

pub fn reload_map_after_surfing_minigame(cpu: &mut Cpu) {
    log::debug!("reload_map_after_surfing_minigame()");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.call(0x0061); // DisableLCD

    reset_map_variables(cpu);

    cpu.a = cpu.borrow_wram().cur_map();
    cpu.stack_push(0x0001);
    switch_to_map_rom_bank(cpu);

    load_screen_related_data(cpu);

    copy_map_view_to_vram(cpu, vram::V_BG_MAP0);
    copy_map_view_to_vram(cpu, vram::V_BG_MAP1);

    cpu.call(0x007b); // EnableLCD
    cpu.call(0x3e1e); // ReloadMapSpriteTilePatterns

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon

    // SetMapSpecificScriptFlagsOnMapReload
    macros::farcall::farcall(cpu, 0x3c, 0x42da);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn reload_map_after_printer(cpu: &mut Cpu) {
    log::debug!("reload_map_after_printer()");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.a = cpu.borrow_wram().cur_map();

    cpu.stack_push(0x0001);
    switch_to_map_rom_bank(cpu);

    cpu.call(0x083c); // LoadTileBlockMap

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon

    // SetMapSpecificScriptFlagsOnMapReload
    macros::farcall::farcall(cpu, 0x3c, 0x42da);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn reset_map_variables(cpu: &mut Cpu) {
    log::trace!("reset_map_variables()");

    cpu.borrow_wram_mut().set_map_view_vram_pointer(0x9800);
    cpu.write_byte(hram::H_SCY, 0);
    cpu.write_byte(hram::H_SCX, 0);
    cpu.borrow_wram_mut().set_walk_counter(0);
    cpu.write_byte(wram::W_UNUSED_D119, 0);
    cpu.borrow_wram_mut().set_sprite_set_id(0);
    cpu.borrow_wram_mut().set_walk_bike_surf_state_copy(0);
}

pub fn copy_map_view_to_vram(cpu: &mut Cpu, dst: u16) {
    log::trace!("copy_map_view_to_vram({:04x})", dst);

    cpu.set_hl(wram::W_TILE_MAP);
    cpu.set_de(dst);

    for _ in 0..gfx_constants::SCREEN_HEIGHT {
        for x in 0..gfx_constants::SCREEN_WIDTH {
            let byte = cpu.read_byte(cpu.hl() + (x as u16));
            cpu.write_byte(cpu.de() + (x as u16), byte);
        }

        cpu.set_hl(cpu.hl() + (gfx_constants::SCREEN_WIDTH as u16));
        cpu.set_de(cpu.de() + (gfx_constants::BG_MAP_WIDTH as u16));
    }
}

/// Function to switch to the ROM bank that a map is stored in.
///
/// Input: a = map number
pub fn switch_to_map_rom_bank(cpu: &mut Cpu) {
    // 3f:43e4 MapHeaderBanks
    const MAP_HEADER_BANKS: usize = (0x3f * 0x4000) | (0x43e4 & 0x3fff);

    log::trace!("switch_to_map_rom_bank({:02x})", cpu.a);

    let map_number = cpu.a as usize;
    let target_bank = ROM[MAP_HEADER_BANKS + map_number];

    cpu.a = target_bank;
    cpu.call(0x3e7e); // BankswitchCommon

    cpu.pc = cpu.stack_pop(); // ret
}

/// Output:
/// hl = pointer to the map header of the current map
pub fn get_map_header_pointer(cpu: &mut Cpu) {
    // 3f:41f2 MapHeaderPointers
    const MAP_HEADER_POINTERS: usize = (0x3f * 0x4000) | (0x41f2 & 0x3fff);

    log::debug!("get_map_header_pointer()");

    let cur_map = cpu.borrow_wram().cur_map() as usize;
    let pointer = MAP_HEADER_POINTERS + (cur_map * 2);

    // Pointers are stored in little-endian format
    cpu.set_hl(u16::from_le_bytes([ROM[pointer], ROM[pointer + 1]]));

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn ignore_input_for_half_second(cpu: &mut Cpu) {
    log::debug!("ignore_input_for_half_second()");

    cpu.borrow_wram_mut().set_ignore_input_counter(30);

    // set ignore input bit
    cpu.a = cpu.read_byte(wram::W_D730) | 0b00100110;
    cpu.write_byte(wram::W_D730, cpu.a);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn reset_using_strength_out_of_battle_bit(cpu: &mut Cpu) {
    log::debug!("reset_using_strength_out_of_battle_bit()");

    let value = cpu.read_byte(wram::W_D728);
    cpu.write_byte(wram::W_D728, value & !(1 << 0));

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn force_bike_or_surf(cpu: &mut Cpu) {
    log::debug!("force_bike_or_surf()");

    cpu.b = 0x05; // BANK(RedSprite)
    cpu.set_hl(0x07d7); // LoadPlayerSpriteGraphics (in bank 0)
    cpu.call(0x3e84); // Bankswitch

    // update map/player state?
    cpu.call(0x216b); // PlayDefaultMusic

    cpu.pc = cpu.stack_pop(); // ret
}

// Handle the player jumping down a ledge in the overworld.
pub fn handle_mid_jump(cpu: &mut Cpu) {
    if (cpu.read_byte(wram::W_D736) & (1 << 6)) != 0 {
        macros::farcall::farcall(cpu, 0x1c, 0x48df); // _HandleMidJump
    }

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn is_spinning(cpu: &mut Cpu) {
    let w_d736 = cpu.read_byte(wram::W_D736);

    // if spinning
    if (w_d736 & (1 << 7)) != 0 {
        log::debug!("is_spinning() spinnig");
        macros::farcall::farcall(cpu, 0x11, 0x5077); // LoadSpinnerArrowTiles
    } else {
        log::trace!("is_spinning() not spinning");
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input:
/// hl = pointer to length of object_event list
/// Output:
/// hl = pointer to just after list of object_event
pub fn init_sprites(cpu: &mut Cpu) {
    log::debug!("init_sprites()");

    let num_sprites = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);

    // save the number of sprites
    cpu.borrow_wram_mut().set_num_sprites(num_sprites);

    zero_sprite_state_data(cpu);
    disable_regular_sprites(cpu);

    // Zero out map sprite data
    for i in 0..0x20 {
        cpu.write_byte(wram::W_MAP_SPRITE_DATA + i, 0);
    }

    // copy sprite data
    for i in 0..(num_sprites as u16) {
        // x#SPRITESTATEDATA1_PICTUREID
        let pic_id = cpu.read_byte(cpu.hl());
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + (i * 0x0010), pic_id);

        // x#SPRITESTATEDATA2_MAPY
        let map_y = cpu.read_byte(cpu.hl() + 1);
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + (i * 0x0010) + 0x0104, map_y);

        // x#SPRITESTATEDATA2_MAPX
        let map_x = cpu.read_byte(cpu.hl() + 2);
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + (i * 0x0010) + 0x0105, map_x);

        // x#SPRITESTATEDATA2_MOVEMENTBYTE1
        let mb1 = cpu.read_byte(cpu.hl() + 3);
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + (i * 0x0010) + 0x0106, mb1);

        // save movement byte 2
        let mb2 = cpu.read_byte(cpu.hl() + 4);
        cpu.write_byte(hram::H_LOAD_SPRITE_TEMP1, mb2);

        // save text ID and flags byte
        let txt_id = cpu.read_byte(cpu.hl() + 5);
        cpu.write_byte(hram::H_LOAD_SPRITE_TEMP2, txt_id);

        cpu.set_hl(cpu.hl() + 6);
        load_sprite(cpu, i * 2);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Zero out sprite state data for sprites 01..=14 \
/// Sprite 15 is used for Pikachu
fn zero_sprite_state_data(cpu: &mut Cpu) {
    log::trace!("zero_sprite_state_data()");

    const SPRITE_COUNT: u16 = 14;
    const DATA_SIZE: u16 = 0x10;

    for i in 0..(SPRITE_COUNT * DATA_SIZE) {
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + i, 0);
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA2 + i, 0);
    }
}

/// Disable SPRITESTATEDATA1_IMAGEINDEX (set to $ff) for sprites 01..=14
fn disable_regular_sprites(cpu: &mut Cpu) {
    log::trace!("disable_regular_sprites()");

    for i in 0..14 {
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1_IMAGE_INDEX + (i * 0x10), 0xff);
    }
}

/// Input:
/// data_offset = 2x index into MAP_SPRITE_DATA and W_MAP_SPRITE_EXTRA_DATA
/// hl = pointer to last part (arg 7) of object_event
fn load_sprite(cpu: &mut Cpu, data_offset: u16) {
    log::trace!(
        "load_sprite({:02x}, hl = {:02x}:{:04x})",
        data_offset,
        cpu.bank(),
        cpu.hl()
    );

    // store movement byte 2 in byte 0 of sprite entry
    let temp1 = cpu.read_byte(hram::H_LOAD_SPRITE_TEMP1);
    cpu.write_byte(wram::W_MAP_SPRITE_DATA + data_offset, temp1);

    // store text ID in byte 1 of sprite entry
    let temp2 = cpu.read_byte(hram::H_LOAD_SPRITE_TEMP2);
    cpu.write_byte(wram::W_MAP_SPRITE_DATA + data_offset + 1, temp2 & 0x3f);

    // temp2 -> temp1
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP1, temp2);

    if (temp2 & (1 << 6)) != 0 {
        load_sprite_trainer_sprite(cpu, data_offset);
    } else if (temp2 & (1 << 7)) != 0 {
        load_sprite_item_ball_sprite(cpu, data_offset);
    } else {
        // zero both bytes, since regular sprites don't use this extra space
        cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset, 0);
        cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset + 1, 0);
    }
}

fn load_sprite_trainer_sprite(cpu: &mut Cpu, data_offset: u16) {
    // save trainer class
    let trainer_class = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP1, trainer_class);

    // save trainer number (within class)
    let trainer_no = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP2, trainer_no);

    // store trainer class in byte 0 of the entry
    cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset, trainer_class);

    // store trainer number in byte 1 of the entry
    cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset + 1, trainer_no);
}

fn load_sprite_item_ball_sprite(cpu: &mut Cpu, data_offset: u16) {
    // save item number
    let item_number = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP1, item_number);

    // store item number in byte 0 of the entry
    cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset, item_number);

    // zero byte 1, since it is not used
    cpu.write_byte(wram::W_MAP_SPRITE_EXTRA_DATA + data_offset + 1, 0);
}

/// Return carry if Up+Select+B, Start or A are pressed in c frames. \
/// Used only in the intro and title screen.
pub fn check_for_user_interruption(cpu: &mut Cpu) {
    let frames = cpu.c;

    for _ in 0..frames {
        home::vblank::delay_frame(cpu);

        let saved_bc = cpu.bc();
        cpu.call(0x381e); // JoypadLowSensitivity
        cpu.set_bc(saved_bc);

        let joy_held = cpu.read_byte(hram::H_JOY_HELD);

        if joy_held == (D_UP | SELECT | B_BUTTON) {
            cpu.set_flag(CpuFlag::C, true);
            cpu.pc = cpu.stack_pop(); // ret
            log::debug!("check_for_user_interruption() == {}", true);
            return;
        }

        let joy_5 = cpu.read_byte(hram::H_JOY_5);

        if (joy_5 & (START | A_BUTTON)) != 0 {
            cpu.set_flag(CpuFlag::C, true);
            cpu.pc = cpu.stack_pop(); // ret
            log::debug!("check_for_user_interruption() == {}", true);
            return;
        }
    }

    cpu.set_flag(CpuFlag::C, false);
    cpu.pc = cpu.stack_pop(); // ret
    log::trace!("check_for_user_interruption() == {}", false);
}

/// Load position data for destination warp when switching maps
pub fn load_destination_warp_position(cpu: &mut Cpu, warp_id: u8, warp_data: u16) {
    log::debug!(
        "load_destination_warp_position(warp_id={}, warp_data={:04x})",
        warp_id,
        warp_data
    );

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    // Load bank
    let bank = cpu.borrow_wram().predef_parent_bank();
    cpu.borrow_wram_mut().set_loaded_rom_bank(bank);
    cpu.write_byte(MBC1_ROM_BANK, bank);

    // Read data
    let src = warp_data + (warp_id * 4) as u16;
    let pointer = u16::from_be_bytes([cpu.read_byte(src), cpu.read_byte(src + 1)]);
    let y = cpu.read_byte(src + 2);
    let x = cpu.read_byte(src + 3);

    // Write data
    let wram = cpu.borrow_wram_mut();
    wram.set_current_tile_block_map_view_pointer(pointer);
    wram.set_y_coord(y);
    wram.set_x_coord(x);

    // Restore bank
    cpu.borrow_wram_mut().set_loaded_rom_bank(saved_bank);
    cpu.write_byte(MBC1_ROM_BANK, saved_bank);
}
