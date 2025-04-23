use crate::{
    cpu::{Cpu, CpuFlag},
    game::ram::wram,
};

pub fn check_event(cpu: &mut Cpu, event_index: u16) -> bool {
    let event_byte = event_index / 8;
    let event_bit = (event_index % 8) as u8;

    // ld a, [wEventFlags + event_byte]
    cpu.a = cpu.read_byte(wram::W_EVENT_FLAGS + event_byte);
    cpu.pc += 3;
    cpu.cycle(16);

    // bit (\1) % 8, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << event_bit)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    (cpu.a & (1 << event_bit)) != 0
}

pub fn set_event(cpu: &mut Cpu, event_index: u16) {
    let event_byte = event_index / 8;
    let event_bit = (event_index % 8) as u8;

    // ld hl, wEventFlags + event_byte
    cpu.set_hl(wram::W_EVENT_FLAGS + event_byte);
    cpu.pc += 3;
    cpu.cycle(12);

    // set (\1) % 8, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << event_bit));
    }
    cpu.pc += 2;
    cpu.cycle(16);
}
