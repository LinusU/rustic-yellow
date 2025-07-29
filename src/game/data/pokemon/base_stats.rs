use crate::{game::constants::pokemon_data_constants::GrowthRate, rom::ROM, PokemonSpecies};

const BASE_STATS: usize = (0x0e * 0x4000) | (0x43de & 0x3fff);
const BASE_DATA_SIZE: usize = 28;

impl PokemonSpecies {
    pub fn base_hp(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 1]
    }

    pub fn base_attack(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 2]
    }

    pub fn base_defense(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 3]
    }

    pub fn base_speed(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 4]
    }

    pub fn base_special(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 5]
    }

    pub fn growth_rate(&self) -> GrowthRate {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 19].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_rate() {
        assert_eq!(PokemonSpecies::Arbok.growth_rate(), GrowthRate::MediumFast);
        assert_eq!(PokemonSpecies::Eevee.growth_rate(), GrowthRate::MediumFast);
        assert_eq!(PokemonSpecies::Abra.growth_rate(), GrowthRate::MediumSlow);
        assert_eq!(PokemonSpecies::Gastly.growth_rate(), GrowthRate::MediumSlow);
        assert_eq!(PokemonSpecies::Chansey.growth_rate(), GrowthRate::Fast);
        assert_eq!(PokemonSpecies::Jigglypuff.growth_rate(), GrowthRate::Fast);
        assert_eq!(PokemonSpecies::Aerodactyl.growth_rate(), GrowthRate::Slow);
        assert_eq!(PokemonSpecies::Dragonair.growth_rate(), GrowthRate::Slow);
    }
}
