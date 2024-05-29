use crate::cpu::Cpu;

pub fn farcall(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld b, BANK(\1)
    cpu.b = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // call Bankswitch
    {
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e84);
        cpu.pc = pc + 3;
    }
}

pub fn callfar(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, BANK(\1)
    cpu.b = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // call Bankswitch
    {
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e84);
        cpu.pc = pc + 3;
    }
}

pub fn callabd_modify_pikachu_happiness(cpu: &mut Cpu, pikahappy: u8) {
    // ld hl, ModifyPikachuHappiness
    cpu.set_hl(0x430a);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, BANK(ModifyPikachuHappiness)
    cpu.b = 0x3d;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld d, \1
    cpu.d = pikahappy;
    cpu.pc += 2;
    cpu.cycle(8);

    // call Bankswitch
    {
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e84);
        cpu.pc = pc + 3;
    }
}
