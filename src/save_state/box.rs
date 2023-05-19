use super::{DeterminantValues, PartyPokemon, PokeString, PokemonSpecies};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BoxedPokemon {
    pub species: PokemonSpecies,
    pub hp: u16,
    pub level: u8,
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
    pub ot_name: PokeString,
    pub nickname: Option<PokeString>,
}

impl From<PartyPokemon> for BoxedPokemon {
    fn from(pokemon: PartyPokemon) -> BoxedPokemon {
        BoxedPokemon {
            species: pokemon.species,
            hp: pokemon.hp,
            level: pokemon.level,
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
            ot_name: pokemon.ot_name,
            nickname: pokemon.nickname,
        }
    }
}

pub struct BoxView<'a> {
    data: &'a [u8],
}

impl BoxView<'_> {
    pub fn new(data: &[u8]) -> BoxView<'_> {
        BoxView { data }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn full(&self) -> bool {
        self.data[0] >= 20
    }

    pub fn get(&self, index: usize) -> Option<BoxedPokemon> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            eprintln!("List terminated before expected length");
            return None;
        }

        let offset = 22 + (index * 33);
        let species = PokemonSpecies::from_index(self.data[offset]).unwrap();

        let ot_name = PokeString::from_bytes(&self.data[(682 + (index * 11))..], 11);

        let nickname = PokeString::from_bytes(&self.data[(902 + (index * 11))..], 11);
        let nickname = if nickname == species.name() {
            None
        } else {
            Some(nickname)
        };

        Some(BoxedPokemon {
            species,
            hp: u16::from_be_bytes([self.data[offset + 1], self.data[offset + 2]]),
            level: self.data[offset + 3],
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
            ot_name,
            nickname,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = BoxedPokemon> + '_ {
        BoxIter::new(self)
    }
}

pub struct BoxViewMut<'a> {
    data: &'a mut [u8],
}

impl BoxViewMut<'_> {
    pub fn new(data: &mut [u8]) -> BoxViewMut<'_> {
        BoxViewMut { data }
    }

    pub fn len(&self) -> usize {
        BoxView::new(self.data).len()
    }

    pub fn full(&self) -> bool {
        BoxView::new(self.data).full()
    }

    pub fn get(&self, index: usize) -> Option<BoxedPokemon> {
        BoxView::new(self.data).get(index)
    }

    pub fn set(&mut self, index: usize, pokemon: BoxedPokemon) {
        assert!(index < self.len());

        self.data[1 + index] = pokemon.species.into_index();

        let offset = 22 + (index * 33);
        self.data[offset] = pokemon.species.into_index();
        self.data[offset + 1] = (pokemon.hp >> 8) as u8;
        self.data[offset + 2] = (pokemon.hp & 0xff) as u8;
        self.data[offset + 3] = pokemon.level;
        self.data[offset + 4] = pokemon.status;
        self.data[offset + 5] = pokemon.type1;
        self.data[offset + 6] = pokemon.type2;
        self.data[offset + 7] = pokemon.catch_rate;
        self.data[offset + 8] = pokemon.moves[0];
        self.data[offset + 9] = pokemon.moves[1];
        self.data[offset + 10] = pokemon.moves[2];
        self.data[offset + 11] = pokemon.moves[3];
        self.data[offset + 12] = (pokemon.ot_id >> 8) as u8;
        self.data[offset + 13] = (pokemon.ot_id & 0xff) as u8;
        self.data[offset + 14] = (pokemon.exp >> 16) as u8;
        self.data[offset + 15] = (pokemon.exp >> 8) as u8;
        self.data[offset + 16] = (pokemon.exp & 0xff) as u8;
        self.data[offset + 17] = (pokemon.hp_exp >> 8) as u8;
        self.data[offset + 18] = (pokemon.hp_exp & 0xff) as u8;
        self.data[offset + 19] = (pokemon.attack_exp >> 8) as u8;
        self.data[offset + 20] = (pokemon.attack_exp & 0xff) as u8;
        self.data[offset + 21] = (pokemon.defense_exp >> 8) as u8;
        self.data[offset + 22] = (pokemon.defense_exp & 0xff) as u8;
        self.data[offset + 23] = (pokemon.speed_exp >> 8) as u8;
        self.data[offset + 24] = (pokemon.speed_exp & 0xff) as u8;
        self.data[offset + 25] = (pokemon.special_exp >> 8) as u8;
        self.data[offset + 26] = (pokemon.special_exp & 0xff) as u8;
        self.data[offset + 27] = pokemon.dvs.0;
        self.data[offset + 28] = pokemon.dvs.1;
        self.data[offset + 29] = pokemon.pp[0];
        self.data[offset + 30] = pokemon.pp[1];
        self.data[offset + 31] = pokemon.pp[2];
        self.data[offset + 32] = pokemon.pp[3];

        let mut ot_name_bytes = pokemon.ot_name.iter();
        self.data[682 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[683 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[684 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[685 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[686 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[687 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[688 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[689 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[690 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[691 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[692 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);

        let name = pokemon.nickname.unwrap_or_else(|| pokemon.species.name());
        let mut name_bytes = name.iter();
        self.data[902 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[903 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[904 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[905 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[906 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[907 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[908 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[909 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[910 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[911 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
        self.data[912 + (index * 11)] = name_bytes.next().unwrap_or(0x50);
    }

    pub fn clear(&mut self) {
        self.data[0] = 0;
        self.data[1] = 0xff;
    }

    pub fn push(&mut self, pokemon: BoxedPokemon) {
        let old_len = self.len();
        assert!(old_len < 20);

        self.data[0] += 1;
        self.set(old_len, pokemon);
        self.data[1 + old_len + 1] = 0xff;
    }

    pub fn swap_remove(&mut self, index: usize) -> BoxedPokemon {
        let old_len = self.len();
        assert!(index < old_len);

        let removed = self.get(index).unwrap();
        let new_len = old_len - 1;

        if index < new_len {
            let moved = self.get(new_len).unwrap();
            self.set(index, moved);
        }

        self.data[0] -= 1;
        self.data[1 + new_len] = 0xff;

        removed
    }
}

struct BoxIter<'a> {
    data: &'a BoxView<'a>,
    index: usize,
}

impl<'a> BoxIter<'a> {
    fn new(data: &'a BoxView<'a>) -> BoxIter<'a> {
        BoxIter { data, index: 0 }
    }
}

impl<'a> Iterator for BoxIter<'a> {
    type Item = BoxedPokemon;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.data.get(self.index);
        self.index += 1;
        result
    }
}
