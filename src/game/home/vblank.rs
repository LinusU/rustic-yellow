use crate::{cpu::Cpu, game::ram::hram};

/// Wait for the next vblank interrupt.
pub fn delay_frame(cpu: &mut Cpu) {
    const HAS_VBLANKED: u8 = 0;
    const NOT_VBLANKED: u8 = 1;

    cpu.write_byte(hram::H_VBLANK_OCCURRED, NOT_VBLANKED);
    cpu.cycle(20);

    loop {
        cpu.halted = true;
        cpu.cycle(4);

        let result = cpu.read_byte(hram::H_VBLANK_OCCURRED);
        cpu.cycle(16);

        if result == HAS_VBLANKED {
            cpu.cycle(8);
            break;
        }

        cpu.cycle(12);
    }

    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
