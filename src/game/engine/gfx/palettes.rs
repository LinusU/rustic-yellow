use crate::{cpu::Cpu, game::ram::wram};

pub fn load_sgb(cpu: &mut Cpu) {
    // This function should only be called once
    assert_eq!(cpu.read_byte(wram::W_ON_SGB), 0x00);
    cpu.write_byte(wram::W_ON_SGB, 0x01);

    // ret
    cpu.pc = cpu.stack_pop();
}
