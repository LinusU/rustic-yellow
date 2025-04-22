use crate::{
    cpu::Cpu,
    game::{macros, ram::wram},
};

pub fn return_to_cable_club_room(cpu: &mut Cpu) {
    log::info!("return_to_cable_club_room()");

    cpu.call(0x3dd8); // GBPalWhiteOutWithDelay3

    {
        let saved_font_loaded = cpu.borrow_wram().font_loaded();

        cpu.borrow_wram_mut().set_font_loaded(false);
        cpu.write_byte(wram::W_D72D, 0);
        cpu.borrow_wram_mut().set_destination_warp_id(0xff);
        cpu.call(0x0ecb); // LoadMapData
        macros::farcall::farcall(cpu, 0x03, 0x407c); // ClearVariablesOnEnterMap

        cpu.borrow_wram_mut().set_font_loaded(saved_font_loaded);
    }

    cpu.call(0x1ebd); // GBFadeInFromWhite

    cpu.pc = cpu.stack_pop(); // ret
}
