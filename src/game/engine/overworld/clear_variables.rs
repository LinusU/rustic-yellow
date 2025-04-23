use crate::{
    cpu::Cpu,
    game::{
        constants::{gfx_constants, hardware_constants},
        ram::{hram, wram},
    },
};

pub fn clear_variables_on_enter_map(cpu: &mut Cpu) {
    cpu.write_byte(hram::H_WY, gfx_constants::SCREEN_HEIGHT_PX);
    cpu.write_byte(hardware_constants::R_WY, gfx_constants::SCREEN_HEIGHT_PX);
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 0);
    cpu.borrow_wram_mut().set_step_counter(0);
    cpu.write_byte(wram::W_LONE_ATTACK_NO, 0);
    cpu.write_byte(hram::H_JOY_PRESSED, 0);
    cpu.write_byte(hram::H_JOY_RELEASED, 0);
    cpu.write_byte(hram::H_JOY_HELD, 0);
    cpu.write_byte(wram::W_ACTION_RESULT_OR_TOOK_BATTLE_TURN, 0);
    cpu.borrow_wram_mut().set_card_key_door_yx(0, 0);

    // This loop clears a lot of variables that have overlapping memory addresses
    for i in wram::W_WHICH_TRADE..wram::W_STANDING_ON_WARP_PAD_OR_HOLE {
        cpu.write_byte(i, 0);
    }
}
