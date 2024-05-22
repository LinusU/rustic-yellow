use crate::{
    save_state::{BoxView, BoxViewMut, PartyView, PartyViewMut},
    PokemonSpecies,
};

const WRAM_SIZE: usize = 0x8000;

const BATTLE_MON_START: usize = 0x1013;
const BOX_DATA_START: usize = 0x1a7f;
const PARTY_DATA_START: usize = 0x1162;

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

pub struct GameState {
    data: [u8; WRAM_SIZE],
}

impl GameState {
    pub fn new() -> GameState {
        let mut data = [0; WRAM_SIZE];

        fill_random(&mut data, 42);

        GameState { data }
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
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

    /// after a battle, you have at least 3 steps before a random battle can occur
    pub fn number_of_no_random_battle_steps_left(&self) -> u8 {
        self.data[0x113b]
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

    pub fn cur_map(&self) -> u8 {
        self.data[0x135d]
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

    /// Offset of the current top menu item from the beginning of the list.
    ///
    /// Keeps track of what section of the list is on screen.
    pub fn set_list_scroll_offset(&mut self, value: u8) {
        self.data[0x0c36] = value;
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
    // pub const W_AUDIO_FADE_OUT_CONTROL: u16 = 0xcfc6;
    pub fn set_audio_fade_out_control(&mut self, value: u8) {
        self.data[0x0fc6] = value;
    }

    /// Low health alarm counter/enable. \
    /// high bit = enable, others = timer to cycle frequencies
    pub fn set_low_health_alarm(&mut self, value: u8) {
        self.data[0x1082] = value;
    }
}
