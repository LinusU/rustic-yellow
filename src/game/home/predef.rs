use crate::{cpu::Cpu, game::ram::wram};

/// Restore the contents of register pairs when GetPredefPointer was called.
pub fn get_predef_registers(cpu: &mut Cpu) {
    cpu.h = cpu.read_byte(wram::W_PREDEF_HL);
    cpu.l = cpu.read_byte(wram::W_PREDEF_HL + 1);

    cpu.d = cpu.read_byte(wram::W_PREDEF_DE);
    cpu.e = cpu.read_byte(wram::W_PREDEF_DE + 1);

    cpu.b = cpu.read_byte(wram::W_PREDEF_BC);
    cpu.c = cpu.read_byte(wram::W_PREDEF_BC + 1);

    cpu.pc = cpu.stack_pop(); // ret
}
