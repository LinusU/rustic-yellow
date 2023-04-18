use crate::{cpu::Cpu, game::ram::hram};

/// Wait for the next vblank interrupt.
pub fn delay_frame(cpu: &mut Cpu) {
    const HAS_VBLANKED: u8 = 0;
    const NOT_VBLANKED: u8 = 1;

    cpu.write_byte(hram::H_VBLANK_OCCURRED, NOT_VBLANKED);

    loop {
        cpu.halted = true;
        cpu.cycle(4);

        if cpu.read_byte(hram::H_VBLANK_OCCURRED) == HAS_VBLANKED {
            break;
        }
    }
}
