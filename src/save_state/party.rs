use crate::rom::ROM;

use super::{BoxedPokemon, DeterminantValues, PokeString, PokemonSpecies};

trait PokemonSpeciesStats {
    fn base_hp(&self) -> u8;
    fn base_attack(&self) -> u8;
    fn base_defense(&self) -> u8;
    fn base_speed(&self) -> u8;
    fn base_special(&self) -> u8;
    fn growth_rate(&self) -> u8;
}

const BASE_STATS: usize = 0x0383de;
const BASE_DATA_SIZE: usize = 28;

impl PokemonSpeciesStats for PokemonSpecies {
    fn base_hp(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 1]
    }

    fn base_attack(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 2]
    }

    fn base_defense(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 3]
    }

    fn base_speed(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 4]
    }

    fn base_special(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 5]
    }

    fn growth_rate(&self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (*self as usize - 1)) + 19]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PartyPokemon {
    pub species: PokemonSpecies,
    pub hp: u16,
    pub box_level: u8,
    pub status: u8,
    pub type1: u8,
    pub type2: u8,
    pub catch_rate: u8,
    pub moves: [u8; 4],
    pub ot_id: u16,
    pub exp: u32,
    pub hp_exp: u16,
    pub attack_exp: u16,
    pub defense_exp: u16,
    pub speed_exp: u16,
    pub special_exp: u16,
    pub dvs: DeterminantValues,
    pub pp: [u8; 4],
    pub level: u8,
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special: u16,
    pub ot_name: PokeString,
    pub nickname: Option<PokeString>,
}

impl From<BoxedPokemon> for PartyPokemon {
    fn from(pokemon: BoxedPokemon) -> PartyPokemon {
        let mut level = 1;
        loop {
            level += 1;

            let exp_req = match pokemon.species.growth_rate() {
                // GROWTH_MEDIUM_FAST
                0 => level * level * level,
                // GROWTH_SLIGHTLY_FAST
                1 => unimplemented!(),
                // GROWTH_SLIGHTLY_SLOW
                2 => unimplemented!(),
                // GROWTH_MEDIUM_SLOW
                3 => (6 * level * level * level / 5) + (100 * level) - (15 * level * level) - 140,
                // GROWTH_FAST
                4 => 4 * level * level * level / 5,
                // GROWTH_SLOW
                5 => 5 * level * level * level / 4,
                _ => unreachable!(),
            };

            if exp_req > pokemon.exp {
                level -= 1;
                break;
            }

            if level == 100 {
                break;
            }
        }

        let level = level as u8;

        // HP: (((Base + IV) * 2 + ceil(Sqrt(stat exp)) / 4) * Level) / 100 + Level + 10
        let max_hp = (((pokemon.species.base_hp() as u16 + pokemon.dvs.hp() as u16) * 2
            + (pokemon.hp_exp as f32).sqrt().ceil() as u16 / 4)
            * level as u16)
            / 100
            + level as u16
            + 10;

        // non-HP: (((Base + IV) * 2 + ceil(Sqrt(stat exp)) / 4) * Level) / 100 + 5
        let attack = (((pokemon.species.base_attack() as u16 + pokemon.dvs.attack() as u16) * 2
            + (pokemon.attack_exp as f32).sqrt().ceil() as u16 / 4)
            * level as u16)
            / 100
            + 5;
        let defense = (((pokemon.species.base_defense() as u16 + pokemon.dvs.defense() as u16)
            * 2
            + (pokemon.defense_exp as f32).sqrt().ceil() as u16 / 4)
            * level as u16)
            / 100
            + 5;
        let speed = (((pokemon.species.base_speed() as u16 + pokemon.dvs.speed() as u16) * 2
            + (pokemon.speed_exp as f32).sqrt().ceil() as u16 / 4)
            * level as u16)
            / 100
            + 5;
        let special = (((pokemon.species.base_special() as u16 + pokemon.dvs.special() as u16)
            * 2
            + (pokemon.special_exp as f32).sqrt().ceil() as u16 / 4)
            * level as u16)
            / 100
            + 5;

        PartyPokemon {
            species: pokemon.species,
            hp: pokemon.hp,
            box_level: pokemon.level,
            status: pokemon.status,
            type1: pokemon.type1,
            type2: pokemon.type2,
            catch_rate: pokemon.catch_rate,
            moves: pokemon.moves,
            ot_id: pokemon.ot_id,
            exp: pokemon.exp,
            hp_exp: pokemon.hp_exp,
            attack_exp: pokemon.attack_exp,
            defense_exp: pokemon.defense_exp,
            speed_exp: pokemon.speed_exp,
            special_exp: pokemon.special_exp,
            dvs: pokemon.dvs,
            pp: pokemon.pp,
            level,
            max_hp,
            attack,
            defense,
            speed,
            special,
            ot_name: pokemon.ot_name,
            nickname: pokemon.nickname,
        }
    }
}

pub struct PartyView<'a> {
    data: &'a [u8],
}

impl PartyView<'_> {
    pub fn new(data: &[u8]) -> PartyView<'_> {
        PartyView { data }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn get(&self, index: usize) -> Option<PartyPokemon> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            log::error!("List terminated before expected length");
            return None;
        }

        let offset = 8 + (index * 44);
        let species = PokemonSpecies::from_index(self.data[offset]).unwrap();

        let ot_name = PokeString::from_bytes(&self.data[272 + (index * 11)..], 11);

        let nickname = PokeString::from_bytes(&self.data[338 + (index * 11)..], 11);
        let nickname = if nickname == species.name() {
            None
        } else {
            Some(nickname)
        };

        Some(PartyPokemon {
            species,
            hp: u16::from_be_bytes([self.data[offset + 1], self.data[offset + 2]]),
            box_level: self.data[offset + 3],
            status: self.data[offset + 4],
            type1: self.data[offset + 5],
            type2: self.data[offset + 6],
            catch_rate: self.data[offset + 7],
            moves: [
                self.data[offset + 8],
                self.data[offset + 9],
                self.data[offset + 10],
                self.data[offset + 11],
            ],
            ot_id: u16::from_be_bytes([self.data[offset + 12], self.data[offset + 13]]),
            exp: u32::from_be_bytes([
                0,
                self.data[offset + 14],
                self.data[offset + 15],
                self.data[offset + 16],
            ]),
            hp_exp: u16::from_be_bytes([self.data[offset + 17], self.data[offset + 18]]),
            attack_exp: u16::from_be_bytes([self.data[offset + 19], self.data[offset + 20]]),
            defense_exp: u16::from_be_bytes([self.data[offset + 21], self.data[offset + 22]]),
            speed_exp: u16::from_be_bytes([self.data[offset + 23], self.data[offset + 24]]),
            special_exp: u16::from_be_bytes([self.data[offset + 25], self.data[offset + 26]]),
            dvs: DeterminantValues::from_bytes([self.data[offset + 27], self.data[offset + 28]]),
            pp: [
                self.data[offset + 29],
                self.data[offset + 30],
                self.data[offset + 31],
                self.data[offset + 32],
            ],
            level: self.data[offset + 33],
            max_hp: u16::from_be_bytes([self.data[offset + 34], self.data[offset + 35]]),
            attack: u16::from_be_bytes([self.data[offset + 36], self.data[offset + 37]]),
            defense: u16::from_be_bytes([self.data[offset + 38], self.data[offset + 39]]),
            speed: u16::from_be_bytes([self.data[offset + 40], self.data[offset + 41]]),
            special: u16::from_be_bytes([self.data[offset + 42], self.data[offset + 43]]),
            ot_name,
            nickname,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = PartyPokemon> + '_ {
        PartyIter::new(self)
    }
}

pub struct PartyViewMut<'a> {
    data: &'a mut [u8],
}

impl PartyViewMut<'_> {
    pub fn new(data: &mut [u8]) -> PartyViewMut<'_> {
        PartyViewMut { data }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn get(&self, index: usize) -> Option<PartyPokemon> {
        PartyView::new(&self.data).get(index)
    }

    pub fn set(&mut self, index: usize, pokemon: PartyPokemon) {
        assert!(index < self.len());

        self.data[1 + index] = pokemon.species.into_index();
        self.data[8 + (index * 44)] = pokemon.species.into_index();
        self.data[9 + (index * 44)] = (pokemon.hp >> 8) as u8;
        self.data[10 + (index * 44)] = (pokemon.hp & 0xff) as u8;
        self.data[11 + (index * 44)] = pokemon.box_level;
        self.data[12 + (index * 44)] = pokemon.status;
        self.data[13 + (index * 44)] = pokemon.type1;
        self.data[14 + (index * 44)] = pokemon.type2;
        self.data[15 + (index * 44)] = pokemon.catch_rate;
        self.data[16 + (index * 44)] = pokemon.moves[0];
        self.data[17 + (index * 44)] = pokemon.moves[1];
        self.data[18 + (index * 44)] = pokemon.moves[2];
        self.data[19 + (index * 44)] = pokemon.moves[3];
        self.data[20 + (index * 44)] = (pokemon.ot_id >> 8) as u8;
        self.data[21 + (index * 44)] = (pokemon.ot_id & 0xff) as u8;
        self.data[22 + (index * 44)] = (pokemon.exp >> 16) as u8;
        self.data[23 + (index * 44)] = (pokemon.exp >> 8) as u8;
        self.data[24 + (index * 44)] = (pokemon.exp & 0xff) as u8;
        self.data[25 + (index * 44)] = (pokemon.hp_exp >> 8) as u8;
        self.data[26 + (index * 44)] = (pokemon.hp_exp & 0xff) as u8;
        self.data[27 + (index * 44)] = (pokemon.attack_exp >> 8) as u8;
        self.data[28 + (index * 44)] = (pokemon.attack_exp & 0xff) as u8;
        self.data[29 + (index * 44)] = (pokemon.defense_exp >> 8) as u8;
        self.data[30 + (index * 44)] = (pokemon.defense_exp & 0xff) as u8;
        self.data[31 + (index * 44)] = (pokemon.speed_exp >> 8) as u8;
        self.data[32 + (index * 44)] = (pokemon.speed_exp & 0xff) as u8;
        self.data[33 + (index * 44)] = (pokemon.special_exp >> 8) as u8;
        self.data[34 + (index * 44)] = (pokemon.special_exp & 0xff) as u8;
        self.data[35 + (index * 44)] = pokemon.dvs.0;
        self.data[36 + (index * 44)] = pokemon.dvs.1;
        self.data[37 + (index * 44)] = pokemon.pp[0];
        self.data[38 + (index * 44)] = pokemon.pp[1];
        self.data[39 + (index * 44)] = pokemon.pp[2];
        self.data[40 + (index * 44)] = pokemon.pp[3];
        self.data[41 + (index * 44)] = pokemon.level;
        self.data[42 + (index * 44)] = (pokemon.max_hp >> 8) as u8;
        self.data[43 + (index * 44)] = (pokemon.max_hp & 0xff) as u8;
        self.data[44 + (index * 44)] = (pokemon.attack >> 8) as u8;
        self.data[45 + (index * 44)] = (pokemon.attack & 0xff) as u8;
        self.data[46 + (index * 44)] = (pokemon.defense >> 8) as u8;
        self.data[47 + (index * 44)] = (pokemon.defense & 0xff) as u8;
        self.data[48 + (index * 44)] = (pokemon.speed >> 8) as u8;
        self.data[49 + (index * 44)] = (pokemon.speed & 0xff) as u8;
        self.data[50 + (index * 44)] = (pokemon.special >> 8) as u8;
        self.data[51 + (index * 44)] = (pokemon.special & 0xff) as u8;

        let mut ot_name_bytes = pokemon.ot_name.iter();
        self.data[272 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[273 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[274 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[275 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[276 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[277 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[278 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[279 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[280 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[281 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[282 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);

        let name = pokemon.nickname.unwrap_or_else(|| pokemon.species.name());
        let mut name_bytes = name.iter();
        self.data[338 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[339 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[340 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[341 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[342 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[343 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[344 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[345 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[346 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[347 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[348 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
    }

    pub fn push(&mut self, pokemon: PartyPokemon) {
        assert!(self.data[0] < 6);
        let index = self.data[0] as usize;
        self.data[0] += 1;
        self.set(index, pokemon);
    }

    pub fn remove(&mut self, index: usize) -> PartyPokemon {
        let prev_len = self.data[0] as usize;
        assert!(index < prev_len);

        let pokemon = self.get(index).unwrap();

        // Move all the pokemon after the removed up one slot
        for i in (index + 1)..prev_len {
            self.set(i - 1, self.get(i).unwrap());
        }

        let next_len = prev_len - 1;

        self.data[0] = next_len as u8;
        self.data[1 + next_len] = 0xff;
        pokemon
    }
}

struct PartyIter<'a> {
    data: &'a PartyView<'a>,
    index: usize,
}

impl<'a> PartyIter<'a> {
    fn new(data: &'a PartyView<'a>) -> PartyIter<'a> {
        PartyIter { data, index: 0 }
    }
}

impl<'a> Iterator for PartyIter<'a> {
    type Item = PartyPokemon;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.index);
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_boxed_pokemon() {
        let ot_name = PokeString::from_bytes(&[0x80, 0x50], 2);

        assert_eq!(
            PartyPokemon::from(BoxedPokemon {
                species: PokemonSpecies::Haunter,
                hp: 0,
                level: 33,
                status: 0,
                type1: 8,
                type2: 3,
                catch_rate: 90,
                moves: [92, 95, 101, 102],
                ot_id: 35244,
                exp: 38268,
                hp_exp: 7502,
                attack_exp: 2973,
                defense_exp: 5593,
                speed_exp: 8007,
                special_exp: 7849,
                dvs: DeterminantValues::from_bytes([(7 << 4) | 13, (10 << 4) | 2]),
                pp: [8, 19, 64, 10],
                ot_name: ot_name.clone(),
                nickname: None,
            }),
            PartyPokemon {
                species: PokemonSpecies::Haunter,
                hp: 0,
                box_level: 33,
                status: 0,
                type1: 8,
                type2: 3,
                catch_rate: 90,
                moves: [92, 95, 101, 102],
                ot_id: 35244,
                exp: 38268,
                hp_exp: 7502,
                attack_exp: 2973,
                defense_exp: 5593,
                speed_exp: 8007,
                special_exp: 7849,
                dvs: DeterminantValues::from_bytes([(7 << 4) | 13, (10 << 4) | 2]),
                pp: [8, 19, 64, 10],
                level: 35,
                max_hp: 92,
                attack: 49,
                defense: 51,
                speed: 86,
                special: 94,
                ot_name: ot_name.clone(),
                nickname: None,
            }
        );

        assert_eq!(
            PartyPokemon::from(BoxedPokemon {
                species: PokemonSpecies::Venonat,
                hp: 0,
                level: 17,
                status: 0,
                type1: 7,
                type2: 3,
                catch_rate: 190,
                moves: [33, 50, 48, 0],
                ot_id: 35244,
                exp: 5295,
                hp_exp: 126,
                attack_exp: 157,
                defense_exp: 110,
                speed_exp: 161,
                special_exp: 104,
                dvs: DeterminantValues::from_bytes([(4 << 4) | 15, (13 << 4) | 5]),
                pp: [33, 20, 20, 0],
                ot_name: ot_name.clone(),
                nickname: None,
            }),
            PartyPokemon {
                species: PokemonSpecies::Venonat,
                hp: 0,
                box_level: 17,
                status: 0,
                type1: 7,
                type2: 3,
                catch_rate: 190,
                moves: [33, 50, 48, 0],
                ot_id: 35244,
                exp: 5295,
                hp_exp: 126,
                attack_exp: 157,
                defense_exp: 110,
                speed_exp: 161,
                special_exp: 104,
                dvs: DeterminantValues::from_bytes([(4 << 4) | 15, (13 << 4) | 5]),
                pp: [33, 20, 20, 0],
                level: 17,
                max_hp: 50,
                attack: 25,
                defense: 27,
                speed: 25,
                special: 20,
                ot_name: ot_name.clone(),
                nickname: None,
            },
        );

        assert_eq!(
            PartyPokemon::from(BoxedPokemon {
                species: PokemonSpecies::Kadabra,
                hp: 0,
                level: 49,
                status: 0,
                type1: 24,
                type2: 24,
                catch_rate: 200,
                moves: [94, 148, 25, 60],
                ot_id: 35244,
                exp: 118745,
                hp_exp: 17041,
                attack_exp: 14599,
                defense_exp: 14701,
                speed_exp: 14418,
                special_exp: 13767,
                dvs: DeterminantValues::from_bytes([(7 << 4) | 11, (9 << 4) | 9]),
                pp: [10, 20, 4, 20],
                ot_name: ot_name.clone(),
                nickname: None,
            }),
            PartyPokemon {
                species: PokemonSpecies::Kadabra,
                hp: 0,
                box_level: 49,
                status: 0,
                type1: 24,
                type2: 24,
                catch_rate: 200,
                moves: [94, 148, 25, 60],
                ot_id: 35244,
                exp: 118745,
                hp_exp: 17041,
                attack_exp: 14599,
                defense_exp: 14701,
                speed_exp: 14418,
                special_exp: 13767,
                dvs: DeterminantValues::from_bytes([(7 << 4) | 11, (9 << 4) | 9]),
                pp: [10, 20, 4, 20],
                level: 50,
                max_hp: 131,
                attack: 62,
                defense: 61,
                speed: 134,
                special: 148,
                ot_name: ot_name.clone(),
                nickname: None,
            },
        );

        assert_eq!(
            PartyPokemon::from(BoxedPokemon {
                species: PokemonSpecies::Nidoqueen,
                hp: 156,
                level: 0,
                status: 0,
                type1: 3,
                type2: 4,
                catch_rate: 235,
                moves: [57, 34, 38, 70],
                ot_id: 35244,
                exp: 100845,
                hp_exp: 12370,
                attack_exp: 15256,
                defense_exp: 13881,
                speed_exp: 14838,
                special_exp: 12393,
                dvs: DeterminantValues::from_bytes([(4 << 4) | 10, (7 << 4) | 12]),
                pp: [15, 15, 15, 15],
                ot_name: ot_name.clone(),
                nickname: None,
            }),
            PartyPokemon {
                species: PokemonSpecies::Nidoqueen,
                hp: 156,
                box_level: 0,
                status: 0,
                type1: 3,
                type2: 4,
                catch_rate: 235,
                moves: [57, 34, 38, 70],
                ot_id: 35244,
                exp: 100845,
                hp_exp: 12370,
                attack_exp: 15256,
                defense_exp: 13881,
                speed_exp: 14838,
                special_exp: 12393,
                dvs: DeterminantValues::from_bytes([(4 << 4) | 10, (7 << 4) | 12]),
                pp: [15, 15, 15, 15],
                level: 47,
                max_hp: 156,
                attack: 100,
                defense: 109,
                speed: 97,
                special: 99,
                ot_name: ot_name.clone(),
                nickname: None,
            },
        );

        assert_eq!(
            PartyPokemon::from(BoxedPokemon {
                species: PokemonSpecies::Charizard,
                hp: 92,
                level: 25,
                status: 0,
                type1: 20,
                type2: 2,
                catch_rate: 45,
                moves: [130, 15, 53, 19],
                ot_id: 35244,
                exp: 105177,
                hp_exp: 13504,
                attack_exp: 19099,
                defense_exp: 11840,
                speed_exp: 14029,
                special_exp: 12773,
                dvs: DeterminantValues::from_bytes([(11 << 4) | 9, (13 << 4) | 11]),
                pp: [15, 30, 15, 15],
                ot_name: ot_name.clone(),
                nickname: None,
            }),
            PartyPokemon {
                species: PokemonSpecies::Charizard,
                hp: 92,
                box_level: 25,
                status: 0,
                type1: 20,
                type2: 2,
                catch_rate: 45,
                moves: [130, 15, 53, 19],
                ot_id: 35244,
                exp: 105177,
                hp_exp: 13504,
                attack_exp: 19099,
                defense_exp: 11840,
                speed_exp: 14029,
                special_exp: 12773,
                dvs: DeterminantValues::from_bytes([(11 << 4) | 9, (13 << 4) | 11]),
                pp: [15, 30, 15, 15],
                level: 48,
                max_hp: 161,
                attack: 112,
                defense: 101,
                speed: 127,
                special: 110,
                ot_name: ot_name.clone(),
                nickname: None,
            },
        );
    }
}
