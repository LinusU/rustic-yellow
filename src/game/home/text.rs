use crate::{
    gpu::{GpuAtlas, GpuLayer, GpuTile},
    save_state::PokeString,
};

const BOX_TOP_LEFT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 0, 0);
const BOX_TOP_RIGHT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 2, 0);
const BOX_BOTTOM_LEFT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 0, 2);
const BOX_BOTTOM_RIGHT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 2, 2);
const BOX_TOP: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 1, 0);
const BOX_BOTTOM: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 1, 2);
const BOX_LEFT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 0, 1);
const BOX_RIGHT: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 2, 1);
const BOX_CENTER: GpuTile = GpuTile::new(GpuAtlas::BoxBorder, 1, 1);

/// Draw a `w` × `h` text box at `x`, `y`.
pub fn text_box_border(layer: &mut GpuLayer, x: usize, y: usize, mut w: usize, mut h: usize) {
    // Add two to account for border, and remove one to make comparisons easier.
    w += 1;
    h += 1;

    for dy in 0..=h {
        for dx in 0..=w {
            let tile = match (dx, dy) {
                (0, 0) => BOX_TOP_LEFT,
                (dx, 0) if dx == w => BOX_TOP_RIGHT,
                (0, dy) if dy == h => BOX_BOTTOM_LEFT,
                (dx, dy) if dx == w && dy == h => BOX_BOTTOM_RIGHT,
                (0, _) => BOX_LEFT,
                (dx, _) if dx == w => BOX_RIGHT,
                (_, 0) => BOX_TOP,
                (_, dy) if dy == h => BOX_BOTTOM,
                _ => BOX_CENTER,
            };

            layer.set_background(x + dx, y + dy, tile);
        }
    }
}

pub fn place_char(layer: &mut GpuLayer, x: usize, y: usize, chr: char) {
    let tile = match chr {
        'A'..='P' => GpuTile::new(GpuAtlas::Font, (chr as usize) - ('A' as usize), 0),
        'Q'..='Z' => GpuTile::new(GpuAtlas::Font, (chr as usize) - ('Q' as usize), 1),
        ':' => GpuTile::new(GpuAtlas::Font, 12, 1),
        ';' => GpuTile::new(GpuAtlas::Font, 13, 1),
        'a'..='p' => GpuTile::new(GpuAtlas::Font, (chr as usize) - ('a' as usize), 2),
        'q'..='z' => GpuTile::new(GpuAtlas::Font, (chr as usize) - ('q' as usize), 3),
        'é' => GpuTile::new(GpuAtlas::Font, 10, 3),
        ' ' => GpuTile::new(GpuAtlas::Font, 15, 3),
        '𝗟' => GpuTile::new(GpuAtlas::Font, 14, 5),
        '\'' => GpuTile::new(GpuAtlas::Font, 0, 6),
        '𝔭' => GpuTile::new(GpuAtlas::Font, 1, 6),
        '𝔪' => GpuTile::new(GpuAtlas::Font, 2, 6),
        '-' => GpuTile::new(GpuAtlas::Font, 3, 6),
        '?' => GpuTile::new(GpuAtlas::Font, 6, 6),
        '!' => GpuTile::new(GpuAtlas::Font, 7, 6),
        '.' => GpuTile::new(GpuAtlas::Font, 8, 6),
        '▷' => GpuTile::new(GpuAtlas::Font, 12, 6),
        '▶' => GpuTile::new(GpuAtlas::Font, 13, 6),
        '♂' => GpuTile::new(GpuAtlas::Font, 15, 6),
        '¥' => GpuTile::new(GpuAtlas::Font, 0, 7),
        '/' => GpuTile::new(GpuAtlas::Font, 3, 7),
        ',' => GpuTile::new(GpuAtlas::Font, 4, 7),
        '♀' => GpuTile::new(GpuAtlas::Font, 5, 7),
        '0'..='9' => GpuTile::new(GpuAtlas::Font, (chr as usize) - ('0' as usize) + 6, 7),
        _ => panic!("Invalid character: {}", chr),
    };

    layer.set_background(x, y, tile);
}

pub fn place_string(layer: &mut GpuLayer, x: usize, y: usize, string: &str) {
    for (idx, chr) in string.chars().enumerate() {
        place_char(layer, x + idx, y, chr);
    }
}

pub fn place_poke_string(layer: &mut GpuLayer, mut x: usize, y: usize, string: &PokeString) {
    for byte in string {
        match byte {
            0x80..=0xff => {
                let offset = (byte as usize) - 0x80;
                let tile = GpuTile::new(GpuAtlas::Font, offset % 16, offset / 16);

                layer.set_background(x, y, tile);
                x += 1;
            }

            _ => todo!("poke string: {:02x}", byte),
        }
    }
}
