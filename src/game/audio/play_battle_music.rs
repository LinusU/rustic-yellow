use crate::{
    cpu::Cpu,
    game::constants::{music_constants, trainer_constants},
};

pub fn play_battle_music(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_audio_fade_out_control(0);
    cpu.borrow_wram_mut().set_low_health_alarm(0);

    cpu.call(0x2233); // StopAllMusic
    cpu.call(0x1e64); // DelayFrame

    cpu.c = 0x08; // BANK(Music_GymLeaderBattle)

    if cpu.borrow_wram().gym_leader_no() != 0 {
        cpu.a = music_constants::MUSIC_GYM_LEADER_BATTLE;
    } else {
        cpu.a = match cpu.borrow_wram().cur_opponent() {
            opp if opp < trainer_constants::OPP_ID_OFFSET => music_constants::MUSIC_WILD_BATTLE,
            trainer_constants::OPP_RIVAL3 => music_constants::MUSIC_FINAL_BATTLE,
            trainer_constants::OPP_LANCE => music_constants::MUSIC_GYM_LEADER_BATTLE,
            _ => music_constants::MUSIC_TRAINER_BATTLE,
        };
    }

    match cpu.a {
        music_constants::MUSIC_GYM_LEADER_BATTLE => {
            log::debug!("PlayBattleMusic: MUSIC_GYM_LEADER_BATTLE")
        }
        music_constants::MUSIC_FINAL_BATTLE => {
            log::debug!("PlayBattleMusic: MUSIC_FINAL_BATTLE")
        }
        music_constants::MUSIC_WILD_BATTLE => {
            log::debug!("PlayBattleMusic: MUSIC_WILD_BATTLE")
        }
        music_constants::MUSIC_TRAINER_BATTLE => {
            log::debug!("PlayBattleMusic: MUSIC_TRAINER_BATTLE")
        }
        _ => unreachable!(),
    }

    cpu.pc = 0x2211; // jp PlayMusic
}
