use crate::{
    cpu::Cpu,
    game::{
        constants::{battle_constants, input_constants},
        ram::wram,
    },
};

pub fn pallet_town_script4(cpu: &mut Cpu) {
    let starter_index = cpu.starter.into_index();

    // start the pikachu battle
    cpu.write_byte(
        wram::W_JOY_IGNORE,
        !(input_constants::A_BUTTON | input_constants::B_BUTTON),
    );
    cpu.write_byte(wram::W_LIST_SCROLL_OFFSET, 0);
    cpu.write_byte(wram::W_BATTLE_TYPE, battle_constants::BATTLE_TYPE_PIKACHU);
    cpu.borrow_wram_mut().set_cur_opponent(starter_index);
    cpu.write_byte(wram::W_CUR_ENEMY_LVL, 5);

    // trigger the next script
    cpu.write_byte(wram::W_PALLET_TOWN_CUR_SCRIPT, 5);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
