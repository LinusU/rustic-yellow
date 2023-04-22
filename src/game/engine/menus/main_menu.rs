use crate::{
    cpu::Cpu,
    game::{
        constants,
        engine::movie,
        home,
        ram::{hram, sram, vram, wram},
    },
    saves, KeypadKey,
};

pub fn main_menu(cpu: &mut Cpu) {
    init_options(cpu);

    cpu.write_byte(wram::W_OPTIONS_INITIALIZED, 0);

    let has_saves = match saves::list_save_files() {
        Ok(files) => !files.is_empty(),
        Err(e) => {
            eprintln!("Error listing save files: {}", e);
            false
        }
    };

    cpu.write_byte(wram::W_SAVE_FILE_STATUS, if has_saves { 2 } else { 1 });

    cpu.write_byte(
        wram::W_LINK_STATE,
        constants::serial_constants::LINK_STATE_NONE,
    );

    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 1, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 2, 0);
    cpu.write_byte(wram::W_PARTY_AND_BILLS_PC_SAVED_MENU_ITEM + 3, 0);

    cpu.write_byte(wram::W_DEFAULT_MAP, 0);

    // Toggle link feature bit off
    {
        let v = cpu.read_byte(wram::W_D72E);
        cpu.write_byte(wram::W_D72E, v & !(1 << 6));
    }

    let mut selected = 0;
    let layer = cpu.gpu_push_layer();

    loop {
        let selected = super::menu_single_choice(
            cpu,
            layer,
            &mut selected,
            (0, 0),
            &["CONTINUE", "NEW GAME"][if has_saves { 0.. } else { 1.. }],
        );

        match (selected, has_saves) {
            (None, _) => {
                cpu.gpu_pop_layer(layer);
                return;
            }

            (Some(0), true) => {
                if main_menu_select_save(cpu) {
                    cpu.gpu_pop_layer(layer);
                    prepare_for_game(cpu);
                    return cpu.jump(0x5c83); // MainMenu.pressedA
                }
            }

            (Some(0), false) | (Some(1), true) => {
                cpu.gpu_pop_layer(layer);
                prepare_for_game(cpu);
                return cpu.jump(0x5cd2); // StartNewGame
            }

            _ => unreachable!(),
        }
    }
}

fn main_menu_select_save(cpu: &mut Cpu) -> bool {
    let list = match saves::list_save_files() {
        Ok(ref files) if files.is_empty() => {
            return false;
        }
        Ok(files) => files,
        Err(error) => {
            eprintln!("Error listing save files: {}", error);
            return false;
        }
    };

    let mut selected = 0;
    let layer = cpu.gpu_push_layer();

    loop {
        let selected = super::menu_single_choice(
            cpu,
            layer,
            &mut selected,
            (0, 0),
            &list
                .iter()
                .map(|save| save.name.as_ref())
                .collect::<Vec<_>>()[..],
        );

        match selected {
            None => {
                cpu.gpu_pop_layer(layer);
                return false;
            }

            Some(selected) => {
                let save = &list[selected];

                cpu.replace_ram(std::fs::read(&save.path).unwrap());

                super::save::load_sav(cpu);

                if display_continue_game_info(cpu) {
                    cpu.gpu_pop_layer(layer);
                    return true;
                }
            }
        }
    }
}

fn display_continue_game_info(cpu: &mut Cpu) -> bool {
    let name = check_for_player_name_in_sram(cpu);
    let badges = cpu.read_byte(wram::W_OBTAINED_BADGES).count_ones();
    let num_owned = read_num_owned_mons(cpu);
    let hours = cpu.read_byte(wram::W_PLAY_TIME_HOURS);
    let minutes = cpu.read_byte(wram::W_PLAY_TIME_MINUTES);

    let layer = cpu.gpu_push_layer();

    home::text::text_box_border(cpu.gpu_mut_layer(layer), 4, 7, 14, 8);

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 9, "PLAYER");
    home::text::place_string(cpu.gpu_mut_layer(layer), 12, 9, &name);

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 11, "BADGES");
    home::text::place_string(cpu.gpu_mut_layer(layer), 17, 11, &format!("{:2}", badges));

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 13, "POKéDEX");
    home::text::place_string(
        cpu.gpu_mut_layer(layer),
        16,
        13,
        &format!("{:3}", num_owned),
    );

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 15, "TIME");
    home::text::place_string(
        cpu.gpu_mut_layer(layer),
        13,
        15,
        &format!("{:3}:{:02}", hours, minutes),
    );

    cpu.gpu_update_screen();

    let result = loop {
        match cpu.keypad_wait() {
            KeypadKey::A => {
                break true;
            }
            KeypadKey::B => {
                break false;
            }
            _ => {}
        }
    };

    cpu.gpu_pop_layer(layer);
    result
}

/// Check if the player name data in SRAM has a string terminator character
/// (indicating that a name may have been saved there) and return whether it does
pub fn check_for_player_name_in_sram(cpu: &mut Cpu) -> String {
    super::save::enable_sram_and_latch_clock_data(cpu);
    cpu.write_byte(constants::hardware_constants::MBC1_SRAM_BANK, 1);

    let mut result = String::with_capacity(constants::text_constants::NAME_LENGTH as usize);

    for i in 0..=constants::text_constants::NAME_LENGTH {
        let ch = cpu.read_byte(sram::S_PLAYER_NAME + (i as u16));

        match ch {
            0x50 => {
                break;
            }
            0x80..=0x99 => {
                result.push((('A' as u8) + (ch - 0x80)) as char);
            }
            0xa0..=0xb9 => {
                result.push((('a' as u8) + (ch - 0xa0)) as char);
            }
            0xf6..=0xff => {
                result.push((('0' as u8) + (ch - 0xf6)) as char);
            }
            _ => panic!("Invalid character in player name: {:02x}", ch),
        }
    }

    super::save::disable_sram_and_prepare_clock_data(cpu);

    result
}

fn read_num_owned_mons(cpu: &mut Cpu) -> u32 {
    let mut num_owned = 0;

    for addr in wram::W_POKEDEX_OWNED..wram::W_POKEDEX_OWNED_END {
        let byte = cpu.read_byte(addr);
        num_owned += byte.count_ones();
    }

    num_owned
}

pub fn init_options(cpu: &mut Cpu) {
    cpu.write_byte(
        wram::W_LETTER_PRINTING_DELAY_FLAGS,
        constants::misc_constants::TEXT_DELAY_FAST,
    );
    cpu.write_byte(
        wram::W_OPTIONS,
        constants::misc_constants::TEXT_DELAY_MEDIUM,
    );
    cpu.write_byte(wram::W_PRINTER_SETTINGS, 64); // audio?
}

/// These things were handled when navigating to, and when running the main
/// menu. The rest of the game depends on them being set up, so we do it here.
fn prepare_for_game(cpu: &mut Cpu) {
    home::palettes::gb_pal_white_out_with_delay3(cpu);

    cpu.stack_push(0x0001);
    home::palettes::run_default_palette_command(cpu);

    cpu.write_byte(hram::H_WY, 0);
    cpu.write_byte(hram::H_AUTO_BG_TRANSFER_ENABLED, 1);

    cpu.call(0x16dd); // ClearScreen
    cpu.call(0x0082); // ClearSprites

    movie::title::title_screen_copy_tile_map_to_vram(cpu, vram::V_BG_MAP0);
    movie::title::title_screen_copy_tile_map_to_vram(cpu, vram::V_BG_MAP1);

    home::palettes::delay3(cpu);
    cpu.call(0x1e6f); // LoadGBPal

    cpu.call(0x3683); // LoadFontTilePatterns
    cpu.call(0x36a3); // LoadTextBoxTilePatterns
}
