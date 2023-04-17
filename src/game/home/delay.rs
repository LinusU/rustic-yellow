use crate::cpu::Cpu;

use super::vblank;

/// wait c frames
pub fn delay_frames(cpu: &mut Cpu) {
    assert!(cpu.c > 0);

    loop {
        // call DelayFrame
        cpu.stack_push(0x0001);
        cpu.cycle(24);
        vblank::delay_frame(cpu);

        // dec c
        cpu.c -= 1;
        cpu.cycle(4);

        // jr nz, DelayFrames
        if cpu.c == 0 {
            cpu.cycle(8);
            break;
        } else {
            cpu.cycle(12);
        }
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
