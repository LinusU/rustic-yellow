use crate::{
    cpu::Cpu,
    game::ram::wram,
    save_state::{BoxedPokemon, PokemonSpecies},
};

pub fn is_this_partymon_starter_pikachu(cpu: &mut Cpu, pokemon: &BoxedPokemon) -> bool {
    if pokemon.species != PokemonSpecies::Pikachu {
        return false;
    }

    let player_id = u16::from_be_bytes([
        cpu.read_byte(wram::W_PLAYER_ID),
        cpu.read_byte(wram::W_PLAYER_ID + 1),
    ]);

    if pokemon.ot_id != player_id {
        return false;
    }

    let mut name_bytes = pokemon.ot_name.iter();

    // The original game only compares the first 6 letters
    for i in 0..6 {
        if name_bytes.next().unwrap_or(0x50) != cpu.read_byte(wram::W_PLAYER_NAME + i) {
            return false;
        }
    }

    true
}
