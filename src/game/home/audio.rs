use crate::{
    cpu::Cpu,
    game::{
        audio::{music::Music, music_sfx::MusicSfx, sfx::Sfx},
        ram::wram,
    },
};

pub fn play_sound(cpu: &mut Cpu) {
    // Note: Not sure why we are reading W_AUDIO_SAVED_ROM_BANK instead of
    // W_AUDIO_ROM_BANK, but if we don't we sometimes play the wrong audio...
    let bank = cpu.read_byte(wram::W_AUDIO_SAVED_ROM_BANK);

    let pitch = cpu.read_byte(wram::W_FREQUENCY_MODIFIER);
    let length = cpu.read_byte(wram::W_TEMPO_MODIFIER) as i8;

    if cpu.a == 0xff {
        // Stop all sounds?
    } else if let Some(music) = Music::from_bank_and_id(bank, cpu.a) {
        cpu.start_music(music);
    } else if let Some(music_sfx) = MusicSfx::from_bank_and_id(bank, cpu.a, pitch, length) {
        cpu.play_sfx(music_sfx);
    } else if let Some(mut sfx) = Sfx::from_bank_and_id(bank, cpu.a) {
        if sfx.is_cry() {
            sfx.tweak(pitch, length);
        }

        cpu.play_sfx(sfx);
    } else {
        eprintln!(
            "Don't know what to play: {:02x}:{:04x} (id = {})",
            bank,
            0x4000 + (cpu.a as u16) * 3,
            cpu.a,
        );

        // eprintln!("W_AUDIO_ROM_BANK: {:02x}", cpu.read_byte(wram::W_AUDIO_ROM_BANK));
        // eprintln!("W_AUDIO_SAVED_ROM_BANK: {:02x}", cpu.read_byte(wram::W_AUDIO_SAVED_ROM_BANK));
        // eprintln!("W_NEW_SOUND_ID: {:02x}", cpu.read_byte(wram::W_NEW_SOUND_ID));
        // eprintln!("W_LAST_MUSIC_SOUND_ID: {:02x}", cpu.read_byte(wram::W_LAST_MUSIC_SOUND_ID));
        // eprintln!("CPU: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}", cpu.a, cpu.b, cpu.c, cpu.d, cpu.e, cpu.f, cpu.h, cpu.l);
    }

    // Run GameBoy code as well so that everything works like normally
    cpu.stack_push(cpu.hl());
    cpu.pc = 0x2239;
    cpu.cycle(16);
}
