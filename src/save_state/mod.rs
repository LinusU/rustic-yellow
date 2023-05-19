use std::{
    io::{self, Read},
    path::PathBuf,
};

mod r#box;
mod party;
mod species;
mod string;

pub use party::{PartyPokemon, PartyView, PartyViewMut};
pub use r#box::{BoxView, BoxViewMut, BoxedPokemon};
pub use species::PokemonSpecies;
pub use string::PokeString;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DeterminantValues(u8, u8);

impl DeterminantValues {
    fn from_bytes(bytes: [u8; 2]) -> DeterminantValues {
        DeterminantValues(bytes[0], bytes[1])
    }

    fn hp(&self) -> u8 {
        ((self.attack() & 1) << 3)
            | ((self.defense() & 1) << 2)
            | ((self.speed() & 1) << 1)
            | (self.special() & 1)
    }

    fn attack(&self) -> u8 {
        (self.0 & 0b1111_0000) >> 4
    }

    fn defense(&self) -> u8 {
        self.0 & 0b0000_1111
    }

    fn speed(&self) -> u8 {
        (self.1 & 0b1111_0000) >> 4
    }

    fn special(&self) -> u8 {
        self.1 & 0b0000_1111
    }
}

impl std::fmt::Debug for DeterminantValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeterminantValues")
            .field("hp", &self.hp())
            .field("attack", &self.attack())
            .field("defense", &self.defense())
            .field("speed", &self.speed())
            .field("special", &self.special())
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum BoxId {
    Current,
    Box1,
    Box2,
    Box3,
    Box4,
    Box5,
    Box6,
    Box7,
    Box8,
    Box9,
    Box10,
    Box11,
    Box12,
}

impl BoxId {
    pub fn sram_offset(&self) -> usize {
        match self {
            BoxId::Current => 0x30c0,
            BoxId::Box1 => 0x4000,
            BoxId::Box2 => 0x4462,
            BoxId::Box3 => 0x48c4,
            BoxId::Box4 => 0x4d26,
            BoxId::Box5 => 0x5188,
            BoxId::Box6 => 0x55ea,
            BoxId::Box7 => 0x6000,
            BoxId::Box8 => 0x6462,
            BoxId::Box9 => 0x68c4,
            BoxId::Box10 => 0x6d26,
            BoxId::Box11 => 0x7188,
            BoxId::Box12 => 0x75ea,
        }
    }
}

pub struct SaveState {
    data: [u8; 0x8000],
}

impl SaveState {
    pub fn new() -> SaveState {
        SaveState { data: [0; 0x8000] }
    }

    pub fn from_file(path: &PathBuf) -> io::Result<SaveState> {
        let mut file = std::fs::File::open(path)?;
        let mut data = [0; 0x8000];
        file.read_exact(&mut data)?;
        Ok(SaveState { data })
    }

    pub fn write_to_file(&self, path: &PathBuf) -> io::Result<()> {
        std::fs::write(path, &self.data)
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn player_name(&self) -> PokeString {
        PokeString::from_bytes(&self.data[0x2598..], 11)
    }

    pub fn count_badges(&self) -> u32 {
        self.data[0x2602].count_ones()
    }

    pub fn count_owned_mons(&self) -> u32 {
        let mut result = 0;

        for addr in 0x25a3..=0x25b5 {
            result += self.data[addr].count_ones();
        }

        result
    }

    pub fn r#box(&self, id: BoxId) -> BoxView<'_> {
        BoxView::new(&self.data[id.sram_offset()..])
    }

    pub fn box_mut(&mut self, id: BoxId) -> BoxViewMut<'_> {
        BoxViewMut::new(&mut self.data[id.sram_offset()..])
    }
}
