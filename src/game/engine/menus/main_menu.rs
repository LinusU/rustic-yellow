use crate::{
    cpu::Cpu,
    game::{
        constants, home, macros,
        ram::{hram, wram},
    },
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

    loop {
        home::delay::delay_frames(cpu, 20);

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

        home::copy2::clear_screen(cpu);

        cpu.stack_push(0x0001);
        home::palettes::run_default_palette_command(cpu);

        cpu.call(0x36a3); // call LoadTextBoxTilePatterns
        cpu.call(0x3683); // call LoadFontTilePatterns

        // Print text with no delay between each letter
        {
            let v = cpu.read_byte(wram::W_D730);
            cpu.write_byte(wram::W_D730, v | (1 << 6));
        }

        if has_saves {
            // there's a save file
            home::text::text_box_border(cpu, 0, 0, 13, 6);
            home::text::place_string(cpu, 2, 2, "CONTINUE");
            home::text::place_string(cpu, 2, 4, "NEW GAME");
            home::text::place_string(cpu, 2, 6, "OPTION");
        } else {
            home::text::text_box_border(cpu, 0, 0, 13, 4);
            home::text::place_string(cpu, 2, 2, "NEW GAME");
            home::text::place_string(cpu, 2, 4, "OPTION");
        }

        // Print text with delay between each letter
        {
            let v = cpu.read_byte(wram::W_D730);
            cpu.write_byte(wram::W_D730, v & !(1 << 6));
        }

        cpu.call(0x231c); // call UpdateSprites

        cpu.write_byte(wram::W_CURRENT_MENU_ITEM, 0);
        cpu.write_byte(wram::W_LAST_MENU_ITEM, 0);
        cpu.write_byte(wram::W_MENU_JOYPAD_POLL_COUNT, 0);

        cpu.write_byte(wram::W_TOP_MENU_ITEM_X, 1);
        cpu.write_byte(wram::W_TOP_MENU_ITEM_Y, 2);

        cpu.write_byte(
            wram::W_MENU_WATCHED_KEYS,
            constants::input_constants::A_BUTTON
                | constants::input_constants::B_BUTTON
                | constants::input_constants::START,
        );

        {
            let v = cpu.read_byte(wram::W_SAVE_FILE_STATUS);
            cpu.write_byte(wram::W_MAX_MENU_ITEM, v);
        }

        cpu.call(0x3aab); // call HandleMenuInput

        if (cpu.a & constants::input_constants::B_BUTTON) != 0 {
            return cpu.jump(0x4171); // jump DisplayTitleScreen
        }

        home::delay::delay_frames(cpu, 20);

        cpu.b = cpu.read_byte(wram::W_CURRENT_MENU_ITEM);

        if cpu.read_byte(wram::W_SAVE_FILE_STATUS) != 2 {
            // If there's no save file, increment the current menu item so that the numbers
            // are the same whether or not there's a save file.
            cpu.b += 1;
        }

        match cpu.b {
            0 => {
                main_menu_select_save(cpu);
            }
            1 => {
                return cpu.jump(0x5cd2); // StartNewGame
            }
            2 => {
                cpu.call(0x5df2); // DisplayOptionMenu
                cpu.write_byte(wram::W_OPTIONS_INITIALIZED, 1);
            }
            _ => unreachable!("Invalid menu item: {}", cpu.b),
        }
    }
}

fn main_menu_select_save(cpu: &mut Cpu) {
    let list = match saves::list_save_files() {
        Ok(ref files) if files.is_empty() => {
            return;
        }
        Ok(files) => files,
        Err(error) => {
            eprintln!("Error listing save files: {}", error);
            return;
        }
    };

    let first_page = &list[0..8.min(list.len())];
    let height = (first_page.len() as u8) * 2;

    home::text::text_box_border(cpu, 0, 0, 18, height);

    for (i, save) in first_page.iter().enumerate() {
        let y = (i as u8) * 2 + 2;
        home::text::place_string(cpu, 2, y, &save.name);
    }

    cpu.write_byte(wram::W_MAX_MENU_ITEM, (first_page.len() as u8) - 1);
    cpu.call(0x3aab); // call HandleMenuInput

    if (cpu.a & constants::input_constants::B_BUTTON) != 0 {
        return;
    }

    let selected = cpu.read_byte(wram::W_CURRENT_MENU_ITEM) as usize;
    let save = &list[selected];

    cpu.replace_ram(std::fs::read(&save.path).unwrap());

    macros::predef::predef_call!(cpu, LoadSAV);

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
            return cpu.jump(0x5c83); // MainMenu.pressedA
        }

        if (btn & constants::input_constants::B_BUTTON) != 0 {
            break;
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
