use crate::cpu::Cpu;

pub fn modify_pikachu_happiness(cpu: &mut Cpu, d: u8) {
    // 3d:430a ModifyPikachuHappiness
    cpu.set_hl(0x430a);
    cpu.b = 0x3d;
    cpu.d = d;

    // call Bankswitch
    cpu.call(0x3e84);
}
