use pokemon_synthesizer::SoundIterator;
use rodio::Source;

use crate::{rom::ROM, sound2::Sfx as SfxTrait};

pub const DENIED: Sfx = Sfx::new(0x02, 0x41ef);
pub const PRESS_AB: Sfx = Sfx::new(0x02, 0x41b0);

pub const CRY_00: Sfx = Sfx::new(0x02, 0x403c);
pub const CRY_01: Sfx = Sfx::new(0x02, 0x4045);
pub const CRY_02: Sfx = Sfx::new(0x02, 0x404e);
pub const CRY_03: Sfx = Sfx::new(0x02, 0x4057);
pub const CRY_04: Sfx = Sfx::new(0x02, 0x4060);
pub const CRY_05: Sfx = Sfx::new(0x02, 0x4069);
pub const CRY_06: Sfx = Sfx::new(0x02, 0x4072);
pub const CRY_07: Sfx = Sfx::new(0x02, 0x407b);
pub const CRY_08: Sfx = Sfx::new(0x02, 0x4084);
pub const CRY_09: Sfx = Sfx::new(0x02, 0x408d);
pub const CRY_0A: Sfx = Sfx::new(0x02, 0x4096);
pub const CRY_0B: Sfx = Sfx::new(0x02, 0x409f);
pub const CRY_0C: Sfx = Sfx::new(0x02, 0x40a8);
pub const CRY_0D: Sfx = Sfx::new(0x02, 0x40b1);
pub const CRY_0E: Sfx = Sfx::new(0x02, 0x40ba);
pub const CRY_0F: Sfx = Sfx::new(0x02, 0x40c3);
pub const CRY_10: Sfx = Sfx::new(0x02, 0x40cc);
pub const CRY_11: Sfx = Sfx::new(0x02, 0x40d5);
pub const CRY_12: Sfx = Sfx::new(0x02, 0x40de);
pub const CRY_13: Sfx = Sfx::new(0x02, 0x40e7);
pub const CRY_14: Sfx = Sfx::new(0x02, 0x40f0);
pub const CRY_15: Sfx = Sfx::new(0x02, 0x40f9);
pub const CRY_16: Sfx = Sfx::new(0x02, 0x4102);
pub const CRY_17: Sfx = Sfx::new(0x02, 0x410b);
pub const CRY_18: Sfx = Sfx::new(0x02, 0x4114);
pub const CRY_19: Sfx = Sfx::new(0x02, 0x411d);
pub const CRY_1A: Sfx = Sfx::new(0x02, 0x4126);
pub const CRY_1B: Sfx = Sfx::new(0x02, 0x412f);
pub const CRY_1C: Sfx = Sfx::new(0x02, 0x4138);
pub const CRY_1D: Sfx = Sfx::new(0x02, 0x4141);
pub const CRY_1E: Sfx = Sfx::new(0x02, 0x414a);
pub const CRY_1F: Sfx = Sfx::new(0x02, 0x4153);
pub const CRY_20: Sfx = Sfx::new(0x02, 0x415c);
pub const CRY_21: Sfx = Sfx::new(0x02, 0x4165);
pub const CRY_22: Sfx = Sfx::new(0x02, 0x416e);
pub const CRY_23: Sfx = Sfx::new(0x02, 0x4177);
pub const CRY_24: Sfx = Sfx::new(0x02, 0x4180);
pub const CRY_25: Sfx = Sfx::new(0x02, 0x4189);

#[derive(Debug, Clone, Copy)]
pub struct Sfx {
    bank: u8,
    addr: u16,
    pitch: u8,
    length: i8,
}

impl Sfx {
    const fn new(bank: u8, addr: u16) -> Self {
        Self {
            bank,
            addr,
            pitch: 0,
            length: 0,
        }
    }

    #[rustfmt::skip]
    pub fn from_bank_and_id(bank: u8, id: u8) -> Option<Sfx> {
        match (bank, id) {
            // Instruments
            (0x02, 1..=19) => Some(Sfx::new(0x1f, 0x4000 + ((id as u16) * 3))),
            (0x08, 1..=19) => Some(Sfx::new(0x1f, 0x4000 + ((id as u16) * 3))),
            (0x1f, 1..=19) => Some(Sfx::new(0x1f, 0x4000 + ((id as u16) * 3))),
            (0x20, 1..=19) => Some(Sfx::new(0x1f, 0x4000 + ((id as u16) * 3))),

            // Cries
            (_, 20) => Some(CRY_00),
            (_, 23) => Some(CRY_01),
            (_, 26) => Some(CRY_02),
            (_, 29) => Some(CRY_03),
            (_, 32) => Some(CRY_04),
            (_, 35) => Some(CRY_05),
            (_, 38) => Some(CRY_06),
            (_, 41) => Some(CRY_07),
            (_, 44) => Some(CRY_08),
            (_, 47) => Some(CRY_09),
            (_, 50) => Some(CRY_0A),
            (_, 53) => Some(CRY_0B),
            (_, 56) => Some(CRY_0C),
            (_, 59) => Some(CRY_0D),
            (_, 62) => Some(CRY_0E),
            (_, 65) => Some(CRY_0F),
            (_, 68) => Some(CRY_10),
            (_, 71) => Some(CRY_11),
            (_, 74) => Some(CRY_12),
            (_, 77) => Some(CRY_13),
            (_, 80) => Some(CRY_14),
            (_, 83) => Some(CRY_15),
            (_, 86) => Some(CRY_16),
            (_, 89) => Some(CRY_17),
            (_, 92) => Some(CRY_18),
            (_, 95) => Some(CRY_19),
            (_, 98) => Some(CRY_1A),
            (_, 101) => Some(CRY_1B),
            (_, 104) => Some(CRY_1C),
            (_, 107) => Some(CRY_1D),
            (_, 110) => Some(CRY_1E),
            (_, 113) => Some(CRY_1F),
            (_, 116) => Some(CRY_20),
            (_, 119) => Some(CRY_21),
            (_, 122) => Some(CRY_22),
            (_, 125) => Some(CRY_23),
            (_, 128) => Some(CRY_24),
            (_, 131) => Some(CRY_25),

            // Bank 02
            (0x02, 134) => { eprintln!("Missing sound: SFX_Get_Item1_1"); None },
            (0x02, 137) => { eprintln!("Missing sound: SFX_Get_Item2_1"); None },
            (0x02, 140) => Some(Sfx::new(0x02, 0x41a4)), // SFX_Tink_1
            (0x02, 141) => Some(Sfx::new(0x02, 0x41a7)), // SFX_Heal_HP_1
            (0x02, 142) => Some(Sfx::new(0x02, 0x41aa)), // SFX_Heal_Ailment_1
            (0x02, 143) => Some(Sfx::new(0x02, 0x41ad)), // SFX_Start_Menu_1
            (0x02, 144) => Some(Sfx::new(0x02, 0x41b0)), // SFX_Press_AB_1
            (0x02, 145) => { eprintln!("Missing sound: SFX_Pokedex_Rating_1"); None },
            (0x02, 148) => { eprintln!("Missing sound: SFX_Get_Key_Item_1"); None },
            (0x02, 151) => Some(Sfx::new(0x02, 0x41c5)), // SFX_Poisoned_1
            (0x02, 152) => Some(Sfx::new(0x02, 0x41c8)), // SFX_Trade_Machine_1
            (0x02, 153) => Some(Sfx::new(0x02, 0x41cb)), // SFX_Turn_On_PC_1
            (0x02, 154) => Some(Sfx::new(0x02, 0x41ce)), // SFX_Turn_Off_PC_1
            (0x02, 155) => Some(Sfx::new(0x02, 0x41d1)), // SFX_Enter_PC_1
            (0x02, 156) => Some(Sfx::new(0x02, 0x41d4)), // SFX_Shrink_1
            (0x02, 157) => Some(Sfx::new(0x02, 0x41d7)), // SFX_Switch_1
            (0x02, 158) => Some(Sfx::new(0x02, 0x41da)), // SFX_Healing_Machine_1
            (0x02, 159) => Some(Sfx::new(0x02, 0x41dd)), // SFX_Teleport_Exit1_1
            (0x02, 160) => Some(Sfx::new(0x02, 0x41e0)), // SFX_Teleport_Enter1_1
            (0x02, 161) => Some(Sfx::new(0x02, 0x41e3)), // SFX_Teleport_Exit2_1
            (0x02, 162) => Some(Sfx::new(0x02, 0x41e6)), // SFX_Ledge_1
            (0x02, 163) => Some(Sfx::new(0x02, 0x41e9)), // SFX_Teleport_Enter2_1
            (0x02, 164) => Some(Sfx::new(0x02, 0x41ec)), // SFX_Fly_1
            (0x02, 165) => Some(Sfx::new(0x02, 0x41ef)), // SFX_Denied_1
            (0x02, 167) => Some(Sfx::new(0x02, 0x41f5)), // SFX_Arrow_Tiles_1
            (0x02, 168) => Some(Sfx::new(0x02, 0x41f8)), // SFX_Push_Boulder_1
            (0x02, 169) => Some(Sfx::new(0x02, 0x41fb)), // SFX_SS_Anne_Horn_1
            (0x02, 171) => Some(Sfx::new(0x02, 0x4201)), // SFX_Withdraw_Deposit_1
            (0x02, 172) => Some(Sfx::new(0x02, 0x4204)), // SFX_Cut_1
            (0x02, 173) => Some(Sfx::new(0x02, 0x4207)), // SFX_Go_Inside_1
            (0x02, 174) => Some(Sfx::new(0x02, 0x420a)), // SFX_Swap_1
            (0x02, 176) => Some(Sfx::new(0x02, 0x4210)), // SFX_59_1
            (0x02, 178) => Some(Sfx::new(0x02, 0x4216)), // SFX_Purchase_1
            (0x02, 180) => Some(Sfx::new(0x02, 0x421c)), // SFX_Collision_1
            (0x02, 181) => Some(Sfx::new(0x02, 0x421f)), // SFX_Go_Outside_1
            (0x02, 182) => Some(Sfx::new(0x02, 0x4222)), // SFX_Save_1
            (0x02, 184) => { eprintln!("Missing sound: SFX_Pokeflute"); None },
            (0x02, 185) => Some(Sfx::new(0x02, 0x422b)), // SFX_Safari_Zone_PA

            // Bank 08
            (0x08, 134) => { eprintln!("Missing sound: SFX_Level_Up"); None },
            (0x08, 137) => { eprintln!("Missing sound: SFX_Get_Item2_2"); None },
            (0x08, 140) => Some(Sfx::new(0x08, 0x41a4)), // SFX_Tink_2
            (0x08, 141) => Some(Sfx::new(0x08, 0x41a7)), // SFX_Heal_HP_2
            (0x08, 142) => Some(Sfx::new(0x08, 0x41aa)), // SFX_Heal_Ailment_2
            (0x08, 143) => Some(Sfx::new(0x08, 0x41ad)), // SFX_Start_Menu_2
            (0x08, 144) => Some(Sfx::new(0x08, 0x41b0)), // SFX_Press_AB_2
            (0x08, 145) => Some(Sfx::new(0x08, 0x41b3)), // SFX_Ball_Toss
            (0x08, 147) => Some(Sfx::new(0x08, 0x41b9)), // SFX_Ball_Poof
            (0x08, 149) => Some(Sfx::new(0x08, 0x41bf)), // SFX_Faint_Thud
            (0x08, 151) => Some(Sfx::new(0x08, 0x41c5)), // SFX_Run
            (0x08, 152) => Some(Sfx::new(0x08, 0x41c8)), // SFX_Dex_Page_Added
            (0x08, 154) => { eprintln!("Missing sound: SFX_Caught_Mon"); None },
            (0x08, 157) => Some(Sfx::new(0x08, 0x41d7)), // SFX_Peck
            (0x08, 158) => Some(Sfx::new(0x08, 0x41da)), // SFX_Faint_Fall
            (0x08, 159) => Some(Sfx::new(0x08, 0x41dd)), // SFX_Battle_09
            (0x08, 160) => Some(Sfx::new(0x08, 0x41e0)), // SFX_Pound
            (0x08, 161) => Some(Sfx::new(0x08, 0x41e3)), // SFX_Battle_0B
            (0x08, 162) => Some(Sfx::new(0x08, 0x41e6)), // SFX_Battle_0C
            (0x08, 163) => Some(Sfx::new(0x08, 0x41e9)), // SFX_Battle_0D
            (0x08, 164) => Some(Sfx::new(0x08, 0x41ec)), // SFX_Battle_0E
            (0x08, 165) => Some(Sfx::new(0x08, 0x41ef)), // SFX_Battle_0F
            (0x08, 166) => Some(Sfx::new(0x08, 0x41f2)), // SFX_Damage
            (0x08, 167) => Some(Sfx::new(0x08, 0x41f5)), // SFX_Not_Very_Effective
            (0x08, 168) => Some(Sfx::new(0x08, 0x41f8)), // SFX_Battle_12
            (0x08, 169) => Some(Sfx::new(0x08, 0x41fb)), // SFX_Battle_13
            (0x08, 170) => Some(Sfx::new(0x08, 0x41fe)), // SFX_Battle_14
            (0x08, 171) => Some(Sfx::new(0x08, 0x4201)), // SFX_Vine_Whip
            (0x08, 172) => Some(Sfx::new(0x08, 0x4204)), // SFX_Battle_16
            (0x08, 173) => Some(Sfx::new(0x08, 0x4207)), // SFX_Battle_17
            (0x08, 174) => Some(Sfx::new(0x08, 0x420a)), // SFX_Battle_18
            (0x08, 175) => Some(Sfx::new(0x08, 0x420d)), // SFX_Battle_19
            (0x08, 176) => Some(Sfx::new(0x08, 0x4210)), // SFX_Super_Effective
            (0x08, 177) => Some(Sfx::new(0x08, 0x4213)), // SFX_Battle_1B
            (0x08, 178) => Some(Sfx::new(0x08, 0x4216)), // SFX_Battle_1C
            (0x08, 179) => Some(Sfx::new(0x08, 0x4219)), // SFX_Doubleslap
            (0x08, 180) => Some(Sfx::new(0x08, 0x421c)), // SFX_Battle_1E
            (0x08, 182) => Some(Sfx::new(0x08, 0x4222)), // SFX_Horn_Drill
            (0x08, 183) => Some(Sfx::new(0x08, 0x4225)), // SFX_Battle_20
            (0x08, 184) => Some(Sfx::new(0x08, 0x4228)), // SFX_Battle_21
            (0x08, 185) => Some(Sfx::new(0x08, 0x422b)), // SFX_Battle_22
            (0x08, 186) => Some(Sfx::new(0x08, 0x422e)), // SFX_Battle_23
            (0x08, 187) => Some(Sfx::new(0x08, 0x4231)), // SFX_Battle_24
            (0x08, 189) => Some(Sfx::new(0x08, 0x4237)), // SFX_Battle_25
            (0x08, 190) => Some(Sfx::new(0x08, 0x423a)), // SFX_Battle_26
            (0x08, 191) => Some(Sfx::new(0x08, 0x423d)), // SFX_Battle_27
            (0x08, 194) => Some(Sfx::new(0x08, 0x4246)), // SFX_Battle_28
            (0x08, 197) => Some(Sfx::new(0x08, 0x424f)), // SFX_Battle_29
            (0x08, 199) => Some(Sfx::new(0x08, 0x4255)), // SFX_Battle_2A
            (0x08, 202) => Some(Sfx::new(0x08, 0x425e)), // SFX_Battle_2B
            (0x08, 204) => Some(Sfx::new(0x08, 0x4264)), // SFX_Battle_2C
            (0x08, 207) => Some(Sfx::new(0x08, 0x426d)), // SFX_Psybeam
            (0x08, 210) => Some(Sfx::new(0x08, 0x4276)), // SFX_Battle_2E
            (0x08, 213) => Some(Sfx::new(0x08, 0x427f)), // SFX_Battle_2F
            (0x08, 216) => Some(Sfx::new(0x08, 0x4288)), // SFX_Psychic_M
            (0x08, 219) => Some(Sfx::new(0x08, 0x4291)), // SFX_Battle_31
            (0x08, 221) => Some(Sfx::new(0x08, 0x4297)), // SFX_Battle_32
            (0x08, 223) => Some(Sfx::new(0x08, 0x429d)), // SFX_Battle_33
            (0x08, 225) => Some(Sfx::new(0x08, 0x42a3)), // SFX_Battle_34
            (0x08, 228) => { eprintln!("Missing sound: SFX_Battle_35"); None },
            (0x08, 230) => Some(Sfx::new(0x08, 0x42b2)), // SFX_Battle_36
            (0x08, 233) => Some(Sfx::new(0x08, 0x42bb)), // SFX_Silph_Scope

            // Bank 1f
            (0x1f, 134) => { eprintln!("Missing sound: SFX_Get_Item1_3"); None },
            (0x1f, 137) => { eprintln!("Missing sound: SFX_Get_Item2_3"); None },
            (0x1f, 140) => Some(Sfx::new(0x1f, 0x41a4)), // SFX_Tink_3
            (0x1f, 141) => Some(Sfx::new(0x1f, 0x41a7)), // SFX_Heal_HP_3
            (0x1f, 142) => Some(Sfx::new(0x1f, 0x41aa)), // SFX_Heal_Ailment_3
            (0x1f, 143) => Some(Sfx::new(0x1f, 0x41ad)), // SFX_Start_Menu_3
            (0x1f, 144) => Some(Sfx::new(0x1f, 0x41b0)), // SFX_Press_AB_3
            (0x1f, 145) => { eprintln!("Missing sound: SFX_Pokedex_Rating_3"); None },
            (0x1f, 148) => { eprintln!("Missing sound: SFX_Get_Key_Item_3"); None },
            (0x1f, 151) => Some(Sfx::new(0x1f, 0x41c5)), // SFX_Poisoned_3
            (0x1f, 152) => Some(Sfx::new(0x1f, 0x41c8)), // SFX_Trade_Machine_3
            (0x1f, 153) => Some(Sfx::new(0x1f, 0x41cb)), // SFX_Turn_On_PC_3
            (0x1f, 154) => Some(Sfx::new(0x1f, 0x41ce)), // SFX_Turn_Off_PC_3
            (0x1f, 155) => Some(Sfx::new(0x1f, 0x41d1)), // SFX_Enter_PC_3
            (0x1f, 156) => Some(Sfx::new(0x1f, 0x41d4)), // SFX_Shrink_3
            (0x1f, 157) => Some(Sfx::new(0x1f, 0x41d7)), // SFX_Switch_3
            (0x1f, 158) => Some(Sfx::new(0x1f, 0x41da)), // SFX_Healing_Machine_3
            (0x1f, 159) => Some(Sfx::new(0x1f, 0x41dd)), // SFX_Teleport_Exit1_3
            (0x1f, 160) => Some(Sfx::new(0x1f, 0x41e0)), // SFX_Teleport_Enter1_3
            (0x1f, 161) => Some(Sfx::new(0x1f, 0x41e3)), // SFX_Teleport_Exit2_3
            (0x1f, 162) => Some(Sfx::new(0x1f, 0x41e6)), // SFX_Ledge_3
            (0x1f, 163) => Some(Sfx::new(0x1f, 0x41e9)), // SFX_Teleport_Enter2_3
            (0x1f, 164) => Some(Sfx::new(0x1f, 0x41ec)), // SFX_Fly_3
            (0x1f, 165) => Some(Sfx::new(0x1f, 0x41ef)), // SFX_Denied_3
            (0x1f, 167) => Some(Sfx::new(0x1f, 0x41f5)), // SFX_Arrow_Tiles_3
            (0x1f, 168) => Some(Sfx::new(0x1f, 0x41f8)), // SFX_Push_Boulder_3
            (0x1f, 169) => Some(Sfx::new(0x1f, 0x41fb)), // SFX_SS_Anne_Horn_3
            (0x1f, 171) => Some(Sfx::new(0x1f, 0x4201)), // SFX_Withdraw_Deposit_3
            (0x1f, 172) => Some(Sfx::new(0x1f, 0x4204)), // SFX_Cut_3
            (0x1f, 173) => Some(Sfx::new(0x1f, 0x4207)), // SFX_Go_Inside_3
            (0x1f, 174) => Some(Sfx::new(0x1f, 0x420a)), // SFX_Swap_3
            (0x1f, 176) => Some(Sfx::new(0x1f, 0x4210)), // SFX_59_3
            (0x1f, 178) => Some(Sfx::new(0x1f, 0x4216)), // SFX_Purchase_3
            (0x1f, 180) => Some(Sfx::new(0x1f, 0x421c)), // SFX_Collision_3
            (0x1f, 181) => Some(Sfx::new(0x1f, 0x421f)), // SFX_Go_Outside_3
            (0x1f, 182) => Some(Sfx::new(0x1f, 0x4222)), // SFX_Save_3
            (0x1f, 184) => Some(Sfx::new(0x1f, 0x4228)), // SFX_Intro_Lunge
            (0x1f, 185) => Some(Sfx::new(0x1f, 0x422b)), // SFX_Intro_Hip
            (0x1f, 186) => Some(Sfx::new(0x1f, 0x422e)), // SFX_Intro_Hop
            (0x1f, 187) => Some(Sfx::new(0x1f, 0x4231)), // SFX_Intro_Raise
            (0x1f, 188) => Some(Sfx::new(0x1f, 0x4234)), // SFX_Intro_Crash
            (0x1f, 189) => Some(Sfx::new(0x1f, 0x4237)), // SFX_Intro_Whoosh
            (0x1f, 190) => Some(Sfx::new(0x1f, 0x423a)), // SFX_Slots_Stop_Wheel
            (0x1f, 191) => Some(Sfx::new(0x1f, 0x423d)), // SFX_Slots_Reward
            (0x1f, 192) => Some(Sfx::new(0x1f, 0x4240)), // SFX_Slots_New_Spin
            (0x1f, 194) => Some(Sfx::new(0x1f, 0x4246)), // SFX_Shooting_Star

            // Bank 20
            (0x20, 20) => Some(Sfx::new(0x20, 0x403c)), // SFX_Cry00_4
            (0x20, 23) => Some(Sfx::new(0x20, 0x4045)), // SFX_Cry01_4
            (0x20, 26) => Some(Sfx::new(0x20, 0x404e)), // SFX_Cry02_4
            (0x20, 29) => Some(Sfx::new(0x20, 0x4057)), // SFX_Cry03_4
            (0x20, 32) => Some(Sfx::new(0x20, 0x4060)), // SFX_Cry04_4
            (0x20, 35) => Some(Sfx::new(0x20, 0x4069)), // SFX_Cry05_4
            (0x20, 38) => Some(Sfx::new(0x20, 0x4072)), // SFX_Cry06_4
            (0x20, 41) => Some(Sfx::new(0x20, 0x407b)), // SFX_Cry07_4
            (0x20, 44) => Some(Sfx::new(0x20, 0x4084)), // SFX_Cry08_4
            (0x20, 47) => Some(Sfx::new(0x20, 0x408d)), // SFX_Cry09_4
            (0x20, 50) => Some(Sfx::new(0x20, 0x4096)), // SFX_Cry0A_4
            (0x20, 53) => Some(Sfx::new(0x20, 0x409f)), // SFX_Cry0B_4
            (0x20, 56) => Some(Sfx::new(0x20, 0x40a8)), // SFX_Cry0C_4
            (0x20, 59) => Some(Sfx::new(0x20, 0x40b1)), // SFX_Cry0D_4
            (0x20, 62) => Some(Sfx::new(0x20, 0x40ba)), // SFX_Cry0E_4
            (0x20, 65) => Some(Sfx::new(0x20, 0x40c3)), // SFX_Cry0F_4
            (0x20, 68) => Some(Sfx::new(0x20, 0x40cc)), // SFX_Cry10_4
            (0x20, 71) => Some(Sfx::new(0x20, 0x40d5)), // SFX_Cry11_4
            (0x20, 74) => Some(Sfx::new(0x20, 0x40de)), // SFX_Cry12_4
            (0x20, 77) => Some(Sfx::new(0x20, 0x40e7)), // SFX_Cry13_4
            (0x20, 80) => Some(Sfx::new(0x20, 0x40f0)), // SFX_Cry14_4
            (0x20, 83) => Some(Sfx::new(0x20, 0x40f9)), // SFX_Cry15_4
            (0x20, 86) => Some(Sfx::new(0x20, 0x4102)), // SFX_Cry16_4
            (0x20, 89) => Some(Sfx::new(0x20, 0x410b)), // SFX_Cry17_4
            (0x20, 92) => Some(Sfx::new(0x20, 0x4114)), // SFX_Cry18_4
            (0x20, 95) => Some(Sfx::new(0x20, 0x411d)), // SFX_Cry19_4
            (0x20, 98) => Some(Sfx::new(0x20, 0x4126)), // SFX_Cry1A_4
            (0x20, 101) => Some(Sfx::new(0x20, 0x412f)), // SFX_Cry1B_4
            (0x20, 104) => Some(Sfx::new(0x20, 0x4138)), // SFX_Cry1C_4
            (0x20, 107) => Some(Sfx::new(0x20, 0x4141)), // SFX_Cry1D_4
            (0x20, 110) => Some(Sfx::new(0x20, 0x414a)), // SFX_Cry1E_4
            (0x20, 113) => Some(Sfx::new(0x20, 0x4153)), // SFX_Cry1F_4
            (0x20, 116) => Some(Sfx::new(0x20, 0x415c)), // SFX_Cry20_4
            (0x20, 119) => Some(Sfx::new(0x20, 0x4165)), // SFX_Cry21_4
            (0x20, 122) => Some(Sfx::new(0x20, 0x416e)), // SFX_Cry22_4
            (0x20, 125) => Some(Sfx::new(0x20, 0x4177)), // SFX_Cry23_4
            (0x20, 128) => Some(Sfx::new(0x20, 0x4180)), // SFX_Cry24_4
            (0x20, 131) => Some(Sfx::new(0x20, 0x4189)), // SFX_Cry25_4
            (0x20, 134) => { eprintln!("Missing sound: SFX_Get_Item1_4"); None },
            (0x20, 137) => { eprintln!("Missing sound: SFX_Get_Item2_4"); None },
            (0x20, 140) => Some(Sfx::new(0x20, 0x41a4)), // SFX_Tink_4
            (0x20, 141) => Some(Sfx::new(0x20, 0x41a7)), // SFX_Heal_HP_4
            (0x20, 142) => Some(Sfx::new(0x20, 0x41aa)), // SFX_Heal_Ailment_4
            (0x20, 143) => Some(Sfx::new(0x20, 0x41ad)), // SFX_Start_Menu_4
            (0x20, 144) => Some(Sfx::new(0x20, 0x41b0)), // SFX_Press_AB_4
            (0x20, 145) => Some(Sfx::new(0x20, 0x41b3)), // SFX_Surfing_Jump
            (0x20, 146) => Some(Sfx::new(0x20, 0x41b6)), // SFX_Surfing_Flip
            (0x20, 147) => Some(Sfx::new(0x20, 0x41b9)), // SFX_Surfing_Crash
            (0x20, 148) => Some(Sfx::new(0x20, 0x41bc)), // SFX_Unknown_802cc
            (0x20, 149) => Some(Sfx::new(0x20, 0x41bf)), // SFX_Surfing_Land
            (0x20, 150) => { eprintln!("Missing sound: SFX_Get_Item2_4_2"); None },

            _ => None
        }
    }

    pub fn is_cry(&self) -> bool {
        const CRY_SFX_START: u16 = 0x403c;
        const CRY_SFX_END: u16 = 0x418c;

        (CRY_SFX_START..CRY_SFX_END).contains(&self.addr)
    }

    pub fn is_battle_sfx(&self) -> bool {
        if self.bank != 0x08 {
            return false;
        }

        const BATTLE_SFX_START: u16 = 0x41d7;
        const BATTLE_SFX_END: u16 = 0x42bc;

        (BATTLE_SFX_START..BATTLE_SFX_END).contains(&self.addr)
    }

    pub fn tweak(&mut self, pitch: u8, length: i8) {
        self.pitch = pitch;
        self.length = length;
    }

    pub fn tweaked(&self, pitch: u8, length: i8) -> Self {
        Self {
            bank: self.bank,
            addr: self.addr,
            pitch,
            length,
        }
    }
}

pub struct SynthesizerSource<'a>(SoundIterator<'a>);

impl<'a> SynthesizerSource<'a> {
    fn new(source: SoundIterator<'a>) -> SynthesizerSource<'a> {
        SynthesizerSource(source)
    }
}

impl Iterator for SynthesizerSource<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Source for SynthesizerSource<'_> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.0.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl SfxTrait<SynthesizerSource<'static>> for Sfx {
    fn open(self) -> SynthesizerSource<'static> {
        SynthesizerSource::new(
            pokemon_synthesizer::synthesis(ROM, self.bank, self.addr, self.pitch, self.length)
                .iter(),
        )
    }
}
