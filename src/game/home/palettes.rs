use crate::{
    cpu::Cpu,
    game::{constants, macros},
};

use super::delay::delay_frames;

pub fn gb_pal_white_out_with_delay3(cpu: &mut Cpu) {
    gb_pal_white_out(cpu);
    delay3(cpu);
}

/// The bg map is updated each frame in thirds.
/// Wait three frames to let the bg map fully update.
pub fn delay3(cpu: &mut Cpu) {
    delay_frames(cpu, 3);
}

/// Reset BGP and OBP0.
pub fn gb_pal_normal(cpu: &mut Cpu) {
    cpu.write_byte(constants::hardware_constants::R_BGP, 0b11100100);
    cpu.write_byte(constants::hardware_constants::R_OBP0, 0b11010000);
    cpu.write_byte(constants::hardware_constants::R_OBP1, 0);

    cpu.call(0x3021); // UpdateGBCPal_BGP
    cpu.call(0x3040); // UpdateGBCPal_OBP0
    cpu.call(0x3061); // UpdateGBCPal_OBP1
}

/// White out all palettes.
pub fn gb_pal_white_out(cpu: &mut Cpu) {
    cpu.write_byte(constants::hardware_constants::R_BGP, 0);
    cpu.write_byte(constants::hardware_constants::R_OBP0, 0);
    cpu.write_byte(constants::hardware_constants::R_OBP1, 0);

    cpu.call(0x3021); // UpdateGBCPal_BGP
    cpu.call(0x3040); // UpdateGBCPal_OBP0
    cpu.call(0x3061); // UpdateGBCPal_OBP1
}

pub fn run_default_palette_command(cpu: &mut Cpu) {
    run_palette_command(cpu, constants::palette_constants::SET_PAL_DEFAULT);
}

pub fn run_palette_command(cpu: &mut Cpu, palette: u8) {
    cpu.b = palette;
    macros::predef::predef_jump!(cpu, _RunPaletteCommand);
}
