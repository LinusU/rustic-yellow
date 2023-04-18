use crate::{
    cpu::Cpu,
    game::{constants, macros},
};

use super::delay::delay_frames;

/// The bg map is updated each frame in thirds.
/// Wait three frames to let the bg map fully update.
pub fn delay3(cpu: &mut Cpu) {
    delay_frames(cpu, 3);
}

pub fn run_default_palette_command(cpu: &mut Cpu) {
    run_palette_command(cpu, constants::palette_constants::SET_PAL_DEFAULT);
}

pub fn run_palette_command(cpu: &mut Cpu, palette: u8) {
    cpu.b = palette;
    macros::predef::predef_jump!(cpu, _RunPaletteCommand);
}
