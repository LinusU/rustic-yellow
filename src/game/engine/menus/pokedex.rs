use crate::{
    game::{
        constants::pokemon_constants::NUM_POKEMON_INDEXES, data::pokemon::dex_order::POKEDEX_ORDER,
    },
    rom::ROM,
};

/// converts the index number at wd11e to a Pokédex number
pub fn index_to_pokedex(index: u8) -> u8 {
    assert!(index > 0 && index <= NUM_POKEMON_INDEXES);
    ROM[POKEDEX_ORDER + (index as usize - 1)]
}
