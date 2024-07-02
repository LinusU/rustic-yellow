use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            audio_constants::CHAN5,
            gfx_constants,
            hardware_constants::MBC1_ROM_BANK,
            input_constants::{A_BUTTON, B_BUTTON, D_DOWN, D_LEFT, D_RIGHT, D_UP, SELECT, START},
            map_constants::{
                INDIGO_PLATEAU, LAST_MAP, ROCKET_HIDEOUT_B1F, ROCKET_HIDEOUT_B2F,
                ROCKET_HIDEOUT_B4F, ROCK_TUNNEL_1F, ROUTE_17, ROUTE_23, SS_ANNE_3F,
            },
            map_data_constants::{EAST_F, MAP_BORDER, NORTH_F, SOUTH_F, WEST_F},
            music_constants::{SFX_COLLISION, SFX_GO_INSIDE, SFX_GO_OUTSIDE},
            palette_constants,
            sprite_data_constants::{
                PLAYER_DIR_DOWN, PLAYER_DIR_LEFT, PLAYER_DIR_RIGHT, PLAYER_DIR_UP,
                SPRITE_FACING_DOWN, SPRITE_FACING_LEFT, SPRITE_FACING_RIGHT, SPRITE_FACING_UP,
            },
            tileset_constants::{CEMETERY, FACILITY, OVERWORLD, PLATEAU, SHIP, SHIP_PORT},
        },
        data::tilesets::bike_riding_tilesets::BIKE_RIDING_TILESETS,
        home, macros,
        ram::{hram, vram, wram},
    },
    game_state::BattleResult,
    rom::ROM,
};

const TILE_PAIR_COLLISIONS_LAND: u16 = 0x0ada;
const TILE_PAIR_COLLISIONS_WATER: u16 = 0x0afc;

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
        map_entry_after_battle(cpu);
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

// check if the player has stepped onto a warp after having collided
pub fn check_warps_collision(cpu: &mut Cpu) {
    log::debug!("check_warps_collision()");

    cpu.c = cpu.borrow_wram().number_of_warps();
    cpu.set_hl(wram::W_WARP_ENTRIES);

    loop {
        let warp_y = cpu.read_byte(cpu.hl());

        if warp_y == cpu.borrow_wram().y_coord() {
            let warp_x = cpu.read_byte(cpu.hl() + 1);

            if warp_x == cpu.borrow_wram().x_coord() {
                let warp_id = cpu.read_byte(cpu.hl() + 2);
                cpu.borrow_wram_mut().set_destination_warp_id(warp_id);

                let map_id = cpu.read_byte(cpu.hl() + 3);
                cpu.borrow_wram_mut().set_warp_destination_map(map_id);

                log::debug!("check_warps_collision() - found warp");
                return warp_found2(cpu);
            }
        }

        cpu.set_hl(cpu.hl() + 4);
        cpu.c -= 1;

        if cpu.c == 0 {
            // jp OverworldLoop
            cpu.pc = 0x0242;
            return;
        }
    }
}

/// Input: hl = pointer to warp entry
pub fn warp_found1(cpu: &mut Cpu) {
    log::debug!("warp_found1()");

    let warp_id = cpu.read_byte(cpu.hl());
    cpu.borrow_wram_mut().set_destination_warp_id(warp_id);

    let map_id = cpu.read_byte(cpu.hl() + 1);
    cpu.borrow_wram_mut().set_warp_destination_map(map_id);

    warp_found2(cpu)
}

/// Input: c = which warp, where 0 is last warp
pub fn warp_found2(cpu: &mut Cpu) {
    log::debug!("warp_found2()");

    let warp_id = cpu.borrow_wram().number_of_warps() - cpu.c;
    cpu.borrow_wram_mut().set_warped_from_which_warp(warp_id);

    let map_id = cpu.borrow_wram().cur_map();
    cpu.borrow_wram_mut().set_warped_from_which_map(map_id);

    cpu.stack_push(0x0001);
    check_if_in_outside_map(cpu);
    let in_outside_map = cpu.flag(CpuFlag::Z);

    let destination = cpu.borrow_wram().warp_destination_map();

    match (in_outside_map, destination) {
        // this is for handling "outside" maps that can't have the 0xFF destination map
        (true, _) => {
            cpu.borrow_wram_mut().set_last_map(map_id);
            cpu.borrow_wram_mut().set_cur_map(destination);

            if destination == ROCK_TUNNEL_1F {
                cpu.borrow_wram_mut().set_map_pal_offset(0x06);
                cpu.call(0x1eb6); // GBFadeOutToBlack
            }

            macros::farcall::callfar(cpu, 0x3f, 0x45fa); // SetPikachuSpawnOutside

            play_map_change_sound(cpu);
        }

        // for maps that can have the 0xFF destination map, which means to return to the outside map
        // not all these maps are necessarily indoors, though
        (false, LAST_MAP) => {
            macros::farcall::callfar(cpu, 0x3f, 0x469a); // SetPikachuSpawnBackOutside

            let map_id = cpu.borrow_wram().last_map();
            cpu.borrow_wram_mut().set_cur_map(map_id);

            play_map_change_sound(cpu);
            cpu.borrow_wram_mut().set_map_pal_offset(0);
        }

        // if not going back to the previous map
        (false, destination) => {
            cpu.borrow_wram_mut().set_cur_map(destination);

            macros::farcall::farcall(cpu, 0x1c, 0x47e7); // IsPlayerStandingOnWarpPadOrHole
            let on_warp_pad_or_hole = cpu.borrow_wram().standing_on_warp_pad_or_hole();

            // is the player on a warp pad?
            if on_warp_pad_or_hole == 1 {
                macros::farcall::farcall(cpu, 0x1c, 0x4615); // _LeaveMapAnim
                cpu.borrow_wram_mut().set_used_warp_pad(true);
            } else {
                play_map_change_sound(cpu);
            }

            // Clear bit 0 & 1
            let value = cpu.read_byte(wram::W_D736);
            cpu.write_byte(wram::W_D736, value & !((1 << 0) | (1 << 1)));

            macros::farcall::callfar(cpu, 0x3f, 0x465b); // SetPikachuSpawnWarpPad
        }
    }

    // have the player's sprite step out from the door (if there is one)
    let value = cpu.read_byte(wram::W_D736);
    cpu.write_byte(wram::W_D736, value | (1 << 0));

    ignore_input_for_half_second(cpu);
    enter_map(cpu)
}

/// if no matching warp was found
pub fn check_map_connections(cpu: &mut Cpu) {
    log::trace!("check_map_connections()");

    let x_coord = cpu.borrow_wram().x_coord();

    if x_coord == 0xff {
        return check_map_connections_move_to_west(cpu);
    }

    if x_coord == cpu.borrow_wram().current_map_width_2() {
        return check_map_connections_move_to_east(cpu);
    }

    let y_coord = cpu.borrow_wram().y_coord();

    if y_coord == 0xff {
        return check_map_connections_move_to_north(cpu);
    }

    if y_coord == cpu.borrow_wram().current_map_height_2() {
        return check_map_connections_move_to_south(cpu);
    }

    // jp OverworldLoop
    cpu.pc = 0x0242;
}

pub fn check_map_connections_move_to_west(cpu: &mut Cpu) {
    log::debug!("check_map_connections() - moving to west map");

    let connected_map = cpu.borrow_wram().west().connected_map();
    cpu.borrow_wram_mut().set_cur_map(connected_map);

    // new X coordinate upon entering west map
    let x_alignment = cpu.borrow_wram().west().connected_map_x_alignment();
    cpu.borrow_wram_mut().set_x_coord(x_alignment);

    // Y adjustment upon entering west map
    let mut y_coord = cpu.borrow_wram().y_coord();
    y_coord += cpu.borrow_wram().west().connected_map_y_alignment();
    cpu.borrow_wram_mut().set_y_coord(y_coord);

    // pointer to upper left corner of map without adjustment for Y position
    let mut ptr = cpu.borrow_wram().west().connected_map_view_pointer();

    let map_width = cpu.borrow_wram().west().connected_map_width();
    ptr += ((map_width as u16) + (MAP_BORDER * 2)) * ((y_coord >> 1) as u16);

    // pointer to upper left corner of current tile block map section
    cpu.borrow_wram_mut()
        .set_current_tile_block_map_view_pointer(ptr);

    check_map_connections_load_new_map(cpu)
}

pub fn check_map_connections_move_to_east(cpu: &mut Cpu) {
    log::debug!("check_map_connections() - moving to east map");

    let connected_map = cpu.borrow_wram().east().connected_map();
    cpu.borrow_wram_mut().set_cur_map(connected_map);

    // new X coordinate upon entering east map
    let x_alignment = cpu.borrow_wram().east().connected_map_x_alignment();
    cpu.borrow_wram_mut().set_x_coord(x_alignment);

    // Y adjustment upon entering east map
    let mut y_coord = cpu.borrow_wram().y_coord();
    y_coord += cpu.borrow_wram().east().connected_map_y_alignment();
    cpu.borrow_wram_mut().set_y_coord(y_coord);

    // pointer to upper left corner of map without adjustment for Y position
    let mut ptr = cpu.borrow_wram().east().connected_map_view_pointer();

    let map_width = cpu.borrow_wram().east().connected_map_width();
    ptr += ((map_width as u16) + (MAP_BORDER * 2)) * ((y_coord >> 1) as u16);

    // pointer to upper left corner of current tile block map section
    cpu.borrow_wram_mut()
        .set_current_tile_block_map_view_pointer(ptr);

    check_map_connections_load_new_map(cpu)
}

fn check_map_connections_move_to_north(cpu: &mut Cpu) {
    log::debug!("check_map_connections() - moving to north map");

    let connected_map = cpu.borrow_wram().north().connected_map();
    cpu.borrow_wram_mut().set_cur_map(connected_map);

    // new Y coordinate upon entering north map
    let y_alignment = cpu.borrow_wram().north().connected_map_y_alignment();
    cpu.borrow_wram_mut().set_y_coord(y_alignment);

    // X adjustment upon entering north map
    let mut x_coord = cpu.borrow_wram().x_coord();
    x_coord += cpu.borrow_wram().north().connected_map_x_alignment();
    cpu.borrow_wram_mut().set_x_coord(x_coord);

    // pointer to upper left corner of map without adjustment for X position
    let mut ptr = cpu.borrow_wram().north().connected_map_view_pointer();

    ptr += (x_coord >> 1) as u16;

    // pointer to upper left corner of current tile block map section
    cpu.borrow_wram_mut()
        .set_current_tile_block_map_view_pointer(ptr);

    check_map_connections_load_new_map(cpu)
}

fn check_map_connections_move_to_south(cpu: &mut Cpu) {
    log::debug!("check_map_connections() - moving to south map");

    let connected_map = cpu.borrow_wram().south().connected_map();
    cpu.borrow_wram_mut().set_cur_map(connected_map);

    // new Y coordinate upon entering south map
    let y_alignment = cpu.borrow_wram().south().connected_map_y_alignment();
    cpu.borrow_wram_mut().set_y_coord(y_alignment);

    // X adjustment upon entering south map
    let mut x_coord = cpu.borrow_wram().x_coord();
    x_coord += cpu.borrow_wram().south().connected_map_x_alignment();
    cpu.borrow_wram_mut().set_x_coord(x_coord);

    // pointer to upper left corner of map without adjustment for X position
    let mut ptr = cpu.borrow_wram().south().connected_map_view_pointer();

    ptr += (x_coord >> 1) as u16;

    // pointer to upper left corner of current tile block map section
    cpu.borrow_wram_mut()
        .set_current_tile_block_map_view_pointer(ptr);

    check_map_connections_load_new_map(cpu);
}

// load the connected map that was entered
fn check_map_connections_load_new_map(cpu: &mut Cpu) {
    cpu.borrow_wram_mut()
        .set_pikachu_overworld_state_flag_4(true);
    cpu.borrow_wram_mut().set_pikachu_spawn_state(2);

    load_map_header(cpu);
    cpu.call(0x2176); // PlayDefaultMusicFadeOutCurrent

    cpu.b = palette_constants::SET_PAL_OVERWORLD;
    cpu.call(0x3e05); // RunPaletteCommand

    // Since the sprite set shouldn't change, this will just update VRAM slots at
    // x#SPRITESTATEDATA2_IMAGEBASEOFFSET without loading any tile patterns.
    cpu.call(0x3dba); // InitMapSprites

    // call LoadTileBlockMap
    cpu.stack_push(0x0001);
    load_tile_block_map(cpu);

    // jp OverworldLoopLessDelay
    cpu.pc = 0x0245;
}

fn play_map_change_sound(cpu: &mut Cpu) {
    log::trace!("play_map_change_sound()");

    let tileset = cpu.borrow_wram().cur_map_tileset();

    if matches!(tileset, FACILITY | CEMETERY) {
        return play_map_change_sound_play_sound(cpu, SFX_GO_OUTSIDE);
    }

    // upper left tile of the 4x4 square the player's sprite is standing on
    let standing_on_tile = cpu.read_byte(macros::coords::coord!(8, 8));

    // door tile in tileset 0
    if standing_on_tile == 0x0b {
        play_map_change_sound_play_sound(cpu, SFX_GO_INSIDE)
    } else {
        play_map_change_sound_play_sound(cpu, SFX_GO_OUTSIDE)
    }
}

fn play_map_change_sound_play_sound(cpu: &mut Cpu, sfx: u8) {
    cpu.a = sfx;
    cpu.call(0x2238); // PlaySound

    if cpu.borrow_wram().map_pal_offset() == 0 {
        cpu.call(0x1eb6); // GBFadeOutToBlack
    }
}

/// Output: z flag is set if the player is in an outside map (a town or route)
pub fn check_if_in_outside_map(cpu: &mut Cpu) {
    let outside = matches!(cpu.borrow_wram().cur_map_tileset(), OVERWORLD | PLATEAU);

    log::debug!("check_if_in_outside_map() == {}", outside);

    cpu.set_flag(CpuFlag::Z, outside);
    cpu.pc = cpu.stack_pop(); // ret
}

/// This function is an extra check that sometimes has to pass in order to warp, beyond just standing on a warp. The
/// "sometimes" qualification is necessary because of CheckWarpsNoCollision's behavior. Depending on the map, either
/// "function 1" or "function 2" is used for the check.
///
/// "function 1" passes when the player is at the edge of the map and is facing towards the outside of the map \
/// "function 2" passes when the the tile in front of the player is among a certain set
///
/// Output: sets carry if the check passes, otherwise clears carry
pub fn extra_warp_check(cpu: &mut Cpu) {
    log::trace!("extra_warp_check()");

    let cur_map = cpu.borrow_wram().cur_map();
    let cur_map_tileset = cpu.borrow_wram().cur_map_tileset();

    match (cur_map, cur_map_tileset) {
        (SS_ANNE_3F, _) => extra_warp_check_use_function1(cpu),
        (ROCKET_HIDEOUT_B1F, _) => extra_warp_check_use_function2(cpu),
        (ROCKET_HIDEOUT_B2F, _) => extra_warp_check_use_function2(cpu),
        (ROCKET_HIDEOUT_B4F, _) => extra_warp_check_use_function2(cpu),
        (ROCK_TUNNEL_1F, _) => extra_warp_check_use_function2(cpu),

        (_, OVERWORLD) => extra_warp_check_use_function2(cpu), // Outside tileset
        (_, SHIP) => extra_warp_check_use_function2(cpu),      // S.S. Anne tileset
        (_, SHIP_PORT) => extra_warp_check_use_function2(cpu), // Vermilion Port tileset
        (_, PLATEAU) => extra_warp_check_use_function2(cpu),   // Indigo Plateau tileset

        _ => extra_warp_check_use_function1(cpu),
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn extra_warp_check_use_function1(cpu: &mut Cpu) {
    log::debug!("extra_warp_check() IsPlayerFacingEdgeOfMap");
    cpu.b = 0x03; // BANK(IsPlayerFacingEdgeOfMap)
    cpu.set_hl(0x4148); // IsPlayerFacingEdgeOfMap
    cpu.call(0x3e84); // Bankswitch
}

fn extra_warp_check_use_function2(cpu: &mut Cpu) {
    log::debug!("extra_warp_check() IsWarpTileInFrontOfPlayer");
    cpu.b = 0x03; // BANK(IsWarpTileInFrontOfPlayer)
    cpu.set_hl(0x4197); // IsWarpTileInFrontOfPlayer
    cpu.call(0x3e84); // Bankswitch
}

fn map_entry_after_battle(cpu: &mut Cpu) {
    log::trace!("map_entry_after_battle()");

    // for enabling warp testing after collisions
    macros::farcall::farcall(cpu, 0x03, 0x40a6); // IsPlayerStandingOnWarp

    if cpu.borrow_wram().map_pal_offset() == 0 {
        cpu.call(0x1ebd); // GBFadeInFromWhite
    } else {
        cpu.call(0x1e6f); // LoadGBPal
    }
}

/// For when all the player's pokemon faint.
///
/// Does not print the "blacked out" message.
pub fn handle_black_out(cpu: &mut Cpu) {
    log::debug!("handle_black_out()");

    cpu.call(0x1eb6); // GBFadeOutToBlack

    cpu.a = 0x08;
    cpu.stack_push(0x0001);
    stop_music(cpu);

    // Reset "blacked out" bit
    let value = cpu.read_byte(wram::W_D72E);
    cpu.write_byte(wram::W_D72E, value & !(1 << 5));

    cpu.a = 0x01; // BANK(PrepareForSpecialWarp) and BANK(SpecialEnterMap)
    cpu.call(0x3e7e); // BankswitchCommon

    // ResetStatusAndHalveMoneyOnBlackout
    macros::farcall::callfar(cpu, 0x3c, 0x4274);

    cpu.call(0x6042); // PrepareForSpecialWarp
    cpu.call(0x2176); // PlayDefaultMusicFadeOutCurrent
    cpu.call(0x5ce4); // SpecialEnterMap

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input: a = fade counter
pub fn stop_music(cpu: &mut Cpu) {
    let fade_counter = cpu.a;

    log::debug!("stop_music(fade_counter={})", fade_counter);

    cpu.borrow_wram_mut()
        .set_audio_fade_out_control(fade_counter);

    cpu.call(0x2233); // StopAllMusic

    while cpu.borrow_wram().audio_fade_out_control() != 0 {
        cpu.cycle(4);
    }

    cpu.call(0x1dd0); // StopAllSounds

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn handle_fly_warp_or_dungeon_warp(cpu: &mut Cpu) {
    log::debug!("handle_fly_warp_or_dungeon_warp()");

    cpu.call(0x231c); // UpdateSprites
    home::palettes::delay3(cpu);

    cpu.borrow_wram_mut().set_battle_result(BattleResult::Win);
    cpu.borrow_wram_mut().set_is_in_battle(0);
    cpu.borrow_wram_mut().set_map_pal_offset(0);

    cpu.borrow_wram_mut().set_fly_or_dungeon_warp(true);
    cpu.borrow_wram_mut().set_forced_to_ride_bike(false);

    macros::farcall::farcall(cpu, 0x1c, 0x4615); // _LeaveMapAnim

    stop_bike_surf(cpu);

    cpu.a = 0x01; // BANK(PrepareForSpecialWarp) and BANK(SpecialEnterMap)
    cpu.call(0x3e7e); // BankswitchCommon

    cpu.call(0x6042); // PrepareForSpecialWarp
    cpu.call(0x5ce4); // SpecialEnterMap

    cpu.pc = cpu.stack_pop(); // ret
}

fn stop_bike_surf(cpu: &mut Cpu) {
    log::trace!("stop_bike_surf()");

    let is_walking = cpu.borrow_wram().walk_bike_surf_state() == 0;

    if !is_walking {
        cpu.borrow_wram_mut().set_walk_bike_surf_state(0);

        if cpu.borrow_wram().jumped_into_hole() {
            cpu.call(0x216b); // PlayDefaultMusic
        }
    }
}

/// Loads sprite graphics based on whether the player is standing, biking, or surfing.
pub fn load_player_sprite_graphics(cpu: &mut Cpu) {
    log::debug!("load_player_sprite_graphics()");

    match cpu.borrow_wram().walk_bike_surf_state() {
        0 => load_walking_player_sprite_graphics(cpu),
        1 => {
            // If the bike can't be used, start walking instead.
            cpu.stack_push(0x0001);
            is_bike_riding_allowed(cpu);

            if cpu.flag(CpuFlag::C) {
                load_bike_player_sprite_graphics(cpu);
                cpu.pc = cpu.stack_pop(); // ret
            } else {
                load_player_sprite_graphics_start_walking(cpu)
            }
        }
        2 => {
            if cpu.read_byte(hram::H_TILE_ANIMATIONS) == 0 {
                load_player_sprite_graphics_start_walking(cpu)
            } else {
                load_surfing_player_sprite_graphics(cpu);
                cpu.pc = cpu.stack_pop(); // ret
            }
        }
        i => {
            log::error!("invalid walk_bike_surf_state: {}", i);
            load_walking_player_sprite_graphics(cpu)
        }
    }
}

fn load_player_sprite_graphics_start_walking(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_walk_bike_surf_state(0);
    cpu.borrow_wram_mut().set_walk_bike_surf_state_copy(0);

    load_walking_player_sprite_graphics(cpu)
}

/// Output: sets carry if biking is allowed
pub fn is_bike_riding_allowed(cpu: &mut Cpu) {
    log::trace!("is_bike_riding_allowed()");

    // The bike can be used on Route 23 and Indigo Plateau,
    // or maps with tilesets in BikeRidingTilesets.

    if cpu.borrow_wram().cur_map() == ROUTE_23 {
        return is_bike_riding_allowed_allowed(cpu);
    }

    if cpu.borrow_wram().cur_map() == INDIGO_PLATEAU {
        return is_bike_riding_allowed_allowed(cpu);
    }

    let tileset = cpu.borrow_wram().cur_map_tileset();

    if BIKE_RIDING_TILESETS.contains(&tileset) {
        return is_bike_riding_allowed_allowed(cpu);
    }

    log::debug!("is_bike_riding_allowed() == false");
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc = cpu.stack_pop(); // ret
}

fn is_bike_riding_allowed_allowed(cpu: &mut Cpu) {
    log::debug!("is_bike_riding_allowed() == true");
    cpu.set_flag(CpuFlag::C, true);
    cpu.pc = cpu.stack_pop(); // ret
}

/// Loads the tile pattern data of the current tileset into VRAM.
pub fn load_tileset_tile_pattern_data(cpu: &mut Cpu) {
    log::debug!("load_tileset_tile_pattern_data()");

    let tileset_gfx_pointer = cpu.borrow_wram().tileset_gfx_pointer();

    cpu.set_hl(tileset_gfx_pointer);
    cpu.set_de(vram::V_TILESET);
    cpu.set_bc(0x600);
    cpu.a = cpu.borrow_wram().tileset_bank();
    cpu.call(0x009d); // FarCopyData

    cpu.pc = cpu.stack_pop(); // ret
}

/// Loads the current maps complete tile map (which references blocks, not individual tiles) to C6E8.
///
/// It can also load partial tile maps of connected maps into a border of length 3 around the current map.
pub fn load_tile_block_map(cpu: &mut Cpu) {
    log::debug!("load_tile_block_map()");

    let bg = cpu.borrow_wram().map_background_tile();

    // fill C6E8-CBFB with the background tile
    for ptr in wram::W_OVERWORLD_MAP..wram::W_OVERWORLD_MAP_END {
        cpu.write_byte(ptr, bg);
    }

    let map_height = cpu.borrow_wram().cur_map_height();
    let map_width = cpu.borrow_wram().cur_map_width() as u16;
    let map_stride = map_width + (MAP_BORDER * 2);

    // load tile map of current map (made of tile block IDs)
    // a 3-byte border at the edges of the map is kept so that there is space for map connections
    // make space for north border (next 3 lines), and then the west border (next 3 bytes)
    let mut dst = wram::W_OVERWORLD_MAP + (map_stride * MAP_BORDER) + MAP_BORDER;
    let mut src = cpu.borrow_wram().cur_map_data_ptr();

    for _ in 0..map_height {
        for i in 0..map_width {
            let byte = cpu.read_byte(src + i);
            cpu.write_byte(dst + i, byte);
        }

        dst += map_stride;
        src += map_width;
    }

    if cpu.borrow_wram().north().connected_map() != 0xff {
        cpu.a = cpu.borrow_wram().north().connected_map();
        cpu.stack_push(0x0001);
        switch_to_map_rom_bank(cpu);

        let connection_strip_src = cpu.borrow_wram().north().connection_strip_src();
        let connection_strip_dest = cpu.borrow_wram().north().connection_strip_dest();
        let connection_strip_width = cpu.borrow_wram().north().connection_strip_length();
        let connected_map_width = cpu.borrow_wram().north().connected_map_width();

        cpu.set_hl(connection_strip_src);
        cpu.set_de(connection_strip_dest);
        load_north_south_connections_tile_map(cpu, connection_strip_width, connected_map_width);
    }

    if cpu.borrow_wram().south().connected_map() != 0xff {
        cpu.a = cpu.borrow_wram().south().connected_map();
        cpu.stack_push(0x0001);
        switch_to_map_rom_bank(cpu);

        let connection_strip_src = cpu.borrow_wram().south().connection_strip_src();
        let connection_strip_dest = cpu.borrow_wram().south().connection_strip_dest();
        let connection_strip_width = cpu.borrow_wram().south().connection_strip_length();
        let connected_map_width = cpu.borrow_wram().south().connected_map_width();

        cpu.set_hl(connection_strip_src);
        cpu.set_de(connection_strip_dest);
        load_north_south_connections_tile_map(cpu, connection_strip_width, connected_map_width);
    }

    if cpu.borrow_wram().west().connected_map() != 0xff {
        cpu.a = cpu.borrow_wram().west().connected_map();
        cpu.stack_push(0x0001);
        switch_to_map_rom_bank(cpu);

        let connection_strip_src = cpu.borrow_wram().west().connection_strip_src();
        let connection_strip_dest = cpu.borrow_wram().west().connection_strip_dest();
        let connection_strip_length = cpu.borrow_wram().west().connection_strip_length();
        let connected_map_width = cpu.borrow_wram().west().connected_map_width();

        cpu.set_hl(connection_strip_src);
        cpu.set_de(connection_strip_dest);
        load_east_west_connections_tile_map(cpu, connection_strip_length, connected_map_width);
    }

    if cpu.borrow_wram().east().connected_map() != 0xff {
        cpu.a = cpu.borrow_wram().east().connected_map();
        cpu.stack_push(0x0001);
        switch_to_map_rom_bank(cpu);

        let connection_strip_src = cpu.borrow_wram().east().connection_strip_src();
        let connection_strip_dest = cpu.borrow_wram().east().connection_strip_dest();
        let connection_strip_length = cpu.borrow_wram().east().connection_strip_length();
        let connected_map_width = cpu.borrow_wram().east().connected_map_width();

        cpu.set_hl(connection_strip_src);
        cpu.set_de(connection_strip_dest);
        load_east_west_connections_tile_map(cpu, connection_strip_length, connected_map_width);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input: hl = src, de = dest
fn load_north_south_connections_tile_map(cpu: &mut Cpu, strip_length: u8, connected_width: u8) {
    log::trace!("load_north_south_connections_tile_map()");

    let current_width = cpu.borrow_wram().cur_map_width() as u16;

    for _ in 0..MAP_BORDER {
        for i in 0..strip_length {
            let byte = cpu.read_byte(cpu.hl() + (i as u16));
            cpu.write_byte(cpu.de() + (i as u16), byte);
        }

        cpu.set_hl(cpu.hl() + (connected_width as u16));
        cpu.set_de(cpu.de() + current_width + (MAP_BORDER * 2));
    }
}

/// Input: hl = src, de = dest
fn load_east_west_connections_tile_map(cpu: &mut Cpu, strip_length: u8, connected_width: u8) {
    log::trace!("load_east_west_connections_tile_map()");

    let current_width = cpu.borrow_wram().cur_map_width() as u16;

    for _ in 0..strip_length {
        for i in 0..MAP_BORDER {
            let byte = cpu.read_byte(cpu.hl() + i);
            cpu.write_byte(cpu.de() + i, byte);
        }

        cpu.set_hl(cpu.hl() + (connected_width as u16));
        cpu.set_de(cpu.de() + current_width + (MAP_BORDER * 2));
    }
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
/// Output: hl = ???
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

/// Check if the player will jump down a ledge and check if the tile ahead is passable (when not surfing)
///
/// Sets the carry flag if there is a collision, and unsets it if there isn't a collision
pub fn collision_check_on_land(cpu: &mut Cpu) {
    log::trace!("collision_check_on_land()");

    // no collisions when the player jumping
    if (cpu.read_byte(wram::W_D736) & (1 << 6)) != 0 {
        return collision_check_on_land_no_collision(cpu);
    }

    // no collisions when the player's movements are being controlled by the game
    if cpu.borrow_wram().simulated_joypad_states_index() != 0 {
        return collision_check_on_land_no_collision(cpu);
    }

    // the direction that the player is trying to go in
    cpu.d = cpu.borrow_wram().player_direction();
    cpu.a = cpu.read_byte(wram::W_SPRITE_PLAYER_STATE_DATA1_COLLISION_DATA);

    // check if a sprite is in the direction the player is trying to go
    if (cpu.a & cpu.d) != 0 {
        return collision_check_on_land_collision(cpu);
    }

    cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, 0);

    // check for sprite collisions again? when does the above check fail to detect a sprite collision?
    cpu.stack_push(0x0001);
    is_sprite_in_front_of_player(cpu);

    if !cpu.flag(CpuFlag::C) {
        return collision_check_on_land_asm_0a5c(cpu);
    }

    // res 7, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << 7));
    }

    cpu.a = cpu.read_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID);

    // was there a sprite collision?
    if cpu.a == 0 {
        return collision_check_on_land_asm_0a5c(cpu);
    }

    // if no sprite collision
    if cpu.a != 0xf {
        return collision_check_on_land_collision(cpu);
    }

    cpu.call(0x154a); // CheckPikachuFollowingPlayer

    if !cpu.flag(CpuFlag::Z) {
        return collision_check_on_land_collision(cpu);
    }

    if (cpu.read_byte(hram::H_JOY_HELD) & 0x2) != 0 {
        return collision_check_on_land_asm_0a5c(cpu);
    }

    cpu.set_hl(wram::W_D435);
    let w_d435 = cpu.read_byte(cpu.hl());

    if w_d435 == 0 {
        return collision_check_on_land_asm_0a5c(cpu);
    }

    cpu.write_byte(wram::W_D435, w_d435 - 1);

    if w_d435 > 1 {
        return collision_check_on_land_collision(cpu);
    }

    collision_check_on_land_asm_0a5c(cpu);
}

fn collision_check_on_land_asm_0a5c(cpu: &mut Cpu) {
    if check_for_jumping_and_tile_pair_collisions(cpu, TILE_PAIR_COLLISIONS_LAND) {
        return collision_check_on_land_collision(cpu);
    }

    cpu.stack_push(0x0001);
    check_tile_passable(cpu);

    if cpu.flag(CpuFlag::C) {
        return collision_check_on_land_collision(cpu);
    }

    collision_check_on_land_no_collision(cpu)
}

fn collision_check_on_land_collision(cpu: &mut Cpu) {
    cpu.a = cpu.read_byte(wram::W_CHANNEL_SOUND_IDS + (CHAN5 as u16));

    // play collision sound (if it's not already playing)
    if cpu.a != SFX_COLLISION {
        cpu.a = SFX_COLLISION;
        cpu.call(0x2238); // PlaySound
    }

    cpu.set_flag(CpuFlag::C, true);
    log::debug!("collision_check_on_land() collision");

    cpu.pc = cpu.stack_pop(); // ret
}

fn collision_check_on_land_no_collision(cpu: &mut Cpu) {
    cpu.set_flag(CpuFlag::C, false);
    log::trace!("collision_check_on_land() no collision");

    cpu.pc = cpu.stack_pop(); // ret
}

/// Check if the tile in front of the player is passable
///
/// Clears carry if it is, sets carry if not
pub fn check_tile_passable(cpu: &mut Cpu) {
    log::trace!("check_tile_passable()");

    // get tile in front of player
    macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

    cpu.c = cpu.borrow_wram().tile_in_front_of_player();
    cpu.call(0x15c3); // IsTilePassable

    cpu.pc = cpu.stack_pop(); // ret
}

/// Check if the player is going to jump down a small ledge and check
/// for collisions that only occur between certain pairs of tiles.
pub fn check_for_jumping_and_tile_pair_collisions(cpu: &mut Cpu, table: u16) -> bool {
    log::trace!("check_for_jumping_and_tile_pair_collisions()");

    // get the tile in front of the player
    macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

    let saved_de = cpu.de();
    let saved_bc = cpu.bc();

    // check if the player is trying to jump a ledge
    macros::farcall::farcall(cpu, 0x06, 0x67f4); // HandleLedges

    cpu.set_bc(saved_bc);
    cpu.set_de(saved_de);
    cpu.set_hl(table);

    // is the player jumping?
    if (cpu.read_byte(wram::W_D736) & (1 << 6)) != 0 {
        log::trace!("check_for_jumping_and_tile_pair_collisions() == false");
        return false;
    }

    // if not jumping
    cpu.stack_push(0x0001);
    check_for_tile_pair_collisions2(cpu);
    cpu.flag(CpuFlag::C)
}

pub fn check_for_tile_pair_collisions2(cpu: &mut Cpu) {
    log::trace!("check_for_tile_pair_collisions2()");

    let tile = cpu.read_byte(macros::coords::coord!(8, 9));
    cpu.borrow_wram_mut().set_tile_player_standing_on(tile);

    check_for_tile_pair_collisions(cpu);
}

/// Input: hl = pointer to TilePairCollisions* table \
/// Output: carry flag set if there is a collision, unset if there isn't
pub fn check_for_tile_pair_collisions(cpu: &mut Cpu) {
    log::trace!("check_for_tile_pair_collisions()");

    let cur_tileset = cpu.borrow_wram().cur_map_tileset();
    let cur_tile = cpu.borrow_wram().tile_player_standing_on();
    let next_tile = cpu.borrow_wram().tile_in_front_of_player();

    for i in 0.. {
        let pair_tileset = cpu.read_byte(cpu.hl() + (i * 3));

        if pair_tileset == 0xff {
            log::trace!("check_for_tile_pair_collisions() == false");
            cpu.set_flag(CpuFlag::C, false);
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }

        if pair_tileset != cur_tileset {
            continue;
        }

        let pair_lhs = cpu.read_byte(cpu.hl() + (i * 3) + 1);

        if pair_lhs != cur_tile && pair_lhs != next_tile {
            continue;
        }

        let pair_rhs = cpu.read_byte(cpu.hl() + (i * 3) + 2);

        if pair_rhs != cur_tile && pair_rhs != next_tile {
            continue;
        }

        log::debug!("check_for_tile_pair_collisions() == true");
        cpu.set_flag(CpuFlag::C, true);
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }
}

/// Build a tile map from the tile block map based on the current X/Y coordinates of the player's character
pub fn load_current_map_view(cpu: &mut Cpu) {
    log::trace!("load_current_map_view()");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    // switch to ROM bank that contains tile data
    cpu.a = cpu.borrow_wram().tileset_bank();
    cpu.call(0x3e7e); // BankswitchCommon

    // address of upper left corner of current map view
    let mut src = cpu.borrow_wram().current_tile_block_map_view_pointer();
    let map_width = cpu.borrow_wram().cur_map_width() as u16;

    // tile map pointer
    let mut dst = wram::W_TILE_MAP_BACKUP;

    // each loop iteration fills in one row of tile blocks
    for _ in 0..5 {
        // loop to draw each tile block of the current row
        for i in 0..6 {
            let tile_block = cpu.read_byte(src + i);

            draw_tile_block(cpu, tile_block, dst + (i * 4));
        }

        src += map_width + (MAP_BORDER * 2);
        dst += 0x60;
    }

    let mut src = wram::W_TILE_MAP_BACKUP;

    if cpu.borrow_wram().y_block_coord() > 0 {
        src += 0x30;
    }

    if cpu.borrow_wram().x_block_coord() > 0 {
        src += 0x02;
    }

    // base address for the tiles that are directly transferred to VRAM during V-blank
    let mut dst = macros::coords::coord!(0, 0);

    for _ in 0..gfx_constants::SCREEN_HEIGHT {
        for i in 0..gfx_constants::SCREEN_WIDTH {
            let byte = cpu.read_byte(src + (i as u16));
            cpu.write_byte(dst + (i as u16), byte);
        }

        src += gfx_constants::SCREEN_WIDTH as u16 + 4;
        dst += gfx_constants::SCREEN_WIDTH as u16;
    }

    // restore previous ROM bank
    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon
    cpu.pc = cpu.stack_pop(); // ret
}

pub fn advance_player_sprite(cpu: &mut Cpu) {
    log::trace!("advance_player_sprite()");

    let enabled = cpu.borrow_wram().update_sprites_enabled();

    cpu.borrow_wram_mut().set_update_sprites_enabled(0xff);

    // _AdvancePlayerSprite
    macros::farcall::callfar(cpu, 0x3c, 0x410c);

    cpu.borrow_wram_mut().set_update_sprites_enabled(enabled);

    cpu.pc = cpu.stack_pop(); // ret
}

// The following 6 functions are used to tell the V-blank handler to redraw the
// portion of the map that was newly exposed due to the player's movement.

pub fn schedule_north_row_redraw(cpu: &mut Cpu) {
    cpu.set_hl(macros::coords::coord!(0, 0));
    cpu.stack_push(0x0001);
    copy_to_redraw_row_or_column_src_tiles(cpu);

    let vram_ptr = cpu.borrow_wram().map_view_vram_pointer();

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_dest(vram_ptr);

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_mode(gfx_constants::REDRAW_ROW);

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input: hl = source pointer
pub fn copy_to_redraw_row_or_column_src_tiles(cpu: &mut Cpu) {
    const BYTES: u16 = (gfx_constants::SCREEN_WIDTH as u16) * 2;

    log::trace!("copy_to_redraw_row_or_column_src_tiles()");

    for i in 0..BYTES {
        let byte = cpu.read_byte(cpu.hl() + i);
        cpu.write_byte(wram::W_REDRAW_ROW_OR_COLUMN_SRC_TILES + i, byte);
    }

    cpu.set_hl(cpu.hl() + BYTES);
    cpu.set_de(wram::W_REDRAW_ROW_OR_COLUMN_SRC_TILES + BYTES);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn schedule_south_row_redraw(cpu: &mut Cpu) {
    log::trace!("schedule_south_row_redraw()");

    cpu.set_hl(macros::coords::coord!(0, 16));
    cpu.stack_push(0x0001);
    copy_to_redraw_row_or_column_src_tiles(cpu);

    let vram_ptr = cpu.borrow_wram().map_view_vram_pointer();
    let vram_ptr = ((vram_ptr + 0x200) & 0x03ff) | 0x9800;

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_dest(vram_ptr);

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_mode(gfx_constants::REDRAW_ROW);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn schedule_east_column_redraw(cpu: &mut Cpu) {
    log::trace!("schedule_east_column_redraw()");

    cpu.set_hl(macros::coords::coord!(18, 0));
    schedule_column_redraw_helper(cpu);

    let vram_ptr = cpu.borrow_wram().map_view_vram_pointer();

    let hi_bits = vram_ptr & 0xffe0;
    let low_bits = vram_ptr.wrapping_add(18) & 0x001f;

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_dest(hi_bits | low_bits);

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_mode(gfx_constants::REDRAW_COL);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn schedule_column_redraw_helper(cpu: &mut Cpu) {
    log::trace!("schedule_column_redraw_helper()");

    cpu.set_de(wram::W_REDRAW_ROW_OR_COLUMN_SRC_TILES);

    for _ in 0..gfx_constants::SCREEN_HEIGHT {
        cpu.a = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.de(), cpu.a);

        cpu.a = cpu.read_byte(cpu.hl() + 1);
        cpu.write_byte(cpu.de() + 1, cpu.a);

        cpu.set_hl(cpu.hl() + gfx_constants::SCREEN_WIDTH as u16);
        cpu.set_de(cpu.de() + 2);
    }
}

pub fn schedule_west_column_redraw(cpu: &mut Cpu) {
    log::trace!("schedule_west_column_redraw()");

    cpu.set_hl(macros::coords::coord!(0, 0));
    schedule_column_redraw_helper(cpu);

    let vram_ptr = cpu.borrow_wram().map_view_vram_pointer();

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_dest(vram_ptr);

    cpu.borrow_wram_mut()
        .set_redraw_row_or_column_mode(gfx_constants::REDRAW_COL);

    cpu.pc = cpu.stack_pop(); // ret
}

/// Write the tiles that make up a tile block to memory
fn draw_tile_block(cpu: &mut Cpu, tile_block: u8, mut dest: u16) {
    log::trace!("draw_tile_block({}, {:04x})", tile_block, dest);

    // pointer to tiles
    let pointer = cpu.borrow_wram().tileset_blocks_pointer();
    let offset = (tile_block as u16) * 0x10;

    let mut src = pointer + offset;

    for _ in 0..4 {
        for i in 0..4 {
            let tile = cpu.read_byte(src + i);
            cpu.write_byte(dest + i, tile);
        }

        src += 4;
        dest += 0x18;
    }
}

/// Update joypad state and simulate button presses
pub fn joypad_overworld(cpu: &mut Cpu) {
    log::trace!("joypad_overworld()");

    cpu.write_byte(wram::W_SPRITE_PLAYER_STATE_DATA1_Y_STEP_VECTOR, 0);
    cpu.write_byte(wram::W_SPRITE_PLAYER_STATE_DATA1_X_STEP_VECTOR, 0);

    cpu.stack_push(0x0001);
    run_map_script(cpu);

    cpu.call(0x01b9); // Joypad
    force_bike_down(cpu);
    are_inputs_simulated(cpu);

    cpu.pc = cpu.stack_pop(); // ret
}

fn force_bike_down(cpu: &mut Cpu) {
    log::trace!("force_bike_down()");

    // check if a trainer wants a challenge
    let flags_d733 = cpu.read_byte(wram::W_FLAGS_D733);

    if (flags_d733 & (1 << 3)) != 0 {
        return;
    }

    if cpu.borrow_wram().cur_map() != ROUTE_17 {
        return;
    }

    let joy_held = cpu.read_byte(hram::H_JOY_HELD);

    // on the cycling road, if there isn't a trainer and the player isn't pressing buttons, simulate a down press
    if (joy_held & (D_DOWN | D_UP | D_LEFT | D_RIGHT | B_BUTTON | A_BUTTON)) == 0 {
        cpu.write_byte(hram::H_JOY_HELD, D_DOWN);
    }
}

fn are_inputs_simulated(cpu: &mut Cpu) {
    log::trace!("are_inputs_simulated()");

    let w_d730 = cpu.read_byte(wram::W_D730);

    if (w_d730 & (1 << 7)) == 0 {
        return;
    }

    log::debug!("are_inputs_simulated() joypad is being simulated");

    match get_simulated_input(cpu) {
        None => {
            // if done simulating button presses
            cpu.borrow_wram_mut().set_simulated_joypad_states_index(0);
            cpu.write_byte(wram::W_SIMULATED_JOYPAD_STATES_END, 0);
            cpu.write_byte(wram::W_JOY_IGNORE, 0);
            cpu.write_byte(hram::H_JOY_HELD, 0);

            let w_d736 = cpu.read_byte(wram::W_D736) & 0xf8;
            cpu.write_byte(wram::W_D736, w_d736);

            let w_d730 = cpu.read_byte(wram::W_D730);
            cpu.write_byte(wram::W_D730, w_d730 & !(1 << 7));
        }
        Some(input) => {
            // store simulated button press in joypad state
            cpu.write_byte(hram::H_JOY_HELD, input);

            if input == 0 {
                cpu.write_byte(hram::H_JOY_PRESSED, input);
                cpu.write_byte(hram::H_JOY_RELEASED, input);
            }
        }
    }
}

fn get_simulated_input(cpu: &mut Cpu) -> Option<u8> {
    log::trace!("get_simulated_input()");

    let mut idx = cpu.borrow_wram().simulated_joypad_states_index();

    // if the end of the simulated button presses has been reached
    if idx == 0 {
        return None;
    }

    idx -= 1;

    cpu.borrow_wram_mut().set_simulated_joypad_states_index(idx);
    let input = cpu.read_byte(wram::W_SIMULATED_JOYPAD_STATES_END + (idx as u16));

    Some(input)
}

/// Check the tile ahead to determine if the character should get on land or keep surfing.
///
/// Sets carry if there is a collision and clears carry otherwise.
///
/// This function had a bug in Red/Blue, but it was fixed in Yellow.
pub fn collision_check_on_water(cpu: &mut Cpu) {
    log::trace!("collision_check_on_water()");

    // return and clear carry if button presses are being simulated
    if (cpu.read_byte(wram::W_D730) & (1 << 7)) != 0 {
        return collision_check_on_water_no_collision(cpu);
    }

    // the direction that the player is trying to go in
    cpu.d = cpu.borrow_wram().player_direction();
    cpu.a = cpu.read_byte(wram::W_SPRITE_PLAYER_STATE_DATA1_COLLISION_DATA);

    // check if a sprite is in the direction the player is trying to go
    if (cpu.a & cpu.d) != 0 {
        return collision_check_on_water_collision(cpu);
    }

    if check_for_jumping_and_tile_pair_collisions(cpu, TILE_PAIR_COLLISIONS_WATER) {
        return collision_check_on_water_collision(cpu);
    }

    // get tile in front of player (puts it in c and [wTileInFrontOfPlayer])
    macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

    // IsNextTileShoreOrWater
    macros::farcall::callfar(cpu, 0x03, 0x6808);

    if cpu.flag(CpuFlag::C) {
        return collision_check_on_water_no_collision(cpu);
    }

    // tile in front of player
    cpu.c = cpu.borrow_wram().tile_in_front_of_player();
    cpu.call(0x15c3); // IsTilePassable

    if cpu.flag(CpuFlag::C) {
        collision_check_on_water_collision(cpu);
    } else {
        collision_check_on_water_stop_surfing(cpu)
    }
}

fn collision_check_on_water_collision(cpu: &mut Cpu) {
    cpu.a = cpu.read_byte(wram::W_CHANNEL_SOUND_IDS + (CHAN5 as u16));

    // check if collision sound is already playing, and play it if it's not
    if cpu.a != SFX_COLLISION {
        cpu.a = SFX_COLLISION;
        cpu.call(0x2238); // PlaySound
    }

    cpu.set_flag(CpuFlag::C, true);
    log::debug!("collision_check_on_water() collision");

    cpu.pc = cpu.stack_pop(); // ret
}

fn collision_check_on_water_stop_surfing(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_pikachu_spawn_state(0x3);
    cpu.borrow_wram_mut()
        .set_pikachu_overworld_state_flag_5(true);
    cpu.borrow_wram_mut().set_walk_bike_surf_state(0);

    cpu.call(0x07d7); // LoadPlayerSpriteGraphics
    cpu.call(0x216b); // PlayDefaultMusic

    collision_check_on_water_no_collision(cpu)
}

fn collision_check_on_water_no_collision(cpu: &mut Cpu) {
    cpu.set_flag(CpuFlag::C, false);
    log::trace!("collision_check_on_water() no collision");

    cpu.pc = cpu.stack_pop(); // ret
}

/// Run the current map's script
pub fn run_map_script(cpu: &mut Cpu) {
    log::trace!("run_map_script()");

    let saved_hl = cpu.hl();
    let saved_de = cpu.de();
    let saved_bc = cpu.bc();

    // TryPushingBoulder
    macros::farcall::farcall(cpu, 0x03, 0x70a1);

    if cpu.borrow_wram().boulder_dust_animation_pending() {
        // DoBoulderDustAnimation
        macros::farcall::farcall(cpu, 0x03, 0x7131);
    }

    cpu.set_bc(saved_bc);
    cpu.set_de(saved_de);
    cpu.set_hl(saved_hl);

    cpu.call(0x30ae); // RunNPCMovementScript

    // change to the ROM bank the map's data is in
    cpu.a = cpu.borrow_wram().cur_map();
    cpu.stack_push(0x0001);
    switch_to_map_rom_bank(cpu);

    let ptr = u16::from_le_bytes([
        cpu.read_byte(wram::W_CUR_MAP_SCRIPT_PTR),
        cpu.read_byte(wram::W_CUR_MAP_SCRIPT_PTR + 1),
    ]);

    // Jump to the map's script
    cpu.call(ptr);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn load_walking_player_sprite_graphics(cpu: &mut Cpu) {
    // new sprite copy stuff
    cpu.write_byte(wram::W_D473, 0);

    load_player_sprite_graphics_common(cpu, 0x05, 0x4571); // RedSprite
    cpu.pc = cpu.stack_pop(); // ret
}

fn load_surfing_player_sprite_graphics(cpu: &mut Cpu) {
    log::trace!("load_surfing_player_sprite_graphics()");

    let w_d473 = cpu.read_byte(wram::W_D473);
    let w_d472_bit_6 = (cpu.read_byte(wram::W_D472) & (1 << 6)) != 0;

    match (w_d473, w_d472_bit_6) {
        (1, _) => load_player_sprite_graphics_common(cpu, 0x05, 0x7ab1), // SeelSprite
        (2, _) => load_player_sprite_graphics_common(cpu, 0x3f, 0x6def), // SurfingPikachuSprite
        (_, false) => load_player_sprite_graphics_common(cpu, 0x05, 0x7ab1), // SeelSprite
        (_, true) => load_player_sprite_graphics_common(cpu, 0x3f, 0x6def), // SurfingPikachuSprite
    }
}

fn load_bike_player_sprite_graphics(cpu: &mut Cpu) {
    log::trace!("load_bike_player_sprite_graphics()");

    load_player_sprite_graphics_common(cpu, 0x05, 0x43f1); // RedBikeSprite
}

fn load_player_sprite_graphics_common(cpu: &mut Cpu, bank: u8, addr: u16) {
    log::trace!(
        "load_player_sprite_graphics_common({:02x}:{:04x})",
        bank,
        addr
    );

    cpu.b = bank;
    cpu.c = 0xc;
    cpu.set_de(addr);
    cpu.set_hl(vram::V_NPC_SPRITES);
    cpu.call(0x15fe); // CopyVideoData

    cpu.b = bank;
    cpu.c = 0xc;
    cpu.set_de(addr + 0xc0);
    cpu.set_hl(vram::V_NPC_SPRITES2);
    cpu.call(0x15fe); // CopyVideoData
}

// Load data from the map header
fn load_map_header(cpu: &mut Cpu) {
    log::trace!("load_map_header()");

    // MarkTownVisitedAndLoadMissableObjects
    macros::farcall::farcall(cpu, 0x03, 0x6f93);

    let cur_map_tileset = cpu.read_byte(wram::W_CUR_MAP_TILESET);
    cpu.write_byte(wram::W_UNUSED_D119, cur_map_tileset);

    cpu.a = cpu.borrow_wram().cur_map();
    cpu.stack_push(0x0001);
    switch_to_map_rom_bank(cpu);

    let cur_map_tileset = cpu.read_byte(wram::W_CUR_MAP_TILESET);

    cpu.b = cur_map_tileset;
    cpu.a = cur_map_tileset & !(1 << 7);

    let previous_tileset = cpu.a;
    cpu.write_byte(wram::W_CUR_MAP_TILESET, previous_tileset);
    cpu.borrow_wram_mut().set_previous_tileset(previous_tileset);

    if (cpu.b & (1 << 7)) != 0 {
        return;
    }

    let cur_map = cpu.borrow_wram().cur_map();
    let map_header_pointer = get_map_header_pointer(cur_map);
    cpu.set_hl(map_header_pointer);

    // ld de, wCurMapHeader
    cpu.set_de(wram::W_CUR_MAP_HEADER);
    const CUR_MAP_HEADER_SIZE: u16 = wram::W_CUR_MAP_HEADER_END - wram::W_CUR_MAP_HEADER;

    for i in 0..CUR_MAP_HEADER_SIZE {
        let byte = cpu.read_byte(cpu.hl() + i);
        cpu.write_byte(cpu.de() + i, byte);
    }

    cpu.set_hl(cpu.hl() + CUR_MAP_HEADER_SIZE);
    cpu.set_de(cpu.de() + CUR_MAP_HEADER_SIZE);

    // initialize all the connected maps to disabled at first, before loading the actual values
    cpu.write_byte(wram::W_NORTH_CONNECTED_MAP, 0xff);
    cpu.write_byte(wram::W_SOUTH_CONNECTED_MAP, 0xff);
    cpu.write_byte(wram::W_WEST_CONNECTED_MAP, 0xff);
    cpu.write_byte(wram::W_EAST_CONNECTED_MAP, 0xff);

    // copy connection data (if any) to WRAM
    cpu.b = cpu.read_byte(wram::W_CUR_MAP_CONNECTIONS);

    if (cpu.b & (1 << NORTH_F)) != 0 {
        cpu.set_de(wram::W_NORTH_CONNECTION_HEADER);
        cpu.stack_push(0x0001);
        copy_map_connection_header(cpu);
    }

    if (cpu.b & (1 << SOUTH_F)) != 0 {
        cpu.set_de(wram::W_SOUTH_CONNECTION_HEADER);
        cpu.stack_push(0x0001);
        copy_map_connection_header(cpu);
    }

    if (cpu.b & (1 << WEST_F)) != 0 {
        cpu.set_de(wram::W_WEST_CONNECTION_HEADER);
        cpu.stack_push(0x0001);
        copy_map_connection_header(cpu);
    }

    if (cpu.b & (1 << EAST_F)) != 0 {
        cpu.set_de(wram::W_EAST_CONNECTION_HEADER);
        cpu.stack_push(0x0001);
        copy_map_connection_header(cpu);
    }

    // Save object data pointer
    let object_data_pointer =
        u16::from_le_bytes([cpu.read_byte(cpu.hl()), cpu.read_byte(cpu.hl() + 1)]);

    cpu.set_hl(object_data_pointer);

    log::debug!("object_data_pointer = {:#04x}", object_data_pointer);

    let map_background_tile = cpu.read_byte(object_data_pointer);
    cpu.set_hl(object_data_pointer + 1);
    cpu.borrow_wram_mut()
        .set_map_background_tile(map_background_tile);

    let number_of_warps = cpu.read_byte(object_data_pointer + 1);
    cpu.set_hl(object_data_pointer + 1 + 1);
    cpu.borrow_wram_mut().set_number_of_warps(number_of_warps);

    cpu.set_de(wram::W_WARP_ENTRIES);

    // one warp per loop iteration
    for _ in 0..number_of_warps {
        for i in 0..4 {
            let byte = cpu.read_byte(cpu.hl() + i);
            cpu.write_byte(cpu.de() + i, byte);
        }

        cpu.set_hl(cpu.hl() + 4);
        cpu.set_de(cpu.de() + 4);
    }

    // number of signs
    let num_signs = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.borrow_wram_mut().set_num_signs(num_signs);

    if num_signs > 0 {
        copy_sign_data(cpu, num_signs);
    }

    // did a battle happen immediately before this?
    let battle_happened = (cpu.read_byte(wram::W_D72E) & (1 << 5)) != 0;

    // if so, skip this because battles don't destroy this data
    if !battle_happened {
        init_sprites(cpu);
    }

    macros::predef::predef_call!(cpu, LoadTilesetHeader);

    if !battle_happened {
        // SchedulePikachuSpawnForAfterText
        macros::farcall::callfar(cpu, 0x3f, 0x44fa);
    }

    // LoadWildData
    macros::farcall::callfar(cpu, 0x03, 0x4b62);

    // map height in 4x4 tile blocks
    let cur_map_height = cpu.borrow_wram().cur_map_height();

    // store map height in 2x2 tile blocks
    cpu.borrow_wram_mut()
        .set_current_map_height_2(cur_map_height * 2);

    // map width in 4x4 tile blocks
    let cur_map_width = cpu.borrow_wram().cur_map_width();

    // map width in 2x2 tile blocks
    cpu.borrow_wram_mut()
        .set_current_map_width_2(cur_map_width * 2);

    let cur_map = cpu.borrow_wram().cur_map();
    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.a = 0x3f; // BANK(MapSongBanks)
    cpu.call(0x3e7e); // BankswitchCommon

    const MAP_SONG_BANKS: u16 = 0x4000;
    cpu.set_hl(MAP_SONG_BANKS + (cur_map as u16) * 2);

    // Music 1
    let map_music_sound_id = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.borrow_wram_mut()
        .set_map_music_sound_id(map_music_sound_id);

    // Music 2
    let map_music_rom_bank = cpu.read_byte(cpu.hl());
    cpu.borrow_wram_mut()
        .set_map_music_rom_bank(map_music_rom_bank);

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon
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
pub fn copy_sign_data(cpu: &mut Cpu, num_signs: u8) {
    log::trace!("copy_sign_data()");

    cpu.set_de(wram::W_SIGN_COORDS);
    cpu.set_bc(wram::W_SIGN_TEXT_IDS);

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
}

pub fn load_map_data(cpu: &mut Cpu) {
    log::debug!("load_map_data()");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.call(0x0061); // DisableLCD

    reset_map_variables(cpu);

    cpu.call(0x36a3); // LoadTextBoxTilePatterns
    load_map_header(cpu);

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

/// Returns pointer to the map header of the map
fn get_map_header_pointer(map_id: u8) -> u16 {
    // 3f:41f2 MapHeaderPointers
    const MAP_HEADER_POINTERS: usize = (0x3f * 0x4000) | (0x41f2 & 0x3fff);

    log::trace!("get_map_header_pointer({:02x})", map_id);

    let pointer = MAP_HEADER_POINTERS + ((map_id as usize) * 2);

    u16::from_le_bytes([ROM[pointer], ROM[pointer + 1]])
}

fn ignore_input_for_half_second(cpu: &mut Cpu) {
    log::trace!("ignore_input_for_half_second()");

    cpu.borrow_wram_mut().set_ignore_input_counter(30);

    // set ignore input bit
    cpu.a = cpu.read_byte(wram::W_D730) | 0b00100110;
    cpu.write_byte(wram::W_D730, cpu.a);
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
    log::trace!("init_sprites()");

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
    let pointer = u16::from_le_bytes([cpu.read_byte(src), cpu.read_byte(src + 1)]);
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
