use crate::{
    cpu::Cpu,
    game::{constants::battle_constants::NUM_MOVES, data::moves::moves::MOVES, ram::wram},
};

/// Restore HP and PP
pub fn heal_party(cpu: &mut Cpu) {
    log::info!("heal_party()");

    for i in 0usize.. {
        let Some(mut pokemon) = cpu.borrow_wram().party().get(i) else {
            break;
        };

        pokemon.status = 0;
        pokemon.hp = pokemon.max_hp;

        // Reset move PPs
        for move_idx in 0..(NUM_MOVES as usize) {
            if pokemon.moves[move_idx] == 0 {
                break;
            }

            let old_pp = pokemon.pp[move_idx];
            let move_pp = MOVES[(pokemon.moves[move_idx] as usize) - 1].pp;

            pokemon.pp[move_idx] = (old_pp & 0xc0) + move_pp;
        }

        cpu.borrow_wram_mut().party_mut().set(i, pokemon);

        cpu.write_byte(wram::W_D11E, 0);
        cpu.write_byte(wram::W_WHICH_POKEMON, i as u8);
        cpu.call(0x654a); // RestoreBonusPP
    }

    cpu.pc = cpu.stack_pop(); // ret
}
