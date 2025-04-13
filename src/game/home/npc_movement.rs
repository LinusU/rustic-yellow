use crate::{cpu::Cpu, game::ram::wram};

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
