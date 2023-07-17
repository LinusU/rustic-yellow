use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            input_constants, palette_constants, pikachu_emotion_constants, pokemon_constants,
            trainer_constants,
        },
        engine::menus::pokedex,
        home, macros,
        ram::{hram, vram, wram},
    },
};

pub fn init_battle(cpu: &mut Cpu) {
    if cpu.borrow_wram().cur_opponent() == 0 {
        determine_wild_opponent(cpu)
    } else {
        init_opponent(cpu)
    }
}

pub fn init_opponent(cpu: &mut Cpu) {
    let opponent = cpu.borrow_wram().cur_opponent();
    log::debug!("Starting battle with Trainer {}", opponent);

    cpu.write_byte(wram::W_CF91, opponent);
    cpu.borrow_wram_mut().set_enemy_mon_species2(opponent);

    init_battle_common(cpu)
}

fn determine_wild_opponent(cpu: &mut Cpu) {
    // Allow wild battles to be avoided by holding down B in debug mode
    if cpu.borrow_wram().debug_mode() {
        let held = cpu.read_byte(hram::H_JOY_HELD);

        if (held & input_constants::B_BUTTON) != 0 {
            cpu.pc = cpu.stack_pop();
            return;
        }
    }

    if cpu.borrow_wram().number_of_no_random_battle_steps_left() != 0 {
        cpu.pc = cpu.stack_pop();
        return;
    }

    macros::farcall::callfar(cpu, 0x04, 0x783a); // TryDoWildEncounter

    // TryDoWildEncounter returns success in Z
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        return;
    }

    init_battle_common(cpu);
}

fn init_battle_common(cpu: &mut Cpu) {
    // Save Map Pal Offset
    let map_pal_offset = cpu.borrow_wram().map_pal_offset();
    cpu.stack_push((map_pal_offset as u16) << 8);

    // Save Letter Printing Delay Flags
    let letter_printing_delay_flags = cpu.borrow_wram().letter_printing_delay_flags();
    cpu.stack_push((letter_printing_delay_flags as u16) << 8);

    // Reset Letter Printing Delay Flags
    cpu.borrow_wram_mut()
        .set_letter_printing_delay_flags(letter_printing_delay_flags & !(1 << 1));

    cpu.call(0x6236); // InitBattleVariables

    let enemy = cpu.borrow_wram().enemy_mon_species2();
    log::debug!("init_battle_common: enemy={}", enemy);

    if enemy < trainer_constants::OPP_ID_OFFSET {
        log::debug!("init_battle_common: wild battle");
        return init_wild_battle(cpu);
    }

    cpu.borrow_wram_mut()
        .set_trainer_class(enemy - trainer_constants::OPP_ID_OFFSET);

    log::debug!(
        "GetTrainerInformation({})",
        enemy - trainer_constants::OPP_ID_OFFSET
    );
    cpu.call(0x3563); // GetTrainerInformation

    macros::farcall::callfar(cpu, 0x0e, 0x5bb6); // ReadTrainer
    macros::farcall::callfar(cpu, 0x0f, 0x6db8); // DoBattleTransitionAndInitBattleVariables

    cpu.stack_push(0x0001);
    _load_trainer_pic(cpu);

    cpu.borrow_wram_mut().set_enemy_mon_species2(0);
    cpu.write_byte(hram::H_START_TILE_ID, 0);
    cpu.borrow_wram_mut().set_ai_count(0xff);

    cpu.set_hl(macros::coords::coord!(12, 0));

    macros::predef::predef_call!(cpu, CopyUncompressedPicToTilemap);

    cpu.borrow_wram_mut().set_enemy_mon_party_pos(0xff);
    cpu.borrow_wram_mut().set_is_in_battle(2);

    // Is this a major story battle?
    if cpu.borrow_wram().gym_leader_no() != 0 {
        // useless since already in bank 3d
        macros::farcall::callabd_modify_pikachu_happiness(
            cpu,
            pikachu_emotion_constants::PIKAHAPPY_GYMLEADER,
        );
    }

    _init_battle_common(cpu)
}

fn init_wild_battle(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_is_in_battle(1);

    macros::farcall::callfar(cpu, 0x0f, 0x6c87); // LoadEnemyMonData
    macros::farcall::callfar(cpu, 0x0f, 0x6db8); // DoBattleTransitionAndInitBattleVariables

    if cpu.borrow_wram().cur_opponent() == pokemon_constants::RESTLESS_SOUL {
        return init_wild_battle_is_ghost(cpu);
    }

    macros::farcall::callfar(cpu, 0x0f, 0x59ac); // IsGhostBattle

    // jr nz, .isNoGhost
    if !cpu.flag(CpuFlag::Z) {
        init_wild_battle_is_no_ghost(cpu)
    } else {
        init_wild_battle_is_ghost(cpu)
    }
}

fn init_wild_battle_is_ghost(cpu: &mut Cpu) {
    const GHOST_PIC: u16 = 0x6920;

    cpu.write_byte(wram::W_MON_H_SPRITE_DIM, 0x66);

    cpu.write_byte(wram::W_MON_H_FRONT_SPRITE, (GHOST_PIC & 0xff) as u8);
    cpu.write_byte(wram::W_MON_H_FRONT_SPRITE + 1, (GHOST_PIC >> 8) as u8);

    // set name to "GHOST"
    cpu.write_byte(wram::W_ENEMY_MON_NICK, 0x86); // G
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 1, 0x87); // H
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 2, 0x8e); // O
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 3, 0x92); // S
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 4, 0x93); // T
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 5, 0x50); // string terminator

    cpu.set_hl(wram::W_ENEMY_MON_NICK + 5); // Probably not needed

    let saved_cf91 = cpu.read_byte(wram::W_CF91);
    cpu.write_byte(wram::W_CF91, pokemon_constants::MON_GHOST);
    cpu.set_de(vram::V_FRONT_PIC);

    cpu.stack_push(0x0001);
    home::pics::load_mon_front_sprite(cpu);

    // Restore CF91
    cpu.write_byte(wram::W_CF91, saved_cf91);

    init_wild_battle_sprite_loaded(cpu)
}

fn init_wild_battle_is_no_ghost(cpu: &mut Cpu) {
    cpu.set_de(vram::V_FRONT_PIC);

    cpu.stack_push(0x0001);
    home::pics::load_mon_front_sprite(cpu);

    init_wild_battle_sprite_loaded(cpu);
}

fn init_wild_battle_sprite_loaded(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_trainer_class(0);
    cpu.write_byte(hram::H_START_TILE_ID, 0);
    cpu.set_hl(macros::coords::coord!(12, 0));

    macros::predef::predef_call!(cpu, CopyUncompressedPicToTilemap);

    _init_battle_common(cpu);
}

/// Common code that executes after init battle code specific to trainer or wild battles
fn _init_battle_common(cpu: &mut Cpu) {
    cpu.b = palette_constants::SET_PAL_BATTLE_BLACK;
    cpu.call(0x3e05); // RunPaletteCommand

    macros::farcall::callfar(cpu, 0x0f, 0x404c); // SlidePlayerAndEnemySilhouettesOnScreen

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 0);

    cpu.set_hl(0x6159); // .emptyString
    cpu.call(0x3c36); // PrintText

    cpu.call(0x370f); // SaveScreenTilesToBuffer1
    cpu.call(0x16dd); // ClearScreen

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_DEST + 1, 0x98);
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 0x01);

    cpu.call(0x3ddb); // Delay3

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_DEST + 1, 0x9c);

    cpu.call(0x371b); // LoadScreenTilesFromBuffer1

    cpu.set_hl(macros::coords::coord!(9, 7));
    cpu.b = 5;
    cpu.c = 10;
    cpu.call(0x1692); // ClearScreenArea

    cpu.set_hl(macros::coords::coord!(1, 0));
    cpu.b = 4;
    cpu.c = 10;
    cpu.call(0x1692); // ClearScreenArea

    cpu.call(0x0082); // ClearSprites

    // Draw enemy HUD and HP bar if it's a wild battle
    if cpu.borrow_wram().is_in_battle() == 1 {
        cpu.set_hl(0x4eb1); // DrawEnemyHUDAndHPBar
        cpu.b = 0x0f; // BANK(DrawEnemyHUDAndHPBar)
        cpu.call(0x3e84); // Bankswitch
    }

    macros::farcall::callfar(cpu, 0x0f, 0x4127); // StartBattle
    macros::farcall::callfar(cpu, 0x04, 0x7765); // EndOfBattle

    let letter_printing_delay_flags = (cpu.stack_pop() >> 8) as u8;
    cpu.borrow_wram_mut()
        .set_letter_printing_delay_flags(letter_printing_delay_flags);

    let map_pal_offset = (cpu.stack_pop() >> 8) as u8;
    cpu.borrow_wram_mut().set_map_pal_offset(map_pal_offset);

    // Restore TILE_ANIMATIONS
    let saved_tile_animations = cpu.read_byte(wram::W_SAVED_TILE_ANIMATIONS);
    cpu.write_byte(hram::H_TILE_ANIMATIONS, saved_tile_animations);

    // Return that a battle was had
    cpu.set_flag(CpuFlag::C, true);

    cpu.pc = cpu.stack_pop();
}

pub fn _load_trainer_pic(cpu: &mut Cpu) {
    let bank = if cpu.read_byte(wram::W_LINK_STATE) == 0 {
        // this is where all the trainer pics are (not counting Red's)
        0x13 // BANK("Pics 6")
    } else {
        0x04 // BANK(RedPicFront)
    };

    let addr = cpu.borrow_wram().trainer_pic_pointer() as usize;

    log::debug!("Loading trainer pic at {:02x}:{:04x}", bank, addr);

    let pic = (bank * 0x4000) | (addr & 0x3FFF);
    let pic = pokemon_sprite_compression::gen1::decompress(&crate::rom::ROM[pic..]);

    for (offset, byte) in pic.iter().enumerate() {
        cpu.write_byte(vram::V_FRONT_PIC + offset as u16, *byte);
    }

    cpu.pc = cpu.stack_pop();
}

/// Assumes the monster's attributes have been loaded with GetMonHeader.
pub fn load_mon_back_pic(cpu: &mut Cpu) {
    let pokemon_index = cpu.read_byte(wram::W_BATTLE_MON_SPECIES2);
    let pokedex_no = pokedex::index_to_pokedex(pokemon_index);

    // Probably not needed, but is done by the GameBoy code
    {
        cpu.write_byte(wram::W_CF91, pokemon_index);

        // hlcoord 1, 5
        cpu.set_hl(macros::coords::coord!(1, 5));
        cpu.pc += 3;
        cpu.cycle(12);

        // lb bc, 7, 8
        cpu.b = 7;
        cpu.c = 8;
        cpu.pc += 3;
        cpu.cycle(12);

        // call ClearScreenArea
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        home::copy2::clear_screen_area(cpu);
        assert_eq!(cpu.pc, pc);
    }

    let source_data = home::pics::read_crystal_pokemon_sprite(pokedex_no as usize, true);
    assert_eq!(source_data.len(), 48 * 48 / 4);

    let sprite_data = home::pics::center_pokemon_sprite(&source_data, 6, 6);

    // Probably not needed, but is done by the GameBoy code
    for (idx, data) in sprite_data.iter().enumerate() {
        cpu.write_byte(vram::V_SPRITES + (idx as u16), *data);
    }

    for (idx, data) in sprite_data.iter().enumerate() {
        cpu.write_byte(vram::V_BACK_PIC + (idx as u16), *data);
    }

    // ret
    cpu.pc = cpu.stack_pop();
}
