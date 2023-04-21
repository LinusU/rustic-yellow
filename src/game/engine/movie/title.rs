use crate::{
    cpu::Cpu,
    game::{engine::menus, home},
};

pub fn display_title_screen_go_to_main_menu(cpu: &mut Cpu) {
    cpu.e = 10;
    cpu.call(0x4387); // TitleScreen_PlayPikachuPCM

    // Make sure the Pikachu sound is done playing
    home::delay::delay_frames(cpu, 20);

    menus::main_menu::main_menu(cpu);

    cpu.pc = 0x42a4; // DisplayTitleScreen.titleScreenLoop
}
