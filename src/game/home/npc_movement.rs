use crate::{
    cpu::{Cpu, CpuFlag},
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
    cpu.pc = 0x30ae;

    // ld hl, wd736
    cpu.set_hl(wram::W_D736);
    cpu.pc += 3;
    cpu.cycle(12);

    // bit 0, [hl]
    let value = cpu.read_byte(cpu.hl());
    cpu.set_flag(CpuFlag::Z, (value & (1 << 0)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(16);

    // res 0, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << 0));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // jr nz, .playerStepOutFromDoor
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return run_npc_movement_script_player_step_out_from_door(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

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

    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // add a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.a & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.a as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld d, 0
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, .NPCMovementScriptPointerTables
    cpu.set_hl(0x30dc);
    cpu.pc += 1;
    cpu.cycle(4);

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld h, [hl]
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld l, a
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh a, [hLoadedROMBank]
    cpu.a = cpu.borrow_wram().loaded_rom_bank();
    cpu.pc += 2;
    cpu.cycle(12);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wNPCMovementScriptBank]
    cpu.a = cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_BANK);
    cpu.pc += 3;
    cpu.cycle(16);

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // ld a, [wNPCMovementScriptFunctionNum]
    cpu.a = cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_FUNCTION_NUM);
    cpu.pc += 3;
    cpu.cycle(16);

    // call CallFunctionInTable
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3d93); // CallFunctionInTable
        cpu.pc = pc;
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn run_npc_movement_script_player_step_out_from_door(cpu: &mut Cpu) {
    cpu.pc = 0x30e2;

    // farjp PlayerStepOutFromDoor
    macros::farcall::farcall(cpu, 0x06, 0x64ea);
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
