use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::sprite_data_constants::{
            PLAYER_DIR_DOWN, PLAYER_DIR_LEFT, PLAYER_DIR_RIGHT, PLAYER_DIR_UP, SPRITE_FACING_DOWN,
            SPRITE_FACING_LEFT, SPRITE_FACING_RIGHT, SPRITE_FACING_UP,
        },
        macros,
        ram::{hram, wram},
    },
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

    if cpu.a > 0 {
        // if there are signs
        // get the coordinates in front of the player in de
        macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

        // call SignLoop
        cpu.call(0x09f2);

        if cpu.flag(CpuFlag::C) {
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
/// In: d = sign y, e = sign x
/// Out: flag C = facing sign
pub fn sign_loop(cpu: &mut Cpu) {
    // start of sign coordinates
    cpu.set_hl(wram::W_SIGN_COORDS);

    // number of signs in the map
    cpu.b = cpu.borrow_wram().num_signs();
    cpu.c = 0;

    let facing_sign = loop {
        cpu.c += 1;

        let sign_y = cpu.read_byte(cpu.hl());
        let sign_x = cpu.read_byte(cpu.hl() + 1);

        cpu.set_hl(cpu.hl() + 2);

        if sign_y == cpu.d && sign_x == cpu.e {
            // store sign text ID
            let text_id = cpu.read_byte(wram::W_SIGN_TEXT_IDS + ((cpu.c - 1) as u16));
            cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, text_id);

            break true;
        }

        cpu.b -= 1;

        if cpu.b == 0 {
            break false;
        }
    };

    cpu.set_flag(CpuFlag::C, facing_sign);
    log::debug!("sign_loop(y = {}, x = {}) == {}", cpu.d, cpu.e, facing_sign);

    cpu.pc = cpu.stack_pop(); // ret
}

// Handle the player jumping down a ledge in the overworld.
pub fn handle_mid_jump(cpu: &mut Cpu) {
    if (cpu.read_byte(wram::W_D736) & (1 << 6)) != 0 {
        macros::farcall::farcall(cpu, 0x1c, 0x48df); // _HandleMidJump
    }

    cpu.pc = cpu.stack_pop(); // ret
}