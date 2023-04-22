use crate::{cpu::Cpu, game::ram::wram, sound2::Music};

fn music_from_bank_and_id(bank: u8, id: u8) -> Option<Music> {
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

pub fn play_music(cpu: &mut Cpu) {
    cpu.write_byte(wram::W_AUDIO_ROM_BANK, cpu.c);
    cpu.write_byte(wram::W_AUDIO_SAVED_ROM_BANK, cpu.c);

    if let Some(music) = music_from_bank_and_id(cpu.c, cpu.a) {
        cpu.start_music(music);
    } else {
        let offset = 0x4000 + ((cpu.a as u16) * 3);
        eprintln!(
            "Unknown music: {:02x}:{:04x} (id = {})",
            cpu.c, offset, cpu.a
        );
    }

    cpu.pc = cpu.stack_pop();
}

pub fn play_sound(cpu: &mut Cpu) {
    let bank = cpu.read_byte(wram::W_AUDIO_ROM_BANK);

    // For some reason in a few cases music is played via PlaySound instead of PlayMusic
    if let Some(music) = music_from_bank_and_id(bank, cpu.a) {
        cpu.start_music(music);
        cpu.pc = cpu.stack_pop();
        return;
    }

    // Fallback to native sound
    cpu.stack_push(cpu.hl());
    cpu.pc = 0x2239;
}
