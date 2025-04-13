use crate::{
    cpu::{Cpu, CpuFlag},
    game::ram::wram,
};

// not zero if an NPC movement script is running, the player character is
// automatically stepping down from a door, or joypad states are being simulated
pub fn is_player_character_being_controlled_by_game(cpu: &mut Cpu) {
    cpu.pc = 0x309d;

    // ld a, [wNPCMovementScriptPointerTableNum]
    cpu.a = cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        log::trace!(
            "is_player_character_being_controlled_by_game() = {}",
            cpu.flag(CpuFlag::Z)
        );
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [wd736]
    cpu.a = cpu.read_byte(wram::W_D736);
    cpu.pc += 3;
    cpu.cycle(16);

    // currently stepping down from door bit
    // bit 1, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << 1)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ret nz
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        log::trace!(
            "is_player_character_being_controlled_by_game() = {}",
            cpu.flag(CpuFlag::Z)
        );
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [wd730]
    cpu.a = cpu.read_byte(wram::W_D730);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, $80
    cpu.a &= 0x80;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
    log::trace!(
        "is_player_character_being_controlled_by_game() = {}",
        cpu.flag(CpuFlag::Z)
    );
}
