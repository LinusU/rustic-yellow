use crate::cpu::Cpu;

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
