use crate::{
    cpu::{Cpu, CpuFlag},
    game::macros,
};

/// given an item_id in b
/// set zero flag if item isn't in player's bag
/// else reset zero flag
/// related to Pokémon Tower and ghosts
pub fn is_item_in_bag(cpu: &mut Cpu) {
    let item_id = cpu.b;

    macros::predef::predef_call!(cpu, GetQuantityOfItemInBag);
    log::debug!("is_item_in_bag({:#04x}) == {}", item_id, cpu.b != 0);

    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.pc = cpu.stack_pop(); // ret
}
