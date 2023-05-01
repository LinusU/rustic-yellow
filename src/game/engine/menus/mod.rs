use crate::{
    cpu::Cpu,
    game::{audio, home::text},
    keypad::KeypadKey,
};

pub mod main_menu;
pub mod save;

pub fn menu_single_choice(
    cpu: &mut Cpu,
    layer: usize,
    selected: &mut usize,
    pos: (usize, usize),
    choices: &[&str],
) -> Option<usize> {
    let width = usize::min(choices.iter().map(|s| s.len()).max().unwrap_or(0) + 2, 18);

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
