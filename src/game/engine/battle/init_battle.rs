use crate::{
    cpu::Cpu,
    game::{
        engine::menus::pokedex,
        home, macros,
        ram::{vram, wram},
    },
};

/// Assumes the monster's attributes have been loaded with GetMonHeader.
pub fn load_mon_back_pic(cpu: &mut Cpu) {
    let pokemon_index = cpu.read_byte(wram::W_BATTLE_MON_SPECIES2);
    let pokedex_no = pokedex::index_to_pokedex(pokemon_index);

    // Probably not needed, but is done by the GameBoy code
    {
        cpu.write_byte(wram::W_CF91, pokemon_index);

        // hlcoord 1, 5
        cpu.set_hl(macros::coords::coord!(1, 5));
        cpu.pc += 3;
        cpu.cycle(12);

        // lb bc, 7, 8
        cpu.b = 7;
        cpu.c = 8;
        cpu.pc += 3;
        cpu.cycle(12);

        // call ClearScreenArea
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        home::copy2::clear_screen_area(cpu);
        assert_eq!(cpu.pc, pc);
    }

    let source_data = home::pics::read_crystal_pokemon_sprite(pokedex_no as usize, true);
    assert_eq!(source_data.len(), 48 * 48 / 4);

    let sprite_data = home::pics::center_pokemon_sprite(&source_data, 6, 6);

    // Probably not needed, but is done by the GameBoy code
    for (idx, data) in sprite_data.iter().enumerate() {
        cpu.write_byte(vram::V_SPRITES + (idx as u16), *data);
    }

    for (idx, data) in sprite_data.iter().enumerate() {
        cpu.write_byte(vram::V_BACK_PIC + (idx as u16), *data);
    }

    // ret
    cpu.pc = cpu.stack_pop();
}
