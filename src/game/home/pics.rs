use crate::{
    cpu::Cpu,
    game::{engine::menus::pokedex, ram::wram},
    rom::CRYSTAL_ROM,
};

#[rustfmt::skip]
const REVERSE_LOOKUP: [u8; 256] = [
    0,  128, 64, 192, 32, 160,  96, 224, 16, 144, 80, 208, 48, 176, 112, 240,
    8,  136, 72, 200, 40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248,
    4,  132, 68, 196, 36, 164, 100, 228, 20, 148, 84, 212, 52, 180, 116, 244,
    12, 140, 76, 204, 44, 172, 108, 236, 28, 156, 92, 220, 60, 188, 124, 252,
    2,  130, 66, 194, 34, 162,  98, 226, 18, 146, 82, 210, 50, 178, 114, 242,
    10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122, 250,
    6,  134, 70, 198, 38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246,
    14, 142, 78, 206, 46, 174, 110, 238, 30, 158, 94, 222, 62, 190, 126, 254,
    1,  129, 65, 193, 33, 161,  97, 225, 17, 145, 81, 209, 49, 177, 113, 241,
    9,  137, 73, 201, 41, 169, 105, 233, 25, 153, 89, 217, 57, 185, 121, 249,
    5,  133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53, 181, 117, 245,
    13, 141, 77, 205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253,
    3,  131, 67, 195, 35, 163,  99, 227, 19, 147, 83, 211, 51, 179, 115, 243,
    11, 139, 75, 203, 43, 171, 107, 235, 27, 155, 91, 219, 59, 187, 123, 251,
    7,  135, 71, 199, 39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119, 247,
    15, 143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63, 191, 127, 255
];

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
            log::error!("Unknown dimensions: {}x{}", w, h);
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

    // These images aren't in Pokemon Crystal, so use the old sprites
    const FOSSIL_KABUTOPS: u8 = 0xb6;
    const FOSSIL_AERODACTYL: u8 = 0xb7;
    const MON_GHOST: u8 = 0xb8;

    if pokemon_index == FOSSIL_KABUTOPS
        || pokemon_index == FOSSIL_AERODACTYL
        || pokemon_index == MON_GHOST
    {
        let sprite_data = match pokemon_index {
            FOSSIL_KABUTOPS => {
                pokemon_sprite_compression::gen1::decompress(&crate::rom::ROM[0x02fb92..])
            }
            FOSSIL_AERODACTYL => {
                pokemon_sprite_compression::gen1::decompress(&crate::rom::ROM[0x0367a1..])
            }
            MON_GHOST => pokemon_sprite_compression::gen1::decompress(&crate::rom::ROM[0x036920..]),
            _ => unreachable!(),
        };

        let width = ((sprite_data.len() / 0x10) as f64).sqrt() as u8;
        let sprite_data = center_pokemon_sprite(&sprite_data, width, width);

        for (offset, byte) in sprite_data.iter().enumerate() {
            cpu.write_byte(cpu.de() + offset as u16, *byte);
        }

        cpu.pc = cpu.stack_pop();
        return;
    }

    // Fall back to GameBoy function if the pokemon is not in the pokedex
    if pokedex_no == 0 {
        log::error!(
            "Unknown pokemon index passed to load_mon_front_sprite: {}",
            pokemon_index
        );

        // push de
        cpu.stack_push(cpu.de());
        cpu.pc += 1;
        cpu.cycle(16);
        return;
    }

    const POKEMON_BASE_DATA: usize = (0x14 * 0x4000) | (0x5424 & 0x3fff);
    const BASE_DATA_SIZE: usize = 32;

    let offset = POKEMON_BASE_DATA + (pokedex_no - 1) * BASE_DATA_SIZE;
    assert_eq!(CRYSTAL_ROM[offset], pokedex_no as u8);

    let dimensions = CRYSTAL_ROM[offset + 17];
    let mut source_data = read_crystal_pokemon_sprite(pokedex_no, false);

    if cpu.read_byte(wram::W_SPRITE_FLIPPED) != 0 {
        for byte in source_data.iter_mut() {
            *byte = REVERSE_LOOKUP[*byte as usize];
        }
    }

    let sprite_data = center_pokemon_sprite(&source_data, dimensions >> 4, dimensions & 0xf);

    for (offset, byte) in sprite_data.iter().enumerate() {
        cpu.write_byte(cpu.de() + offset as u16, *byte);
    }

    cpu.pc = cpu.stack_pop();
}
