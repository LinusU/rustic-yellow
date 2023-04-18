use crate::{
    cpu::Cpu,
    game::{constants, macros},
};

use super::delay::delay_frames;

/// The bg map is updated each frame in thirds.
/// Wait three frames to let the bg map fully update.
pub fn delay3(cpu: &mut Cpu) {
    // ld c, 3
    cpu.c = 0x03;
    cpu.cycle(8);

    // jp DelayFrames
    cpu.cycle(16);
    delay_frames(cpu);
}

pub fn run_default_palette_command(cpu: &mut Cpu) {
    cpu.b = constants::palette_constants::SET_PAL_DEFAULT;
    cpu.cycle(8);
    run_palette_command(cpu);
}

pub fn run_palette_command(cpu: &mut Cpu) {
    // Useless W_ON_SGB check
    cpu.cycle(16);
    cpu.cycle(4);
    cpu.cycle(8);

    macros::predef::predef_jump!(cpu, _RunPaletteCommand);
}
