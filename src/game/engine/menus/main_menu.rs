use crate::{
    cpu::Cpu,
    game::{
        constants, home, macros,
        ram::{hram, wram},
    },
    saves, KeypadKey,
};

pub fn main_menu(cpu: &mut Cpu) {
    // FIXME: Implement our own audio system that isn't dependent of the CPU cycling
    cpu.call(0x2233); // StopAllMusic

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

    cpu.stack_push(0x0001);
    home::palettes::run_default_palette_command(cpu);

    cpu.call(0x36a3); // call LoadTextBoxTilePatterns
    cpu.call(0x3683); // call LoadFontTilePatterns

    let layer = cpu.gpu_push_layer();

    if has_saves {
        home::text::text_box_border(cpu.gpu_mut_layer(layer), 0, 0, 13, 6);
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, 2, "CONTINUE");
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, 4, "NEW GAME");
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, 6, "OPTION");
    } else {
        home::text::text_box_border(cpu.gpu_mut_layer(layer), 0, 0, 13, 4);
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, 2, "NEW GAME");
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, 4, "OPTION");
    }

    let mut selected = 0;
    let max_menu_item = if has_saves { 2 } else { 1 };

    loop {
        cpu.gpu_mut_layer(layer)
            .set_background(1, selected * 2 + 2, home::text::SELECTED_ITEM);

        cpu.gpu_update_screen();
        let key = cpu.keypad_wait();

        match key {
            KeypadKey::B => {
                cpu.gpu_pop_layer(layer);
                return cpu.jump(0x4171); // jump DisplayTitleScreen
            }

            KeypadKey::Up if selected > 0 => {
                cpu.gpu_mut_layer(layer)
                    .set_background(1, selected * 2 + 2, home::text::EMPTY);
                selected -= 1;
                continue;
            }

            KeypadKey::Down if selected < max_menu_item => {
                cpu.gpu_mut_layer(layer)
                    .set_background(1, selected * 2 + 2, home::text::EMPTY);
                selected += 1;
                continue;
            }

            KeypadKey::A => {}
            _ => {
                continue;
            }
        }

        // If there's no save file, increment the current menu item so that the numbers
        // are the same whether or not there's a save file.
        let selected = if has_saves { selected } else { selected + 1 };

        match selected {
            0 => {
                if main_menu_select_save(cpu) {
                    cpu.gpu_pop_layer(layer);

                    if main_menu_display_save_info(cpu) {
                        return cpu.jump(0x5c83); // MainMenu.pressedA
                    } else {
                        return cpu.jump(0x4171); // jump DisplayTitleScreen
                    }
                }
            }
            1 => {
                cpu.gpu_pop_layer(layer);
                return cpu.jump(0x5cd2); // StartNewGame
            }
            2 => {
                cpu.gpu_pop_layer(layer);
                cpu.call(0x5df2); // DisplayOptionMenu
                cpu.write_byte(wram::W_OPTIONS_INITIALIZED, 1);
                return cpu.jump(0x4171); // jump DisplayTitleScreen
            }
            _ => unreachable!("Invalid menu item: {}", selected),
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

    let first_page = &list[0..8.min(list.len())];
    let height = first_page.len() * 2;

    let layer = cpu.gpu_push_layer();
    home::text::text_box_border(cpu.gpu_mut_layer(layer), 0, 0, 18, height);

    for (i, save) in first_page.iter().enumerate() {
        home::text::place_string(cpu.gpu_mut_layer(layer), 2, i * 2 + 2, &save.name);
    }

    let mut selected = 0;

    loop {
        cpu.gpu_mut_layer(layer)
            .set_background(1, selected * 2 + 2, home::text::SELECTED_ITEM);

        cpu.gpu_update_screen();
        let key = cpu.keypad_wait();

        match key {
            KeypadKey::B => {
                cpu.gpu_pop_layer(layer);
                return false;
            }

            KeypadKey::Up if selected > 0 => {
                cpu.gpu_mut_layer(layer)
                    .set_background(1, selected * 2 + 2, home::text::EMPTY);
                selected -= 1;
                continue;
            }

            KeypadKey::Down if selected < first_page.len() - 1 => {
                cpu.gpu_mut_layer(layer)
                    .set_background(1, selected * 2 + 2, home::text::EMPTY);
                selected += 1;
                continue;
            }

            KeypadKey::A => {
                break;
            }
            _ => {
                continue;
            }
        }
    }

    let save = &list[selected];

    cpu.replace_ram(std::fs::read(&save.path).unwrap());

    macros::predef::predef_call!(cpu, LoadSAV);

    cpu.gpu_pop_layer(layer);
    true
}

fn main_menu_display_save_info(cpu: &mut Cpu) -> bool {
    cpu.call(0x5d1f); // DisplayContinueGameInfo

    {
        let v = cpu.read_byte(wram::W_CURRENT_MAP_SCRIPT_FLAGS);
        cpu.write_byte(wram::W_CURRENT_MAP_SCRIPT_FLAGS, v | (1 << 5));
    }

    loop {
        cpu.write_byte(hram::H_JOY_PRESSED, 0);
        cpu.write_byte(hram::H_JOY_RELEASED, 0);
        cpu.write_byte(hram::H_JOY_HELD, 0);
        cpu.call(0x01b9); // Joypad

        let btn = cpu.read_byte(hram::H_JOY_HELD);

        if (btn & constants::input_constants::A_BUTTON) != 0 {
            return true;
        }

        if (btn & constants::input_constants::B_BUTTON) != 0 {
            return false;
        }
    }
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
