use crate::{
    cpu::Cpu,
    game::{engine::menus::pokedex, ram::wram},
    rom::CRYSTAL_ROM,
};

pub fn center_pokemon_sprite(input: &[u8], w: u8, h: u8) -> [u8; 7 * 7 * 16] {
    let mut sprite_data = [0; 7 * 7 * 16];

    match (w, h) {
        (5, 5) => {
            for offset in (9 * 16)..(14 * 16) {
                sprite_data[offset] = input[offset - (9 * 16)];
            }

            for offset in (16 * 16)..(21 * 16) {
                sprite_data[offset] = input[offset - (11 * 16)];
            }

            for offset in (23 * 16)..(28 * 16) {
                sprite_data[offset] = input[offset - (13 * 16)];
            }

            for offset in (30 * 16)..(35 * 16) {
                sprite_data[offset] = input[offset - (15 * 16)];
            }

            for offset in (37 * 16)..(42 * 16) {
                sprite_data[offset] = input[offset - (17 * 16)];
            }
        }
        (6, 6) => {
            for offset in (8 * 16)..(14 * 16) {
                sprite_data[offset] = input[offset - (8 * 16)];
            }

            for offset in (15 * 16)..(21 * 16) {
                sprite_data[offset] = input[offset - (9 * 16)];
            }

            for offset in (22 * 16)..(28 * 16) {
                sprite_data[offset] = input[offset - (10 * 16)];
            }

            for offset in (29 * 16)..(35 * 16) {
                sprite_data[offset] = input[offset - (11 * 16)];
            }

            for offset in (36 * 16)..(42 * 16) {
                sprite_data[offset] = input[offset - (12 * 16)];
            }

            for offset in (43 * 16)..(49 * 16) {
                sprite_data[offset] = input[offset - (13 * 16)];
            }
        }
        (7, 7) => {
            for (offset, byte) in input.iter().enumerate().take(7 * 7 * 16) {
                sprite_data[offset] = *byte;
            }
        }
        _ => {
            eprintln!("Unknown dimensions: {}x{}", w, h);
        }
    }

    sprite_data
}

pub fn read_crystal_pokemon_sprite(pokedex_no: usize, back: bool) -> Vec<u8> {
    const POKEMON_PIC_POINTERS: usize = 0x48 * 0x4000;
    const PICS_FIX: usize = 0x36;

    let mut offset = POKEMON_PIC_POINTERS + (pokedex_no - 1) * 6;

    if back {
        offset += 3;
    }

    let bank = (CRYSTAL_ROM[offset] as usize) + PICS_FIX;
    let addr = CRYSTAL_ROM[offset + 1] as usize + ((CRYSTAL_ROM[offset + 2] as usize) << 8);

    let ptr = (bank * 0x4000) | (addr & 0x3fff);
    pokemon_sprite_compression::gen2::decompress(&CRYSTAL_ROM[ptr..])
}

// de: destination location
pub fn load_mon_front_sprite(cpu: &mut Cpu) {
    let pokemon_index = cpu.read_byte(wram::W_MON_H_INDEX);
    let pokedex_no = pokedex::index_to_pokedex(pokemon_index) as usize;

    const POKEMON_BASE_DATA: usize = (0x14 * 0x4000) | (0x5424 & 0x3fff);
    const BASE_DATA_SIZE: usize = 32;

    let offset = POKEMON_BASE_DATA + (pokedex_no - 1) * BASE_DATA_SIZE;
    assert_eq!(CRYSTAL_ROM[offset], pokedex_no as u8);

    let dimensions = CRYSTAL_ROM[offset + 17];
    let source_data = read_crystal_pokemon_sprite(pokedex_no, false);

    let sprite_data = center_pokemon_sprite(&source_data, dimensions >> 4, dimensions & 0xf);

    for (offset, byte) in sprite_data.iter().enumerate() {
        cpu.write_byte(cpu.de() + offset as u16, *byte);
    }

    cpu.pc = cpu.stack_pop();
}
