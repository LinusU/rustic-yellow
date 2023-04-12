use std::ops::Generator;

use crate::{cpu::Cpu, game::ram::hram::H_GBC, yield_from};

use super::init::init;

pub fn start(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
    move || {
        // Start::
        yield cpu.mmu.do_cycle(20);

        // _Start::
        cpu.mmu.wb(H_GBC, true.into());
        yield cpu.mmu.do_cycle(56);

        yield_from!(init(cpu));
    }
}
