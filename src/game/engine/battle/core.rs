use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::{
        item_constants::SILPH_SCOPE,
        map_constants::{POKEMON_TOWER_1F, POKEMON_TOWER_7F},
    },
};

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
        cpu.call(0x3422); // IsItemInBag
        log::debug!("is_item_in_bag(SILPH_SCOPE) == {}", !cpu.flag(CpuFlag::Z));
    }

    log::debug!("is_ghost_battle() == {}", cpu.flag(CpuFlag::Z));
    cpu.pc = cpu.stack_pop(); // ret
}
