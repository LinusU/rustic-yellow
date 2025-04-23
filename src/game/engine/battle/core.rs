use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            audio_constants::{BIT_LOW_HEALTH_ALARM, CHAN5},
            gfx_constants::HP_BAR_RED,
            item_constants::SILPH_SCOPE,
            map_constants::{POKEMON_TOWER_1F, POKEMON_TOWER_7F},
        },
        home, macros,
        ram::{hram, wram},
    },
};

pub fn draw_player_hud_and_hp_bar(cpu: &mut Cpu) {
    log::info!("draw_player_hud_and_hp_bar()");

    cpu.pc = 0x4e25;

    // xor a, a
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

    // hlcoord 9, 7
    cpu.set_hl(macros::coords::coord!(9, 7));
    cpu.pc += 3;
    cpu.cycle(12);

    // lb bc, 5, 11
    cpu.b = 5;
    cpu.c = 11;
    cpu.pc += 3;
    cpu.cycle(12);

    // call ClearScreenArea
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1692); // ClearScreenArea
        cpu.pc = pc;
    }

    // callfar PlacePlayerHUDTiles
    macros::farcall::callfar(cpu, 0x0e, 0x69a3);

    // hlcoord 18, 9
    cpu.set_hl(macros::coords::coord!(18, 9));
    cpu.pc += 3;
    cpu.cycle(12);

    // ld [hl], $73
    cpu.write_byte(cpu.hl(), 0x73);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld de, wBattleMonNick
    cpu.set_de(wram::W_BATTLE_MON_NICK);
    cpu.pc += 3;
    cpu.cycle(12);

    // hlcoord 10, 7
    cpu.set_hl(macros::coords::coord!(10, 7));
    cpu.pc += 3;
    cpu.cycle(12);

    // call CenterMonName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x4f61); // CenterMonName
        cpu.pc = pc;
    }

    // call PlaceString
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1723); // PlaceString
        cpu.pc = pc;
    }

    // ld hl, wBattleMonSpecies
    cpu.set_hl(wram::W_BATTLE_MON_SPECIES);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wLoadedMon
    cpu.set_de(wram::W_LOADED_MON);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, wBattleMonDVs - wBattleMonSpecies
    cpu.set_bc(wram::W_BATTLE_MON_DVS - wram::W_BATTLE_MON_SPECIES);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x00b1); // CopyData
        cpu.pc = pc;
    }

    // ld hl, wBattleMonLevel
    cpu.set_hl(wram::W_BATTLE_MON_LEVEL);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wLoadedMonLevel
    cpu.set_de(wram::W_LOADED_MON_LEVEL);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, wBattleMonPP - wBattleMonLevel
    cpu.set_bc(wram::W_BATTLE_MON_PP - wram::W_BATTLE_MON_LEVEL);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x00b1); // CopyData
        cpu.pc = pc;
    }

    // hlcoord 14, 8
    cpu.set_hl(macros::coords::coord!(14, 8));
    cpu.pc += 3;
    cpu.cycle(12);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld de, wLoadedMonStatus
    cpu.set_de(wram::W_LOADED_MON_STATUS);
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintStatusConditionNotFainted
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x12f3); // PrintStatusConditionNotFainted
        cpu.pc = pc;
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // jr nz, .doNotPrintLevel
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return draw_player_hudand_hpbar_do_not_print_level(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call PrintLevel
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1303); // PrintLevel
        cpu.pc = pc;
    }

    draw_player_hudand_hpbar_do_not_print_level(cpu);
}

fn draw_player_hudand_hpbar_do_not_print_level(cpu: &mut Cpu) {
    cpu.pc = 0x4e73;

    // ld a, [wLoadedMonSpecies]
    cpu.a = cpu.read_byte(wram::W_LOADED_MON_SPECIES);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurPartySpecies], a
    cpu.write_byte(wram::W_CUR_PARTY_SPECIES, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // hlcoord 10, 9
    cpu.set_hl(macros::coords::coord!(10, 9));
    cpu.pc += 3;
    cpu.cycle(12);

    // predef DrawHP
    macros::predef::predef_call!(cpu, DrawHP);

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hAutoBGTransferEnabled], a
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld hl, wPlayerHPBarColor
    cpu.set_hl(wram::W_PLAYER_HP_BAR_COLOR);
    cpu.pc += 3;
    cpu.cycle(12);

    // call GetBattleHealthBarColor
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x4f55); // GetBattleHealthBarColor
        cpu.pc = pc;
    }

    // ld hl, wBattleMonHP
    cpu.set_hl(wram::W_BATTLE_MON_HP);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // or a, [hl]
    cpu.a |= cpu.read_byte(cpu.hl());
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(8);

    // jr z, .fainted
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return draw_player_hudand_hpbar_fainted(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wLowHealthAlarmDisabled]
    cpu.a = cpu.read_byte(wram::W_LOW_HEALTH_ALARM_DISABLED);
    cpu.pc += 3;
    cpu.cycle(16);

    // has the alarm been disabled because the player has already won?
    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // if so, return
    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [wPlayerHPBarColor]
    cpu.a = cpu.read_byte(wram::W_PLAYER_HP_BAR_COLOR);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp HP_BAR_RED
    cpu.set_flag(CpuFlag::Z, cpu.a == HP_BAR_RED);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (HP_BAR_RED & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < HP_BAR_RED);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .setLowHealthAlarm
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return draw_player_hudand_hpbar_set_low_health_alarm(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    draw_player_hudand_hpbar_fainted(cpu);
}

fn draw_player_hudand_hpbar_fainted(cpu: &mut Cpu) {
    cpu.pc = 0x4e9e;

    // ld hl, wLowHealthAlarm
    cpu.set_hl(wram::W_LOW_HEALTH_ALARM);
    cpu.pc += 3;
    cpu.cycle(12);

    // bit BIT_LOW_HEALTH_ALARM, [hl]
    let value = cpu.read_byte(cpu.hl());
    cpu.set_flag(CpuFlag::Z, (value & (1 << BIT_LOW_HEALTH_ALARM)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(16);

    // ld [hl], 0
    cpu.write_byte(cpu.hl(), 0);
    cpu.pc += 2;
    cpu.cycle(12);

    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wChannelSoundIDs + CHAN5], a
    cpu.write_byte(wram::W_CHANNEL_SOUND_IDS + (CHAN5 as u16), cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn draw_player_hudand_hpbar_set_low_health_alarm(cpu: &mut Cpu) {
    cpu.pc = 0x4eab;

    // ld hl, wLowHealthAlarm
    cpu.set_hl(wram::W_LOW_HEALTH_ALARM);
    cpu.pc += 3;
    cpu.cycle(12);

    // set BIT_LOW_HEALTH_ALARM, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << BIT_LOW_HEALTH_ALARM));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

/// Sets the Z flag if the player is in a ghost battle.
pub fn is_ghost_battle(cpu: &mut Cpu) {
    // If we are not in a battle, then we are not in a ghost battle.
    if cpu.borrow_wram().is_in_battle() != 1 {
        log::debug!("is_ghost_battle() == false");
        cpu.set_flag(CpuFlag::Z, false);
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    let cur_map = cpu.borrow_wram().cur_map();

    // If we are not in the Pokemon Tower, then we are not in a ghost battle.
    if !(POKEMON_TOWER_1F..=POKEMON_TOWER_7F).contains(&cur_map) {
        log::debug!("is_ghost_battle() == false");
        cpu.set_flag(CpuFlag::Z, false);
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    // If we have the Silph Scope, then we are not in a ghost battle.
    {
        cpu.b = SILPH_SCOPE;
        cpu.stack_push(0x0001);
        home::map_objects::is_item_in_bag(cpu);
    }

    log::debug!("is_ghost_battle() == {}", cpu.flag(CpuFlag::Z));
    cpu.pc = cpu.stack_pop(); // ret
}
