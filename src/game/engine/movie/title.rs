use crate::{
    cpu::Cpu,
    game::{engine::menus, home, ram::hram},
};

pub fn display_title_screen_go_to_main_menu(cpu: &mut Cpu) {
    cpu.e = 10;
    cpu.call(0x4387); // TitleScreen_PlayPikachuPCM

    // Make sure the Pikachu sound is done playing
    home::delay::delay_frames(cpu, 20);

    menus::main_menu::main_menu(cpu);

    cpu.pc = 0x42a4; // DisplayTitleScreen.titleScreenLoop
}

pub fn title_screen_copy_tile_map_to_vram(cpu: &mut Cpu, dst: u16) {
    // This function only writes the high byte of the destination
    // address, presumably because the low byte is always 0x00.
    assert_eq!(cpu.read_byte(hram::H_AUTO_BG_TRANSFER_DEST), 0);

    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_DEST + 1, (dst >> 8) as u8);
    home::palettes::delay3(cpu);
}
