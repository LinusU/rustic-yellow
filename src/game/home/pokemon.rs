use crate::{cpu::Cpu, game::data::pokemon::cries, save_state::PokemonSpecies};

pub fn play_cry(cpu: &mut Cpu, species: PokemonSpecies) {
    cpu.play_sfx(cries::cry_data(species));
}
