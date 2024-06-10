use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            hardware_constants::MBC1_ROM_BANK,
            input_constants::{A_BUTTON, B_BUTTON, D_UP, SELECT, START},
            sprite_data_constants::{
                PLAYER_DIR_DOWN, PLAYER_DIR_LEFT, PLAYER_DIR_RIGHT, PLAYER_DIR_UP,
                SPRITE_FACING_DOWN, SPRITE_FACING_LEFT, SPRITE_FACING_RIGHT, SPRITE_FACING_UP,
            },
        },
        home, macros,
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

// Handle the player jumping down a ledge in the overworld.
pub fn handle_mid_jump(cpu: &mut Cpu) {
    if (cpu.read_byte(wram::W_D736) & (1 << 6)) != 0 {
        macros::farcall::farcall(cpu, 0x1c, 0x48df); // _HandleMidJump
    }

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn init_sprites(cpu: &mut Cpu) {
    log::debug!("init_sprites()");

    cpu.pc = 0x1006;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // save the number of sprites
    // ld [wNumSprites], a
    let num_sprites = cpu.a;
    cpu.borrow_wram_mut().set_num_sprites(num_sprites);
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

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // call ZeroSpriteStateData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        zero_sprite_state_data(cpu);
        cpu.pc = pc;
    }

    // call DisableRegularSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        disable_regular_sprites(cpu);
        cpu.pc = pc;
    }

    // ld hl, wMapSpriteData
    cpu.set_hl(wram::W_MAP_SPRITE_DATA);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, $20
    cpu.set_bc(0x20);
    cpu.pc += 3;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // call FillMemory
    {
        cpu.pc += 3;
        cpu.cycle(24);
        cpu.call(0x166e);
    }

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

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

    // ld a, [wNumSprites]
    cpu.a = cpu.borrow_wram().num_sprites();
    cpu.pc += 3;
    cpu.cycle(16);

    // are sprites existant?
    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // don't copy sprite data if not
    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld c, $0
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld de, wSprite01StateData1
    cpu.set_de(wram::W_SPRITE01_STATE_DATA1);
    cpu.pc += 3;
    cpu.cycle(12);

    // copy sprite stuff?
    init_sprites_load_sprite_loop(cpu);
}

fn init_sprites_load_sprite_loop(cpu: &mut Cpu) {
    cpu.pc = 0x102b;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // x#SPRITESTATEDATA1_PICTUREID
    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc d
    cpu.set_flag(CpuFlag::H, (cpu.d & 0x0f) == 0x0f);
    cpu.d = cpu.d.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.d == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, e
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

    // add $4
    {
        let value = cpu.read_byte(cpu.pc + 1);
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (value & 0x0f) > 0x0f);
        cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (value as u16) > 0xff);
        cpu.a = cpu.a.wrapping_add(value);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // x#SPRITESTATEDATA2_MAPY
    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc e
    cpu.set_flag(CpuFlag::H, (cpu.e & 0x0f) == 0x0f);
    cpu.e = cpu.e.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.e == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // x#SPRITESTATEDATA2_MAPX
    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc e
    cpu.set_flag(CpuFlag::H, (cpu.e & 0x0f) == 0x0f);
    cpu.e = cpu.e.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.e == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // x#SPRITESTATEDATA2_MOVEMENTBYTE1
    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // save movement byte 2
    // ldh [hLoadSpriteTemp1], a
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // save text ID and flags byte
    // ldh [hLoadSpriteTemp2], a
    cpu.write_byte(hram::H_LOAD_SPRITE_TEMP2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // call LoadSprite
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        load_sprite(cpu);
        cpu.pc = pc;
    }

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // dec d
    cpu.set_flag(CpuFlag::H, (cpu.d & 0x0f) == 0x00);
    cpu.d = cpu.d.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.d == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, e
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

    // add $a
    {
        let value = cpu.read_byte(cpu.pc + 1);
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (value & 0x0f) > 0x0f);
        cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (value as u16) > 0xff);
        cpu.a = cpu.a.wrapping_add(value);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // inc c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x0f);
    cpu.c = cpu.c.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // inc c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x0f);
    cpu.c = cpu.c.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loadSpriteLoop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return init_sprites_load_sprite_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

/// Zero out sprite state data for sprites 01..=14 \
/// Sprite 15 is used for Pikachu
pub fn zero_sprite_state_data(cpu: &mut Cpu) {
    log::debug!("zero_sprite_state_data()");

    const SPRITE_COUNT: u16 = 14;
    const DATA_SIZE: u16 = 0x10;

    for i in 0..(SPRITE_COUNT * DATA_SIZE) {
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1 + i, 0);
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA2 + i, 0);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Disable SPRITESTATEDATA1_IMAGEINDEX (set to $ff) for sprites 01..=14
pub fn disable_regular_sprites(cpu: &mut Cpu) {
    log::debug!("disable_regular_sprites()");

    for i in 0..14 {
        cpu.write_byte(wram::W_SPRITE01_STATE_DATA1_IMAGE_INDEX + (i * 0x10), 0xff);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Input:
/// c = 2x index into MAP_SPRITE_DATA and W_MAP_SPRITE_EXTRA_DATA
/// hl = pointer to last part (arg 7) of object_event
pub fn load_sprite(cpu: &mut Cpu) {
    let data_offset = cpu.c as u16;

    log::debug!(
        "load_sprite(c = {:02x}, hl = {:02x}:{:04x})",
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

    cpu.pc = cpu.stack_pop(); // ret
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
