use crate::{
    cpu::{Cpu, CpuFlag},
    game::{constants::move_constants::MoveId, macros, ram::wram},
    PokemonSpecies,
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

/// set bit 6 of wd472 if any Pikachu with Surf is in party \
/// set bit 7 of wd472 if starter Pikachu is in party (with or without Surf)
///
/// Switches to Bank 0x3f (IsStarterPikachuInOurParty)
pub fn is_surfing_pikachu_in_party(cpu: &mut Cpu) {
    let mut wd472 = cpu.read_byte(wram::W_D472) & 0x3f;

    for mon in cpu.borrow_wram().party().iter() {
        if mon.species == PokemonSpecies::Pikachu && mon.moves.contains(&(MoveId::Surf as u8)) {
            wd472 |= 1 << 6;
        }
    }

    macros::farcall::callfar(cpu, 0x3f, 0x4db8); // IsStarterPikachuInOurParty

    if cpu.flag(CpuFlag::C) {
        wd472 |= 1 << 7;
    }

    cpu.write_byte(wram::W_D472, wd472);

    log::trace!(
        "is_surfing_pikachu_in_party() == surf:{} starter:{}",
        wd472 & (1 << 6) != 0,
        wd472 & (1 << 7) != 0,
    );
}
