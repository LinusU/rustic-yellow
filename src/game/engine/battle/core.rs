use image_gameboy::GameBoy2bpp;

use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            audio_constants::{BIT_LOW_HEALTH_ALARM, CHAN5},
            gfx_constants::{HP_BAR_RED, LEN_2BPP_TILE},
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

const BATTLE_HUD_1: &[u8] = include_bytes!("../../../../gfx/battle/battle_hud_1.png");
const BATTLE_HUD_2: &[u8] = include_bytes!("../../../../gfx/battle/battle_hud_2.png");
const BATTLE_HUD_3: &[u8] = include_bytes!("../../../../gfx/battle/battle_hud_3.png");

pub fn png_to_2bpp(bytes: &[u8]) -> image::ImageResult<Vec<u8>> {
    Ok(image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?.into_gb2bpp())
}

pub fn load_hud_tile_patterns(cpu: &mut Cpu) {
    log::debug!("load_hud_tile_patterns()");

    cpu.wait_for_blank();

    let battle_hud_1 = png_to_2bpp(BATTLE_HUD_1).unwrap();
    let addr = vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x6d;

    for (i, &byte) in battle_hud_1.iter().enumerate() {
        cpu.write_byte(addr + i as u16, byte);
    }

    let battle_hud_2 = png_to_2bpp(BATTLE_HUD_2).unwrap();
    let addr = vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x73;

    for (i, &byte) in battle_hud_2.iter().enumerate() {
        cpu.write_byte(addr + i as u16, byte);
    }

    let battle_hud_3 = png_to_2bpp(BATTLE_HUD_3).unwrap();
    let addr = vram::V_CHARS_2 + LEN_2BPP_TILE as u16 * 0x76;

    for (i, &byte) in battle_hud_3.iter().enumerate() {
        cpu.write_byte(addr + i as u16, byte);
    }

    cpu.pc = cpu.stack_pop(); // ret
}
