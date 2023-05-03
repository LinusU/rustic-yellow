use crate::{
    cpu::Cpu,
    game::{
        engine::menus::pokedex,
        home, macros,
        ram::{vram, wram},
    },
    rom::CRYSTAL_ROM,
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

    const POKEMON_PIC_POINTERS: usize = 0x48 * 0x4000;
    const PICS_FIX: usize = 0x36;

    let offset = POKEMON_PIC_POINTERS + ((pokedex_no - 1) as usize) * 6;

    let bank = (CRYSTAL_ROM[offset + 3] as usize) + PICS_FIX;
    let addr = CRYSTAL_ROM[offset + 4] as usize + ((CRYSTAL_ROM[offset + 5] as usize) << 8);

    let ptr = (bank * 0x4000) | (addr & 0x3fff);
    let source_data = pokemon_sprite_compression::gen2::decompress(&CRYSTAL_ROM[ptr..]);

    assert_eq!(source_data.len(), 48 * 48 / 4);

    // The source data is 48x48, but we need it to be 56x56, pad with 00 which is transparent.
    let mut sprite_data = Vec::with_capacity(56 * 56 / 4);

    // Sprite data is stored in tiles of 8x8 pixels (16 bytes). The first tile is the top left,
    // the second one is the one below that, etc.
    let mut src_pos = 0;

    // First we skip 7 tiles which are the first column of the sprite.
    for _ in 0..7 {
        sprite_data.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    // Then we copy the 6 columns of the sprite.
    for _ in 1..7 {
        // Top row is empty
        sprite_data.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        // 6 tiles
        for _ in 1..7 {
            // 16 bytes per tiles
            for _ in 0..16 {
                sprite_data.push(source_data[src_pos]);
                src_pos += 1;
            }
        }
    }

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
