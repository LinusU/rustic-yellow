use image_gameboy::GameBoy2bpp;

use crate::{
    cpu::Cpu,
    game::{constants::gfx_constants::LEN_2BPP_TILE, ram::vram},
};

const EXP_BAR_PNG: &[u8] = include_bytes!("../../../gfx/exp_bar.png");
const HP_BAR_AND_STATUS_PNG: &[u8] = include_bytes!("../../../gfx/font_battle_extra.png");

pub fn png_to_2bpp(bytes: &[u8]) -> image::ImageResult<Vec<u8>> {
    Ok(image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?.into_gb2bpp())
}

pub fn load_hp_bar_and_status_tile_patterns(cpu: &mut Cpu) {
    log::debug!("load_hp_bar_and_status_tile_patterns()");

    let data = png_to_2bpp(HP_BAR_AND_STATUS_PNG).unwrap();
    let target = vram::V_CHARS_2 + (LEN_2BPP_TILE as u16) * 0x62;

    for (i, &byte) in data.iter().enumerate() {
        cpu.write_byte(target + i as u16, byte);
    }

    let data = png_to_2bpp(EXP_BAR_PNG).unwrap();
    let target = vram::V_CHARS_1 + (LEN_2BPP_TILE as u16) * 0x40;

    for (i, &byte) in data.iter().enumerate() {
        cpu.write_byte(target + i as u16, byte);
    }

    cpu.pc = cpu.stack_pop(); // ret
}
