use std::fs::File;
use std::io::BufReader;

use rodio::Decoder;

use crate::game::resources_root;
use crate::sound2::Sfx as SfxTrait;

/// A sound effect that is implemented as a music track.
/// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MusicSfx {
    Battle35_00_80,
    Battle35_11_18,
    Battle35_42_01,
    CaughtPokemon,
    LevelUp,
    ObtainedItem,
    ObtainedKeyItem,
    PokedexRating,
    Pokeflute,
    PokemonEvolved,
}

impl MusicSfx {
    pub fn from_bank_and_id(bank: u8, id: u8, pitch: u8, length: i8) -> Option<MusicSfx> {
        match (bank, id, pitch, length) {
            (0x02, 134, _, _) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_1
            (0x02, 137, _, _) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_1
            (0x02, 145, _, _) => Some(MusicSfx::PokedexRating), // SFX_Pokedex_Rating_1
            (0x02, 148, _, _) => Some(MusicSfx::ObtainedKeyItem), // SFX_Get_Key_Item_1
            (0x02, 184, _, _) => Some(MusicSfx::Pokeflute),    // SFX_Pokeflute
            (0x08, 134, _, _) => Some(MusicSfx::LevelUp),      // SFX_Level_Up
            (0x08, 137, _, _) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_2
            (0x08, 154, _, _) => Some(MusicSfx::CaughtPokemon), // SFX_Caught_Mon
            (0x1f, 134, _, _) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_3
            (0x1f, 137, _, _) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_3
            (0x1f, 145, _, _) => Some(MusicSfx::PokedexRating), // SFX_Pokedex_Rating_3
            (0x1f, 148, _, _) => Some(MusicSfx::ObtainedKeyItem), // SFX_Get_Key_Item_3
            (0x20, 134, _, _) => Some(MusicSfx::ObtainedItem), // SFX_Get_Item1_4
            (0x20, 137, _, _) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_4
            (0x20, 150, _, _) => Some(MusicSfx::PokemonEvolved), // SFX_Get_Item2_4_2

            (0x08, 228, 0x00, -128) => Some(MusicSfx::Battle35_00_80), // SING
            (0x08, 228, 0x11, 0x18) => Some(MusicSfx::Battle35_11_18), // HYPNOSIS
            (0x08, 228, 0x42, 0x01) => Some(MusicSfx::Battle35_42_01), // DREAM_EATER

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
            MusicSfx::Battle35_00_80 => open_music_sfx("SFX_Battle_35_00_80.flac"),
            MusicSfx::Battle35_11_18 => open_music_sfx("SFX_Battle_35_11_18.flac"),
            MusicSfx::Battle35_42_01 => open_music_sfx("SFX_Battle_35_42_01.flac"),
            MusicSfx::CaughtPokemon => open_music_sfx("15 - Caught a Pokémon!.flac"),
            MusicSfx::LevelUp => open_music_sfx("10 - Level Up!.flac"),
            MusicSfx::ObtainedItem => open_music_sfx("12 - Obtained an Item!.flac"),
            MusicSfx::ObtainedKeyItem => open_music_sfx("06 - Obtained a Key Item!.flac"),
            MusicSfx::PokedexRating => open_music_sfx("34 - Pokédex Evaluation- You're on Your Way!.flac"),
            MusicSfx::Pokeflute => open_music_sfx("SFX_Pokeflute.flac"),
            MusicSfx::PokemonEvolved => open_music_sfx("33 - Congratulations! Your Pokémon Evolved!.flac"),
        }
    }
}
