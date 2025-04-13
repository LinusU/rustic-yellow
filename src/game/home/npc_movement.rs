use crate::{
    cpu::Cpu,
    game::{macros, ram::wram},
};

// not zero if an NPC movement script is running, the player character is
// automatically stepping down from a door, or joypad states are being simulated
pub fn is_player_character_being_controlled_by_game(cpu: &mut Cpu) -> bool {
    if cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM) != 0 {
        return true;
    }

    // currently stepping down from door bit
    if (cpu.read_byte(wram::W_D736) & 1) != 0 {
        return true;
    }

    // wd730 bit 5: ignore joypad input
    if (cpu.read_byte(wram::W_D730) & 0x80) != 0 {
        return true;
    }

    false
}

pub fn run_npc_movement_script(cpu: &mut Cpu) {
    let value = cpu.read_byte(wram::W_D736);

    // Should step out of door?
    if (value & 1) != 0 {
        log::trace!("step out of door");
        cpu.write_byte(wram::W_D736, value & !1);
        macros::farcall::farcall(cpu, 0x06, 0x64ea); // PlayerStepOutFromDoor
        return;
    }

    match cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM) {
        0 => return,
        1 => cpu.set_hl(0x654c), // PalletMovementScriptPointerTable
        2 => cpu.set_hl(0x6622), // PewterMuseumGuyMovementScriptPointerTable
        3 => cpu.set_hl(0x6685), // PewterGymGuyMovementScriptPointerTable
        n => panic!("Invalid pointer table number: {}", n),
    }

    log::trace!("run_npc_movement_script");

    let saved_bank = cpu.borrow_wram().loaded_rom_bank();

    cpu.a = cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_BANK);
    cpu.call(0x3e7e); // BankswitchCommon

    cpu.a = cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_FUNCTION_NUM);
    cpu.call(0x3d93); // CallFunctionInTable

    cpu.a = saved_bank;
    cpu.call(0x3e7e); // BankswitchCommon
}
