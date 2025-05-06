use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            audio_constants::{BIT_LOW_HEALTH_ALARM, CHAN5},
            gfx_constants::{HP_BAR_RED, LEN_2BPP_TILE},
            hardware_constants::R_LCDC,
            item_constants::SILPH_SCOPE,
            map_constants::{POKEMON_TOWER_1F, POKEMON_TOWER_7F},
        },
        home, macros,
        ram::{hram, vram, wram},
    },
};

pub fn draw_player_hud_and_hp_bar(cpu: &mut Cpu) {
    log::trace!("draw_player_hud_and_hp_bar()");

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 0);

    cpu.set_hl(macros::coords::coord!(9, 7));
    cpu.a = 0;
    cpu.b = 5;
    cpu.c = 11;
    cpu.call(0x1692); // ClearScreenArea

    macros::farcall::callfar(cpu, 0x0e, 0x69a3); // PlacePlayerHUDTiles

    cpu.write_byte(macros::coords::coord!(18, 9), 0x73);

    cpu.set_de(wram::W_BATTLE_MON_NICK);
    cpu.set_hl(macros::coords::coord!(10, 7));
    cpu.call(0x4f61); // CenterMonName
    cpu.call(0x1723); // PlaceString

    cpu.set_hl(wram::W_BATTLE_MON_SPECIES);
    cpu.set_de(wram::W_LOADED_MON);
    cpu.set_bc(wram::W_BATTLE_MON_DVS - wram::W_BATTLE_MON_SPECIES);
    cpu.call(0x00b1); // CopyData

    cpu.set_hl(wram::W_BATTLE_MON_LEVEL);
    cpu.set_de(wram::W_LOADED_MON_LEVEL);
    cpu.set_bc(wram::W_BATTLE_MON_PP - wram::W_BATTLE_MON_LEVEL);
    cpu.call(0x00b1); // CopyData

    cpu.set_hl(macros::coords::coord!(14, 8) + 1);
    cpu.set_de(wram::W_LOADED_MON_STATUS);
    cpu.call(0x12f3); // PrintStatusConditionNotFainted
    let no_condition = cpu.flag(CpuFlag::Z);

    if no_condition {
        cpu.set_hl(macros::coords::coord!(14, 8));
        cpu.call(0x1303); // PrintLevel
    }

    cpu.a = cpu.read_byte(wram::W_LOADED_MON_SPECIES);
    cpu.write_byte(wram::W_CUR_PARTY_SPECIES, cpu.a);

    cpu.set_hl(macros::coords::coord!(10, 9));
    macros::predef::predef_call!(cpu, DrawHP);
    let hp_bar_width = cpu.e;

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 1);

    cpu.set_hl(wram::W_PLAYER_HP_BAR_COLOR);
    cpu.e = hp_bar_width;
    cpu.call(0x4f55); // GetBattleHealthBarColor

    cpu.a = cpu.read_byte(wram::W_BATTLE_MON_HP);
    cpu.a |= cpu.read_byte(wram::W_BATTLE_MON_HP + 1);

    if cpu.a != 0 {
        // Return if the alarm been disabled because the player has already won
        if cpu.read_byte(wram::W_LOW_HEALTH_ALARM_DISABLED) != 0 {
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }

        if cpu.read_byte(wram::W_PLAYER_HP_BAR_COLOR) == HP_BAR_RED {
            let value = cpu.read_byte(wram::W_LOW_HEALTH_ALARM);
            let value = value | (1 << BIT_LOW_HEALTH_ALARM);
            cpu.write_byte(wram::W_LOW_HEALTH_ALARM, value);

            cpu.pc = cpu.stack_pop(); // ret
            return;
        }
    }

    let low_health_alarm = cpu.read_byte(wram::W_LOW_HEALTH_ALARM);
    cpu.write_byte(wram::W_LOW_HEALTH_ALARM, 0);

    if (low_health_alarm & (1 << BIT_LOW_HEALTH_ALARM)) != 0 {
        cpu.write_byte(wram::W_CHANNEL_SOUND_IDS + (CHAN5 as u16), 0);
    }

    cpu.pc = cpu.stack_pop(); // ret
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

pub fn load_hud_tile_patterns(cpu: &mut Cpu) {
    log::debug!("load_hud_tile_patterns()");

    cpu.pc = 0x6fe7;

    // ldh a, [rLCDC]
    cpu.a = cpu.read_byte(R_LCDC);
    cpu.pc += 2;
    cpu.cycle(12);

    // is LCD disabled?
    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr c, .lcdEnabled
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return load_hud_tile_patterns_lcd_enabled(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    load_hud_tile_patterns_lcd_disabled(cpu);
}

fn load_hud_tile_patterns_lcd_disabled(cpu: &mut Cpu) {
    cpu.pc = 0x6fec;

    // ld hl, BattleHudTiles1
    cpu.set_hl(0x4c00); // BattleHudTiles1
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, vChars2 tile $6d
    cpu.set_de(vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x6d);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, BattleHudTiles1End - BattleHudTiles1
    cpu.set_bc(0x4c18 - 0x4c00); // BattleHudTiles1End - BattleHudTiles1
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, BANK(BattleHudTiles1)
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);

    // call FarCopyDataDouble
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x15d4); // FarCopyDataDouble
        cpu.pc = pc;
    }

    // ld hl, BattleHudTiles2
    cpu.set_hl(0x4c18); // BattleHudTiles2
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, vChars2 tile $73
    cpu.set_de(vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x73);

    // ld bc, BattleHudTiles3End - BattleHudTiles2
    cpu.set_bc(0x4c48 - 0x4c18); // BattleHudTiles3End - BattleHudTiles2
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, BANK(BattleHudTiles2)
    cpu.a = 0x04;
    cpu.pc += 2;
    cpu.cycle(8);

    // jp FarCopyDataDouble
    cpu.cycle(16);
    cpu.pc = 0x15d4; // FarCopyDataDouble
}

fn load_hud_tile_patterns_lcd_enabled(cpu: &mut Cpu) {
    cpu.pc = 0x7008;

    // ld de, BattleHudTiles1
    cpu.set_de(0x4c00); // BattleHudTiles1
    cpu.pc += 3;
    cpu.cycle(12);

    // ld hl, vChars2 tile $6d
    cpu.set_hl(vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x6d);

    // lb bc, BANK(BattleHudTiles1), (BattleHudTiles1End - BattleHudTiles1) / $8

    // call CopyVideoDataDouble
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1636); // CopyVideoDataDouble
        cpu.pc = pc;
    }

    // ld de, BattleHudTiles2
    cpu.set_de(0x4c18); // BattleHudTiles2
    cpu.pc += 3;
    cpu.cycle(12);

    // ld hl, vChars2 tile $73
    cpu.set_hl(vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x73);

    // lb bc, BANK(BattleHudTiles2), (BattleHudTiles3End - BattleHudTiles2) / $8
    cpu.b = 0x04; // BANK(BattleHudTiles2)
    cpu.c = ((0x4c48 - 0x4c18) / 8) as u8; // (BattleHudTiles3End - BattleHudTiles2) / $8

    // jp CopyVideoDataDouble
    cpu.cycle(16);
    cpu.pc = 0x1636; // CopyVideoDataDouble
}
