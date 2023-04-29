use crate::{cpu::Cpu, game::ram::wram, sound2::Music};

fn music_from_bank_and_id(bank: u8, id: u8) -> Option<Music> {
    match (bank, id) {
        (0x02, 186) => Some(Music::PalletTown),
        (0x02, 189) => Some(Music::Pokecenter),
        (0x02, 192) => Some(Music::Gym),
        (0x02, 195) => Some(Music::Cities1),
        (0x02, 199) => Some(Music::Cities2),
        (0x02, 202) => Some(Music::Celadon),
        (0x02, 205) => Some(Music::Cinnabar),
        (0x02, 208) => Some(Music::Vermilion),
        (0x02, 212) => Some(Music::Lavender),
        (0x02, 216) => Some(Music::SSAnne),
        (0x02, 219) => Some(Music::MeetProfOak),
        (0x02, 222) => Some(Music::MeetRival),
        (0x02, 225) => Some(Music::MuseumGuy),
        (0x02, 229) => Some(Music::SafariZone),
        (0x02, 232) => Some(Music::PkmnHealed),
        (0x02, 235) => Some(Music::Routes1),
        (0x02, 239) => Some(Music::Routes2),
        (0x02, 243) => Some(Music::Routes3),
        (0x02, 247) => Some(Music::Routes4),
        (0x02, 251) => Some(Music::IndigoPlateau),

        (0x08, 234) => Some(Music::GymLeaderBattle),
        (0x08, 237) => Some(Music::TrainerBattle),
        (0x08, 240) => Some(Music::WildBattle),
        (0x08, 243) => Some(Music::FinalBattle),
        (0x08, 246) => Some(Music::DefeatedTrainer),
        (0x08, 249) => Some(Music::DefeatedWildMon),
        (0x08, 252) => Some(Music::DefeatedGymLeader),

        (0x1f, 195) => Some(Music::TitleScreen),
        (0x1f, 199) => Some(Music::Credits),
        (0x1f, 202) => Some(Music::HallOfFame),
        (0x1f, 205) => Some(Music::OaksLab),
        (0x1f, 208) => Some(Music::JigglypuffSong),
        (0x1f, 210) => Some(Music::BikeRiding),
        (0x1f, 214) => Some(Music::Surfing),
        (0x1f, 217) => Some(Music::GameCorner),
        (0x1f, 220) => Some(Music::YellowIntro),
        (0x1f, 223) => Some(Music::Dungeon1),
        (0x1f, 227) => Some(Music::Dungeon2),
        (0x1f, 231) => Some(Music::Dungeon3),
        (0x1f, 235) => Some(Music::CinnabarMansion),
        (0x1f, 239) => Some(Music::PokemonTower),
        (0x1f, 242) => Some(Music::SilphCo),
        (0x1f, 245) => Some(Music::MeetEvilTrainer),
        (0x1f, 248) => Some(Music::MeetFemaleTrainer),
        (0x1f, 251) => Some(Music::MeetMaleTrainer),

        (0x20, 153) => Some(Music::SurfingPikachu),
        (0x20, 156) => Some(Music::MeetJessieJames),
        (0x20, 159) => Some(Music::YellowUnusedSong),
        (0x20, 163) => Some(Music::GBPrinter),

        _ => None,
    }
}

#[rustfmt::skip]
fn sfx_from_bank_and_id(bank: u8, id: u8) -> Option<(u8, u16)> {
    match (bank, id) {
        // Instruments
        (0x02, 1..=19) => Some((0x1f, 0x4000 + ((id as u16) * 3))),
        (0x08, 1..=19) => Some((0x1f, 0x4000 + ((id as u16) * 3))),
        (0x1f, 1..=19) => Some((0x1f, 0x4000 + ((id as u16) * 3))),
        (0x20, 1..=19) => Some((0x1f, 0x4000 + ((id as u16) * 3))),

        // Bank 02
        (0x02, 20) => Some((0x02, 0x403c)), // SFX_Cry00_1
        (0x02, 23) => Some((0x02, 0x4045)), // SFX_Cry01_1
        (0x02, 26) => Some((0x02, 0x404e)), // SFX_Cry02_1
        (0x02, 29) => Some((0x02, 0x4057)), // SFX_Cry03_1
        (0x02, 32) => Some((0x02, 0x4060)), // SFX_Cry04_1
        (0x02, 35) => Some((0x02, 0x4069)), // SFX_Cry05_1
        (0x02, 38) => Some((0x02, 0x4072)), // SFX_Cry06_1
        (0x02, 41) => Some((0x02, 0x407b)), // SFX_Cry07_1
        (0x02, 44) => Some((0x02, 0x4084)), // SFX_Cry08_1
        (0x02, 47) => Some((0x02, 0x408d)), // SFX_Cry09_1
        (0x02, 50) => Some((0x02, 0x4096)), // SFX_Cry0A_1
        (0x02, 53) => Some((0x02, 0x409f)), // SFX_Cry0B_1
        (0x02, 56) => Some((0x02, 0x40a8)), // SFX_Cry0C_1
        (0x02, 59) => Some((0x02, 0x40b1)), // SFX_Cry0D_1
        (0x02, 62) => Some((0x02, 0x40ba)), // SFX_Cry0E_1
        (0x02, 65) => Some((0x02, 0x40c3)), // SFX_Cry0F_1
        (0x02, 68) => Some((0x02, 0x40cc)), // SFX_Cry10_1
        (0x02, 71) => Some((0x02, 0x40d5)), // SFX_Cry11_1
        (0x02, 74) => Some((0x02, 0x40de)), // SFX_Cry12_1
        (0x02, 77) => Some((0x02, 0x40e7)), // SFX_Cry13_1
        (0x02, 80) => Some((0x02, 0x40f0)), // SFX_Cry14_1
        (0x02, 83) => Some((0x02, 0x40f9)), // SFX_Cry15_1
        (0x02, 86) => Some((0x02, 0x4102)), // SFX_Cry16_1
        (0x02, 89) => Some((0x02, 0x410b)), // SFX_Cry17_1
        (0x02, 92) => Some((0x02, 0x4114)), // SFX_Cry18_1
        (0x02, 95) => Some((0x02, 0x411d)), // SFX_Cry19_1
        (0x02, 98) => Some((0x02, 0x4126)), // SFX_Cry1A_1
        (0x02, 101) => Some((0x02, 0x412f)), // SFX_Cry1B_1
        (0x02, 104) => Some((0x02, 0x4138)), // SFX_Cry1C_1
        (0x02, 107) => Some((0x02, 0x4141)), // SFX_Cry1D_1
        (0x02, 110) => Some((0x02, 0x414a)), // SFX_Cry1E_1
        (0x02, 113) => Some((0x02, 0x4153)), // SFX_Cry1F_1
        (0x02, 116) => Some((0x02, 0x415c)), // SFX_Cry20_1
        (0x02, 119) => Some((0x02, 0x4165)), // SFX_Cry21_1
        (0x02, 122) => Some((0x02, 0x416e)), // SFX_Cry22_1
        (0x02, 125) => Some((0x02, 0x4177)), // SFX_Cry23_1
        (0x02, 128) => Some((0x02, 0x4180)), // SFX_Cry24_1
        (0x02, 131) => Some((0x02, 0x4189)), // SFX_Cry25_1
        (0x02, 134) => { eprintln!("Missing sound: SFX_Get_Item1_1"); None },
        (0x02, 137) => { eprintln!("Missing sound: SFX_Get_Item2_1"); None },
        (0x02, 140) => Some((0x02, 0x41a4)), // SFX_Tink_1
        (0x02, 141) => Some((0x02, 0x41a7)), // SFX_Heal_HP_1
        (0x02, 142) => Some((0x02, 0x41aa)), // SFX_Heal_Ailment_1
        (0x02, 143) => Some((0x02, 0x41ad)), // SFX_Start_Menu_1
        (0x02, 144) => Some((0x02, 0x41b0)), // SFX_Press_AB_1
        (0x02, 145) => { eprintln!("Missing sound: SFX_Pokedex_Rating_1"); None },
        (0x02, 148) => { eprintln!("Missing sound: SFX_Get_Key_Item_1"); None },
        (0x02, 151) => Some((0x02, 0x41c5)), // SFX_Poisoned_1
        (0x02, 152) => Some((0x02, 0x41c8)), // SFX_Trade_Machine_1
        (0x02, 153) => Some((0x02, 0x41cb)), // SFX_Turn_On_PC_1
        (0x02, 154) => Some((0x02, 0x41ce)), // SFX_Turn_Off_PC_1
        (0x02, 155) => Some((0x02, 0x41d1)), // SFX_Enter_PC_1
        (0x02, 156) => Some((0x02, 0x41d4)), // SFX_Shrink_1
        (0x02, 157) => Some((0x02, 0x41d7)), // SFX_Switch_1
        (0x02, 158) => Some((0x02, 0x41da)), // SFX_Healing_Machine_1
        (0x02, 159) => Some((0x02, 0x41dd)), // SFX_Teleport_Exit1_1
        (0x02, 160) => Some((0x02, 0x41e0)), // SFX_Teleport_Enter1_1
        (0x02, 161) => Some((0x02, 0x41e3)), // SFX_Teleport_Exit2_1
        (0x02, 162) => Some((0x02, 0x41e6)), // SFX_Ledge_1
        (0x02, 163) => Some((0x02, 0x41e9)), // SFX_Teleport_Enter2_1
        (0x02, 164) => Some((0x02, 0x41ec)), // SFX_Fly_1
        (0x02, 165) => Some((0x02, 0x41ef)), // SFX_Denied_1
        (0x02, 167) => Some((0x02, 0x41f5)), // SFX_Arrow_Tiles_1
        (0x02, 168) => Some((0x02, 0x41f8)), // SFX_Push_Boulder_1
        (0x02, 169) => Some((0x02, 0x41fb)), // SFX_SS_Anne_Horn_1
        (0x02, 171) => Some((0x02, 0x4201)), // SFX_Withdraw_Deposit_1
        (0x02, 172) => Some((0x02, 0x4204)), // SFX_Cut_1
        (0x02, 173) => Some((0x02, 0x4207)), // SFX_Go_Inside_1
        (0x02, 174) => Some((0x02, 0x420a)), // SFX_Swap_1
        (0x02, 176) => Some((0x02, 0x4210)), // SFX_59_1
        (0x02, 178) => Some((0x02, 0x4216)), // SFX_Purchase_1
        (0x02, 180) => Some((0x02, 0x421c)), // SFX_Collision_1
        (0x02, 181) => Some((0x02, 0x421f)), // SFX_Go_Outside_1
        (0x02, 182) => Some((0x02, 0x4222)), // SFX_Save_1
        (0x02, 184) => { eprintln!("Missing sound: SFX_Pokeflute"); None },
        (0x02, 185) => Some((0x02, 0x422b)), // SFX_Safari_Zone_PA

        // Bank 08
        (0x08, 20) => Some((0x08, 0x403c)), // SFX_Cry00_2
        (0x08, 23) => Some((0x08, 0x4045)), // SFX_Cry01_2
        (0x08, 26) => Some((0x08, 0x404e)), // SFX_Cry02_2
        (0x08, 29) => Some((0x08, 0x4057)), // SFX_Cry03_2
        (0x08, 32) => Some((0x08, 0x4060)), // SFX_Cry04_2
        (0x08, 35) => Some((0x08, 0x4069)), // SFX_Cry05_2
        (0x08, 38) => Some((0x08, 0x4072)), // SFX_Cry06_2
        (0x08, 41) => Some((0x08, 0x407b)), // SFX_Cry07_2
        (0x08, 44) => Some((0x08, 0x4084)), // SFX_Cry08_2
        (0x08, 47) => Some((0x08, 0x408d)), // SFX_Cry09_2
        (0x08, 50) => Some((0x08, 0x4096)), // SFX_Cry0A_2
        (0x08, 53) => Some((0x08, 0x409f)), // SFX_Cry0B_2
        (0x08, 56) => Some((0x08, 0x40a8)), // SFX_Cry0C_2
        (0x08, 59) => Some((0x08, 0x40b1)), // SFX_Cry0D_2
        (0x08, 62) => Some((0x08, 0x40ba)), // SFX_Cry0E_2
        (0x08, 65) => Some((0x08, 0x40c3)), // SFX_Cry0F_2
        (0x08, 68) => Some((0x08, 0x40cc)), // SFX_Cry10_2
        (0x08, 71) => Some((0x08, 0x40d5)), // SFX_Cry11_2
        (0x08, 74) => Some((0x08, 0x40de)), // SFX_Cry12_2
        (0x08, 77) => Some((0x08, 0x40e7)), // SFX_Cry13_2
        (0x08, 80) => Some((0x08, 0x40f0)), // SFX_Cry14_2
        (0x08, 83) => Some((0x08, 0x40f9)), // SFX_Cry15_2
        (0x08, 86) => Some((0x08, 0x4102)), // SFX_Cry16_2
        (0x08, 89) => Some((0x08, 0x410b)), // SFX_Cry17_2
        (0x08, 92) => Some((0x08, 0x4114)), // SFX_Cry18_2
        (0x08, 95) => Some((0x08, 0x411d)), // SFX_Cry19_2
        (0x08, 98) => Some((0x08, 0x4126)), // SFX_Cry1A_2
        (0x08, 101) => Some((0x08, 0x412f)), // SFX_Cry1B_2
        (0x08, 104) => Some((0x08, 0x4138)), // SFX_Cry1C_2
        (0x08, 107) => Some((0x08, 0x4141)), // SFX_Cry1D_2
        (0x08, 110) => Some((0x08, 0x414a)), // SFX_Cry1E_2
        (0x08, 113) => Some((0x08, 0x4153)), // SFX_Cry1F_2
        (0x08, 116) => Some((0x08, 0x415c)), // SFX_Cry20_2
        (0x08, 119) => Some((0x08, 0x4165)), // SFX_Cry21_2
        (0x08, 122) => Some((0x08, 0x416e)), // SFX_Cry22_2
        (0x08, 125) => Some((0x08, 0x4177)), // SFX_Cry23_2
        (0x08, 128) => Some((0x08, 0x4180)), // SFX_Cry24_2
        (0x08, 131) => Some((0x08, 0x4189)), // SFX_Cry25_2
        (0x08, 134) => { eprintln!("Missing sound: SFX_Level_Up"); None },
        (0x08, 137) => { eprintln!("Missing sound: SFX_Get_Item2_2"); None },
        (0x08, 140) => Some((0x08, 0x41a4)), // SFX_Tink_2
        (0x08, 141) => Some((0x08, 0x41a7)), // SFX_Heal_HP_2
        (0x08, 142) => Some((0x08, 0x41aa)), // SFX_Heal_Ailment_2
        (0x08, 143) => Some((0x08, 0x41ad)), // SFX_Start_Menu_2
        (0x08, 144) => Some((0x08, 0x41b0)), // SFX_Press_AB_2
        (0x08, 145) => Some((0x08, 0x41b3)), // SFX_Ball_Toss
        (0x08, 147) => Some((0x08, 0x41b9)), // SFX_Ball_Poof
        (0x08, 149) => Some((0x08, 0x41bf)), // SFX_Faint_Thud
        (0x08, 151) => Some((0x08, 0x41c5)), // SFX_Run
        (0x08, 152) => Some((0x08, 0x41c8)), // SFX_Dex_Page_Added
        (0x08, 154) => { eprintln!("Missing sound: SFX_Caught_Mon"); None },
        (0x08, 157) => Some((0x08, 0x41d7)), // SFX_Peck
        (0x08, 158) => Some((0x08, 0x41da)), // SFX_Faint_Fall
        (0x08, 159) => Some((0x08, 0x41dd)), // SFX_Battle_09
        (0x08, 160) => Some((0x08, 0x41e0)), // SFX_Pound
        (0x08, 161) => Some((0x08, 0x41e3)), // SFX_Battle_0B
        (0x08, 162) => Some((0x08, 0x41e6)), // SFX_Battle_0C
        (0x08, 163) => Some((0x08, 0x41e9)), // SFX_Battle_0D
        (0x08, 164) => Some((0x08, 0x41ec)), // SFX_Battle_0E
        (0x08, 165) => Some((0x08, 0x41ef)), // SFX_Battle_0F
        (0x08, 166) => Some((0x08, 0x41f2)), // SFX_Damage
        (0x08, 167) => Some((0x08, 0x41f5)), // SFX_Not_Very_Effective
        (0x08, 168) => Some((0x08, 0x41f8)), // SFX_Battle_12
        (0x08, 169) => Some((0x08, 0x41fb)), // SFX_Battle_13
        (0x08, 170) => Some((0x08, 0x41fe)), // SFX_Battle_14
        (0x08, 171) => Some((0x08, 0x4201)), // SFX_Vine_Whip
        (0x08, 172) => Some((0x08, 0x4204)), // SFX_Battle_16
        (0x08, 173) => Some((0x08, 0x4207)), // SFX_Battle_17
        (0x08, 174) => Some((0x08, 0x420a)), // SFX_Battle_18
        (0x08, 175) => Some((0x08, 0x420d)), // SFX_Battle_19
        (0x08, 176) => Some((0x08, 0x4210)), // SFX_Super_Effective
        (0x08, 177) => Some((0x08, 0x4213)), // SFX_Battle_1B
        (0x08, 178) => Some((0x08, 0x4216)), // SFX_Battle_1C
        (0x08, 179) => Some((0x08, 0x4219)), // SFX_Doubleslap
        (0x08, 180) => Some((0x08, 0x421c)), // SFX_Battle_1E
        (0x08, 182) => Some((0x08, 0x4222)), // SFX_Horn_Drill
        (0x08, 183) => Some((0x08, 0x4225)), // SFX_Battle_20
        (0x08, 184) => Some((0x08, 0x4228)), // SFX_Battle_21
        (0x08, 185) => Some((0x08, 0x422b)), // SFX_Battle_22
        (0x08, 186) => Some((0x08, 0x422e)), // SFX_Battle_23
        (0x08, 187) => Some((0x08, 0x4231)), // SFX_Battle_24
        (0x08, 189) => Some((0x08, 0x4237)), // SFX_Battle_25
        (0x08, 190) => Some((0x08, 0x423a)), // SFX_Battle_26
        (0x08, 191) => Some((0x08, 0x423d)), // SFX_Battle_27
        (0x08, 194) => Some((0x08, 0x4246)), // SFX_Battle_28
        (0x08, 197) => Some((0x08, 0x424f)), // SFX_Battle_29
        (0x08, 199) => Some((0x08, 0x4255)), // SFX_Battle_2A
        (0x08, 202) => Some((0x08, 0x425e)), // SFX_Battle_2B
        (0x08, 204) => Some((0x08, 0x4264)), // SFX_Battle_2C
        (0x08, 207) => Some((0x08, 0x426d)), // SFX_Psybeam
        (0x08, 210) => Some((0x08, 0x4276)), // SFX_Battle_2E
        (0x08, 213) => Some((0x08, 0x427f)), // SFX_Battle_2F
        (0x08, 216) => Some((0x08, 0x4288)), // SFX_Psychic_M
        (0x08, 219) => Some((0x08, 0x4291)), // SFX_Battle_31
        (0x08, 221) => Some((0x08, 0x4297)), // SFX_Battle_32
        (0x08, 223) => Some((0x08, 0x429d)), // SFX_Battle_33
        (0x08, 225) => Some((0x08, 0x42a3)), // SFX_Battle_34
        (0x08, 228) => { eprintln!("Missing sound: SFX_Battle_35"); None },
        (0x08, 230) => Some((0x08, 0x42b2)), // SFX_Battle_36
        (0x08, 233) => Some((0x08, 0x42bb)), // SFX_Silph_Scope

        // Bank 1f
        (0x1f, 20) => Some((0x1f, 0x403c)), // SFX_Cry00_3
        (0x1f, 23) => Some((0x1f, 0x4045)), // SFX_Cry01_3
        (0x1f, 26) => Some((0x1f, 0x404e)), // SFX_Cry02_3
        (0x1f, 29) => Some((0x1f, 0x4057)), // SFX_Cry03_3
        (0x1f, 32) => Some((0x1f, 0x4060)), // SFX_Cry04_3
        (0x1f, 35) => Some((0x1f, 0x4069)), // SFX_Cry05_3
        (0x1f, 38) => Some((0x1f, 0x4072)), // SFX_Cry06_3
        (0x1f, 41) => Some((0x1f, 0x407b)), // SFX_Cry07_3
        (0x1f, 44) => Some((0x1f, 0x4084)), // SFX_Cry08_3
        (0x1f, 47) => Some((0x1f, 0x408d)), // SFX_Cry09_3
        (0x1f, 50) => Some((0x1f, 0x4096)), // SFX_Cry0A_3
        (0x1f, 53) => Some((0x1f, 0x409f)), // SFX_Cry0B_3
        (0x1f, 56) => Some((0x1f, 0x40a8)), // SFX_Cry0C_3
        (0x1f, 59) => Some((0x1f, 0x40b1)), // SFX_Cry0D_3
        (0x1f, 62) => Some((0x1f, 0x40ba)), // SFX_Cry0E_3
        (0x1f, 65) => Some((0x1f, 0x40c3)), // SFX_Cry0F_3
        (0x1f, 68) => Some((0x1f, 0x40cc)), // SFX_Cry10_3
        (0x1f, 71) => Some((0x1f, 0x40d5)), // SFX_Cry11_3
        (0x1f, 74) => Some((0x1f, 0x40de)), // SFX_Cry12_3
        (0x1f, 77) => Some((0x1f, 0x40e7)), // SFX_Cry13_3
        (0x1f, 80) => Some((0x1f, 0x40f0)), // SFX_Cry14_3
        (0x1f, 83) => Some((0x1f, 0x40f9)), // SFX_Cry15_3
        (0x1f, 86) => Some((0x1f, 0x4102)), // SFX_Cry16_3
        (0x1f, 89) => Some((0x1f, 0x410b)), // SFX_Cry17_3
        (0x1f, 92) => Some((0x1f, 0x4114)), // SFX_Cry18_3
        (0x1f, 95) => Some((0x1f, 0x411d)), // SFX_Cry19_3
        (0x1f, 98) => Some((0x1f, 0x4126)), // SFX_Cry1A_3
        (0x1f, 101) => Some((0x1f, 0x412f)), // SFX_Cry1B_3
        (0x1f, 104) => Some((0x1f, 0x4138)), // SFX_Cry1C_3
        (0x1f, 107) => Some((0x1f, 0x4141)), // SFX_Cry1D_3
        (0x1f, 110) => Some((0x1f, 0x414a)), // SFX_Cry1E_3
        (0x1f, 113) => Some((0x1f, 0x4153)), // SFX_Cry1F_3
        (0x1f, 116) => Some((0x1f, 0x415c)), // SFX_Cry20_3
        (0x1f, 119) => Some((0x1f, 0x4165)), // SFX_Cry21_3
        (0x1f, 122) => Some((0x1f, 0x416e)), // SFX_Cry22_3
        (0x1f, 125) => Some((0x1f, 0x4177)), // SFX_Cry23_3
        (0x1f, 128) => Some((0x1f, 0x4180)), // SFX_Cry24_3
        (0x1f, 131) => Some((0x1f, 0x4189)), // SFX_Cry25_3
        (0x1f, 134) => { eprintln!("Missing sound: SFX_Get_Item1_3"); None },
        (0x1f, 137) => { eprintln!("Missing sound: SFX_Get_Item2_3"); None },
        (0x1f, 140) => Some((0x1f, 0x41a4)), // SFX_Tink_3
        (0x1f, 141) => Some((0x1f, 0x41a7)), // SFX_Heal_HP_3
        (0x1f, 142) => Some((0x1f, 0x41aa)), // SFX_Heal_Ailment_3
        (0x1f, 143) => Some((0x1f, 0x41ad)), // SFX_Start_Menu_3
        (0x1f, 144) => Some((0x1f, 0x41b0)), // SFX_Press_AB_3
        (0x1f, 145) => { eprintln!("Missing sound: SFX_Pokedex_Rating_3"); None },
        (0x1f, 148) => { eprintln!("Missing sound: SFX_Get_Key_Item_3"); None },
        (0x1f, 151) => Some((0x1f, 0x41c5)), // SFX_Poisoned_3
        (0x1f, 152) => Some((0x1f, 0x41c8)), // SFX_Trade_Machine_3
        (0x1f, 153) => Some((0x1f, 0x41cb)), // SFX_Turn_On_PC_3
        (0x1f, 154) => Some((0x1f, 0x41ce)), // SFX_Turn_Off_PC_3
        (0x1f, 155) => Some((0x1f, 0x41d1)), // SFX_Enter_PC_3
        (0x1f, 156) => Some((0x1f, 0x41d4)), // SFX_Shrink_3
        (0x1f, 157) => Some((0x1f, 0x41d7)), // SFX_Switch_3
        (0x1f, 158) => Some((0x1f, 0x41da)), // SFX_Healing_Machine_3
        (0x1f, 159) => Some((0x1f, 0x41dd)), // SFX_Teleport_Exit1_3
        (0x1f, 160) => Some((0x1f, 0x41e0)), // SFX_Teleport_Enter1_3
        (0x1f, 161) => Some((0x1f, 0x41e3)), // SFX_Teleport_Exit2_3
        (0x1f, 162) => Some((0x1f, 0x41e6)), // SFX_Ledge_3
        (0x1f, 163) => Some((0x1f, 0x41e9)), // SFX_Teleport_Enter2_3
        (0x1f, 164) => Some((0x1f, 0x41ec)), // SFX_Fly_3
        (0x1f, 165) => Some((0x1f, 0x41ef)), // SFX_Denied_3
        (0x1f, 167) => Some((0x1f, 0x41f5)), // SFX_Arrow_Tiles_3
        (0x1f, 168) => Some((0x1f, 0x41f8)), // SFX_Push_Boulder_3
        (0x1f, 169) => Some((0x1f, 0x41fb)), // SFX_SS_Anne_Horn_3
        (0x1f, 171) => Some((0x1f, 0x4201)), // SFX_Withdraw_Deposit_3
        (0x1f, 172) => Some((0x1f, 0x4204)), // SFX_Cut_3
        (0x1f, 173) => Some((0x1f, 0x4207)), // SFX_Go_Inside_3
        (0x1f, 174) => Some((0x1f, 0x420a)), // SFX_Swap_3
        (0x1f, 176) => Some((0x1f, 0x4210)), // SFX_59_3
        (0x1f, 178) => Some((0x1f, 0x4216)), // SFX_Purchase_3
        (0x1f, 180) => Some((0x1f, 0x421c)), // SFX_Collision_3
        (0x1f, 181) => Some((0x1f, 0x421f)), // SFX_Go_Outside_3
        (0x1f, 182) => Some((0x1f, 0x4222)), // SFX_Save_3
        (0x1f, 184) => Some((0x1f, 0x4228)), // SFX_Intro_Lunge
        (0x1f, 185) => Some((0x1f, 0x422b)), // SFX_Intro_Hip
        (0x1f, 186) => Some((0x1f, 0x422e)), // SFX_Intro_Hop
        (0x1f, 187) => Some((0x1f, 0x4231)), // SFX_Intro_Raise
        (0x1f, 188) => Some((0x1f, 0x4234)), // SFX_Intro_Crash
        (0x1f, 189) => Some((0x1f, 0x4237)), // SFX_Intro_Whoosh
        (0x1f, 190) => Some((0x1f, 0x423a)), // SFX_Slots_Stop_Wheel
        (0x1f, 191) => Some((0x1f, 0x423d)), // SFX_Slots_Reward
        (0x1f, 192) => Some((0x1f, 0x4240)), // SFX_Slots_New_Spin
        (0x1f, 194) => Some((0x1f, 0x4246)), // SFX_Shooting_Star

        // Bank 20
        (0x20, 20) => Some((0x20, 0x403c)), // SFX_Cry00_4
        (0x20, 23) => Some((0x20, 0x4045)), // SFX_Cry01_4
        (0x20, 26) => Some((0x20, 0x404e)), // SFX_Cry02_4
        (0x20, 29) => Some((0x20, 0x4057)), // SFX_Cry03_4
        (0x20, 32) => Some((0x20, 0x4060)), // SFX_Cry04_4
        (0x20, 35) => Some((0x20, 0x4069)), // SFX_Cry05_4
        (0x20, 38) => Some((0x20, 0x4072)), // SFX_Cry06_4
        (0x20, 41) => Some((0x20, 0x407b)), // SFX_Cry07_4
        (0x20, 44) => Some((0x20, 0x4084)), // SFX_Cry08_4
        (0x20, 47) => Some((0x20, 0x408d)), // SFX_Cry09_4
        (0x20, 50) => Some((0x20, 0x4096)), // SFX_Cry0A_4
        (0x20, 53) => Some((0x20, 0x409f)), // SFX_Cry0B_4
        (0x20, 56) => Some((0x20, 0x40a8)), // SFX_Cry0C_4
        (0x20, 59) => Some((0x20, 0x40b1)), // SFX_Cry0D_4
        (0x20, 62) => Some((0x20, 0x40ba)), // SFX_Cry0E_4
        (0x20, 65) => Some((0x20, 0x40c3)), // SFX_Cry0F_4
        (0x20, 68) => Some((0x20, 0x40cc)), // SFX_Cry10_4
        (0x20, 71) => Some((0x20, 0x40d5)), // SFX_Cry11_4
        (0x20, 74) => Some((0x20, 0x40de)), // SFX_Cry12_4
        (0x20, 77) => Some((0x20, 0x40e7)), // SFX_Cry13_4
        (0x20, 80) => Some((0x20, 0x40f0)), // SFX_Cry14_4
        (0x20, 83) => Some((0x20, 0x40f9)), // SFX_Cry15_4
        (0x20, 86) => Some((0x20, 0x4102)), // SFX_Cry16_4
        (0x20, 89) => Some((0x20, 0x410b)), // SFX_Cry17_4
        (0x20, 92) => Some((0x20, 0x4114)), // SFX_Cry18_4
        (0x20, 95) => Some((0x20, 0x411d)), // SFX_Cry19_4
        (0x20, 98) => Some((0x20, 0x4126)), // SFX_Cry1A_4
        (0x20, 101) => Some((0x20, 0x412f)), // SFX_Cry1B_4
        (0x20, 104) => Some((0x20, 0x4138)), // SFX_Cry1C_4
        (0x20, 107) => Some((0x20, 0x4141)), // SFX_Cry1D_4
        (0x20, 110) => Some((0x20, 0x414a)), // SFX_Cry1E_4
        (0x20, 113) => Some((0x20, 0x4153)), // SFX_Cry1F_4
        (0x20, 116) => Some((0x20, 0x415c)), // SFX_Cry20_4
        (0x20, 119) => Some((0x20, 0x4165)), // SFX_Cry21_4
        (0x20, 122) => Some((0x20, 0x416e)), // SFX_Cry22_4
        (0x20, 125) => Some((0x20, 0x4177)), // SFX_Cry23_4
        (0x20, 128) => Some((0x20, 0x4180)), // SFX_Cry24_4
        (0x20, 131) => Some((0x20, 0x4189)), // SFX_Cry25_4
        (0x20, 134) => { eprintln!("Missing sound: SFX_Get_Item1_4"); None },
        (0x20, 137) => { eprintln!("Missing sound: SFX_Get_Item2_4"); None },
        (0x20, 140) => Some((0x20, 0x41a4)), // SFX_Tink_4
        (0x20, 141) => Some((0x20, 0x41a7)), // SFX_Heal_HP_4
        (0x20, 142) => Some((0x20, 0x41aa)), // SFX_Heal_Ailment_4
        (0x20, 143) => Some((0x20, 0x41ad)), // SFX_Start_Menu_4
        (0x20, 144) => Some((0x20, 0x41b0)), // SFX_Press_AB_4
        (0x20, 145) => Some((0x20, 0x41b3)), // SFX_Surfing_Jump
        (0x20, 146) => Some((0x20, 0x41b6)), // SFX_Surfing_Flip
        (0x20, 147) => Some((0x20, 0x41b9)), // SFX_Surfing_Crash
        (0x20, 148) => Some((0x20, 0x41bc)), // SFX_Unknown_802cc
        (0x20, 149) => Some((0x20, 0x41bf)), // SFX_Surfing_Land
        (0x20, 150) => { eprintln!("Missing sound: SFX_Get_Item2_4_2"); None },

        _ => None
    }
}

fn sfx_is_cry((_, addr): (u8, u16)) -> bool {
    const CRY_SFX_START: u16 = 0x403c;
    const CRY_SFX_END: u16 = 0x418c;

    (CRY_SFX_START..CRY_SFX_END).contains(&addr)
}

pub fn play_sound(cpu: &mut Cpu) {
    // Note: Not sure why we are reading W_AUDIO_SAVED_ROM_BANK instead of
    // W_AUDIO_ROM_BANK, but if we don't we sometimes play the wrong audio...
    let bank = cpu.read_byte(wram::W_AUDIO_SAVED_ROM_BANK);

    if cpu.a == 0xff {
        // Stop all sounds?
    } else if let Some(music) = music_from_bank_and_id(bank, cpu.a) {
        cpu.start_music(music);
    } else if let Some(sfx) = sfx_from_bank_and_id(bank, cpu.a) {
        let mut pitch = 0;
        let mut length = 0;

        if sfx_is_cry(sfx) {
            pitch = cpu.read_byte(wram::W_FREQUENCY_MODIFIER);
            length = cpu.read_byte(wram::W_TEMPO_MODIFIER) as i8;
        }

        cpu.play_sfx(sfx.0, sfx.1, pitch, length);
    } else {
        eprintln!(
            "Don't know what to play: {:02x}:{:04x} (id = {})",
            bank,
            0x4000 + (cpu.a as u16) * 3,
            cpu.a,
        );

        // eprintln!("W_AUDIO_ROM_BANK: {:02x}", cpu.read_byte(wram::W_AUDIO_ROM_BANK));
        // eprintln!("W_AUDIO_SAVED_ROM_BANK: {:02x}", cpu.read_byte(wram::W_AUDIO_SAVED_ROM_BANK));
        // eprintln!("W_NEW_SOUND_ID: {:02x}", cpu.read_byte(wram::W_NEW_SOUND_ID));
        // eprintln!("W_LAST_MUSIC_SOUND_ID: {:02x}", cpu.read_byte(wram::W_LAST_MUSIC_SOUND_ID));
        // eprintln!("CPU: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}", cpu.a, cpu.b, cpu.c, cpu.d, cpu.e, cpu.f, cpu.h, cpu.l);
    }

    // Run GameBoy code as well so that everything works like normally
    cpu.stack_push(cpu.hl());
    cpu.pc = 0x2239;
    cpu.cycle(16);
}
