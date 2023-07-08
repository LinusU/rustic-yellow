use crate::{
    cpu::Cpu,
    game::{
        constants, home,
        ram::{hram, sram, wram},
    },
    save_state::{PokeString, SaveState},
};

pub struct SavSummary {
    pub player_name: PokeString,
    pub num_badges: u32,
    pub owned_mons: u32,
    pub play_time_hh_mm: (u8, u8),
}

pub fn load_sav_summary(data: &SaveState) -> SavSummary {
    const PLAY_TIME_HH: usize = 0x2ced;
    const PLAY_TIME_MM: usize = 0x2cef;

    SavSummary {
        player_name: data.player_name(),
        num_badges: data.count_badges(),
        owned_mons: data.count_owned_mons(),
        play_time_hh_mm: (data.byte(PLAY_TIME_HH), data.byte(PLAY_TIME_MM)),
    }
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

pub fn save_sav_to_sram(cpu: &mut Cpu) {
    cpu.write_byte(wram::W_SAVE_FILE_STATUS, 2);

    cpu.call(0x7ae5); // SaveSAVtoSRAM0
    cpu.call(0x7b32); // SaveSAVtoSRAM1
    cpu.call(0x7b56); // SaveSAVtoSRAM2

    cpu.save_to_disk();

    cpu.pc = cpu.stack_pop();
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
