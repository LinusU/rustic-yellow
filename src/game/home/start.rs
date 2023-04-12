use std::ops::Generator;

use crate::{cpu::Cpu, game::ram::hram::H_GBC, yield_from};

// use super::init::init;

// pub fn start(cpu: &mut Cpu) -> impl Generator<Yield = u32, Return = ()> + '_ {
//     move || {
//         // Start::
//         yield_from!(cpu.cycle(20));

//         // _Start::
//         cpu.write_byte(H_GBC, true.into());
//         yield_from!(cpu.cycle(56));

//         yield_from!(init(cpu));
//     }
// }
