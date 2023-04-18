use crate::cpu::Cpu;

use super::vblank;

/// Wait `frames` frames
pub fn delay_frames(cpu: &mut Cpu, frames: u8) {
    assert!(frames > 0);

    for _ in 0..=frames {
        vblank::delay_frame(cpu);
    }
}
