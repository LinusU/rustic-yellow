use crate::{
    cpu::Cpu,
    game::{
        constants, home, macros,
        ram::wram, engine,
    },
    saves,
};

pub fn start_menu_save_reset(cpu: &mut Cpu) {
    let list = match saves::list_save_files() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error listing save files: {}", e);
            cpu.call(0x36f8); // LoadScreenTilesFromBuffer2
            cpu.pc = 0x28cf;
            return;
        }
    };

    let first_page = &list[0..7.min(list.len())];
    let height = (list.len() as u8) * 2 + 2;

    home::text::text_box_border(cpu, 0, 0, 18, height);
    home::text::place_string(cpu, 2, 2, "NEW FILE");

    for (i, save) in first_page.iter().enumerate() {
        let y = (i as u8) * 2 + 4;
        home::text::place_string(cpu, 2, y, &save.name);
    }

    cpu.write_byte(
        wram::W_MENU_WATCHED_KEYS,
        constants::input_constants::A_BUTTON
            | constants::input_constants::B_BUTTON
            | constants::input_constants::START,
    );

    cpu.write_byte(wram::W_TOP_MENU_ITEM_X, 1);
    cpu.write_byte(wram::W_CURRENT_MENU_ITEM, 0);
    cpu.write_byte(wram::W_MAX_MENU_ITEM, first_page.len() as u8);

    cpu.call(0x3aab); // call HandleMenuInput

    if (cpu.a & constants::input_constants::B_BUTTON) != 0 {
        cpu.call(0x36f8); // LoadScreenTilesFromBuffer2
        cpu.pc = 0x28cf;
        return;
    }

    let selected = cpu.read_byte(wram::W_CURRENT_MENU_ITEM) as usize;

    if selected == 0 {
        let name = engine::menus::naming_screen::display_naming_screen(cpu, "Save file name:");
        assert!(!name.is_empty());
        save_game_to_disk(cpu, &saves::create_save_file(name));
    } else {
        save_game_to_disk(cpu, &list[selected - 1]);
    }

    cpu.call(0x36f8); // LoadScreenTilesFromBuffer2
    cpu.pc = 0x28cf;
}

fn save_game_to_disk(cpu: &mut Cpu, save: &saves::SaveFile) {
    macros::farcall::farcall!(cpu, SaveSAVtoSRAM);

    // Try to backup current file if it exists
    let _ = std::fs::rename(&save.path, save.path.with_extension("bak"));

    std::fs::write(&save.path, cpu.borrow_ram()).unwrap();
}
