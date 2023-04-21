use crate::cpu::Cpu;

/// Copy (bc) `count` bytes from (hl) `src` to (de) `dst`.
pub fn copy_data(cpu: &mut Cpu, src: u16, dst: u16, count: u16) {
    for idx in 0..count {
        let byte = cpu.read_byte(src + idx);
        cpu.write_byte(dst + idx, byte);
    }
}
