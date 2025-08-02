use crate::{cpu::Cpu, game::constants::hardware_constants};

pub fn animation_flash_screen(cpu: &mut Cpu) {
    log::debug!("animation_flash_screen()");

    // save initial palette
    let saved_pal = cpu.read_byte(hardware_constants::R_BGP);

    // 0, 1, 2, 3 (inverted colors)
    cpu.write_byte(hardware_constants::R_BGP, 0b00011011);
    cpu.call(0x3021); // UpdateCGBPal_BGP

    cpu.c = 2;
    cpu.call(0x372f); // DelayFrames

    // white out background
    cpu.write_byte(hardware_constants::R_BGP, 0);
    cpu.call(0x3021); // UpdateCGBPal_BGP

    cpu.c = 2;
    cpu.call(0x372f); // DelayFrames

    // restore initial palette
    cpu.write_byte(hardware_constants::R_BGP, saved_pal);
    cpu.call(0x3021); // UpdateCGBPal_BGP

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn set_animation_bg_palette(cpu: &mut Cpu) {
    cpu.write_byte(hardware_constants::R_BGP, cpu.c);
    cpu.call(0x3021); // UpdateCGBPal_BGP

    cpu.a = cpu.c;

    cpu.pc = cpu.stack_pop(); // ret
}
