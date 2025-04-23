use crate::{
    cpu::Cpu,
    game::{
        constants::input_constants::A_BUTTON,
        macros,
        ram::{hram, wram},
    },
};

pub fn check_for_hidden_object_or_bookshelf_or_card_key_door(cpu: &mut Cpu) {
    log::trace!("check_for_hidden_object_or_bookshelf_or_card_key_door");

    // Return if A button isn't pressed
    if (cpu.read_byte(hram::H_JOY_HELD) & A_BUTTON) == 0 {
        cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, 0xff);
        return;
    }

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.a = 0x3c; // BANK(CheckForHiddenObject)
    cpu.call(0x3e7e); // BankswitchCommon
    cpu.call(0x65f8); // CheckForHiddenObject

    if cpu.read_byte(hram::H_DIDNT_FIND_ANY_HIDDEN_OBJECT) == 0 {
        cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, 0);

        cpu.a = cpu.read_byte(wram::W_HIDDEN_OBJECT_FUNCTION_ROM_BANK);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.call(0x3e98); // JumpToAddress
    } else {
        macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);
        macros::farcall::farcall(cpu, 0x03, 0x79de); // PrintBookshelfText

        if cpu.read_byte(hram::H_INTERACTED_WITH_BOOKSHELF) == 0 {
            cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, 0);
        } else {
            cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, 0xff);
        }
    }

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon

    log::info!(
        "check_for_hidden_object_or_bookshelf_or_card_key_door() = {:02x}",
        cpu.read_byte(hram::H_ITEM_ALREADY_FOUND)
    );
}
