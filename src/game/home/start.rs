use crate::{cpu::Cpu, game::ram::hram::H_GBC};

pub fn start(cpu: &mut Cpu, cycles: &mut u64) {
    // Start::
    cpu.mmu.do_cycle(20);
    cpu.pc = 0x01ab;
    *cycles += 20;

    // _Start::
    cpu.mmu.wb(H_GBC, true.into());
    cpu.mmu.do_cycle(56);
    cpu.pc = 0x1d10;
    *cycles += 56;
}
