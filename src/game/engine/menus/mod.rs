use crate::{
    cpu::Cpu,
    game::{audio, home::text},
    keypad::KeypadKey,
};

pub mod main_menu;
pub mod pokedex;
pub mod save;

pub fn menu_single_choice(
    cpu: &mut Cpu,
    layer: usize,
    selected: &mut usize,
    pos: (usize, usize),
    choices: &[&str],
) -> Option<usize> {
    let width = usize::min(
        choices.iter().map(|s| s.chars().count()).max().unwrap_or(0) + 2,
        18,
    );

    // TODO: Implement scrolling
    let height = usize::min(choices.len(), 8) * 2;

    text::text_box_border(cpu.gpu_mut_layer(layer), pos.0, pos.1, width, height);

    for (i, choice) in choices.iter().enumerate() {
        text::place_string(
            cpu.gpu_mut_layer(layer),
            pos.0 + 2,
            pos.1 + 2 + i * 2,
            choice,
        );
    }

    let max_menu_item = choices.len() - 1;

    text::place_char(
        cpu.gpu_mut_layer(layer),
        pos.0 + 1,
        pos.1 + 2 + *selected * 2,
        '▶',
    );

    loop {
        cpu.gpu_update_screen();
        let key = cpu.keypad_wait();

        match key {
            KeypadKey::A => {
                cpu.play_sfx(audio::sfx::PRESS_AB);
                break Some(*selected);
            }

            KeypadKey::B => {
                cpu.play_sfx(audio::sfx::PRESS_AB);
                break None;
            }

            KeypadKey::Up if *selected > 0 => {
                text::place_char(
                    cpu.gpu_mut_layer(layer),
                    pos.0 + 1,
                    pos.1 + 2 + *selected * 2,
                    ' ',
                );
                *selected -= 1;
                text::place_char(
                    cpu.gpu_mut_layer(layer),
                    pos.0 + 1,
                    pos.1 + 2 + *selected * 2,
                    '▶',
                );
            }

            KeypadKey::Down if *selected < max_menu_item => {
                text::place_char(
                    cpu.gpu_mut_layer(layer),
                    pos.0 + 1,
                    pos.1 + 2 + *selected * 2,
                    ' ',
                );
                *selected += 1;
                text::place_char(
                    cpu.gpu_mut_layer(layer),
                    pos.0 + 1,
                    pos.1 + 2 + *selected * 2,
                    '▶',
                );
            }

            _ => {}
        }
    }
}

fn wait_ab_press(cpu: &mut Cpu) {
    loop {
        match cpu.keypad_wait() {
            KeypadKey::A | KeypadKey::B => {
                cpu.play_sfx(audio::sfx::PRESS_AB);
                break;
            }
            _ => {}
        }
    }
}

pub fn menu_display_confirmation(cpu: &mut Cpu, lines: &[&str]) -> bool {
    let layer = cpu.gpu_push_layer();

    text::text_box_border(cpu.gpu_mut_layer(layer), 0, 12, 18, 4);

    for (x, chr) in lines[0].chars().enumerate() {
        text::place_char(cpu.gpu_mut_layer(layer), x + 1, 14, chr);
        cpu.gpu_update_screen();
    }

    for &line in &lines[1..lines.len() - 1] {
        for (x, chr) in line.chars().enumerate() {
            text::place_char(cpu.gpu_mut_layer(layer), x + 1, 16, chr);
            cpu.gpu_update_screen();
        }

        wait_ab_press(cpu);

        text::place_string(cpu.gpu_mut_layer(layer), 1, 14, "                  ");
        text::place_string(cpu.gpu_mut_layer(layer), 1, 14, line);
        text::place_string(cpu.gpu_mut_layer(layer), 1, 16, "                  ");
    }

    for (x, chr) in lines[lines.len() - 1].chars().enumerate() {
        text::place_char(cpu.gpu_mut_layer(layer), x + 1, 16, chr);
        cpu.gpu_update_screen();
    }

    let result = menu_single_choice(cpu, layer, &mut 0, (13, 6), &["YES", "NO"]);

    cpu.gpu_pop_layer(layer);

    result == Some(0)
}

pub fn menu_display_text(cpu: &mut Cpu, lines: &[&str]) {
    let layer = cpu.gpu_push_layer();

    text::text_box_border(cpu.gpu_mut_layer(layer), 0, 12, 18, 4);

    for (x, chr) in lines[0].chars().enumerate() {
        text::place_char(cpu.gpu_mut_layer(layer), x + 1, 14, chr);
        cpu.gpu_update_screen();
    }

    if lines.len() == 1 {
        wait_ab_press(cpu);
    }

    for &line in &lines[1..] {
        for (x, chr) in line.chars().enumerate() {
            text::place_char(cpu.gpu_mut_layer(layer), x + 1, 16, chr);
            cpu.gpu_update_screen();
        }

        wait_ab_press(cpu);

        text::place_string(cpu.gpu_mut_layer(layer), 1, 14, "                  ");
        text::place_string(cpu.gpu_mut_layer(layer), 1, 14, line);
        text::place_string(cpu.gpu_mut_layer(layer), 1, 16, "                  ");
    }

    cpu.gpu_pop_layer(layer);
}
