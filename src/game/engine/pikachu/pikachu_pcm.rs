use crate::{
    cpu::Cpu,
    game::{home, ram::hram},
};

pub fn play_pikachu_sound_clip(cpu: &mut Cpu) {
    cpu.play_pikachu_cry(cpu.e);
    home::palettes::delay3(cpu);

    cpu.a = cpu.read_byte(hram::H_LOADED_ROM_BANK);
    cpu.pc = cpu.stack_pop();
}
