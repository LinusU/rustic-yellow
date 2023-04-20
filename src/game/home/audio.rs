use crate::cpu::Cpu;

pub fn play_sound(cpu: &mut Cpu, sound: u8) {
    cpu.a = sound;
    cpu.call(0x2238); // PlaySound
}
