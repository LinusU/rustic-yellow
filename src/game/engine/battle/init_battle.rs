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
    cpu.pc = 0x5ff2;

    // ld a, [wCurOpponent]
    cpu.a = cpu.read_byte(wram::W_CUR_OPPONENT);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, DetermineWildOpponent
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return determine_wild_opponent(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    init_opponent(cpu);
}

pub fn init_opponent(cpu: &mut Cpu) {
    cpu.pc = 0x5ff8;

    // ld a, [wCurOpponent]
    cpu.a = cpu.read_byte(wram::W_CUR_OPPONENT);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wcf91], a
    cpu.write_byte(wram::W_CF91, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wEnemyMonSpecies2], a
    cpu.write_byte(wram::W_ENEMY_MON_SPECIES2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // jr InitBattleCommon
    cpu.cycle(12);
    return init_battle_common(cpu);
}

fn determine_wild_opponent(cpu: &mut Cpu) {
    cpu.pc = 0x6003;

    // ld a, [wd732]
    cpu.a = cpu.read_byte(wram::W_D732);
    cpu.pc += 3;
    cpu.cycle(16);

    // bit 1, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << 1)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .notDebug
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return determine_wild_opponent_not_debug(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ldh a, [hJoyHeld]
    cpu.a = cpu.read_byte(hram::H_JOY_HELD);
    cpu.pc += 2;
    cpu.cycle(12);

    // bit BIT_B_BUTTON, a
    cpu.set_flag(
        CpuFlag::Z,
        (cpu.a & (1 << input_constants::BIT_B_BUTTON)) == 0,
    );
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

    determine_wild_opponent_not_debug(cpu);
}

fn determine_wild_opponent_not_debug(cpu: &mut Cpu) {
    cpu.pc = 0x600f;

    // ld a, [wNumberOfNoRandomBattleStepsLeft]
    cpu.a = cpu.read_byte(wram::W_NUMBER_OF_NO_RANDOM_BATTLE_STEPS_LEFT);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // callfar TryDoWildEncounter
    macros::farcall::callfar(cpu, 0x04, 0x783a);

    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    init_battle_common(cpu);
}

fn init_battle_common(cpu: &mut Cpu) {
    cpu.pc = 0x601d;

    // ld a, [wMapPalOffset]
    cpu.a = cpu.read_byte(wram::W_MAP_PAL_OFFSET);
    cpu.pc += 3;
    cpu.cycle(16);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, wLetterPrintingDelayFlags
    cpu.set_hl(wram::W_LETTER_PRINTING_DELAY_FLAGS);
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

    // res 1, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << 1));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // call InitBattleVariables
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // init_battle_variables(cpu);
        cpu.call(0x6236);
        cpu.pc = pc;
    }

    // ld a, [wEnemyMonSpecies2]
    cpu.a = cpu.read_byte(wram::W_ENEMY_MON_SPECIES2);
    cpu.pc += 3;
    cpu.cycle(16);

    // sub OPP_ID_OFFSET
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (trainer_constants::OPP_ID_OFFSET & 0x0f),
    );
    cpu.set_flag(CpuFlag::C, cpu.a < trainer_constants::OPP_ID_OFFSET);
    cpu.a = cpu.a.wrapping_sub(trainer_constants::OPP_ID_OFFSET);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jp c, InitWildBattle
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(16);
        return init_wild_battle(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld [wTrainerClass], a
    cpu.write_byte(wram::W_TRAINER_CLASS, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // call GetTrainerInformation
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // get_trainer_information(cpu);
        cpu.call(0x3563);
        cpu.pc = pc;
    }

    // callfar ReadTrainer
    macros::farcall::callfar(cpu, 0x0e, 0x5bb6);

    // callfar DoBattleTransitionAndInitBattleVariables
    macros::farcall::callfar(cpu, 0x0f, 0x6db8);

    // call _LoadTrainerPic
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        _load_trainer_pic(cpu);
        cpu.pc = pc;
    }

    // xor a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wEnemyMonSpecies2], a
    cpu.write_byte(wram::W_ENEMY_MON_SPECIES2, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hStartTileID], a
    cpu.write_byte(hram::H_START_TILE_ID, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wAICount], a
    cpu.write_byte(wram::W_AI_COUNT, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // hlcoord 12, 0
    cpu.set_hl(macros::coords::coord!(12, 0));
    cpu.pc += 3;
    cpu.cycle(12);

    // predef CopyUncompressedPicToTilemap
    macros::predef::predef_call!(cpu, CopyUncompressedPicToTilemap);

    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wEnemyMonPartyPos], a
    cpu.write_byte(wram::W_ENEMY_MON_PARTY_POS, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, $2
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wIsInBattle], a
    cpu.write_byte(wram::W_IS_IN_BATTLE, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // Is this a major story battle?
    // ld a, [wLoneAttackNo]
    cpu.a = cpu.read_byte(wram::W_LONE_ATTACK_NO);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jp z, _InitBattleCommon
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return _init_battle_common(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // useless since already in bank 3d
    macros::farcall::callabd_modify_pikachu_happiness(
        cpu,
        pikachu_emotion_constants::PIKAHAPPY_GYMLEADER,
    );

    // jp _InitBattleCommon
    cpu.cycle(16);
    return _init_battle_common(cpu);
}

fn init_wild_battle(cpu: &mut Cpu) {
    cpu.pc = 0x607c;

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wIsInBattle], a
    cpu.write_byte(wram::W_IS_IN_BATTLE, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // callfar LoadEnemyMonData
    macros::farcall::callfar(cpu, 0x0f, 0x6c87);

    // callfar DoBattleTransitionAndInitBattleVariables
    macros::farcall::callfar(cpu, 0x0f, 0x6db8);

    // ld a, [wCurOpponent]
    cpu.a = cpu.read_byte(wram::W_CUR_OPPONENT);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp RESTLESS_SOUL
    cpu.set_flag(CpuFlag::Z, cpu.a == pokemon_constants::RESTLESS_SOUL);
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (pokemon_constants::RESTLESS_SOUL & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < pokemon_constants::RESTLESS_SOUL);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .isGhost
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return init_wild_battle_is_ghost(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // callfar IsGhostBattle
    macros::farcall::callfar(cpu, 0x0f, 0x59ac);

    // jr nz, .isNoGhost
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        init_wild_battle_is_no_ghost(cpu)
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
        init_wild_battle_is_ghost(cpu)
    }
}

fn init_wild_battle_is_ghost(cpu: &mut Cpu) {
    cpu.pc = 0x60a2;

    // ld hl, wMonHSpriteDim
    cpu.set_hl(wram::W_MON_H_SPRITE_DIM);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, $66
    cpu.a = 0x66;
    cpu.pc += 2;
    cpu.cycle(8);

    // write sprite dimensions
    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld bc, GhostPic
    cpu.set_bc(0x6920);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // write front sprite pointer
    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [hl], b
    cpu.write_byte(cpu.hl(), cpu.b);
    cpu.pc += 1;
    cpu.cycle(8);

    // set name to "GHOST"
    cpu.write_byte(wram::W_ENEMY_MON_NICK, 0x86); // G
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 1, 0x87); // H
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 2, 0x8e); // O
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 3, 0x92); // S
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 4, 0x93); // T
    cpu.write_byte(wram::W_ENEMY_MON_NICK + 5, 0x50); // string terminator

    // Probably not needed
    cpu.set_hl(wram::W_ENEMY_MON_NICK + 5);
    cpu.pc += 20;
    cpu.cycle(104);

    // ld a, [wcf91]
    cpu.a = cpu.read_byte(wram::W_CF91);
    cpu.pc += 3;
    cpu.cycle(16);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, MON_GHOST
    cpu.a = pokemon_constants::MON_GHOST;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wcf91], a
    cpu.write_byte(wram::W_CF91, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld de, vFrontPic
    cpu.set_de(vram::V_FRONT_PIC);
    cpu.pc += 3;
    cpu.cycle(12);

    // load ghost sprite
    // call LoadMonFrontSprite
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        home::pics::load_mon_front_sprite(cpu);
        cpu.pc = pc;
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wcf91], a
    cpu.write_byte(wram::W_CF91, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // jr .spriteLoaded
    cpu.cycle(12);
    return init_wild_battle_sprite_loaded(cpu);
}

fn init_wild_battle_is_no_ghost(cpu: &mut Cpu) {
    cpu.pc = 0x60d7;

    // ld de, vFrontPic
    cpu.set_de(vram::V_FRONT_PIC);
    cpu.pc += 3;
    cpu.cycle(12);

    // load mon sprite
    // call LoadMonFrontSprite
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        home::pics::load_mon_front_sprite(cpu);
        cpu.pc = pc;
    }

    init_wild_battle_sprite_loaded(cpu);
}

fn init_wild_battle_sprite_loaded(cpu: &mut Cpu) {
    cpu.pc = 0x60dd;

    // xor a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wTrainerClass], a
    cpu.write_byte(wram::W_TRAINER_CLASS, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hStartTileID], a
    cpu.write_byte(hram::H_START_TILE_ID, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // hlcoord 12, 0
    cpu.set_hl(macros::coords::coord!(12, 0));
    cpu.pc += 3;
    cpu.cycle(12);

    // predef CopyUncompressedPicToTilemap
    macros::predef::predef_call!(cpu, CopyUncompressedPicToTilemap);

    // common code that executes after init battle code specific to trainer or wild battles
    _init_battle_common(cpu);
}

fn _init_battle_common(cpu: &mut Cpu) {
    cpu.pc = 0x60eb;

    // ld b, SET_PAL_BATTLE_BLACK
    cpu.b = palette_constants::SET_PAL_BATTLE_BLACK;
    cpu.pc += 2;
    cpu.cycle(8);

    // call RunPaletteCommand
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // run_palette_command(cpu);
        cpu.call(0x3e05);
        cpu.pc = pc;
    }

    // callfar SlidePlayerAndEnemySilhouettesOnScreen
    macros::farcall::callfar(cpu, 0x0f, 0x404c);

    // xor a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hAutoBGTransferEnabled], a
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld hl, .emptyString
    cpu.set_hl(0x6159);
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // print_text(cpu);
        cpu.call(0x3c36);
        cpu.pc = pc;
    }

    // call SaveScreenTilesToBuffer1
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // save_screen_tiles_to_buffer1(cpu);
        cpu.call(0x370f);
        cpu.pc = pc;
    }

    // call ClearScreen
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // clear_screen(cpu);
        cpu.call(0x16dd);
        cpu.pc = pc;
    }

    // ld a, $98
    cpu.a = 0x98;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hAutoBGTransferDest] + 1, a
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_DEST + 1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hAutoBGTransferEnabled], a
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call Delay3
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // delay3(cpu);
        cpu.call(0x3ddb);
        cpu.pc = pc;
    }

    // ld a, $9c
    cpu.a = 0x9c;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hAutoBGTransferDest] + 1, a
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_DEST + 1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call LoadScreenTilesFromBuffer1
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // load_screen_tiles_from_buffer1(cpu);
        cpu.call(0x371b);
        cpu.pc = pc;
    }

    // hlcoord 9, 7
    cpu.set_hl(macros::coords::coord!(9, 7));
    cpu.pc += 3;
    cpu.cycle(12);

    // lb bc, 5, 10
    cpu.b = 5;
    cpu.c = 10;
    cpu.pc += 3;
    cpu.cycle(12);

    // call ClearScreenArea
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // clear_screen_area(cpu);
        cpu.call(0x1692);
        cpu.pc = pc;
    }

    // hlcoord 1, 0
    cpu.set_hl(macros::coords::coord!(1, 0));
    cpu.pc += 3;
    cpu.cycle(12);

    // lb bc, 4, 10
    cpu.b = 4;
    cpu.c = 10;
    cpu.pc += 3;
    cpu.cycle(12);

    // call ClearScreenArea
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // clear_screen_area(cpu);
        cpu.call(0x1692);
        cpu.pc = pc;
    }

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // clear_sprites(cpu);
        cpu.call(0x0082);
        cpu.pc = pc;
    }

    // ld a, [wIsInBattle]
    cpu.a = cpu.read_byte(wram::W_IS_IN_BATTLE);
    cpu.pc += 3;
    cpu.cycle(16);

    // is it a wild battle?
    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, DrawEnemyHUDAndHPBar
    cpu.set_hl(0x4eb1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, BANK(DrawEnemyHUDAndHPBar)
    cpu.b = 0x0f;
    cpu.pc += 2;
    cpu.cycle(8);

    // draw enemy HUD and HP bar if it's a wild battle
    // call z, Bankswitch
    if cpu.flag(CpuFlag::Z) {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // bankswitch(cpu);
        cpu.call(0x3e84);
        cpu.pc = pc;
    }

    // callfar StartBattle
    macros::farcall::callfar(cpu, 0x0f, 0x4127);

    // callfar EndOfBattle
    macros::farcall::callfar(cpu, 0x04, 0x7765);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wLetterPrintingDelayFlags], a
    cpu.write_byte(wram::W_LETTER_PRINTING_DELAY_FLAGS, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wMapPalOffset], a
    cpu.write_byte(wram::W_MAP_PAL_OFFSET, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wSavedTileAnimations]
    cpu.a = cpu.read_byte(wram::W_SAVED_TILE_ANIMATIONS);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hTileAnimations], a
    cpu.write_byte(hram::H_TILE_ANIMATIONS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // scf
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

pub fn _load_trainer_pic(cpu: &mut Cpu) {
    cpu.pc = 0x615a;

    // wd033-wd034 contain pointer to pic
    // ld a, [wTrainerPicPointer]
    cpu.a = cpu.read_byte(wram::W_TRAINER_PIC_POINTER);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wTrainerPicPointer + 1]
    cpu.a = cpu.read_byte(wram::W_TRAINER_PIC_POINTER + 1);
    cpu.pc += 3;
    cpu.cycle(16);

    // de contains pointer to trainer pic
    // ld d, a
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wLinkState]
    cpu.a = cpu.read_byte(wram::W_LINK_STATE);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // this is where all the trainer pics are (not counting Red's)
    // ld a, BANK("Pics 6")
    cpu.a = 0x13;
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .loadSprite
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return _load_trainer_pic_load_sprite(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BANK(RedPicFront)
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);

    _load_trainer_pic_load_sprite(cpu);
}

fn _load_trainer_pic_load_sprite(cpu: &mut Cpu) {
    cpu.pc = 0x616c;

    // call UncompressSpriteFromDE
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        // cpu.stack_push(pc);
        cpu.cycle(24);
        // uncompress_sprite_from_de(cpu);
        cpu.call(0x36e3);
        cpu.pc = pc;
    }

    // ld de, vFrontPic
    cpu.set_de(vram::V_FRONT_PIC);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, $77
    cpu.a = 0x77;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // jp LoadUncompressedSpriteData
    cpu.cycle(16);
    cpu.jump(0x144b);
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
