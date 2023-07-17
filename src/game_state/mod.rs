use crate::save_state::{BoxView, BoxViewMut, PartyView, PartyViewMut};

const WRAM_SIZE: usize = 0x8000;

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

    pub fn set_enemy_mon_species2(&mut self, value: u8) {
        self.data[0x0fd7] = value;
    }

    pub fn set_cur_opponent(&mut self, value: u8) {
        self.data[0x1058] = value;
    }

    pub fn trainer_pic_pointer(&self) -> u16 {
        self.data[0x1032] as u16 | ((self.data[0x1033] as u16) << 8)
    }
}
