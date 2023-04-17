use crate::{
    cpu::Cpu,
    game::{constants::gfx_constants, macros},
};

use super::palettes;

/// Clear wTileMap, then wait for the bg map to update.
pub fn clear_screen(cpu: &mut Cpu) {
    for y in 0..=gfx_constants::SCREEN_HEIGHT {
        for x in 0..=gfx_constants::SCREEN_WIDTH {
            cpu.write_byte(macros::coords::coord!(x, y), 0x7f); // " "
        }
    }

    palettes::delay3(cpu);
}
