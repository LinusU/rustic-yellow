use crate::{
    cpu::Cpu,
    game::{
        constants::{battle_constants, map_constants},
        macros,
        ram::{hram, wram},
    },
    game_state,
};

pub fn init_battle_variables(cpu: &mut Cpu) {
    let tile_animations = cpu.read_byte(hram::H_TILE_ANIMATIONS);
    cpu.write_byte(wram::W_SAVED_TILE_ANIMATIONS, tile_animations);

    cpu.borrow_wram_mut()
        .set_action_result_or_took_battle_turn(0);
    cpu.borrow_wram_mut()
        .set_battle_result(game_state::BattleResult::Win);

    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM, 0);
    cpu.write_byte(wram::W_BAG_SAVED_MENU_ITEM, 0);
    cpu.write_byte(wram::W_BATTLE_AND_START_SAVED_MENU_ITEM, 0);
    cpu.write_byte(wram::W_PLAYER_MOVE_LIST_INDEX, 0);

    cpu.borrow_wram_mut().set_list_scroll_offset(0);
    cpu.borrow_wram_mut()
        .set_critical_hit_or_ohko(game_state::CriticalHitOrOhko::NormalAttack);
    cpu.borrow_wram_mut().battle_mon_mut().set_species(None);
    cpu.borrow_wram_mut().set_party_gain_exp_flags(0);
    cpu.borrow_wram_mut().set_player_mon_number(0);
    cpu.borrow_wram_mut().set_escaped_from_battle(false);
    cpu.borrow_wram_mut().set_map_pal_offset(0);

    cpu.borrow_wram_mut().set_player_hp_bar_color(0);
    cpu.borrow_wram_mut().set_enemy_hp_bar_color(0);

    cpu.borrow_wram_mut().set_can_evolve_flags(0);
    cpu.borrow_wram_mut().set_force_evolution(false);
    cpu.write_byte(wram::W_AI_LAYER2_ENCOURAGEMENT, 0);
    cpu.write_byte(0xccd6, 0); // Unused?
    cpu.write_byte(wram::W_PLAYER_SUBSTITUTE_HP, 0);
    cpu.write_byte(wram::W_ENEMY_SUBSTITUTE_HP, 0);
    cpu.write_byte(wram::W_TEST_BATTLE_PLAYER_SELECTED_MOVE, 0);
    cpu.write_byte(0xccda, 0); // Unused?
    cpu.write_byte(wram::W_MOVE_MENU_TYPE, 0);
    cpu.write_byte(wram::W_PLAYER_SELECTED_MOVE, 0);
    cpu.write_byte(wram::W_ENEMY_SELECTED_MOVE, 0);
    cpu.write_byte(wram::W_LINK_BATTLE_RANDOM_NUMBER_LIST_INDEX, 0);
    cpu.write_byte(wram::W_AI_COUNT, 0);
    cpu.write_byte(0xcce0, 0); // Unused?
    cpu.write_byte(0xcce1, 0); // Unused?
    cpu.write_byte(wram::W_ENEMY_MOVE_LIST_INDEX, 0);
    cpu.write_byte(wram::W_LAST_SWITCH_IN_ENEMY_MON_HP, 0);
    cpu.write_byte(wram::W_LAST_SWITCH_IN_ENEMY_MON_HP + 1, 0);
    cpu.borrow_wram_mut().set_total_pay_day_money(0);
    cpu.write_byte(wram::W_SAFARI_ESCAPE_FACTOR, 0);
    cpu.write_byte(wram::W_SAFARI_BAIT_FACTOR, 0);
    cpu.write_byte(0xccea, 0); // Unused?
    cpu.write_byte(wram::W_TRANSFORMED_ENEMY_MON_ORIGINAL_DVS, 0);
    cpu.write_byte(wram::W_TRANSFORMED_ENEMY_MON_ORIGINAL_DVS + 1, 0);
    cpu.write_byte(wram::W_MON_IS_DISOBEDIENT, 0);
    cpu.write_byte(wram::W_PLAYER_DISABLED_MOVE_NUMBER, 0);
    cpu.write_byte(wram::W_ENEMY_DISABLED_MOVE_NUMBER, 0);
    cpu.write_byte(wram::W_IN_HANDLE_PLAYER_MON_FAINTED, 0);
    cpu.write_byte(wram::W_PLAYER_USED_MOVE, 0);
    cpu.write_byte(wram::W_ENEMY_USED_MOVE, 0);
    cpu.write_byte(wram::W_ENEMY_MON_MINIMIZED, 0);
    cpu.write_byte(wram::W_MOVE_DIDNT_MISS, 0);
    cpu.write_byte(wram::W_PARTY_FOUGHT_CURRENT_ENEMY_FLAGS, 0);
    cpu.write_byte(wram::W_LOW_HEALTH_ALARM_DISABLED, 0);
    cpu.write_byte(wram::W_PLAYER_MON_MINIMIZED, 0);
    cpu.write_byte(0xccf8, 0); // Unused?
    cpu.write_byte(0xccf9, 0); // Unused?
    cpu.write_byte(0xccfa, 0); // Unused?
    cpu.write_byte(0xccfb, 0); // Unused?
    cpu.write_byte(0xccfc, 0); // Unused?
    cpu.write_byte(0xccfd, 0); // Unused?
    cpu.write_byte(0xccfe, 0); // Unused?
    cpu.write_byte(0xccff, 0); // Unused?
    cpu.write_byte(0xcd00, 0); // Unused?
    cpu.write_byte(0xcd01, 0); // Unused?
    cpu.write_byte(0xcd02, 0); // Unused?
    cpu.write_byte(0xcd03, 0); // Unused?
    cpu.write_byte(0xcd04, 0); // Unused?
    cpu.write_byte(wram::W_ENEMY_NUM_HITS, 0);
    cpu.write_byte(0xcd06, 0); // Unused?
    cpu.write_byte(0xcd07, 0); // Unused?
    cpu.write_byte(0xcd08, 0); // Unused?
    cpu.write_byte(0xcd09, 0); // Unused?
    cpu.write_byte(0xcd0a, 0); // Unused?
    cpu.write_byte(0xcd0b, 0); // Unused?
    cpu.write_byte(0xcd0c, 0); // Unused?
    cpu.write_byte(0xcd0d, 0); // Unused?
    cpu.write_byte(0xcd0e, 0); // Unused?

    // POUND
    cpu.write_byte(wram::W_TEST_BATTLE_PLAYER_SELECTED_MOVE, 1);

    let cur_map = cpu.read_byte(wram::W_CUR_MAP);

    if cur_map >= map_constants::SAFARI_ZONE_EAST
        && cur_map < map_constants::SAFARI_ZONE_CENTER_REST_HOUSE
    {
        cpu.write_byte(wram::W_BATTLE_TYPE, battle_constants::BATTLE_TYPE_SAFARI);
    }

    macros::farcall::callfar(cpu, 0x02, 0x5064); // PlayBattleMusic
}
