use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants,
        ram::{sram, wram},
    },
};

pub fn init_options(cpu: &mut Cpu) {
    cpu.write_byte(
        wram::W_LETTER_PRINTING_DELAY_FLAGS,
        constants::misc_constants::TEXT_DELAY_FAST,
    );
    cpu.write_byte(
        wram::W_OPTIONS,
        constants::misc_constants::TEXT_DELAY_MEDIUM,
    );
    cpu.write_byte(wram::W_PRINTER_SETTINGS, 64); // audio?
    cpu.pc = cpu.stack_pop();
}

/// Check if the player name data in SRAM has a string terminator character
/// (indicating that a name may have been saved there) and return whether it does
/// in carry.
pub fn check_for_player_name_in_sram(cpu: &mut Cpu) {
    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_ENABLE,
    );
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 1);
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANK, 1);

    cpu.set_flag(CpuFlag::C, false);

    for i in 0..=constants::text_constants::NAME_LENGTH {
        // 0x50 = string terminator
        if cpu.read_byte(sram::S_PLAYER_NAME + (i as u16)) == 0x50 {
            cpu.set_flag(CpuFlag::C, true);
            break;
        }
    }

    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_DISABLE,
    );
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 0);

    cpu.pc = cpu.stack_pop();
}
