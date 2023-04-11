use crate::{cpu::Cpu, game::ram::hram::H_GBC};

use super::init::init;

pub fn start(cpu: &mut Cpu, cycles: &mut u64) {
    // Start::
    *cycles += cpu.mmu.do_cycle(20) as u64;

    // _Start::
    cpu.mmu.wb(H_GBC, true.into());
    *cycles += cpu.mmu.do_cycle(56) as u64;

    init(cpu, cycles)
}
