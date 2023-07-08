use std::fs::File;
use std::io::BufReader;

use rodio::decoder::LoopedDecoder;
use rodio::Decoder;

use crate::game::resources_root;
use crate::sound2::{Music as MusicTrait, Sfx as SfxTrait};

type MusicDecoder = LoopedDecoder<BufReader<File>>;

fn open_music(name: &str) -> MusicDecoder {
    let root = resources_root().unwrap_or(std::env::current_dir().unwrap());
    let file = File::open(root.join("music").join(name)).unwrap();

    Decoder::new_looped(BufReader::new(file)).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Music {
    PalletTown,
    Pokecenter,
    Gym,
    Cities1,
    Cities2,
    Celadon,
    Cinnabar,
    Vermilion,
    Lavender,
    SSAnne,
    MeetProfOak,
    MeetRival,
    MuseumGuy,
    SafariZone,
    PkmnHealed,
    Routes1,
    Routes2,
    Routes3,
    Routes4,
    IndigoPlateau,
    GymLeaderBattle,
    TrainerBattle,
    WildBattle,
    FinalBattle,
    DefeatedTrainer,
    DefeatedWildMon,
    DefeatedGymLeader,
    TitleScreen,
    Credits,
    HallOfFame,
    OaksLab,
    JigglypuffSong,
    BikeRiding,
    Surfing,
    GameCorner,
    YellowIntro,
    Dungeon1,
    Dungeon2,
    Dungeon3,
    CinnabarMansion,
    PokemonTower,
    SilphCo,
    MeetEvilTrainer,
    MeetFemaleTrainer,
    MeetMaleTrainer,
    SurfingPikachu,
    MeetJessieJames,
    YellowUnusedSong,
    GBPrinter,
}

impl Music {
    pub fn from_bank_and_id(bank: u8, id: u8) -> Option<Music> {
        match (bank, id) {
            (0x02, 186) => Some(Music::PalletTown),
            (0x02, 189) => Some(Music::Pokecenter),
            (0x02, 192) => Some(Music::Gym),
            (0x02, 195) => Some(Music::Cities1),
            (0x02, 199) => Some(Music::Cities2),
            (0x02, 202) => Some(Music::Celadon),
            (0x02, 205) => Some(Music::Cinnabar),
            (0x02, 208) => Some(Music::Vermilion),
            (0x02, 212) => Some(Music::Lavender),
            (0x02, 216) => Some(Music::SSAnne),
            (0x02, 219) => Some(Music::MeetProfOak),
            (0x02, 222) => Some(Music::MeetRival),
            (0x02, 225) => Some(Music::MuseumGuy),
            (0x02, 229) => Some(Music::SafariZone),
            (0x02, 232) => Some(Music::PkmnHealed),
            (0x02, 235) => Some(Music::Routes1),
            (0x02, 239) => Some(Music::Routes2),
            (0x02, 243) => Some(Music::Routes3),
            (0x02, 247) => Some(Music::Routes4),
            (0x02, 251) => Some(Music::IndigoPlateau),

            (0x08, 234) => Some(Music::GymLeaderBattle),
            (0x08, 237) => Some(Music::TrainerBattle),
            (0x08, 240) => Some(Music::WildBattle),
            (0x08, 243) => Some(Music::FinalBattle),
            (0x08, 246) => Some(Music::DefeatedTrainer),
            (0x08, 249) => Some(Music::DefeatedWildMon),
            (0x08, 252) => Some(Music::DefeatedGymLeader),

            (0x1f, 195) => Some(Music::TitleScreen),
            (0x1f, 199) => Some(Music::Credits),
            (0x1f, 202) => Some(Music::HallOfFame),
            (0x1f, 205) => Some(Music::OaksLab),
            (0x1f, 208) => Some(Music::JigglypuffSong),
            (0x1f, 210) => Some(Music::BikeRiding),
            (0x1f, 214) => Some(Music::Surfing),
            (0x1f, 217) => Some(Music::GameCorner),
            (0x1f, 220) => Some(Music::YellowIntro),
            (0x1f, 223) => Some(Music::Dungeon1),
            (0x1f, 227) => Some(Music::Dungeon2),
            (0x1f, 231) => Some(Music::Dungeon3),
            (0x1f, 235) => Some(Music::CinnabarMansion),
            (0x1f, 239) => Some(Music::PokemonTower),
            (0x1f, 242) => Some(Music::SilphCo),
            (0x1f, 245) => Some(Music::MeetEvilTrainer),
            (0x1f, 248) => Some(Music::MeetFemaleTrainer),
            (0x1f, 251) => Some(Music::MeetMaleTrainer),

            (0x20, 153) => Some(Music::SurfingPikachu),
            (0x20, 156) => Some(Music::MeetJessieJames),
            (0x20, 159) => Some(Music::YellowUnusedSong),
            (0x20, 163) => Some(Music::GBPrinter),

            _ => None,
        }
    }
}

impl SfxTrait<MusicDecoder> for Music {
    #[rustfmt::skip]
    fn open(self) -> MusicDecoder {
        match self {
            // Bank 02
            Music::PalletTown => open_music("03 - Pallet Town.flac"),
            Music::Pokecenter => open_music("17 - Pokémon Center.flac"),
            Music::Gym => open_music("23 - Pokémon Gym.flac"),
            Music::Cities1 => open_music("16 - Pewter City.flac"),
            Music::Cities2 => open_music("30 - Cerulean City.flac"),
            Music::Celadon => open_music("41 - Celadon City.flac"),
            Music::Cinnabar => open_music("47 - Cinnabar Island.flac"),
            Music::Vermilion => open_music("35 - Vermilion City.flac"),
            Music::Lavender => open_music("39 - Lavender Town.flac"),
            Music::SSAnne => open_music("36 - S.S. Anne.flac"),
            Music::MeetProfOak => open_music("04 - Professor Oak.flac"),
            Music::MeetRival => open_music("07 - Rival.flac"),
            Music::MuseumGuy => open_music("21 - Hurry Along.flac"),
            Music::SafariZone => open_music("32 - Evolution.flac"),
            Music::PkmnHealed => open_music("18 - Pokémon Healed.flac"),
            Music::Routes1 => open_music("11 - Route 1.flac"),
            Music::Routes2 => open_music("31 - Route 24.flac"),
            Music::Routes3 => open_music("27 - Route 3.flac"),
            Music::Routes4 => open_music("38 - Route 11.flac"),
            Music::IndigoPlateau => open_music("49 - Victory Road.flac"),
            // Bank 08
            Music::GymLeaderBattle => open_music("25 - Battle! (Gym Leader).flac"),
            Music::TrainerBattle => open_music("08 - Battle! (Trainer).flac"),
            Music::WildBattle => open_music("13 - Battle! (Wild Pokémon).flac"),
            Music::FinalBattle => open_music("50 - Final Battle! (Rival).flac"),
            Music::DefeatedTrainer => open_music("09 - Victory! (Trainer).flac"),
            Music::DefeatedWildMon => open_music("14 - Victory! (Wild Pokémon).flac"),
            Music::DefeatedGymLeader => open_music("26 - Victory! (Gym Leader).flac"),
            // Bank 1f
            Music::TitleScreen => open_music("02 - Title Screen (Yellow).flac"),
            Music::Credits => open_music("52 - Ending.flac"),
            Music::HallOfFame => open_music("51 - Hall of Fame.flac"),
            Music::OaksLab => open_music("05 - Oak Pokémon Lab.flac"),
            Music::JigglypuffSong => open_music("22 - Jigglypuff's Song.flac"),
            Music::BikeRiding => open_music("37 - Cycling.flac"),
            Music::Surfing => open_music("46 - Surf.flac"),
            Music::GameCorner => open_music("42 - Game Corner.flac"),
            Music::YellowIntro => open_music("01 - Opening Movie (Yellow).flac"),
            Music::Dungeon1 => open_music("43 - Rocket Hideout.flac"),
            Music::Dungeon2 => open_music("19 - Viridian Forest.flac"),
            Music::Dungeon3 => open_music("29 - Mt. Moon.flac"),
            Music::CinnabarMansion => open_music("48 - Pokémon Mansion.flac"),
            Music::PokemonTower => open_music("40 - Pokémon Tower.flac"),
            Music::SilphCo => open_music("45 - Silph Co..flac"),
            Music::MeetEvilTrainer => open_music("44 - Trainers' Eyes Meet (Team Rocket).flac"),
            Music::MeetFemaleTrainer => open_music("28 - Trainers' Eyes Meet (Girl).flac"),
            Music::MeetMaleTrainer => open_music("20 - Trainers' Eyes Meet (Boy).flac"),
            // Bank 20
            Music::SurfingPikachu => open_music("04 - Pikachu's Beach.flac"),
            Music::MeetJessieJames => open_music("03 - Jessie & James.flac"),
            Music::YellowUnusedSong => open_music("05 - Giovanni [Hidden Track].flac"),

            // The Printer Menu track isn't part of the Soundtrack CD from the Internet Archive, so I'm using the hidden track from the CD instead
            Music::GBPrinter => open_music("05 - Giovanni [Hidden Track].flac"),
        }
    }
}

impl MusicTrait<MusicDecoder> for Music {
    fn id(&self) -> u32 {
        *self as u32
    }
}
