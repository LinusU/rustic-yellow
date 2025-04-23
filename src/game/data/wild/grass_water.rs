use std::num::NonZeroU8;

use crate::PokemonSpecies;

#[derive(Debug)]
pub struct WildPokemonDataEntry {
    encounter_rate: NonZeroU8,
    pub mons: [(PokemonSpecies, u8); 10],
}

impl WildPokemonDataEntry {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            encounter_rate: NonZeroU8::new(bytes[0]).unwrap(),
            mons: [
                (PokemonSpecies::from_index(bytes[2]).unwrap(), bytes[1]),
                (PokemonSpecies::from_index(bytes[4]).unwrap(), bytes[3]),
                (PokemonSpecies::from_index(bytes[6]).unwrap(), bytes[5]),
                (PokemonSpecies::from_index(bytes[8]).unwrap(), bytes[7]),
                (PokemonSpecies::from_index(bytes[10]).unwrap(), bytes[9]),
                (PokemonSpecies::from_index(bytes[12]).unwrap(), bytes[11]),
                (PokemonSpecies::from_index(bytes[14]).unwrap(), bytes[13]),
                (PokemonSpecies::from_index(bytes[16]).unwrap(), bytes[15]),
                (PokemonSpecies::from_index(bytes[18]).unwrap(), bytes[17]),
                (PokemonSpecies::from_index(bytes[20]).unwrap(), bytes[19]),
            ],
        }
    }
}

#[derive(Debug)]
pub struct WildPokemonData {
    pub grass: Option<WildPokemonDataEntry>,
    pub water: Option<WildPokemonDataEntry>,
}

impl WildPokemonData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;

        let grass_rate = bytes[offset];

        let grass = if grass_rate != 0 {
            let entry = WildPokemonDataEntry::from_bytes(&bytes[offset..]);
            offset += 21;
            Some(entry)
        } else {
            None
        };

        let water_rate = bytes[offset];

        let water = if water_rate != 0 {
            let entry = WildPokemonDataEntry::from_bytes(&bytes[offset..]);
            Some(entry)
        } else {
            None
        };

        Self { grass, water }
    }

    pub fn grass_encounter_rate(&self) -> u8 {
        self.grass.as_ref().map_or(0, |g| g.encounter_rate.get())
    }

    pub fn water_encounter_rate(&self) -> u8 {
        self.water.as_ref().map_or(0, |w| w.encounter_rate.get())
    }
}
