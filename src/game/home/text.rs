use crate::{cpu::Cpu, game::home, game::macros};

/// Draw a `w` × `h` text box at `x`, `y`.
pub fn text_box_border(cpu: &mut Cpu, x: u8, y: u8, w: u8, h: u8) {
    cpu.set_hl(macros::coords::coord!(x, y));
    cpu.b = h;
    cpu.c = w;
    cpu.call(0x16f0);
}

pub fn place_string(cpu: &mut Cpu, x: u8, y: u8, string: &str) {
    let ptr = macros::coords::coord!(x, y);

    for (off, chr) in string.chars().enumerate() {
        cpu.write_byte(
            ptr + (off as u16),
            match chr {
                ' ' => 0x7f,
                ':' => 0x9c,
                '▶' => 0xed,
                '0'..='9' => (chr as u8) - ('0' as u8) + 0xf6,
                'A'..='Z' => (chr as u8) - ('A' as u8) + 0x80,
                'a'..='z' => (chr as u8) - ('a' as u8) + 0xa0,
                _ => panic!("Invalid character: {}", chr),
            },
        );

        home::print_text::print_letter_delay(cpu);
    }
}
