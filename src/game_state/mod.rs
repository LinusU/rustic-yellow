use crate::{
    game::constants::sprite_data_constants::PlayerDirection,
    save_state::{BoxView, BoxViewMut, PartyView, PartyViewMut},
    PokemonSpecies,
};

const WRAM_SIZE: usize = 0x8000;
const ZRAM_SIZE: usize = 0x7F;

const BATTLE_MON_START: usize = 0x1013;
const BOX_DATA_START: usize = 0x1a7f;
const PARTY_DATA_START: usize = 0x1162;

// wFlags_0xcd60
/// bit 0: is player engaged by trainer (to avoid being engaged by multiple trainers simultaneously) \
/// bit 1: boulder dust animation (from using Strength) pending \
/// bit 3: using generic PC \
/// bit 5: don't play sound when A or B is pressed in menu \
/// bit 6: tried pushing against boulder once (you need to push twice before it will move)
const W_CD60: usize = 0x0d60;

// wd728
/// bit 0: using Strength outside of battle \
/// bit 1: set by IsSurfingAllowed when surfing's allowed, but the caller resets it after checking the result \
/// bit 3: received Old Rod \
/// bit 4: received Good Rod \
/// bit 5: received Super Rod \
/// bit 6: gave one of the Saffron guards a drink \
/// bit 7: set by ItemUseCardKey, which is leftover code from a previous implementation of the Card Key
const W_D728: usize = 0x1727;

fn fill_random(slice: &mut [u8], start: u32) {
    // Simple LCG to generate (non-cryptographic) random values
    // Each distinct invocation should use a different start value
    const A: u32 = 1103515245;
    const C: u32 = 12345;

    let mut x = start;
    for v in slice.iter_mut() {
        x = x.wrapping_mul(A).wrapping_add(C);
        *v = ((x >> 23) & 0xFF) as u8;
    }
}

#[repr(u8)]
pub enum BattleResult {
    Win = 0,
    Lose = 1,
    Draw = 2,
}

#[repr(u8)]
pub enum CriticalHitOrOhko {
    NormalAttack = 0,
    CriticalHit = 1,
    SuccessfulOhko = 2,
    FailedOhko = 0xff,
}

pub struct BattleMonViewMut<'a> {
    data: &'a mut [u8],
}

impl BattleMonViewMut<'_> {
    pub fn new(data: &mut [u8]) -> BattleMonViewMut<'_> {
        BattleMonViewMut { data }
    }

    pub fn set_species(&mut self, value: Option<PokemonSpecies>) {
        self.data[0] = value.map_or(0, |s| s.into_index());
    }
}

pub struct BattleStatusView<'a> {
    data: &'a [u8],
}

impl BattleStatusView<'_> {
    pub fn new(data: &[u8]) -> BattleStatusView<'_> {
        BattleStatusView { data }
    }

    pub fn storing_energy(&self) -> bool {
        self.data[0] & 1 != 0
    }

    pub fn thrashing_about(&self) -> bool {
        self.data[0] & 2 != 0
    }

    pub fn attacking_multiple_times(&self) -> bool {
        self.data[0] & 4 != 0
    }

    pub fn using_rage(&self) -> bool {
        self.data[1] & 64 != 0
    }

    pub fn transformed(&self) -> bool {
        self.data[2] & 8 != 0
    }
}

pub struct MapConnectionView<'a> {
    data: &'a [u8],
}

impl MapConnectionView<'_> {
    pub fn new(data: &[u8]) -> MapConnectionView<'_> {
        MapConnectionView { data }
    }

    pub fn connected_map(&self) -> u8 {
        self.data[0]
    }

    pub fn connection_strip_src(&self) -> u16 {
        u16::from_le_bytes([self.data[1], self.data[2]])
    }

    pub fn connection_strip_dest(&self) -> u16 {
        u16::from_le_bytes([self.data[3], self.data[4]])
    }

    pub fn connection_strip_length(&self) -> u8 {
        self.data[5]
    }

    pub fn connected_map_width(&self) -> u8 {
        self.data[6]
    }

    pub fn connected_map_y_alignment(&self) -> u8 {
        self.data[7]
    }

    pub fn connected_map_x_alignment(&self) -> u8 {
        self.data[8]
    }

    pub fn connected_map_view_pointer(&self) -> u16 {
        u16::from_le_bytes([self.data[9], self.data[10]])
    }
}

pub struct GameState {
    data: [u8; WRAM_SIZE],
    high_ram: [u8; ZRAM_SIZE],
}

impl GameState {
    pub fn new() -> GameState {
        let mut data = [0; WRAM_SIZE];

        fill_random(&mut data, 42);

        GameState {
            data,
            high_ram: [0; ZRAM_SIZE],
        }
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn high_ram_byte(&self, addr: usize) -> u8 {
        self.high_ram[addr]
    }

    pub fn set_high_ram_byte(&mut self, addr: usize, value: u8) {
        self.high_ram[addr] = value;
    }

    pub fn battle_mon_mut(&mut self) -> BattleMonViewMut<'_> {
        BattleMonViewMut::new(&mut self.data[0x1013..])
    }

    pub fn r#box(&self) -> BoxView<'_> {
        BoxView::new(&self.data[BOX_DATA_START..])
    }

    pub fn box_mut(&mut self) -> BoxViewMut<'_> {
        BoxViewMut::new(&mut self.data[BOX_DATA_START..])
    }

    pub fn party(&self) -> PartyView<'_> {
        PartyView::new(&self.data[PARTY_DATA_START..])
    }

    pub fn party_mut(&mut self) -> PartyViewMut<'_> {
        PartyViewMut::new(&mut self.data[PARTY_DATA_START..])
    }

    pub fn set_sprite_player_state_data1_y_step_vector(&mut self, value: i8) {
        self.data[0x0103] = value as u8;
    }

    pub fn set_sprite_player_state_data1_x_step_vector(&mut self, value: i8) {
        self.data[0x0105] = value as u8;
    }

    /// remnant of debug mode; only set by the debug build. \
    /// if it is set: \
    /// 1. skips most of Prof. Oak's speech, and uses NINTEN as the player's name and SONY as the rival's name \
    /// 2. does not have the player start in floor two of the player's house (instead sending them to [wLastMap]) \
    /// 3. allows wild battles to be avoided by holding down B
    ///
    /// furthermore, in the debug build: \
    /// 4. allows trainers to be avoided by holding down B \
    /// 5. skips Safari Zone step counter by holding down B \
    /// 6. skips the NPC who blocks Route 3 before beating Brock by holding down B \
    /// 7. skips Cerulean City rival battle by holding down B \
    /// 8. skips Pokémon Tower rival battle by holding down B
    pub fn debug_mode(&self) -> bool {
        (self.data[0x1731] & 0b0000_0010) != 0
    }

    /// Target warp is a fly warp or a dungeon warp
    pub fn set_fly_or_dungeon_warp(&mut self, value: bool) {
        if value {
            self.data[0x1731] |= 1 << 2;
        } else {
            self.data[0x1731] &= !(1 << 2);
        }
    }

    /// Used warp pad, escape rope, dig, teleport, or fly, so the target warp is a "fly warp"
    pub fn used_warp_pad(&self) -> bool {
        (self.data[0x1731] & 0b0000_1000) != 0
    }

    pub fn set_used_warp_pad(&mut self, value: bool) {
        if value {
            self.data[0x1731] |= 1 << 3;
        } else {
            self.data[0x1731] &= !(1 << 3);
        }
    }

    /// Jumped into hole (Pokemon Mansion, Seafoam Islands, Victory Road) or went down waterfall (Seafoam Islands), so the target warp is a "dungeon warp"
    pub fn jumped_into_hole(&self) -> bool {
        (self.data[0x1731] & 0b0001_0000) != 0
    }

    // Currently being forced to ride bike (cycling road)
    pub fn set_forced_to_ride_bike(&mut self, value: bool) {
        if value {
            self.data[0x1731] |= 1 << 5;
        } else {
            self.data[0x1731] &= !(1 << 5);
        }
    }

    pub fn standing_on_warp(&self) -> bool {
        (self.data[0x1735] & (1 << 2)) != 0
    }

    pub fn set_standing_on_warp(&mut self, value: bool) {
        if value {
            self.data[0x1735] |= 1 << 2;
        } else {
            self.data[0x1735] &= !(1 << 2);
        }
    }

    pub fn set_warped_from_which_warp(&mut self, value: u8) {
        self.data[0x173a] = value;
    }

    pub fn set_warped_from_which_map(&mut self, value: u8) {
        self.data[0x173b] = value;
    }

    /// `walk_bike_surf_state` is sometimes copied here, but it doesn't seem to be used for anything
    pub fn set_walk_bike_surf_state_copy(&mut self, value: u8) {
        self.data[0x1119] = value;
    }

    /// Non-zero when the whole party has fainted due to out-of-battle poison damage
    pub fn out_of_battle_blackout(&self) -> u8 {
        self.data[0x112c]
    }

    /// Counts down once every step
    pub fn step_counter(&self) -> u8 {
        self.data[0x113a]
    }

    pub fn set_step_counter(&mut self, value: u8) {
        self.data[0x113a] = value;
    }

    /// after a battle, you have at least 3 steps before a random battle can occur
    pub fn number_of_no_random_battle_steps_left(&self) -> u8 {
        self.data[0x113b]
    }

    pub fn set_number_of_no_random_battle_steps_left(&mut self, value: u8) {
        self.data[0x113b] = value;
    }

    /// Offset subtracted from FadePal4 to get the background and object palettes for the current map
    /// normally, it is 0. It is 6 when Flash is needed, causing FadePal2 to be used instead of FadePal4
    pub fn map_pal_offset(&self) -> u8 {
        self.data[0x135c]
    }

    pub fn set_map_pal_offset(&mut self, value: u8) {
        self.data[0x135c] = value;
    }

    /// bit 0: If 0, limit the delay to 1 frame. Note that this has no effect if
    ///        the delay has been disabled entirely through bit 1 of this variable
    ///        or bit 6 of wd730. \
    /// bit 1: If 0, no delay.
    pub fn letter_printing_delay_flags(&self) -> u8 {
        self.data[0x1357]
    }

    pub fn set_letter_printing_delay_flags(&mut self, value: u8) {
        self.data[0x1357] = value;
    }

    pub fn player_move_list_index(&self) -> u8 {
        self.data[0x0c2e]
    }

    pub fn player_mon_number(&self) -> u8 {
        self.data[0x0c2f]
    }

    /// The next simulated joypad state is at wSimulatedJoypadStatesEnd plus this value minus 1
    ///
    /// 0 if the joypad state is not being simulated
    pub fn simulated_joypad_states_index(&self) -> u8 {
        self.data[0x0d38]
    }

    pub fn set_simulated_joypad_states_index(&mut self, value: u8) {
        self.data[0x0d38] = value;
    }

    /// 0 = neither \
    /// 1 = warp pad \
    /// 2 = hole
    pub fn standing_on_warp_pad_or_hole(&self) -> u8 {
        self.data[0x0d5b]
    }

    pub fn set_joy_ignore(&mut self, value: u8) {
        self.data[0x0d6b] = value;
    }

    /// Walk animation counter
    pub fn walk_counter(&self) -> u8 {
        self.data[0x0fc4]
    }

    pub fn set_walk_counter(&mut self, value: u8) {
        self.data[0x0fc4] = value;
    }

    /// Background tile number in front of the player (either 1 or 2 steps ahead)
    pub fn tile_in_front_of_player(&self) -> u8 {
        self.data[0x0fc5]
    }

    pub fn enemy_mon_species2(&self) -> u8 {
        self.data[0x0fd7]
    }

    pub fn set_enemy_mon_species2(&mut self, value: u8) {
        self.data[0x0fd7] = value;
    }

    pub fn set_trainer_class(&mut self, value: u8) {
        self.data[0x1030] = value;
    }

    /// number of times remaining that AI action can occur
    pub fn set_ai_count(&mut self, value: u8) {
        self.data[0x0cdf] = value;
    }

    pub fn set_enemy_mon_party_pos(&mut self, value: u8) {
        self.data[0x0fe7] = value;
    }

    pub fn is_in_battle(&self) -> u8 {
        self.data[0x1056]
    }

    pub fn set_is_in_battle(&mut self, value: u8) {
        self.data[0x1056] = value;
    }

    pub fn gym_leader_no(&self) -> u8 {
        self.data[0x105b]
    }

    pub fn player_battle_status(&self) -> BattleStatusView<'_> {
        BattleStatusView::new(&self.data[0x1061..])
    }

    pub fn cur_map(&self) -> u8 {
        self.data[0x135d]
    }

    pub fn set_cur_map(&mut self, value: u8) {
        self.data[0x135d] = value;
    }

    /// in a wild battle, this is the species of pokemon \
    /// in a trainer battle, this is the trainer class + OPP_ID_OFFSET
    pub fn cur_opponent(&self) -> u8 {
        self.data[0x1058]
    }

    pub fn set_cur_opponent(&mut self, value: u8) {
        self.data[0x1058] = value;
    }

    pub fn trainer_pic_pointer(&self) -> u16 {
        self.data[0x1032] as u16 | ((self.data[0x1033] as u16) << 8)
    }

    pub fn entering_cable_club(&self) -> bool {
        self.data[0x0c47] != 0
    }

    pub fn set_link_timeout_counter(&mut self, value: u8) {
        self.data[0x0c47] = value;
    }

    pub fn check_for_180_degree_turn(&self) -> u8 {
        self.data[0x0c4b]
    }

    pub fn set_check_for_180_degree_turn(&mut self, value: u8) {
        self.data[0x0c4b] = value;
    }

    pub fn is_player_engaged_by_trainer(&self) -> bool {
        self.data[W_CD60] & (1 << 0) != 0
    }

    pub fn boulder_dust_animation_pending(&self) -> bool {
        self.data[W_CD60] & (1 << 1) != 0
    }

    pub fn cd60_unknown_bit_2(&self) -> bool {
        self.data[W_CD60] & (1 << 2) != 0
    }

    pub fn set_cd60_unknown_bit_2(&mut self, value: bool) {
        if value {
            self.data[W_CD60] |= 1 << 2;
        } else {
            self.data[W_CD60] &= !(1 << 2);
        }
    }

    /// This has overlapping related uses. \
    /// When the player tries to use an item or use certain field moves, 0 is stored
    /// when the attempt fails and 1 is stored when the attempt succeeds. \
    /// In addition, some items store 2 for certain types of failures, but this
    /// cannot happen in battle. \
    /// In battle, a non-zero value indicates the player has taken their turn using
    /// something other than a move (e.g. using an item or switching pokemon). \
    /// So, when an item is successfully used in battle, this value becomes non-zero
    /// and the player is not allowed to make a move and the two uses are compatible.
    pub fn set_action_result_or_took_battle_turn(&mut self, value: u8) {
        self.data[0x0d6a] = value;
    }

    pub fn battle_result(&self) -> BattleResult {
        unsafe { std::mem::transmute(self.data[0x0f0b]) }
    }

    pub fn set_battle_result(&mut self, value: BattleResult) {
        self.data[0x0f0b] = value as u8;
    }

    /// Used in CheckForTilePairCollisions2 to store the tile the player is on
    pub fn tile_player_standing_on(&self) -> u8 {
        self.data[0x0f0e]
    }

    pub fn set_tile_player_standing_on(&mut self, value: u8) {
        self.data[0x0f0e] = value;
    }

    /// Offset of the current top menu item from the beginning of the list.
    ///
    /// Keeps track of what section of the list is on screen.
    pub fn set_list_scroll_offset(&mut self, value: u8) {
        self.data[0x0c36] = value;
    }

    /// Which NPC movement script pointer is being used.
    ///
    /// 0 if an NPC movement script is not running.
    pub fn npc_movement_script_pointer_table_num(&self) -> u8 {
        self.data[0x0c57]
    }

    pub fn set_critical_hit_or_ohko(&mut self, value: CriticalHitOrOhko) {
        self.data[0x105d] = value as u8;
    }

    /// Flags that indicate which party members should be be given exp when GainExperience is called.
    pub fn set_party_gain_exp_flags(&mut self, value: u8) {
        self.data[0x1057] = value;
    }

    /// Index in party of currently battling mon.
    pub fn set_player_mon_number(&mut self, value: u8) {
        self.data[0x0c2f] = value;
    }

    /// True when an item or move that allows escape from battle was used.
    pub fn set_escaped_from_battle(&mut self, value: bool) {
        self.data[0x1077] = value as u8;
    }

    pub fn predef_parent_bank(&self) -> u8 {
        self.data[0x0f12]
    }

    pub fn set_player_hp_bar_color(&mut self, value: u8) {
        self.data[0x0f1c] = value;
    }

    pub fn set_enemy_hp_bar_color(&mut self, value: u8) {
        self.data[0x0f1d] = value;
    }

    /// 1 flag for each party member indicating whether it can evolve. \
    /// The purpose of these flags is to track which mons levelled up during the
    /// current battle at the end of the battle when evolution occurs. \
    /// Other methods of evolution simply set it by calling TryEvolvingMon.
    pub fn set_can_evolve_flags(&mut self, value: u8) {
        self.data[0x0cd3] = value;
    }

    pub fn set_force_evolution(&mut self, value: bool) {
        self.data[0x0cd4] = value as u8;
    }

    /// Total amount of money made using Pay Day during the current battle.
    pub fn set_total_pay_day_money(&mut self, value: u32) {
        let digit0 = (value % 10) as u8;
        let digit1 = ((value / 10) % 10) as u8;
        let digit2 = ((value / 100) % 10) as u8;
        let digit3 = ((value / 1000) % 10) as u8;
        let digit4 = ((value / 10000) % 10) as u8;
        let digit5 = ((value / 100000) % 10) as u8;

        self.data[0x0ce5] = (digit5 << 4) | digit4;
        self.data[0x0ce6] = (digit3 << 4) | digit2;
        self.data[0x0ce7] = (digit1 << 4) | digit0;
    }

    /// The desired fade counter reload value is stored here prior to calling
    /// PlaySound in order to cause the current music to fade out before the new
    /// music begins playing. Storing 0 causes no fade out to occur and the new music
    /// to begin immediately.
    ///
    /// This variable has another use related to fade-out, as well. PlaySound stores
    /// the sound ID of the music that should be played after the fade-out is finished
    /// in this variable. FadeOutAudio checks if it's non-zero every V-Blank and
    /// fades out the current audio if it is. Once it has finished fading out the
    /// audio, it zeroes this variable and starts playing the sound ID stored in it.
    pub fn audio_fade_out_control(&self) -> u8 {
        self.data[0x0fc6]
    }

    pub fn set_audio_fade_out_control(&mut self, value: u8) {
        self.data[0x0fc6] = value;
    }

    /// $00 = causes sprites to be hidden and the value to change to $ff \
    /// $01 = enabled \
    /// $ff = disabled \
    /// other values aren't used
    pub fn update_sprites_enabled(&self) -> u8 {
        self.data[0x0fca]
    }

    pub fn set_update_sprites_enabled(&mut self, value: u8) {
        self.data[0x0fca] = value;
    }

    /// Low health alarm counter/enable. \
    /// high bit = enable, others = timer to cycle frequencies
    pub fn set_low_health_alarm(&mut self, value: u8) {
        self.data[0x1082] = value;
    }

    /// Counts downward each frame.
    ///
    /// When it hits 0, bit 5 (ignore input bit) of wd730 is reset.
    pub fn set_ignore_input_counter(&mut self, value: u8) {
        self.data[0x1139] = value;
    }

    pub fn set_map_music_sound_id(&mut self, value: u8) {
        self.data[0x135a] = value;
    }

    pub fn set_map_music_rom_bank(&mut self, value: u8) {
        self.data[0x135b] = value;
    }

    /// Pointer to the upper left corner of the current view in the tile block map
    pub fn current_tile_block_map_view_pointer(&self) -> u16 {
        u16::from_le_bytes([self.data[0x135e], self.data[0x135f]])
    }

    pub fn set_current_tile_block_map_view_pointer(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.data[0x135e] = bytes[0];
        self.data[0x135f] = bytes[1];
    }

    /// Player's Y position on the current map
    pub fn y_coord(&self) -> u8 {
        self.data[0x1360]
    }

    pub fn set_y_coord(&mut self, value: u8) {
        self.data[0x1360] = value;
    }

    /// Player's X position on the current map
    pub fn x_coord(&self) -> u8 {
        self.data[0x1361]
    }

    pub fn set_x_coord(&mut self, value: u8) {
        self.data[0x1361] = value;
    }

    /// Player's Y position (by block)
    pub fn y_block_coord(&self) -> u8 {
        self.data[0x1362]
    }

    pub fn set_y_block_coord(&mut self, value: u8) {
        self.data[0x1362] = value;
    }

    /// Player's X position (by block)
    pub fn x_block_coord(&self) -> u8 {
        self.data[0x1363]
    }

    pub fn set_x_block_coord(&mut self, value: u8) {
        self.data[0x1363] = value;
    }

    pub fn last_map(&self) -> u8 {
        self.data[0x1364]
    }

    pub fn set_last_map(&mut self, value: u8) {
        self.data[0x1364] = value;
    }

    pub fn cur_map_tileset(&self) -> u8 {
        self.data[0x1366]
    }

    pub fn set_cur_map_tileset(&mut self, value: u8) {
        self.data[0x1366] = value;
    }

    pub fn cur_map_height(&self) -> u8 {
        self.data[0x1367]
    }

    pub fn cur_map_width(&self) -> u8 {
        self.data[0x1368]
    }

    pub fn cur_map_data_ptr(&self) -> u16 {
        u16::from_le_bytes([self.data[0x1369], self.data[0x136a]])
    }

    pub fn north(&self) -> MapConnectionView {
        MapConnectionView::new(&self.data[0x1370..])
    }

    pub fn south(&self) -> MapConnectionView {
        MapConnectionView::new(&self.data[0x137b..])
    }

    pub fn west(&self) -> MapConnectionView {
        MapConnectionView::new(&self.data[0x1386..])
    }

    pub fn east(&self) -> MapConnectionView {
        MapConnectionView::new(&self.data[0x1391..])
    }

    /// Sprite set ID for the current map
    pub fn set_sprite_set_id(&mut self, value: u8) {
        self.data[0x13a7] = value;
    }

    /// The tile shown outside the boundaries of the map
    pub fn map_background_tile(&self) -> u8 {
        self.data[0x13ac]
    }

    pub fn set_map_background_tile(&mut self, value: u8) {
        self.data[0x13ac] = value;
    }

    /// Number of warps in current map (up to 32)
    pub fn number_of_warps(&self) -> u8 {
        self.data[0x13ad]
    }

    pub fn set_number_of_warps(&mut self, value: u8) {
        self.data[0x13ad] = value;
    }

    /// If $ff, the player's coordinates are not updated when entering the map
    pub fn destination_warp_id(&self) -> u8 {
        self.data[0x142e]
    }

    pub fn set_destination_warp_id(&mut self, value: u8) {
        self.data[0x142e] = value;
    }

    pub fn set_pikachu_overworld_state_flag_4(&mut self, value: bool) {
        if value {
            self.data[0x142f] |= 1 << 4;
        } else {
            self.data[0x142f] &= !(1 << 4);
        }
    }

    pub fn set_pikachu_overworld_state_flag_5(&mut self, value: bool) {
        if value {
            self.data[0x142f] |= 1 << 5;
        } else {
            self.data[0x142f] &= !(1 << 5);
        }
    }

    pub fn set_pikachu_spawn_state(&mut self, value: u8) {
        self.data[0x1430] = value;
    }

    /// Number of signs in the current map (up to 16)
    pub fn num_signs(&self) -> u8 {
        self.data[0x14af]
    }

    pub fn set_num_signs(&mut self, value: u8) {
        self.data[0x14af] = value;
    }

    /// Number of sprites on the current map (up to 16)
    pub fn set_num_sprites(&mut self, value: u8) {
        self.data[0x14e0] = value;
    }

    /// Map height in 2x2 meta-tiles
    pub fn current_map_height_2(&self) -> u8 {
        self.data[0x1523]
    }

    pub fn set_current_map_height_2(&mut self, value: u8) {
        self.data[0x1523] = value;
    }

    /// Map width in 2x2 meta-tiles
    pub fn current_map_width_2(&self) -> u8 {
        self.data[0x1524]
    }

    pub fn set_current_map_width_2(&mut self, value: u8) {
        self.data[0x1524] = value;
    }

    /// The address of the upper left corner of the visible portion of the BG tile map in VRAM
    pub fn map_view_vram_pointer(&self) -> u16 {
        u16::from_le_bytes([self.data[0x1525], self.data[0x1526]])
    }

    pub fn set_map_view_vram_pointer(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.data[0x1525] = bytes[0];
        self.data[0x1526] = bytes[1];
    }

    /// If the player is moving, the current direction. \
    /// If the player is not moving, zero.
    ///
    /// Map scripts write to this in order to change the player's facing direction.
    pub fn player_moving_direction(&self) -> u8 {
        self.data[0x1527]
    }

    pub fn set_player_moving_direction(&mut self, value: u8) {
        self.data[0x1527] = value;
    }

    /// The direction in which the player was moving before the player last stopped.
    pub fn player_last_stop_direction(&self) -> u8 {
        self.data[0x1528]
    }

    pub fn set_player_last_stop_direction(&mut self, value: u8) {
        self.data[0x1528] = value;
    }

    /// If the player is moving, the current direction \
    /// If the player is not moving, the last the direction in which the player moved
    pub fn player_direction(&self) -> u8 {
        self.data[0x1529]
    }

    pub fn set_player_direction(&mut self, direction: PlayerDirection) {
        self.data[0x1529] = direction as u8;
    }

    pub fn tileset_bank(&self) -> u8 {
        self.data[0x152a]
    }

    pub fn tileset_blocks_pointer(&self) -> u16 {
        u16::from_le_bytes([self.data[0x152b], self.data[0x152c]])
    }

    pub fn tileset_gfx_pointer(&self) -> u16 {
        u16::from_le_bytes([self.data[0x152d], self.data[0x152e]])
    }

    pub fn tileset_talking_over_tiles(&self) -> [u8; 3] {
        [self.data[0x1531], self.data[0x1532], self.data[0x1533]]
    }

    /// 0 = walking \
    /// 1 = biking \
    /// 2 = surfing
    pub fn walk_bike_surf_state(&self) -> u8 {
        self.data[0x16ff]
    }

    pub fn set_walk_bike_surf_state(&mut self, value: u8) {
        self.data[0x16ff] = value;
    }

    pub fn set_using_strength_out_of_battle(&mut self, value: bool) {
        let current = self.data[W_D728];

        if value {
            self.data[W_D728] = current | 1;
        } else {
            self.data[W_D728] = current & !1;
        }
    }

    /// If not set, the 3 minimum steps between random battles have passed
    pub fn block_random_battles(&self) -> bool {
        self.data[0x172b] & 1 == 1
    }

    pub fn set_block_random_battles(&mut self, value: bool) {
        if value {
            self.data[0x172b] |= 1;
        } else {
            self.data[0x172b] &= !1;
        }
    }

    // Do scripted warp (used to warp back to Lavender Town from the top of the pokemon tower)
    pub fn do_scripted_warp(&self) -> bool {
        self.data[0x172c] & (1 << 3) != 0
    }

    pub fn set_do_scripted_warp(&mut self, value: bool) {
        if value {
            self.data[0x172c] |= 1 << 3;
        } else {
            self.data[0x172c] &= !(1 << 3);
        }
    }

    pub fn d730_unknown_bit_2(&self) -> bool {
        self.data[0x172f] & (1 << 2) != 0
    }

    /// Set if joypad states are being simulated in the overworld or an NPC's movement is being scripted
    pub fn joypad_is_simulated(&self) -> bool {
        self.data[0x172f] & (1 << 7) != 0
    }

    pub fn safari_zone_game_over(&self) -> u8 {
        self.data[0x1a45]
    }

    pub fn warp_destination_map(&self) -> u8 {
        self.high_ram[0x0b]
    }

    pub fn set_warp_destination_map(&mut self, value: u8) {
        self.high_ram[0x0b] = value;
    }

    pub fn previous_tileset(&self) -> u8 {
        self.high_ram[0x0b]
    }

    pub fn set_previous_tileset(&mut self, value: u8) {
        self.high_ram[0x0b] = value;
    }

    pub fn loaded_rom_bank(&self) -> u8 {
        self.high_ram[0x38]
    }

    pub fn set_loaded_rom_bank(&mut self, value: u8) {
        self.high_ram[0x38] = value;
    }

    pub fn set_redraw_row_or_column_mode(&mut self, value: u8) {
        self.high_ram[0x50] = value;
    }

    pub fn set_redraw_row_or_column_dest(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.high_ram[0x51] = bytes[0];
        self.high_ram[0x52] = bytes[1];
    }

    /// Controls which tiles are animated.
    ///
    /// 0 = no animations (breaks Surf) \
    /// 1 = water tile $14 is animated \
    /// 2 = water tile $14 and flower tile $03 are animated
    pub fn set_tile_animations(&mut self, value: u8) {
        self.high_ram[0x57] = value;
    }

    pub fn set_moving_bg_tiles_counter1(&mut self, value: u8) {
        self.high_ram[0x58] = value;
    }
}
