use std::fs::File;
use std::io::BufReader;

use rodio::Decoder;

use crate::game::resources_root;
use crate::sound2::Sfx as SfxTrait;

/// A sound effect that is implemented as a music track.
/// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MusicSfx {
    CaughtPokemon,
    LevelUp,
    ObtainedItem,
    ObtainedKeyItem,
    PokedexRating,
    PokemonEvolved,
}

impl MusicSfx {
    pub fn from_bank_and_id(bank: u8, id: u8) -> Option<MusicSfx> {
        match (bank, id) {
            (0x02, 134) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_1
            (0x02, 137) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_1
            (0x02, 145) => Some(MusicSfx::PokedexRating), // SFX_Pokedex_Rating_1
            (0x02, 148) => Some(MusicSfx::ObtainedKeyItem), // SFX_Get_Key_Item_1
            (0x08, 134) => Some(MusicSfx::LevelUp),      // SFX_Level_Up
            (0x08, 137) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_2
            (0x08, 154) => Some(MusicSfx::CaughtPokemon), // SFX_Caught_Mon
            (0x1f, 134) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_3
            (0x1f, 137) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_3
            (0x1f, 145) => Some(MusicSfx::PokedexRating), // SFX_Pokedex_Rating_3
            (0x1f, 148) => Some(MusicSfx::ObtainedKeyItem), // SFX_Get_Key_Item_3
            (0x20, 134) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_4
            (0x20, 137) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_4
            (0x20, 150) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_4_2

            _ => None,
        }
    }
}

type MusicSfxDecoder = Decoder<BufReader<File>>;

fn open_music_sfx(name: &str) -> MusicSfxDecoder {
    let root = resources_root().unwrap_or(std::env::current_dir().unwrap());
    let file = File::open(root.join("music").join(name)).unwrap();

    Decoder::new(BufReader::new(file)).unwrap()
}

impl SfxTrait<MusicSfxDecoder> for MusicSfx {
    #[rustfmt::skip]
    fn open(self) -> MusicSfxDecoder {
        match self {
            MusicSfx::CaughtPokemon => open_music_sfx("15 - Caught a Pokémon!.flac"),
            MusicSfx::LevelUp => open_music_sfx("10 - Level Up!.flac"),
            MusicSfx::ObtainedItem => open_music_sfx("12 - Obtained an Item!.flac"),
            MusicSfx::ObtainedKeyItem => open_music_sfx("06 - Obtained a Key Item!.flac"),
            MusicSfx::PokedexRating => open_music_sfx("34 - Pokédex Evaluation- You're on Your Way!.flac"),
            MusicSfx::PokemonEvolved => open_music_sfx("33 - Congratulations! Your Pokémon Evolved!.flac"),
        }
    }
}
