use crate::{
    cpu::Cpu,
    game::{
        constants,
        engine::movie,
        home,
        ram::{hram, vram, wram},
    },
    keypad::{KeypadKey, TextEvent},
    saves,
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
                if main_menu_new_game(cpu) {
                    cpu.gpu_pop_layer(layer);
                    prepare_for_game(cpu);
                    return cpu.jump(0x5cd2); // StartNewGame
                }
            }

            _ => unreachable!(),
        }
    }
}

fn main_menu_new_game(cpu: &mut Cpu) -> bool {
    let layer = cpu.gpu_push_layer();

    home::text::text_box_border(cpu.gpu_mut_layer(layer), 1, 2, 16, 6);
    home::text::place_string(cpu.gpu_mut_layer(layer), 2, 4, "Save file name:");

    let mut result = String::new();

    loop {
        home::text::place_string(cpu.gpu_mut_layer(layer), 3, 7, &format!("{:-<14}", result));

        cpu.gpu_update_screen();
        let event = cpu.keyboard_text();

        match event {
            TextEvent::Append(c) => {
                if result.len() < 14 {
                    result.push(c);
                }
            }

            TextEvent::Delete => {
                result.pop();
            }

            TextEvent::Cancel => {
                cpu.gpu_pop_layer(layer);
                return false;
            }

            TextEvent::Submit => {
                if result.is_empty() {
                    continue;
                }

                if !saves::save_is_free(&result) {
                    cpu.play_sfx(0x02, 0x41ef, 0, 0);
                    continue;
                }

                saves::create_save_dir().unwrap();

                cpu.set_save_path(saves::get_save_path(&result));

                cpu.gpu_pop_layer(layer);
                return true;
            }
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
                let data = std::fs::read(&save.path).unwrap();

                if display_continue_game_info(cpu, &data) {
                    cpu.replace_ram(data);
                    cpu.set_save_path(save.path.clone());
                    super::save::load_sav(cpu);
                    cpu.gpu_pop_layer(layer);
                    return true;
                }
            }
        }
    }
}

fn display_continue_game_info(cpu: &mut Cpu, data: &[u8]) -> bool {
    let summary = super::save::load_sav_summary(data);

    let layer = cpu.gpu_push_layer();

    home::text::text_box_border(cpu.gpu_mut_layer(layer), 4, 7, 14, 8);

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 9, "PLAYER");
    home::text::place_string(cpu.gpu_mut_layer(layer), 12, 9, &summary.player_name);

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 11, "BADGES");
    home::text::place_string(
        cpu.gpu_mut_layer(layer),
        17,
        11,
        &format!("{:2}", summary.num_badges),
    );

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 13, "POKéDEX");
    home::text::place_string(
        cpu.gpu_mut_layer(layer),
        16,
        13,
        &format!("{:3}", summary.owned_mons),
    );

    home::text::place_string(cpu.gpu_mut_layer(layer), 5, 15, "TIME");
    home::text::place_string(
        cpu.gpu_mut_layer(layer),
        13,
        15,
        &format!(
            "{:3}:{:02}",
            summary.play_time_hh_mm.0, summary.play_time_hh_mm.1
        ),
    );

    cpu.gpu_update_screen();

    let result = loop {
        match cpu.keypad_wait() {
            KeypadKey::A => {
                cpu.play_sfx(0x02, 0x41b0, 0, 0); // SFX_Press_AB
                break true;
            }
            KeypadKey::B => {
                cpu.play_sfx(0x02, 0x41b0, 0, 0); // SFX_Press_AB
                break false;
            }
            _ => {}
        }
    };

    cpu.gpu_pop_layer(layer);
    result
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
