use std::{
    path,
    sync::mpsc::{Receiver, SyncSender},
};

use crate::{
    game_state::GameState,
    gpu::GpuLayer,
    keypad::{KeyboardEvent, KeypadKey, TextEvent},
    mmu::Mmu,
    save_state::SaveState,
    sound2::{Music, Sfx},
    PokemonSpecies,
};
use CpuFlag::{C, H, N, Z};

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

pub struct Cpu {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) pc: u16,
    pub(crate) sp: u16,

    pub(crate) halted: bool,
    pub(crate) ime: bool,
    pub(crate) setdi: u32,
    pub(crate) setei: u32,

    pub(crate) mmu: Mmu,

    pub(crate) starter: PokemonSpecies,
}

impl Cpu {
    pub fn new(
        update_screen: SyncSender<Vec<u8>>,
        keyboard_events: Receiver<KeyboardEvent>,
        starter: PokemonSpecies,
    ) -> Cpu {
        Cpu {
            a: 0x11,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,

            halted: false,
            ime: true,
            setdi: 0,
            setei: 0,

            mmu: Mmu::new(update_screen, keyboard_events),

            starter,
        }
    }

    pub fn sync_audio(&mut self) {
        self.mmu.sound.sync();
    }

    #[rustfmt::skip]
    pub fn call(&mut self, pc: u16) {
        assert_ne!(pc, 0x0000);

        self.stack_push(0x0000);
        self.pc = pc;

        loop {
            match (self.bank(), self.pc) {
                (_, 0x0000) => break,
                (_, 0x0001) => panic!("Invalid call to 0x0001"),
                (_, 0x01d7) => crate::game::home::overworld::enter_map(self),
                (_, 0x0242) => crate::game::home::overworld::overworld_loop(self),
                (_, 0x0245) => crate::game::home::overworld::overworld_loop_less_delay(self),
                (_, 0x0457) => panic!("step_count_check should only be called from Rust"),
                (_, 0x0475) => panic!("all_pokemon_fainted should only be called from Rust"),
                (_, 0x0480) => panic!("new_battle should only be called from Rust"),
                (_, 0x049d) => panic!("do_bike_speedup should only be called from Rust"),
                (_, 0x04bd) => crate::game::home::overworld::check_warps_no_collision(self),
                (_, 0x051a) => crate::game::home::overworld::check_warps_collision(self),
                (_, 0x0543) => panic!("warp_found1 should only be called from Rust"),
                (_, 0x054a) => crate::game::home::overworld::warp_found2(self),
                (_, 0x05db) => crate::game::home::overworld::check_map_connections(self),
                (_, 0x06ef) => panic!("play_map_change_sound should only be called from Rust"),
                (_, 0x0712) => crate::game::home::overworld::check_if_in_outside_map(self),
                (_, 0x071a) => panic!("extra_warp_check should only be called from Rust"),
                (_, 0x0750) => panic!("map_entry_after_battle should only be called from Rust"),
                (_, 0x0762) => panic!("handle_black_out should only be called from Rust"),
                (_, 0x0785) => crate::game::home::overworld::stop_music(self),
                (_, 0x0794) => panic!("handle_fly_warp_or_dungeon_warp should only be called from Rust"),
                (_, 0x07c4) => panic!("stop_bike_surf should only be called from Rust"),
                (_, 0x07d7) => crate::game::home::overworld::load_player_sprite_graphics(self),
                (_, 0x0805) => crate::game::home::overworld::is_bike_riding_allowed(self),
                (_, 0x0828) => crate::game::home::overworld::load_tileset_tile_pattern_data(self),
                (_, 0x083c) => crate::game::home::overworld::load_tile_block_map(self),
                (_, 0x0919) => panic!("load_north_south_connections_tile_map should only be called from Rust"),
                (_, 0x093d) => panic!("load_east_west_connections_tile_map should only be called from Rust"),
                (_, 0x09f2) => panic!("sign_loop should only be called from Rust"),
                (_, 0x095e) => panic!("is_sprite_or_sign_in_front_of_player should only be called from Rust"),
                (_, 0x0983) => crate::game::home::overworld::is_sprite_in_front_of_player(self),
                (_, 0x0985) => crate::game::home::overworld::is_sprite_in_front_of_player2(self),
                (_, 0x0a1c) => panic!("collision_check_on_land should only be called from Rust"),
                (_, 0x0a79) => panic!("check_tile_passable should only be called from Rust"),
                (_, 0x0a86) => panic!("check_for_jumping_and_tile_pair_collisions should only be called from Rust"),
                (_, 0x0aa0) => crate::game::home::overworld::check_for_tile_pair_collisions2(self),
                (_, 0x0aa6) => crate::game::home::overworld::check_for_tile_pair_collisions(self),
                (_, 0x0b06) => crate::game::home::overworld::load_current_map_view(self),
                (_, 0x0b7f) => crate::game::home::overworld::advance_player_sprite(self),
                (_, 0x0b95) => crate::game::home::overworld::schedule_north_row_redraw(self),
                (_, 0x0baa) => crate::game::home::overworld::copy_to_redraw_row_or_column_src_tiles(self),
                (_, 0x0bb6) => crate::game::home::overworld::schedule_south_row_redraw(self),
                (_, 0x0bd7) => crate::game::home::overworld::schedule_east_column_redraw(self),
                (_, 0x0bf6) => panic!("schedule_column_redraw_helper should only be called from Rust"),
                (_, 0x0c0c) => crate::game::home::overworld::schedule_west_column_redraw(self),
                (_, 0x0c21) => panic!("draw_tile_block should only be called from Rust"),
                (_, 0x0c51) => panic!("joypad_overworld should only be called from Rust"),
                (_, 0x0c65) => panic!("force_bike_down should only be called from Rust"),
                (_, 0x0c7b) => panic!("are_inputs_simulated should only be called from Rust"),
                (_, 0x0cb3) => panic!("get_simulated_input should only be called from Rust"),
                (_, 0x0cca) => panic!("collision_check_on_water should only be called from Rust"),
                (_, 0x0d2c) => panic!("run_map_script should only be called from Rust"),
                (_, 0x0d5e) => crate::game::home::overworld::load_walking_player_sprite_graphics(self),
                (_, 0x0d69) => panic!("load_surfing_player_sprite_graphics2 should only be called from Rust"),
                (_, 0x0d83) => panic!("load_surfing_player_sprite_graphics should only be called from Rust"),
                (_, 0x0d8a) => panic!("load_bike_player_sprite_graphics should only be called from Rust"),
                (_, 0x0d8f) => panic!("load_player_sprite_graphics_common should only be called from Rust"),
                (_, 0x0dab) => panic!("load_map_header should only be called from Rust"),
                (_, 0x0eaa) => crate::game::home::overworld::copy_map_connection_header(self),
                (_, 0x0eb3) => panic!("copy_sign_data should only be called from Rust"),
                (_, 0x0ecb) => panic!("load_map_data should only be called from Rust"),
                (_, 0x0f0c) => panic!("load_screen_related_data should only be called from Rust"),
                (_, 0x0f16) => crate::game::home::overworld::reload_map_after_surfing_minigame(self),
                (_, 0x0f3d) => crate::game::home::overworld::reload_map_after_printer(self),
                (_, 0x0f56) => panic!("reset_map_variables should only be called from Rust"),
                (_, 0x0f70) => panic!("copy_map_view_to_vram should only be called from Rust"),
                (_, 0x0f73) => panic!("copy_map_view_to_vram2 should only be called from Rust"),
                (_, 0x0f8b) => crate::game::home::overworld::switch_to_map_rom_bank(self),
                (_, 0x0fa7) => panic!("get_map_header_pointer should only be called from Rust"),
                (_, 0x0fc3) => panic!("ignore_input_for_half_second should only be called from Rust"),
                (_, 0x0fd0) => crate::game::home::overworld::reset_using_strength_out_of_battle_bit(self),
                (_, 0x0fd6) => crate::game::home::overworld::force_bike_or_surf(self),
                (_, 0x0fe1) => panic!("handle_mid_jump should only be called from Rust"),
                (_, 0x0ff0) => panic!("is_spinning should only be called from Rust"),
                (_, 0x1006) => panic!("init_sprites should only be called from Rust"),
                (_, 0x1050) => panic!("zero_sprite_state_data should only be called from Rust"),
                (_, 0x1060) => panic!("disable_regular_sprites should only be called from Rust"),
                (_, 0x106f) => panic!("load_sprite should only be called from Rust"),
                (_, 0x10ba) => crate::game::home::overworld::check_for_user_interruption(self),
                (_, 0x10d5) => panic!("load_destination_warp_position should only be called from Rust"),
                (_, 0x143e) => crate::game::home::pics::load_mon_front_sprite(self),
                (_, 0x2238) => crate::game::home::audio::play_sound(self),
                (_, 0x309d) => panic!("is_player_character_being_controlled_by_game should only be called from Rust"),
                (_, 0x30ae) => panic!("run_npc_movement_script should only be called from Rust"),
                (_, 0x3422) => crate::game::home::map_objects::is_item_in_bag(self),
                (_, 0x342a) => panic!("is_surfing_pikachu_in_party should only be called from Rust"),
                (_, 0x36c3) => crate::game::home::load_font::load_hp_bar_and_status_tile_patterns(self),
                (_, 0x3ed7) => crate::game::home::predef::get_predef_registers(self),
                (_, 0x3ef9) => panic!("check_for_hidden_object_or_bookshelf_or_card_key_door should only be called from Rust"),
                (0x01, 0x42bf) => crate::game::engine::movie::title::display_title_screen_go_to_main_menu(self),
                (0x01, 0x581e) => crate::game::engine::link::cable_club::return_to_cable_club_room(self),
                (0x01, 0x5ba6) => panic!("main_menu should only be called from Rust"),
                (0x01, 0x5dfb) => panic!("check_for_player_name_in_sram should only be called from Rust"),
                (0x02, 0x5064) => crate::game::audio::play_battle_music::play_battle_music(self),
                (0x03, 0x407c) => panic!("clear_variables_on_enter_map should only be called from Rust"),
                (0x03, 0x44f4) => crate::game::engine::overworld::tilesets::load_tileset_header(self),
                (0x03, 0x4b62) => crate::game::engine::overworld::wild_mons::load_wild_data(self),
                (0x03, 0x6807) => crate::game::engine::items::item_effects::hook_send_new_mon_to_box_end(self),
                (0x03, 0x752b) => crate::game::engine::events::heal_party::heal_party(self),
                (0x03, 0x7735) => crate::game::engine::items::get_bag_item_quantity::get_quantity_of_item_in_bag(self),
                (0x04, 0x783a) => crate::game::engine::battle::wild_encounters::try_do_wild_encounter(self),
                (0x06, 0x4f0a) => crate::game::scripts::pallete_town::pallet_town_script4(self),
                (0x07, 0x4b40) => crate::game::scripts::oaks_lab::oaks_lab_player_received_mon_text(self),
                (0x08, 0x5495) => crate::game::engine::pokemon::bills_pc::bills_pc_menu(self),
                (0x0f, 0x4e25) => crate::game::engine::battle::core::draw_player_hud_and_hp_bar(self),
                (0x0f, 0x59ac) => crate::game::engine::battle::core::is_ghost_battle(self),
                (0x15, 0x525f) => crate::game::engine::battle::experience::gain_experience(self),
                (0x15, 0x547b) => panic!("divide_exp_data_by_num_mons_gaining_exp should only be called from Rust"),
                (0x15, 0x54ae) => panic!("boost_exp should only be called from Rust"),
                (0x1c, 0x5eb3) => crate::game::engine::gfx::palettes::run_palette_command(self),
                (0x1c, 0x5ed3) => panic!("set_pal_battle_black should only be called from Rust"),
                (0x1c, 0x5eda) => panic!("set_pal_battle should only be called from Rust"),
                (0x1c, 0x5f26) => panic!("set_pal_town_map should only be called from Rust"),
                (0x1c, 0x5f2d) => panic!("set_pal_status_screen should only be called from Rust"),
                (0x1c, 0x5f59) => panic!("set_pal_party_menu should only be called from Rust"),
                (0x1c, 0x5f60) => panic!("set_pal_pokedex should only be called from Rust"),
                (0x1c, 0x5f7d) => panic!("set_pal_slots should only be called from Rust"),
                (0x1c, 0x5f84) => panic!("set_pal_title_screen should only be called from Rust"),
                (0x1c, 0x5f8b) => panic!("set_pal_generic should only be called from Rust"),
                (0x1c, 0x5f92) => panic!("set_pal_nidorino_intro should only be called from Rust"),
                (0x1c, 0x5f99) => panic!("set_pal_game_freak_intro should only be called from Rust"),
                (0x1c, 0x6001) => panic!("set_pal_pokemon_whole_screen should only be called from Rust"),
                (0x1c, 0x6093) => panic!("determine_palette_id should only be called from Rust"),
                (0x1c, 0x6094) => panic!("determine_palette_id_out_of_battle should only be called from Rust"),
                (0x1c, 0x61f8) => crate::game::engine::gfx::palettes::load_sgb(self),
                (0x1c, 0x6328) => crate::game::engine::gfx::palettes::send_sgb_packets(self),
                (0x1c, 0x6346) => crate::game::engine::gfx::palettes::init_cgb_palettes(self),
                (0x1c, 0x6470) => crate::game::engine::gfx::palettes::transfer_cur_bgp_data(self),
                (0x1c, 0x6511) => crate::game::engine::gfx::palettes::transfer_pal_color_lcd_enabled(self),
                (0x1c, 0x651b) => crate::game::engine::gfx::palettes::transfer_pal_color_lcd_disabled(self),
                (0x1c, 0x65be) => panic!("translate_pal_packet_to_bg_map_attributes should only be called from Rust"),
                (0x1c, 0x7b91) => crate::game::engine::menus::save::save_sav_to_sram(self),
                (0x3c, 0x4000) => crate::game::engine::pikachu::pikachu_pcm::play_pikachu_sound_clip(self),
                (0x3c, 0x4274) => crate::game::engine::events::black_out::reset_status_and_halve_money_on_blackout(self),
                (0x3d, 0x42db) => crate::game::engine::battle::decrement_pp::decrement_pp(self),
                (0x3d, 0x5ff2) => crate::game::engine::battle::init_battle::init_battle(self),
                (0x3d, 0x5ff8) => crate::game::engine::battle::init_battle::init_opponent(self),
                (0x3d, 0x615a) => crate::game::engine::battle::init_battle::_load_trainer_pic(self),
                (0x3d, 0x6178) => crate::game::engine::battle::init_battle::load_mon_back_pic(self),
                (0x3d, 0x61a6) => crate::game::engine::battle::init_battle::animate_sending_out_mon(self),
                (0x3d, 0x61f9) => crate::game::engine::battle::init_battle::copy_uncompressed_pic_to_tilemap(self),
                (0x3d, 0x6203) => crate::game::engine::battle::init_battle::copy_uncompressed_pic_to_hl(self),
                (0x3d, 0x674d) => crate::game::engine::events::give_pokemon::hook_give_pokemon_next_end(self),

                _ => {
                    let ticks = if self.halted { 4 } else { self.step() * 4 };
                    self.cycle(ticks);
                }
            }
        }
    }

    pub fn jump(&mut self, pc: u16) {
        self.call(pc);
        self.pc = self.stack_pop();
    }

    pub fn bank(&self) -> usize {
        self.mmu.mbc.rombank
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.mmu.rb(addr)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.mmu.wb(addr, value);
    }

    pub fn cycle(&mut self, ticks: u32) {
        self.mmu.do_cycle(ticks);
        self.updateime();
        self.handleinterrupt();
    }

    pub fn borrow_sram(&self) -> &SaveState {
        self.mmu.mbc.borrow_sram()
    }

    pub fn borrow_sram_mut(&mut self) -> &mut SaveState {
        self.mmu.mbc.borrow_sram_mut()
    }

    pub fn borrow_wram(&self) -> &GameState {
        self.mmu.borrow_wram()
    }

    pub fn borrow_wram_mut(&mut self) -> &mut GameState {
        self.mmu.borrow_wram_mut()
    }

    pub fn replace_ram(&mut self, ram: SaveState) {
        self.mmu.mbc.replace_ram(ram);
    }

    pub fn set_save_path(&mut self, path: path::PathBuf) {
        self.mmu.mbc.set_save_path(path);
    }

    pub fn save_to_disk(&mut self) {
        self.mmu.mbc.save_to_disk();
    }

    pub fn gpu_push_layer(&mut self) -> usize {
        self.mmu.gpu.layer_push()
    }

    pub fn gpu_pop_layer(&mut self, layer: usize) {
        self.mmu.gpu.layer_pop(layer);
    }

    pub fn gpu_mut_layer(&mut self, layer: usize) -> &mut GpuLayer {
        self.mmu.gpu.layer_mut(layer)
    }

    pub fn gpu_update_screen(&mut self) {
        self.mmu.gpu.update_screen();
    }

    pub fn gpu_set_bg_palette_color(&mut self, palnum: usize, colnum: usize, color: [u8; 3]) {
        self.mmu.gpu.set_bg_palette_color(palnum, colnum, color);
    }

    pub fn keypad_wait(&mut self) -> KeypadKey {
        self.mmu.keypad.wait()
    }

    pub fn keyboard_text(&mut self) -> TextEvent {
        self.mmu.keypad.text()
    }

    pub fn start_music<T, TSource>(&mut self, music: T)
    where
        T: Music<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
        TSource::Item: rodio::Sample + Send,
    {
        self.mmu.sound2.start_music(music)
    }

    pub fn play_sfx<T, TSource>(&mut self, sfx: T)
    where
        T: Sfx<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
        TSource::Item: rodio::Sample + Send,
    {
        self.mmu.sound2.play_sfx(sfx)
    }

    fn fetch_byte(&mut self) -> u8 {
        let b = self.mmu.rb(self.pc);
        self.pc = self.pc.wrapping_add(1);
        b
    }

    fn fetch_word(&mut self) -> u16 {
        let w = self.mmu.rw(self.pc);
        self.pc += 2;
        w
    }

    fn updateime(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0,
        };
        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0,
        };
    }

    fn handleinterrupt(&mut self) {
        if !self.ime && !self.halted {
            return;
        }

        let triggered = self.mmu.inte & self.mmu.intf;
        if triggered == 0 {
            return;
        }

        self.halted = false;
        if !self.ime {
            return;
        }
        self.ime = false;

        let n = triggered.trailing_zeros();
        assert!(n < 5, "Invalid interrupt triggered");
        self.mmu.intf &= !(1 << n);

        let pc = self.pc;
        self.call(0x0040 | ((n as u16) << 3));
        self.pc = pc;
    }

    pub fn stack_push(&mut self, value: u16) {
        self.sp -= 2;
        self.mmu.ww(self.sp, value);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let res = self.mmu.rw(self.sp);
        self.sp += 2;
        res
    }

    #[rustfmt::skip]
    fn step(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => { 1 },
            0x01 => { let v = self.fetch_word(); self.set_bc(v); 3 },
            0x02 => { self.mmu.wb(self.bc(), self.a); 2 },
            0x03 => { self.set_bc(self.bc().wrapping_add(1)); 2 },
            0x04 => { self.b = self.alu_inc(self.b); 1 },
            0x05 => { self.b = self.alu_dec(self.b); 1 },
            0x06 => { self.b = self.fetch_byte(); 2 },
            0x07 => { self.a = self.alu_rlc(self.a); self.set_flag(Z, false); 1 },
            0x08 => { let a = self.fetch_word(); self.mmu.ww(a, self.sp); 5 },
            0x09 => { self.alu_add16(self.bc()); 2 },
            0x0A => { self.a = self.mmu.rb(self.bc()); 2 },
            0x0B => { self.set_bc(self.bc().wrapping_sub(1)); 2 },
            0x0C => { self.c = self.alu_inc(self.c); 1 },
            0x0D => { self.c = self.alu_dec(self.c); 1 },
            0x0E => { self.c = self.fetch_byte(); 2 },
            0x0F => { self.a = self.alu_rrc(self.a); self.set_flag(Z, false); 1 },
            0x10 => { self.mmu.switch_speed(); 1 }, // STOP
            0x11 => { let v = self.fetch_word(); self.set_de(v); 3 },
            0x12 => { self.mmu.wb(self.de(), self.a); 2 },
            0x13 => { self.set_de(self.de().wrapping_add(1)); 2 },
            0x14 => { self.d = self.alu_inc(self.d); 1 },
            0x15 => { self.d = self.alu_dec(self.d); 1 },
            0x16 => { self.d = self.fetch_byte(); 2 },
            0x17 => { self.a = self.alu_rl(self.a); self.set_flag(Z, false); 1 },
            0x18 => { self.cpu_jr(); 3 },
            0x19 => { self.alu_add16(self.de()); 2 },
            0x1A => { self.a = self.mmu.rb(self.de()); 2 },
            0x1B => { self.set_de(self.de().wrapping_sub(1)); 2 },
            0x1C => { self.e = self.alu_inc(self.e); 1 },
            0x1D => { self.e = self.alu_dec(self.e); 1 },
            0x1E => { self.e = self.fetch_byte(); 2 },
            0x1F => { self.a = self.alu_rr(self.a); self.set_flag(Z, false); 1 },
            0x20 => { if !self.flag(Z) { self.cpu_jr(); 3 } else { self.pc += 1; 2 } },
            0x21 => { let v = self.fetch_word(); self.set_hl(v); 3 },
            0x22 => { let addr = self.hli(); self.mmu.wb(addr, self.a); 2 },
            0x23 => { let v = self.hl().wrapping_add(1); self.set_hl(v); 2 },
            0x24 => { self.h = self.alu_inc(self.h); 1 },
            0x25 => { self.h = self.alu_dec(self.h); 1 },
            0x26 => { self.h = self.fetch_byte(); 2 },
            0x27 => { self.alu_daa(); 1 },
            0x28 => { if self.flag(Z) { self.cpu_jr(); 3 } else { self.pc += 1; 2  } },
            0x29 => { let v = self.hl(); self.alu_add16(v); 2 },
            0x2A => { let addr = self.hli(); self.a = self.mmu.rb(addr); 2 },
            0x2B => { let v = self.hl().wrapping_sub(1); self.set_hl(v); 2 },
            0x2C => { self.l = self.alu_inc(self.l); 1 },
            0x2D => { self.l = self.alu_dec(self.l); 1 },
            0x2E => { self.l = self.fetch_byte(); 2 },
            0x2F => { self.a = !self.a; self.set_flag(H, true); self.set_flag(N, true); 1 },
            0x30 => { if !self.flag(C) { self.cpu_jr(); 3 } else { self.pc += 1; 2 } },
            0x31 => { self.sp = self.fetch_word(); 3 },
            0x32 => { let addr = self.hld(); self.mmu.wb(addr, self.a); 2 },
            0x33 => { self.sp = self.sp.wrapping_add(1); 2 },
            0x34 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_inc(v); self.mmu.wb(a, v2); 3 },
            0x35 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_dec(v); self.mmu.wb(a, v2); 3 },
            0x36 => { let v = self.fetch_byte(); self.mmu.wb(self.hl(), v); 3 },
            0x37 => { self.set_flag(C, true); self.set_flag(H, false); self.set_flag(N, false); 1 },
            0x38 => { if self.flag(C) { self.cpu_jr(); 3 } else { self.pc += 1; 2  } },
            0x39 => { self.alu_add16(self.sp); 2 },
            0x3A => { let addr = self.hld(); self.a = self.mmu.rb(addr); 2 },
            0x3B => { self.sp = self.sp.wrapping_sub(1); 2 },
            0x3C => { self.a = self.alu_inc(self.a); 1 },
            0x3D => { self.a = self.alu_dec(self.a); 1 },
            0x3E => { self.a = self.fetch_byte(); 2 },
            0x3F => { let v = !self.flag(C); self.set_flag(C, v); self.set_flag(H, false); self.set_flag(N, false); 1 },
            0x40 => { 1 },
            0x41 => { self.b = self.c; 1 },
            0x42 => { self.b = self.d; 1 },
            0x43 => { self.b = self.e; 1 },
            0x44 => { self.b = self.h; 1 },
            0x45 => { self.b = self.l; 1 },
            0x46 => { self.b = self.mmu.rb(self.hl()); 2 },
            0x47 => { self.b = self.a; 1 },
            0x48 => { self.c = self.b; 1 },
            0x49 => { 1 },
            0x4A => { self.c = self.d; 1 },
            0x4B => { self.c = self.e; 1 },
            0x4C => { self.c = self.h; 1 },
            0x4D => { self.c = self.l; 1 },
            0x4E => { self.c = self.mmu.rb(self.hl()); 2 },
            0x4F => { self.c = self.a; 1 },
            0x50 => { self.d = self.b; 1 },
            0x51 => { self.d = self.c; 1 },
            0x52 => { 1 },
            0x53 => { self.d = self.e; 1 },
            0x54 => { self.d = self.h; 1 },
            0x55 => { self.d = self.l; 1 },
            0x56 => { self.d = self.mmu.rb(self.hl()); 2 },
            0x57 => { self.d = self.a; 1 },
            0x58 => { self.e = self.b; 1 },
            0x59 => { self.e = self.c; 1 },
            0x5A => { self.e = self.d; 1 },
            0x5B => { 1 },
            0x5C => { self.e = self.h; 1 },
            0x5D => { self.e = self.l; 1 },
            0x5E => { self.e = self.mmu.rb(self.hl()); 2 },
            0x5F => { self.e = self.a; 1 },
            0x60 => { self.h = self.b; 1 },
            0x61 => { self.h = self.c; 1 },
            0x62 => { self.h = self.d; 1 },
            0x63 => { self.h = self.e; 1 },
            0x64 => { 1 },
            0x65 => { self.h = self.l; 1 },
            0x66 => { self.h = self.mmu.rb(self.hl()); 2 },
            0x67 => { self.h = self.a; 1 },
            0x68 => { self.l = self.b; 1 },
            0x69 => { self.l = self.c; 1 },
            0x6A => { self.l = self.d; 1 },
            0x6B => { self.l = self.e; 1 },
            0x6C => { self.l = self.h; 1 },
            0x6D => { 1 },
            0x6E => { self.l = self.mmu.rb(self.hl()); 2 },
            0x6F => { self.l = self.a; 1 },
            0x70 => { self.mmu.wb(self.hl(), self.b); 2 },
            0x71 => { self.mmu.wb(self.hl(), self.c); 2 },
            0x72 => { self.mmu.wb(self.hl(), self.d); 2 },
            0x73 => { self.mmu.wb(self.hl(), self.e); 2 },
            0x74 => { self.mmu.wb(self.hl(), self.h); 2 },
            0x75 => { self.mmu.wb(self.hl(), self.l); 2 },
            0x76 => { self.halted = true; 1 },
            0x77 => { self.mmu.wb(self.hl(), self.a); 2 },
            0x78 => { self.a = self.b; 1 },
            0x79 => { self.a = self.c; 1 },
            0x7A => { self.a = self.d; 1 },
            0x7B => { self.a = self.e; 1 },
            0x7C => { self.a = self.h; 1 },
            0x7D => { self.a = self.l; 1 },
            0x7E => { self.a = self.mmu.rb(self.hl()); 2 },
            0x7F => { 1 },
            0x80 => { self.alu_add(self.b, false); 1 },
            0x81 => { self.alu_add(self.c, false); 1 },
            0x82 => { self.alu_add(self.d, false); 1 },
            0x83 => { self.alu_add(self.e, false); 1 },
            0x84 => { self.alu_add(self.h, false); 1 },
            0x85 => { self.alu_add(self.l, false); 1 },
            0x86 => { let v = self.mmu.rb(self.hl()); self.alu_add(v, false); 2 },
            0x87 => { self.alu_add(self.a, false); 1 },
            0x88 => { self.alu_add(self.b, true); 1 },
            0x89 => { self.alu_add(self.c, true); 1 },
            0x8A => { self.alu_add(self.d, true); 1 },
            0x8B => { self.alu_add(self.e, true); 1 },
            0x8C => { self.alu_add(self.h, true); 1 },
            0x8D => { self.alu_add(self.l, true); 1 },
            0x8E => { let v = self.mmu.rb(self.hl()); self.alu_add(v, true); 2 },
            0x8F => { self.alu_add(self.a, true); 1 },
            0x90 => { self.alu_sub(self.b, false); 1 },
            0x91 => { self.alu_sub(self.c, false); 1 },
            0x92 => { self.alu_sub(self.d, false); 1 },
            0x93 => { self.alu_sub(self.e, false); 1 },
            0x94 => { self.alu_sub(self.h, false); 1 },
            0x95 => { self.alu_sub(self.l, false); 1 },
            0x96 => { let v = self.mmu.rb(self.hl()); self.alu_sub(v, false); 2 },
            0x97 => { self.alu_sub(self.a, false); 1 },
            0x98 => { self.alu_sub(self.b, true); 1 },
            0x99 => { self.alu_sub(self.c, true); 1 },
            0x9A => { self.alu_sub(self.d, true); 1 },
            0x9B => { self.alu_sub(self.e, true); 1 },
            0x9C => { self.alu_sub(self.h, true); 1 },
            0x9D => { self.alu_sub(self.l, true); 1 },
            0x9E => { let v = self.mmu.rb(self.hl()); self.alu_sub(v, true); 2 },
            0x9F => { self.alu_sub(self.a, true); 1 },
            0xA0 => { self.alu_and(self.b); 1 },
            0xA1 => { self.alu_and(self.c); 1 },
            0xA2 => { self.alu_and(self.d); 1 },
            0xA3 => { self.alu_and(self.e); 1 },
            0xA4 => { self.alu_and(self.h); 1 },
            0xA5 => { self.alu_and(self.l); 1 },
            0xA6 => { let v = self.mmu.rb(self.hl()); self.alu_and(v); 2 },
            0xA7 => { self.alu_and(self.a); 1 },
            0xA8 => { self.alu_xor(self.b); 1 },
            0xA9 => { self.alu_xor(self.c); 1 },
            0xAA => { self.alu_xor(self.d); 1 },
            0xAB => { self.alu_xor(self.e); 1 },
            0xAC => { self.alu_xor(self.h); 1 },
            0xAD => { self.alu_xor(self.l); 1 },
            0xAE => { let v = self.mmu.rb(self.hl()); self.alu_xor(v); 2 },
            0xAF => { self.alu_xor(self.a); 1 },
            0xB0 => { self.alu_or(self.b); 1 },
            0xB1 => { self.alu_or(self.c); 1 },
            0xB2 => { self.alu_or(self.d); 1 },
            0xB3 => { self.alu_or(self.e); 1 },
            0xB4 => { self.alu_or(self.h); 1 },
            0xB5 => { self.alu_or(self.l); 1 },
            0xB6 => { let v = self.mmu.rb(self.hl()); self.alu_or(v); 2 },
            0xB7 => { self.alu_or(self.a); 1 },
            0xB8 => { self.alu_cp(self.b); 1 },
            0xB9 => { self.alu_cp(self.c); 1 },
            0xBA => { self.alu_cp(self.d); 1 },
            0xBB => { self.alu_cp(self.e); 1 },
            0xBC => { self.alu_cp(self.h); 1 },
            0xBD => { self.alu_cp(self.l); 1 },
            0xBE => { let v = self.mmu.rb(self.hl()); self.alu_cp(v); 2 },
            0xBF => { self.alu_cp(self.a); 1 },
            0xC0 => { if !self.flag(Z) { self.pc = self.stack_pop(); 5 } else { 2 } },
            0xC1 => { let v = self.stack_pop(); self.set_bc(v); 3 },
            0xC2 => { if !self.flag(Z) { self.pc = self.fetch_word(); 4 } else { self.pc += 2; 3 } },
            0xC3 => { self.pc = self.fetch_word(); 4 },
            0xC4 => { if !self.flag(Z) { self.stack_push(self.pc + 2); self.pc = self.fetch_word(); 6 } else { self.pc += 2; 3 } },
            0xC5 => { self.stack_push(self.bc()); 4 },
            0xC6 => { let v = self.fetch_byte(); self.alu_add(v, false); 2 },
            0xC7 => { self.stack_push(self.pc); self.pc = 0x00; 4 },
            0xC8 => { if self.flag(Z) { self.pc = self.stack_pop(); 5 } else { 2 } },
            0xC9 => { self.pc = self.stack_pop(); 4 },
            0xCA => { if self.flag(Z) { self.pc = self.fetch_word(); 4 } else { self.pc += 2; 3 } },
            0xCB => { self.step_cb() },
            0xCC => { if self.flag(Z) { self.stack_push(self.pc + 2); self.pc = self.fetch_word(); 6 } else { self.pc += 2; 3 } },
            0xCD => { self.stack_push(self.pc + 2); self.pc = self.fetch_word(); 6 },
            0xCE => { let v = self.fetch_byte(); self.alu_add(v, true); 2 },
            0xCF => { self.stack_push(self.pc); self.pc = 0x08; 4 },
            0xD0 => { if !self.flag(C) { self.pc = self.stack_pop(); 5 } else { 2 } },
            0xD1 => { let v = self.stack_pop(); self.set_de(v); 3 },
            0xD2 => { if !self.flag(C) { self.pc = self.fetch_word(); 4 } else { self.pc += 2; 3 } },
            0xD4 => { if !self.flag(C) { self.stack_push(self.pc + 2); self.pc = self.fetch_word(); 6 } else { self.pc += 2; 3 } },
            0xD5 => { self.stack_push(self.de()); 4 },
            0xD6 => { let v = self.fetch_byte(); self.alu_sub(v, false); 2 },
            0xD7 => { self.stack_push(self.pc); self.pc = 0x10; 4 },
            0xD8 => { if self.flag(C) { self.pc = self.stack_pop(); 5 } else { 2 } },
            0xD9 => { self.pc = self.stack_pop(); self.setei = 1; 4 },
            0xDA => { if self.flag(C) { self.pc = self.fetch_word(); 4 } else { self.pc += 2; 3 } },
            0xDC => { if self.flag(C) { self.stack_push(self.pc + 2); self.pc = self.fetch_word(); 6 } else { self.pc += 2; 3 } },
            0xDE => { let v = self.fetch_byte(); self.alu_sub(v, true); 2 },
            0xDF => { self.stack_push(self.pc); self.pc = 0x18; 4 },
            0xE0 => { let a = 0xFF00 | self.fetch_byte() as u16; self.mmu.wb(a, self.a); 3 },
            0xE1 => { let v = self.stack_pop(); self.set_hl(v); 3 },
            0xE2 => { self.mmu.wb(0xFF00 | self.c as u16, self.a); 2 },
            0xE5 => { self.stack_push(self.hl()); 4 },
            0xE6 => { let v = self.fetch_byte(); self.alu_and(v); 2 },
            0xE7 => { self.stack_push(self.pc); self.pc = 0x20; 4 },
            0xE8 => { self.sp = self.alu_add16imm(self.sp); 4 },
            0xE9 => { self.pc = self.hl(); 1 },
            0xEA => { let a = self.fetch_word(); self.mmu.wb(a, self.a); 4 },
            0xEE => { let v = self.fetch_byte(); self.alu_xor(v); 2 },
            0xEF => { self.stack_push(self.pc); self.pc = 0x28; 4 },
            0xF0 => { let a = 0xFF00 | self.fetch_byte() as u16; self.a = self.mmu.rb(a); 3 },
            0xF1 => { let v = self.stack_pop() & 0xFFF0; self.set_af(v); 3 },
            0xF2 => { self.a = self.mmu.rb(0xFF00 | self.c as u16); 2 },
            0xF3 => { self.setdi = 2; 1 },
            0xF5 => { self.stack_push(self.af()); 4 },
            0xF6 => { let v = self.fetch_byte(); self.alu_or(v); 2 },
            0xF7 => { self.stack_push(self.pc); self.pc = 0x30; 4 },
            0xF8 => { let r = self.alu_add16imm(self.sp); self.set_hl(r); 3 },
            0xF9 => { self.sp = self.hl(); 2 },
            0xFA => { let a = self.fetch_word(); self.a = self.mmu.rb(a); 4 },
            0xFB => { self.setei = 2; 1 },
            0xFE => { let v = self.fetch_byte(); self.alu_cp(v); 2 },
            0xFF => { self.stack_push(self.pc); self.pc = 0x38; 4 },
            other=> panic!("Instruction {:02X} is not implemented", other),
        }
    }

    #[rustfmt::skip]
    fn step_cb(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => { self.b = self.alu_rlc(self.b); 2 },
            0x01 => { self.c = self.alu_rlc(self.c); 2 },
            0x02 => { self.d = self.alu_rlc(self.d); 2 },
            0x03 => { self.e = self.alu_rlc(self.e); 2 },
            0x04 => { self.h = self.alu_rlc(self.h); 2 },
            0x05 => { self.l = self.alu_rlc(self.l); 2 },
            0x06 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_rlc(v); self.mmu.wb(a, v2); 4 },
            0x07 => { self.a = self.alu_rlc(self.a); 2 },
            0x08 => { self.b = self.alu_rrc(self.b); 2 },
            0x09 => { self.c = self.alu_rrc(self.c); 2 },
            0x0A => { self.d = self.alu_rrc(self.d); 2 },
            0x0B => { self.e = self.alu_rrc(self.e); 2 },
            0x0C => { self.h = self.alu_rrc(self.h); 2 },
            0x0D => { self.l = self.alu_rrc(self.l); 2 },
            0x0E => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_rrc(v); self.mmu.wb(a, v2); 4 },
            0x0F => { self.a = self.alu_rrc(self.a); 2 },
            0x10 => { self.b = self.alu_rl(self.b); 2 },
            0x11 => { self.c = self.alu_rl(self.c); 2 },
            0x12 => { self.d = self.alu_rl(self.d); 2 },
            0x13 => { self.e = self.alu_rl(self.e); 2 },
            0x14 => { self.h = self.alu_rl(self.h); 2 },
            0x15 => { self.l = self.alu_rl(self.l); 2 },
            0x16 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_rl(v); self.mmu.wb(a, v2); 4 },
            0x17 => { self.a = self.alu_rl(self.a); 2 },
            0x18 => { self.b = self.alu_rr(self.b); 2 },
            0x19 => { self.c = self.alu_rr(self.c); 2 },
            0x1A => { self.d = self.alu_rr(self.d); 2 },
            0x1B => { self.e = self.alu_rr(self.e); 2 },
            0x1C => { self.h = self.alu_rr(self.h); 2 },
            0x1D => { self.l = self.alu_rr(self.l); 2 },
            0x1E => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_rr(v); self.mmu.wb(a, v2); 4 },
            0x1F => { self.a = self.alu_rr(self.a); 2 },
            0x20 => { self.b = self.alu_sla(self.b); 2 },
            0x21 => { self.c = self.alu_sla(self.c); 2 },
            0x22 => { self.d = self.alu_sla(self.d); 2 },
            0x23 => { self.e = self.alu_sla(self.e); 2 },
            0x24 => { self.h = self.alu_sla(self.h); 2 },
            0x25 => { self.l = self.alu_sla(self.l); 2 },
            0x26 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_sla(v); self.mmu.wb(a, v2); 4 },
            0x27 => { self.a = self.alu_sla(self.a); 2 },
            0x28 => { self.b = self.alu_sra(self.b); 2 },
            0x29 => { self.c = self.alu_sra(self.c); 2 },
            0x2A => { self.d = self.alu_sra(self.d); 2 },
            0x2B => { self.e = self.alu_sra(self.e); 2 },
            0x2C => { self.h = self.alu_sra(self.h); 2 },
            0x2D => { self.l = self.alu_sra(self.l); 2 },
            0x2E => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_sra(v); self.mmu.wb(a, v2); 4 },
            0x2F => { self.a = self.alu_sra(self.a); 2 },
            0x30 => { self.b = self.alu_swap(self.b); 2 },
            0x31 => { self.c = self.alu_swap(self.c); 2 },
            0x32 => { self.d = self.alu_swap(self.d); 2 },
            0x33 => { self.e = self.alu_swap(self.e); 2 },
            0x34 => { self.h = self.alu_swap(self.h); 2 },
            0x35 => { self.l = self.alu_swap(self.l); 2 },
            0x36 => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_swap(v); self.mmu.wb(a, v2); 4 },
            0x37 => { self.a = self.alu_swap(self.a); 2 },
            0x38 => { self.b = self.alu_srl(self.b); 2 },
            0x39 => { self.c = self.alu_srl(self.c); 2 },
            0x3A => { self.d = self.alu_srl(self.d); 2 },
            0x3B => { self.e = self.alu_srl(self.e); 2 },
            0x3C => { self.h = self.alu_srl(self.h); 2 },
            0x3D => { self.l = self.alu_srl(self.l); 2 },
            0x3E => { let a = self.hl(); let v = self.mmu.rb(a); let v2 = self.alu_srl(v); self.mmu.wb(a, v2); 4 },
            0x3F => { self.a = self.alu_srl(self.a); 2 },
            0x40 => { self.alu_bit(self.b, 0); 2 },
            0x41 => { self.alu_bit(self.c, 0); 2 },
            0x42 => { self.alu_bit(self.d, 0); 2 },
            0x43 => { self.alu_bit(self.e, 0); 2 },
            0x44 => { self.alu_bit(self.h, 0); 2 },
            0x45 => { self.alu_bit(self.l, 0); 2 },
            0x46 => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 0); 3 },
            0x47 => { self.alu_bit(self.a, 0); 2 },
            0x48 => { self.alu_bit(self.b, 1); 2 },
            0x49 => { self.alu_bit(self.c, 1); 2 },
            0x4A => { self.alu_bit(self.d, 1); 2 },
            0x4B => { self.alu_bit(self.e, 1); 2 },
            0x4C => { self.alu_bit(self.h, 1); 2 },
            0x4D => { self.alu_bit(self.l, 1); 2 },
            0x4E => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 1); 3 },
            0x4F => { self.alu_bit(self.a, 1); 2 },
            0x50 => { self.alu_bit(self.b, 2); 2 },
            0x51 => { self.alu_bit(self.c, 2); 2 },
            0x52 => { self.alu_bit(self.d, 2); 2 },
            0x53 => { self.alu_bit(self.e, 2); 2 },
            0x54 => { self.alu_bit(self.h, 2); 2 },
            0x55 => { self.alu_bit(self.l, 2); 2 },
            0x56 => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 2); 3 },
            0x57 => { self.alu_bit(self.a, 2); 2 },
            0x58 => { self.alu_bit(self.b, 3); 2 },
            0x59 => { self.alu_bit(self.c, 3); 2 },
            0x5A => { self.alu_bit(self.d, 3); 2 },
            0x5B => { self.alu_bit(self.e, 3); 2 },
            0x5C => { self.alu_bit(self.h, 3); 2 },
            0x5D => { self.alu_bit(self.l, 3); 2 },
            0x5E => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 3); 3 },
            0x5F => { self.alu_bit(self.a, 3); 2 },
            0x60 => { self.alu_bit(self.b, 4); 2 },
            0x61 => { self.alu_bit(self.c, 4); 2 },
            0x62 => { self.alu_bit(self.d, 4); 2 },
            0x63 => { self.alu_bit(self.e, 4); 2 },
            0x64 => { self.alu_bit(self.h, 4); 2 },
            0x65 => { self.alu_bit(self.l, 4); 2 },
            0x66 => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 4); 3 },
            0x67 => { self.alu_bit(self.a, 4); 2 },
            0x68 => { self.alu_bit(self.b, 5); 2 },
            0x69 => { self.alu_bit(self.c, 5); 2 },
            0x6A => { self.alu_bit(self.d, 5); 2 },
            0x6B => { self.alu_bit(self.e, 5); 2 },
            0x6C => { self.alu_bit(self.h, 5); 2 },
            0x6D => { self.alu_bit(self.l, 5); 2 },
            0x6E => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 5); 3 },
            0x6F => { self.alu_bit(self.a, 5); 2 },
            0x70 => { self.alu_bit(self.b, 6); 2 },
            0x71 => { self.alu_bit(self.c, 6); 2 },
            0x72 => { self.alu_bit(self.d, 6); 2 },
            0x73 => { self.alu_bit(self.e, 6); 2 },
            0x74 => { self.alu_bit(self.h, 6); 2 },
            0x75 => { self.alu_bit(self.l, 6); 2 },
            0x76 => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 6); 3 },
            0x77 => { self.alu_bit(self.a, 6); 2 },
            0x78 => { self.alu_bit(self.b, 7); 2 },
            0x79 => { self.alu_bit(self.c, 7); 2 },
            0x7A => { self.alu_bit(self.d, 7); 2 },
            0x7B => { self.alu_bit(self.e, 7); 2 },
            0x7C => { self.alu_bit(self.h, 7); 2 },
            0x7D => { self.alu_bit(self.l, 7); 2 },
            0x7E => { let v = self.mmu.rb(self.hl()); self.alu_bit(v, 7); 3 },
            0x7F => { self.alu_bit(self.a, 7); 2 },
            0x80 => { self.b &= !(1 << 0); 2 },
            0x81 => { self.c &= !(1 << 0); 2 },
            0x82 => { self.d &= !(1 << 0); 2 },
            0x83 => { self.e &= !(1 << 0); 2 },
            0x84 => { self.h &= !(1 << 0); 2 },
            0x85 => { self.l &= !(1 << 0); 2 },
            0x86 => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 0); self.mmu.wb(a, v); 4 },
            0x87 => { self.a &= !(1 << 0); 2 },
            0x88 => { self.b &= !(1 << 1); 2 },
            0x89 => { self.c &= !(1 << 1); 2 },
            0x8A => { self.d &= !(1 << 1); 2 },
            0x8B => { self.e &= !(1 << 1); 2 },
            0x8C => { self.h &= !(1 << 1); 2 },
            0x8D => { self.l &= !(1 << 1); 2 },
            0x8E => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 1); self.mmu.wb(a, v); 4 },
            0x8F => { self.a &= !(1 << 1); 2 },
            0x90 => { self.b &= !(1 << 2); 2 },
            0x91 => { self.c &= !(1 << 2); 2 },
            0x92 => { self.d &= !(1 << 2); 2 },
            0x93 => { self.e &= !(1 << 2); 2 },
            0x94 => { self.h &= !(1 << 2); 2 },
            0x95 => { self.l &= !(1 << 2); 2 },
            0x96 => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 2); self.mmu.wb(a, v); 4 },
            0x97 => { self.a &= !(1 << 2); 2 },
            0x98 => { self.b &= !(1 << 3); 2 },
            0x99 => { self.c &= !(1 << 3); 2 },
            0x9A => { self.d &= !(1 << 3); 2 },
            0x9B => { self.e &= !(1 << 3); 2 },
            0x9C => { self.h &= !(1 << 3); 2 },
            0x9D => { self.l &= !(1 << 3); 2 },
            0x9E => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 3); self.mmu.wb(a, v); 4 },
            0x9F => { self.a &= !(1 << 3); 2 },
            0xA0 => { self.b &= !(1 << 4); 2 },
            0xA1 => { self.c &= !(1 << 4); 2 },
            0xA2 => { self.d &= !(1 << 4); 2 },
            0xA3 => { self.e &= !(1 << 4); 2 },
            0xA4 => { self.h &= !(1 << 4); 2 },
            0xA5 => { self.l &= !(1 << 4); 2 },
            0xA6 => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 4); self.mmu.wb(a, v); 4 },
            0xA7 => { self.a &= !(1 << 4); 2 },
            0xA8 => { self.b &= !(1 << 5); 2 },
            0xA9 => { self.c &= !(1 << 5); 2 },
            0xAA => { self.d &= !(1 << 5); 2 },
            0xAB => { self.e &= !(1 << 5); 2 },
            0xAC => { self.h &= !(1 << 5); 2 },
            0xAD => { self.l &= !(1 << 5); 2 },
            0xAE => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 5); self.mmu.wb(a, v); 4 },
            0xAF => { self.a &= !(1 << 5); 2 },
            0xB0 => { self.b &= !(1 << 6); 2 },
            0xB1 => { self.c &= !(1 << 6); 2 },
            0xB2 => { self.d &= !(1 << 6); 2 },
            0xB3 => { self.e &= !(1 << 6); 2 },
            0xB4 => { self.h &= !(1 << 6); 2 },
            0xB5 => { self.l &= !(1 << 6); 2 },
            0xB6 => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 6); self.mmu.wb(a, v); 4 },
            0xB7 => { self.a &= !(1 << 6); 2 },
            0xB8 => { self.b &= !(1 << 7); 2 },
            0xB9 => { self.c &= !(1 << 7); 2 },
            0xBA => { self.d &= !(1 << 7); 2 },
            0xBB => { self.e &= !(1 << 7); 2 },
            0xBC => { self.h &= !(1 << 7); 2 },
            0xBD => { self.l &= !(1 << 7); 2 },
            0xBE => { let a = self.hl(); let v = self.mmu.rb(a) & !(1 << 7); self.mmu.wb(a, v); 4 },
            0xBF => { self.a &= !(1 << 7); 2 },
            0xC0 => { self.b |= 1 << 0; 2 },
            0xC1 => { self.c |= 1 << 0; 2 },
            0xC2 => { self.d |= 1 << 0; 2 },
            0xC3 => { self.e |= 1 << 0; 2 },
            0xC4 => { self.h |= 1 << 0; 2 },
            0xC5 => { self.l |= 1 << 0; 2 },
            0xC6 => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 0); self.mmu.wb(a, v); 4 },
            0xC7 => { self.a |= 1 << 0; 2 },
            0xC8 => { self.b |= 1 << 1; 2 },
            0xC9 => { self.c |= 1 << 1; 2 },
            0xCA => { self.d |= 1 << 1; 2 },
            0xCB => { self.e |= 1 << 1; 2 },
            0xCC => { self.h |= 1 << 1; 2 },
            0xCD => { self.l |= 1 << 1; 2 },
            0xCE => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 1); self.mmu.wb(a, v); 4 },
            0xCF => { self.a |= 1 << 1; 2 },
            0xD0 => { self.b |= 1 << 2; 2 },
            0xD1 => { self.c |= 1 << 2; 2 },
            0xD2 => { self.d |= 1 << 2; 2 },
            0xD3 => { self.e |= 1 << 2; 2 },
            0xD4 => { self.h |= 1 << 2; 2 },
            0xD5 => { self.l |= 1 << 2; 2 },
            0xD6 => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 2); self.mmu.wb(a, v); 4 },
            0xD7 => { self.a |= 1 << 2; 2 },
            0xD8 => { self.b |= 1 << 3; 2 },
            0xD9 => { self.c |= 1 << 3; 2 },
            0xDA => { self.d |= 1 << 3; 2 },
            0xDB => { self.e |= 1 << 3; 2 },
            0xDC => { self.h |= 1 << 3; 2 },
            0xDD => { self.l |= 1 << 3; 2 },
            0xDE => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 3); self.mmu.wb(a, v); 4 },
            0xDF => { self.a |= 1 << 3; 2 },
            0xE0 => { self.b |= 1 << 4; 2 },
            0xE1 => { self.c |= 1 << 4; 2 },
            0xE2 => { self.d |= 1 << 4; 2 },
            0xE3 => { self.e |= 1 << 4; 2 },
            0xE4 => { self.h |= 1 << 4; 2 },
            0xE5 => { self.l |= 1 << 4; 2 },
            0xE6 => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 4); self.mmu.wb(a, v); 4 },
            0xE7 => { self.a |= 1 << 4; 2 },
            0xE8 => { self.b |= 1 << 5; 2 },
            0xE9 => { self.c |= 1 << 5; 2 },
            0xEA => { self.d |= 1 << 5; 2 },
            0xEB => { self.e |= 1 << 5; 2 },
            0xEC => { self.h |= 1 << 5; 2 },
            0xED => { self.l |= 1 << 5; 2 },
            0xEE => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 5); self.mmu.wb(a, v); 4 },
            0xEF => { self.a |= 1 << 5; 2 },
            0xF0 => { self.b |= 1 << 6; 2 },
            0xF1 => { self.c |= 1 << 6; 2 },
            0xF2 => { self.d |= 1 << 6; 2 },
            0xF3 => { self.e |= 1 << 6; 2 },
            0xF4 => { self.h |= 1 << 6; 2 },
            0xF5 => { self.l |= 1 << 6; 2 },
            0xF6 => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 6); self.mmu.wb(a, v); 4 },
            0xF7 => { self.a |= 1 << 6; 2 },
            0xF8 => { self.b |= 1 << 7; 2 },
            0xF9 => { self.c |= 1 << 7; 2 },
            0xFA => { self.d |= 1 << 7; 2 },
            0xFB => { self.e |= 1 << 7; 2 },
            0xFC => { self.h |= 1 << 7; 2 },
            0xFD => { self.l |= 1 << 7; 2 },
            0xFE => { let a = self.hl(); let v = self.mmu.rb(a) | (1 << 7); self.mmu.wb(a, v); 4 },
            0xFF => { self.a |= 1 << 7; 2 },
        }
    }

    pub fn alu_add(&mut self, b: u8, usec: bool) {
        let c = if usec && self.flag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
        self.set_flag(N, false);
        self.set_flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
        self.a = r;
    }

    pub fn alu_sub(&mut self, b: u8, usec: bool) {
        let c = if usec && self.flag(C) { 1 } else { 0 };
        let a = self.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.set_flag(N, true);
        self.set_flag(C, (a as u16) < (b as u16) + (c as u16));
        self.a = r;
    }

    pub fn alu_and(&mut self, b: u8) {
        let r = self.a & b;
        self.set_flag(Z, r == 0);
        self.set_flag(H, true);
        self.set_flag(C, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_or(&mut self, b: u8) {
        let r = self.a | b;
        self.set_flag(Z, r == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_xor(&mut self, b: u8) {
        let r = self.a ^ b;
        self.set_flag(Z, r == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.a = r;
    }

    pub fn alu_cp(&mut self, b: u8) {
        let r = self.a;
        self.alu_sub(b, false);
        self.a = r;
    }

    pub fn alu_inc(&mut self, a: u8) -> u8 {
        let r = a.wrapping_add(1);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) == 0x0F);
        self.set_flag(N, false);
        r
    }

    pub fn alu_dec(&mut self, a: u8) -> u8 {
        let r = a.wrapping_sub(1);
        self.set_flag(Z, r == 0);
        self.set_flag(H, (a & 0x0F) == 0);
        self.set_flag(N, true);
        r
    }

    pub fn alu_add16(&mut self, b: u16) {
        let a = self.hl();
        let r = a.wrapping_add(b);
        self.set_flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.set_flag(N, false);
        self.set_flag(C, a > 0xFFFF - b);
        self.set_hl(r);
    }

    pub fn alu_add16imm(&mut self, a: u16) -> u16 {
        let b = self.fetch_byte() as i8 as i16 as u16;
        self.set_flag(N, false);
        self.set_flag(Z, false);
        self.set_flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
        self.set_flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
        a.wrapping_add(b)
    }

    pub fn alu_swap(&mut self, a: u8) -> u8 {
        self.set_flag(Z, a == 0);
        self.set_flag(C, false);
        self.set_flag(H, false);
        self.set_flag(N, false);
        (a >> 4) | (a << 4)
    }

    pub fn alu_srflagupdate(&mut self, r: u8, c: bool) {
        self.set_flag(H, false);
        self.set_flag(N, false);
        self.set_flag(Z, r == 0);
        self.set_flag(C, c);
    }

    pub fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if c { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rl(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = (a << 1) | (if self.flag(C) { 1 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rrc(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if c { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_rr(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (if self.flag(C) { 0x80 } else { 0 });
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_sla(&mut self, a: u8) -> u8 {
        let c = a & 0x80 == 0x80;
        let r = a << 1;
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = (a >> 1) | (a & 0x80);
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let r = a >> 1;
        self.alu_srflagupdate(r, c);
        r
    }

    pub fn alu_bit(&mut self, a: u8, b: u8) {
        let r = a & (1 << (b as u32)) == 0;
        self.set_flag(N, false);
        self.set_flag(H, true);
        self.set_flag(Z, r);
    }

    pub fn alu_daa(&mut self) {
        let mut a = self.a;
        let mut adjust = if self.flag(C) { 0x60 } else { 0x00 };
        if self.flag(H) {
            adjust |= 0x06;
        };
        if !self.flag(N) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.set_flag(C, adjust >= 0x60);
        self.set_flag(H, false);
        self.set_flag(Z, a == 0);
        self.a = a;
    }

    fn cpu_jr(&mut self) {
        let n = self.fetch_byte() as i8;
        self.pc = ((self.pc as u32 as i32) + (n as i32)) as u16;
    }

    // From rboy::Register

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn hld(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res - 1);
        res
    }
    pub fn hli(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res + 1);
        res
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_flag(&mut self, flags: CpuFlag, set: bool) {
        let mask = flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }

    pub fn flag(&self, flags: CpuFlag) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }
}
