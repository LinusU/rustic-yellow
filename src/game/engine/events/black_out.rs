use crate::{
    cpu::Cpu,
    game::{
        macros,
        ram::{hram, wram},
    },
    game_state::{BattleResult, BCD},
};

pub fn reset_status_and_halve_money_on_blackout(cpu: &mut Cpu) {
    log::info!("reset_status_and_halve_money_on_blackout()");

    // Reset player status on blackout
    cpu.write_byte(wram::W_D435, 0);
    cpu.borrow_wram_mut().set_battle_result(BattleResult::Win);
    cpu.borrow_wram_mut().set_walk_bike_surf_state(0);
    cpu.borrow_wram_mut().set_is_in_battle(0);
    cpu.borrow_wram_mut().set_map_pal_offset(0);
    cpu.write_byte(wram::W_NPC_MOVEMENT_SCRIPT_FUNCTION_NUM, 0);
    cpu.write_byte(hram::H_JOY_HELD, 0);
    cpu.write_byte(wram::W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM, 0);
    cpu.borrow_wram_mut().clear_cd60();
    cpu.borrow_wram_mut().set_fly_or_dungeon_warp(true);
    cpu.borrow_wram_mut().set_used_warp_pad(false);
    cpu.borrow_wram_mut().set_map_dest_is_last_blackout(true);
    cpu.borrow_wram_mut().set_joy_ignore(0xff);

    // Halve the player's money
    let money = cpu.borrow_wram().player_money().to_u32();

    log::info!("Halving player's money from {} to {}", money, money / 2);

    cpu.borrow_wram_mut()
        .set_player_money(BCD::from_u32(money / 2));

    macros::predef::predef_jump!(cpu, HealParty);
}
