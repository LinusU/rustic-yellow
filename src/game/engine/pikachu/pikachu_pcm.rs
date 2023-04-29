use crate::cpu::Cpu;

pub fn play_pikachu_sound_clip(cpu: &mut Cpu) {
    cpu.play_pikachu_cry(cpu.e);

    // Run GameBoy code as well so that everything works like normally
    cpu.a = cpu.e;
    cpu.pc = 0x4001;
    cpu.cycle(4);
}
