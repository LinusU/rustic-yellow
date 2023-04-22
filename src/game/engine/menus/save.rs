use crate::{
    cpu::Cpu,
    game::{
        constants, home,
        ram::{hram, sram, wram},
    },
};

pub struct SavSummary {
    pub player_name: String,
    pub num_badges: u32,
    pub owned_mons: u32,
    pub play_time_hh_mm: (u8, u8),
}

pub fn load_sav_summary(data: &[u8]) -> SavSummary {
    const PLAYER_NAME: usize = 0x2598;
    const BADGES: usize = 0x2602;
    const POKEDEX_OWNED: usize = 0x25a3;
    const POKEDEX_OWNED_END: usize = 0x25b6;
    const PLAY_TIME_HH: usize = 0x2ced;
    const PLAY_TIME_MM: usize = 0x2cef;

    let mut result = SavSummary {
        player_name: String::new(),
        num_badges: data[BADGES].count_ones(),
        owned_mons: 0,
        play_time_hh_mm: (data[PLAY_TIME_HH], data[PLAY_TIME_MM]),
    };

    for i in 0..(constants::text_constants::NAME_LENGTH as usize) {
        let ch = data[PLAYER_NAME + i];

        match ch {
            0x50 => {
                break;
            }
            0x80..=0x99 => {
                result.player_name.push((b'A' + (ch - 0x80)) as char);
            }
            0xa0..=0xb9 => {
                result.player_name.push((b'a' + (ch - 0xa0)) as char);
            }
            0xf6..=0xff => {
                result.player_name.push((b'0' + (ch - 0xf6)) as char);
            }
            _ => panic!("Invalid character in player name: {:02x}", ch),
        }
    }

    for addr in POKEDEX_OWNED..POKEDEX_OWNED_END {
        result.owned_mons += data[addr].count_ones();
    }

    result
}

pub fn load_sav(cpu: &mut Cpu) {
    enable_sram_and_latch_clock_data(cpu);
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANK, 1);

    home::copy::copy_data(
        cpu,
        sram::S_PLAYER_NAME,
        wram::W_PLAYER_NAME,
        constants::text_constants::NAME_LENGTH as u16,
    );

    home::copy::copy_data(
        cpu,
        sram::S_MAIN_DATA,
        wram::W_MAIN_DATA_START,
        wram::W_MAIN_DATA_END - wram::W_MAIN_DATA_START,
    );

    {
        let v = cpu.read_byte(wram::W_CUR_MAP_TILESET);
        cpu.write_byte(wram::W_CUR_MAP_TILESET, v | (1 << 7))
    }

    home::copy::copy_data(
        cpu,
        sram::S_SPRITE_DATA,
        wram::W_SPRITE_DATA_START,
        wram::W_SPRITE_DATA_END - wram::W_SPRITE_DATA_START,
    );

    {
        let v = cpu.read_byte(sram::S_TILE_ANIMATIONS);
        cpu.write_byte(hram::H_TILE_ANIMATIONS, v);
    }

    home::copy::copy_data(
        cpu,
        sram::S_CUR_BOX_DATA,
        wram::W_BOX_DATA_START,
        wram::W_BOX_DATA_END - wram::W_BOX_DATA_START,
    );

    home::copy::copy_data(
        cpu,
        sram::S_PARTY_DATA,
        wram::W_PARTY_DATA_START,
        wram::W_PARTY_DATA_END - wram::W_PARTY_DATA_START,
    );

    home::copy::copy_data(
        cpu,
        sram::S_MAIN_DATA,
        wram::W_POKEDEX_OWNED,
        wram::W_POKEDEX_SEEN_END - wram::W_POKEDEX_OWNED,
    );

    disable_sram_and_prepare_clock_data(cpu);
    cpu.write_byte(wram::W_SAVE_FILE_STATUS, 2);
}

pub fn enable_sram_and_latch_clock_data(cpu: &mut Cpu) {
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 1);
    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_ENABLE,
    );
}

pub fn disable_sram_and_prepare_clock_data(cpu: &mut Cpu) {
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 0);
    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_DISABLE,
    );
}
