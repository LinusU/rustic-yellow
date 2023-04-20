use crate::{cpu::Cpu, game::{home, constants, macros}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Case {
    Upper,
    Lower,
}

pub fn display_naming_screen(cpu: &mut Cpu, title: &str) -> String {
    home::palettes::gb_pal_white_out_with_delay3(cpu);
    home::copy2::clear_screen(cpu);
    cpu.call(0x231c); // UpdateSprites
    home::palettes::run_palette_command(cpu, constants::palette_constants::SET_PAL_GENERIC);
    cpu.call(0x36c3); // LoadHpBarAndStatusTilePatterns

    macros::farcall::farcall!(cpu, LoadEDTile);
    macros::farcall::farcall!(cpu, LoadMonPartySpriteGfx);

    home::text::text_box_border(cpu, 0, 4, 18, 9);
    home::text::place_string(cpu, 0, 1, title);

    let mut case = Case::Upper;
    let mut result = String::new();

    let mut cursor = (0, 0);

    loop {
        print_alphabet(cpu, case);
        home::text::place_string(cpu, 1, 2, &result);
        home::text::place_string(cpu, 1 + (result.len() as u8), 2, " ");
        home::text::place_string(cpu, 1 + (cursor.0 * 2), 5 + (cursor.1 * 2), "▶");

        home::palettes::gb_pal_normal(cpu);
        home::palettes::delay3(cpu);

        let btn = home::joypad2::joypad_low_sensitivity(cpu, home::joypad2::JoypadLowSensitivityMode::GetNewlyPressedButtonsOnly);

        if btn == 0 {
            continue;
        }

        match btn.trailing_zeros() as u8 {
            constants::input_constants::BIT_A_BUTTON => {
                let c = match cursor {
                    (8, 2) => ' ',
                    (_, 3) => ' ',
                    (_, 4) => ' ',
                    _ => match case {
                        Case::Upper => (('A' as u8) + (cursor.1 * 9 + cursor.0)) as char,
                        Case::Lower => (('a' as u8) + (cursor.1 * 9 + cursor.0)) as char,
                    },
                };

                result.push(c);
                home::audio::play_sound(cpu, constants::music_constants::SFX_PRESS_AB);
            }
            constants::input_constants::BIT_B_BUTTON => {
                if result.len() > 0 {
                    result.pop();
                    home::audio::play_sound(cpu, constants::music_constants::SFX_PRESS_AB);
                }
            }
            constants::input_constants::BIT_SELECT => {
                case = match case {
                    Case::Upper => Case::Lower,
                    Case::Lower => Case::Upper,
                };
            }
            constants::input_constants::BIT_START => {
                if result.len() > 0 {
                    return result;
                }
            }
            constants::input_constants::BIT_D_RIGHT => {
                if cursor.0 < 8 {
                    cursor.0 += 1;
                } else {
                    cursor.0 = 0;
                }
            }
            constants::input_constants::BIT_D_LEFT => {
                if cursor.0 > 0 {
                    cursor.0 -= 1;
                } else {
                    cursor.0 = 8;
                }
            }
            constants::input_constants::BIT_D_UP => {
                if cursor.1 > 0 {
                    cursor.1 -= 1;
                } else {
                    cursor.1 = 4;
                }
            }
            constants::input_constants::BIT_D_DOWN => {
                if cursor.1 < 4 {
                    cursor.1 += 1;
                } else {
                    cursor.1 = 0;
                }
            }
            _ => unreachable!()
        }
    }
}

fn print_alphabet(cpu: &mut Cpu, case: Case) {
    match case {
        Case::Upper => {
            home::text::place_string(cpu, 1, 5, " A B C D E F G H I");
            home::text::place_string(cpu, 1, 7, " J K L M N O P Q R");
            home::text::place_string(cpu, 1, 9, " S T U V W X Y Z  ");
        }
        Case::Lower => {
            home::text::place_string(cpu, 1, 5, " a b c d e f g h i");
            home::text::place_string(cpu, 1, 7, " j k l m n o p q r");
            home::text::place_string(cpu, 1, 9, " s t u v w x y z  ");
        }
    }

    home::text::place_string(cpu, 1, 11, "                  ");
    home::text::place_string(cpu, 1, 13, "                 ");
    cpu.write_byte(macros::coords::coord!(18, 13), 0xf0); // <ED>

    match case {
        Case::Upper => home::text::place_string(cpu, 1, 15, " lower case"),
        Case::Lower => home::text::place_string(cpu, 1, 15, " upper case"),
    }
}
