use crate::{
    cpu::Cpu,
    game::{
        constants, home, macros,
        ram::{sram, wram},
    },
};

const CONTINUE_TEXT: u16 = 0x5d06;
const NEW_GAME_TEXT: u16 = 0x5d0f;

pub fn main_menu(cpu: &mut Cpu) {
    // call InitOptions
    cpu.stack_push(0x0001);
    init_options(cpu);

    cpu.write_byte(wram::W_OPTIONS_INITIALIZED, 0);
    cpu.write_byte(wram::W_SAVE_FILE_STATUS, 1);

    // call CheckForPlayerNameInSRAM
    if check_for_player_name_in_sram(cpu) {
        macros::predef::predef_call!(cpu, LoadSAV);
    }

    cpu.c = 20;
    cpu.stack_push(0x0001);
    home::delay::delay_frames(cpu);

    cpu.write_byte(
        wram::W_LINK_STATE,
        constants::serial_constants::LINK_STATE_NONE,
    );

    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 1, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 2, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 3, 0);

    cpu.write_byte(wram::W_DEFAULT_MAP, 0);

    // Toggle link feature bit off
    {
        let v = cpu.read_byte(wram::W_D72E);
        cpu.write_byte(wram::W_D72E, v & !(1 << 6));
    }

    cpu.stack_push(0x0001);
    home::copy2::clear_screen(cpu);

    cpu.stack_push(0x0001);
    home::palettes::run_default_palette_command(cpu);

    cpu.call(0x36a3); // call LoadTextBoxTilePatterns
    cpu.call(0x3683); // call LoadFontTilePatterns

    // Print text with no delay between each letter
    {
        let v = cpu.read_byte(wram::W_D730);
        cpu.write_byte(wram::W_D730, v | (1 << 6));
    }

    if cpu.read_byte(wram::W_SAVE_FILE_STATUS) != 1 {
        // there's a save file
        cpu.set_hl(macros::coords::coord!(0, 0));
        cpu.b = 6;
        cpu.c = 13;
        cpu.call(0x16f0); // call TextBoxBorder

        cpu.set_hl(macros::coords::coord!(2, 2));
        cpu.set_de(CONTINUE_TEXT);
        cpu.call(0x1723); // call PlaceString
    } else {
        cpu.set_hl(macros::coords::coord!(0, 0));
        cpu.b = 4;
        cpu.c = 13;
        cpu.call(0x16f0); // call TextBoxBorder

        cpu.set_hl(macros::coords::coord!(2, 2));
        cpu.set_de(NEW_GAME_TEXT);
        cpu.call(0x1723); // call PlaceString
    }

    // Print text with delay between each letter
    {
        let v = cpu.read_byte(wram::W_D730);
        cpu.write_byte(wram::W_D730, v & !(1 << 6));
    }

    cpu.call(0x231c); // call UpdateSprites

    cpu.write_byte(wram::W_CURRENT_MENU_ITEM, 0);
    cpu.write_byte(wram::W_LAST_MENU_ITEM, 0);
    cpu.write_byte(wram::W_MENU_JOYPAD_POLL_COUNT, 0);

    cpu.write_byte(wram::W_TOP_MENU_ITEM_X, 1);
    cpu.write_byte(wram::W_TOP_MENU_ITEM_Y, 2);

    cpu.write_byte(
        wram::W_MENU_WATCHED_KEYS,
        constants::input_constants::A_BUTTON
            | constants::input_constants::B_BUTTON
            | constants::input_constants::START,
    );

    {
        let v = cpu.read_byte(wram::W_SAVE_FILE_STATUS);
        cpu.write_byte(wram::W_MAX_MENU_ITEM, v);
    }

    cpu.call(0x3aab); // call HandleMenuInput

    if (cpu.a & constants::input_constants::B_BUTTON) != 0 {
        eprintln!("B pressed");
        return cpu.jump(0x4171); // jump DisplayTitleScreen
    }

    cpu.c = 20;
    cpu.stack_push(0x0001);
    home::delay::delay_frames(cpu);

    cpu.b = cpu.read_byte(wram::W_CURRENT_MENU_ITEM);

    if cpu.read_byte(wram::W_SAVE_FILE_STATUS) != 2 {
        // If there's no save file, increment the current menu item so that the numbers
        // are the same whether or not there's a save file.
        cpu.b += 1;
    }

    // MainMenu.skipInc
    cpu.jump(0x5c50);
}

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
pub fn check_for_player_name_in_sram(cpu: &mut Cpu) -> bool {
    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_ENABLE,
    );
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 1);
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANK, 1);

    let mut result = false;

    for i in 0..=constants::text_constants::NAME_LENGTH {
        // 0x50 = string terminator
        if cpu.read_byte(sram::S_PLAYER_NAME + (i as u16)) == 0x50 {
            result = true;
            break;
        }
    }

    cpu.write_byte(
        constants::hardware_constants::MBC1_SRAM_ENABLE,
        constants::hardware_constants::SRAM_DISABLE,
    );
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANKING_MODE, 0);

    result
}
