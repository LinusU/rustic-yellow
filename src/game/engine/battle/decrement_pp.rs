use crate::{
    cpu::{Cpu, CpuFlag},
    game::{constants::move_constants::MoveId, ram::wram},
};

/// after using a move, decrement pp in battle and (if not transformed?) in party \
/// In: de = pointer to the move just used
pub fn decrement_pp(cpu: &mut Cpu) {
    let used_move = cpu.read_byte(cpu.de());

    log::debug!("Decrementing PP for move {}", used_move);

    // if the pokemon is using "struggle", there's nothing to do
    // we don't decrement PP for "struggle"
    if used_move == (MoveId::Struggle as u8) {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    let player_battle_status = cpu.borrow_wram().player_battle_status();

    if player_battle_status.storing_energy()
        || player_battle_status.thrashing_about()
        || player_battle_status.attacking_multiple_times()
        || player_battle_status.using_rage()
    {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    // decrement PP in the battle struct
    decrement_pp_decrement_pp(cpu, wram::W_BATTLE_MON_PP);

    // decrement PP in the party struct
    let player_battle_status = cpu.borrow_wram().player_battle_status();

    // Return if transformed. Pokemon Red stores the "current pokemon's" PP
    // separately from the "Pokemon in your party's" PP.  This is
    // duplication -- in all cases *other* than Pokemon with Transform.
    // Normally, this means we have to go on and make the same
    // modification to the "party's pokemon" PP that we made to the
    // "current pokemon's" PP.  But, if we're dealing with a Transformed
    // Pokemon, it has separate PP for the move set that it copied from
    // its opponent, which is *not* the same as its real PP as part of your
    // party.  So we return, and don't do that part.
    if player_battle_status.transformed() {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    // PP of first move (in party)
    let base = wram::W_PARTY_MON1_PP;

    // which mon in party is active
    let player_mon_number = cpu.borrow_wram().player_mon_number() as u16;

    // calculate address of the mon to modify
    let party_mon_size = wram::W_PARTY_MON2 - wram::W_PARTY_MON1;
    let pp_pointer = base + (player_mon_number * party_mon_size);

    decrement_pp_decrement_pp(cpu, pp_pointer);

    cpu.pc = cpu.stack_pop(); // ret
}

fn decrement_pp_decrement_pp(cpu: &mut Cpu, pp_pointer: u16) {
    // calculate the address in memory of the PP we need to decrement
    // based on the move chosen.
    let addr = pp_pointer + cpu.borrow_wram().player_move_list_index() as u16;

    // Decrement PP
    let value = cpu.read_byte(addr);
    cpu.write_byte(addr, value.wrapping_sub(1));
    cpu.set_flag(CpuFlag::Z, value == 1);
    cpu.set_flag(CpuFlag::H, (value & 0x0f) == 0);
    cpu.set_flag(CpuFlag::N, true);
}
