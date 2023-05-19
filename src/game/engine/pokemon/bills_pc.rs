use crate::{
    cpu::Cpu,
    game::{
        audio,
        constants::{
            gfx_constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
            pikachu_emotion_constants::PIKAHAPPY_DEPOSITED,
        },
        engine::{
            events,
            items::item_effects::add_pokemon_to_box,
            menus::{self, pokedex::index_to_pokedex},
            pikachu,
        },
        home, macros,
        ram::{hram, wram},
    },
    gpu::{GpuAtlas, GpuTile},
    keypad::KeypadKey,
    save_state::{BoxId, BoxView, BoxViewMut, BoxedPokemon, PartyPokemon, PartyView, PartyViewMut},
};

fn all_boxed_pokemons(cpu: &mut Cpu) -> Vec<((BoxId, usize), BoxedPokemon)> {
    let mut result = Vec::new();

    result.extend(
        BoxView::new(&cpu.mmu.wram[0x1a7f..])
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Current, i), p)),
    );

    let sram = cpu.borrow_sram();
    result.extend(
        sram.r#box(BoxId::Box1)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box1, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box2)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box2, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box3)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box3, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box4)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box4, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box5)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box5, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box6)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box6, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box7)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box7, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box8)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box8, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box9)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box9, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box10)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box10, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box11)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box11, i), p)),
    );
    result.extend(
        sram.r#box(BoxId::Box12)
            .iter()
            .enumerate()
            .map(|(i, p)| ((BoxId::Box12, i), p)),
    );

    result.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

    result
}

fn pick_pokemon(cpu: &mut Cpu, pos: (usize, usize), choices: &[BoxedPokemon]) -> Option<usize> {
    let layer = cpu.gpu_push_layer();

    let width = (SCREEN_WIDTH as usize) - 2 - pos.0;
    let height = usize::min(
        choices.len() * 3,
        ((SCREEN_HEIGHT as usize) - 2 - pos.1) / 3 * 3,
    );

    home::text::text_box_border(cpu.gpu_mut_layer(layer), pos.0, pos.1, width, height);

    let window_height = height / 3;

    let mut selected = 0;
    let mut scroll_pos = 0;

    loop {
        let window = &choices[scroll_pos..usize::min(scroll_pos + window_height, choices.len())];

        for (i, choice) in window.iter().enumerate() {
            let offset = (choice.species as usize) - 1;
            let offset_x = (offset % 16) * 2;
            let offset_y = (offset / 16) * 2;
            let icon = (
                GpuTile::new(GpuAtlas::PokemonIcons, offset_x, offset_y),
                GpuTile::new(GpuAtlas::PokemonIcons, offset_x + 1, offset_y),
                GpuTile::new(GpuAtlas::PokemonIcons, offset_x, offset_y + 1),
                GpuTile::new(GpuAtlas::PokemonIcons, offset_x + 1, offset_y + 1),
            );

            cpu.gpu_mut_layer(layer)
                .set_background(pos.0 + 2, pos.1 + 1 + i * 3, icon.0);
            cpu.gpu_mut_layer(layer)
                .set_background(pos.0 + 3, pos.1 + 1 + i * 3, icon.1);
            cpu.gpu_mut_layer(layer)
                .set_background(pos.0 + 2, pos.1 + 2 + i * 3, icon.2);
            cpu.gpu_mut_layer(layer)
                .set_background(pos.0 + 3, pos.1 + 2 + i * 3, icon.3);

            if scroll_pos + i == selected {
                cpu.gpu_mut_layer(layer).set_background(
                    pos.0 + 1,
                    pos.1 + 2 + i * 3,
                    GpuTile::new(GpuAtlas::Font, 13, 6),
                );
            } else {
                cpu.gpu_mut_layer(layer).set_background(
                    pos.0 + 1,
                    pos.1 + 2 + i * 3,
                    GpuTile::new(GpuAtlas::BoxBorder, 1, 1),
                );
            }

            home::text::place_string(
                cpu.gpu_mut_layer(layer),
                pos.0 + 5,
                pos.1 + 2 + i * 3,
                "           ",
            );
            home::text::place_poke_string(
                cpu.gpu_mut_layer(layer),
                pos.0 + 5,
                pos.1 + 2 + i * 3,
                choice.nickname.as_ref().unwrap_or(&choice.species.name()),
            );

            home::text::place_string(
                cpu.gpu_mut_layer(layer),
                pos.0 + 5,
                pos.1 + 3 + i * 3,
                &format!("𝗟{:<3}", choice.level),
            );
        }

        cpu.gpu_update_screen();
        let key = cpu.keypad_wait();

        match key {
            KeypadKey::A => {
                cpu.play_sfx(audio::sfx::PRESS_AB);
                cpu.gpu_pop_layer(layer);
                break Some(selected);
            }

            KeypadKey::B => {
                cpu.play_sfx(audio::sfx::PRESS_AB);
                cpu.gpu_pop_layer(layer);
                break None;
            }

            KeypadKey::Up if selected > 0 => {
                selected -= 1;

                if selected < scroll_pos {
                    scroll_pos -= 1;
                }
            }

            KeypadKey::Down if selected < choices.len() - 1 => {
                selected += 1;

                if selected >= scroll_pos + window_height {
                    scroll_pos += 1;
                }
            }

            _ => {}
        }
    }
}

pub fn bills_pc_menu(cpu: &mut Cpu) {
    cpu.pc = 0x5495;

    let mut selected = 0;
    let layer = cpu.gpu_push_layer();

    // Cover previous text box
    for y in 12..18 {
        for x in 0..20 {
            cpu.gpu_mut_layer(layer)
                .set_background(x, y, GpuTile::new(GpuAtlas::BoxBorder, 1, 1));
        }
    }

    loop {
        let selected = menus::menu_single_choice(
            cpu,
            layer,
            &mut selected,
            (0, 0),
            &["WITHDRAW 𝔭𝔪", "DEPOSIT 𝔭𝔪", "RELEASE 𝔭𝔪", "SEE YA!"],
        );

        match selected {
            None | Some(3) => {
                cpu.gpu_pop_layer(layer);

                // UpdateSprites
                cpu.call(0x231c);

                // ExitBillsPc
                cpu.pc = 0x553e;
                return;
            }

            Some(0) => {
                bills_pc_menu_withdraw(cpu);
            }
            Some(1) => {
                bills_pc_menu_deposit(cpu);
            }
            Some(2) => {
                bills_pc_menu_release(cpu);
            }

            _ => unreachable!(),
        }
    }
}

fn bills_pc_menu_withdraw(cpu: &mut Cpu) {
    let party_len = PartyView::new(&cpu.mmu.wram[0x1162..]).len();

    if party_len == 6 {
        menus::menu_display_text(cpu, &["You can't take", "any more POKéMON."]);
        menus::menu_display_text(cpu, &["Deposit POKéMON", "first."]);
        return;
    }

    let sources = all_boxed_pokemons(cpu);

    if sources.len() == 0 {
        menus::menu_display_text(cpu, &["What? There are", "no POKéMON here!"]);
        return;
    }

    let (pointers, pokemons) = sources.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    if let Some(poke_choice) = pick_pokemon(cpu, (3, 1), &pokemons) {
        let (box_id, box_idx) = &pointers[poke_choice];

        eprintln!("box_id: {:?}, box_idx: {:?}", box_id, box_idx);
        let pokemon = if *box_id == BoxId::Current {
            BoxViewMut::new(&mut cpu.mmu.wram[0x1a7f..]).swap_remove(*box_idx)
        } else {
            cpu.borrow_sram_mut().box_mut(*box_id).swap_remove(*box_idx)
        };

        let first_line = format!(
            "{} is",
            pokemon.nickname.as_ref().unwrap_or(&pokemon.species.name())
        );
        let third_line = format!(
            "Got {}.",
            pokemon.nickname.as_ref().unwrap_or(&pokemon.species.name())
        );
        let is_starter = pikachu::pikachu_status::is_this_partymon_starter_pikachu(cpu, &pokemon);

        if is_starter {
            cpu.play_sfx(audio::pikachu_cries::PikachuCry::new(34));
        } else {
            home::pokemon::play_cry(cpu, pokemon.species);
        }

        PartyViewMut::new(&mut cpu.mmu.wram[0x1162..]).push(pokemon.into());

        menus::menu_display_text(cpu, &[&first_line, "taken out.", &third_line]);
    }
}

fn bills_pc_menu_deposit(cpu: &mut Cpu) {
    let pokemons = PartyView::new(&cpu.mmu.wram[0x1162..])
        .iter()
        .map(Into::into)
        .collect::<Vec<BoxedPokemon>>();

    if pokemons.len() == 1 {
        menus::menu_display_text(cpu, &["You can't deposit", "the last POKéMON!"]);
        return;
    }

    if let Some(poke_choice) = pick_pokemon(cpu, (3, 1), &pokemons) {
        let pokemon = PartyView::new(&cpu.mmu.wram[0x1162..])
            .get(poke_choice)
            .unwrap();

        match add_pokemon_to_box(cpu, pokemon.into()) {
            Ok(()) => {
                let mut party = PartyViewMut::new(&mut cpu.mmu.wram[0x1162..]);
                let pokemon = party.remove(poke_choice);

                let first_line = format!(
                    "{} was",
                    pokemon.nickname.as_ref().unwrap_or(&pokemon.species.name())
                );
                let second_line = format!("stored in PC.");

                home::pokemon::play_cry(cpu, pokemon.species);
                events::pikachu_happiness::modify_pikachu_happiness(cpu, PIKAHAPPY_DEPOSITED);
                menus::menu_display_text(cpu, &[&first_line, &second_line]);
            }

            Err(()) => {
                menus::menu_display_text(cpu, &["BOX is full!"]);
            }
        }
    }
}

fn bills_pc_menu_release(cpu: &mut Cpu) {
    let sources = all_boxed_pokemons(cpu);

    if sources.len() == 0 {
        menus::menu_display_text(cpu, &["What? There are", "no POKéMON here!"]);
        return;
    }

    let (pointers, pokemons) = sources.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    if let Some(poke_choice) = pick_pokemon(cpu, (3, 1), &pokemons) {
        if pikachu::pikachu_status::is_this_partymon_starter_pikachu(cpu, &pokemons[poke_choice]) {
            menus::menu_display_text(cpu, &["PIKACHU looks", "unhappy about it!"]);
            return;
        }

        let second_line = format!(
            "{} is",
            pokemons[poke_choice]
                .nickname
                .as_ref()
                .unwrap_or(&pokemons[poke_choice].species.name())
        );

        if menus::menu_display_confirmation(
            cpu,
            &["Once released,", &second_line, "gone forever. OK?"],
        ) {
            let (box_id, box_idx) = &pointers[poke_choice];

            let pokemon = if *box_id == BoxId::Current {
                BoxViewMut::new(&mut cpu.mmu.wram[0x1a7f..]).swap_remove(*box_idx)
            } else {
                cpu.borrow_sram_mut().box_mut(*box_id).swap_remove(*box_idx)
            };

            let first_line = format!(
                "{} was",
                pokemon.nickname.as_ref().unwrap_or(&pokemon.species.name())
            );
            let third_line = format!(
                "Bye {}!",
                pokemon.nickname.as_ref().unwrap_or(&pokemon.species.name())
            );

            home::pokemon::play_cry(cpu, pokemon.species);

            menus::menu_display_text(cpu, &[&first_line, "released outside.", &third_line]);
        }
    }
}

// Todo:
// - ItemUseBall
// - _GivePokemon
// - SendNewMonToBox
// - MoveMon
