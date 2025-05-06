use crate::{
    cpu::Cpu,
    game::{
        constants::{
            gfx_constants::{CONVERT_BGP, CONVERT_OBP0, CONVERT_OBP1},
            hardware_constants::{self, R_LCDC, R_LCDC_ENABLE},
            map_constants::*,
            palette_constants::*,
            pokemon_constants,
            ram_constants::NUM_BADGES,
            tileset_constants::{CAVERN, CEMETERY},
        },
        data::{
            pokemon::palettes::monster_palette,
            sgb::sgb_packets::{PAL_PACKET_EMPTY, PAL_PACKET_POKEDEX},
        },
        macros,
        ram::{hram, wram},
    },
    PokemonSpecies,
};

const CGB_BASE_PAL_POINTERS: u16 = 0xdee1;

pub fn run_palette_command(cpu: &mut Cpu) {
    cpu.call(0x3ed7); // GetPredefRegisters

    let mut pal_fn = cpu.b;

    log::debug!("run_palette_command({:02x})", pal_fn);

    if pal_fn == SET_PAL_DEFAULT {
        pal_fn = cpu.read_byte(wram::W_DEFAULT_PALETTE_COMMAND);
    }

    if pal_fn == SET_PAL_PARTY_MENU_HP_BARS {
        cpu.call(0x618b); // UpdatePartyMenuBlkPacket
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    match pal_fn {
        SET_PAL_BATTLE_BLACK => set_pal_battle_black(cpu),
        SET_PAL_BATTLE => set_pal_battle(cpu),
        SET_PAL_TOWN_MAP => set_pal_town_map(cpu),
        SET_PAL_STATUS_SCREEN => set_pal_status_screen(cpu),
        SET_PAL_POKEDEX => set_pal_pokedex(cpu),
        SET_PAL_SLOTS => set_pal_slots(cpu),
        SET_PAL_TITLE_SCREEN => set_pal_title_screen(cpu),
        SET_PAL_NIDORINO_INTRO => set_pal_nidorino_intro(cpu),
        SET_PAL_GENERIC => set_pal_generic(cpu),
        SET_PAL_OVERWORLD => set_pal_overworld(cpu),
        SET_PAL_PARTY_MENU => set_pal_party_menu(cpu),
        SET_PAL_POKEMON_WHOLE_SCREEN => set_pal_pokemon_whole_screen(cpu),
        SET_PAL_GAME_FREAK_INTRO => set_pal_game_freak_intro(cpu),
        SET_PAL_TRAINER_CARD => set_pal_trainer_card(cpu),
        SET_PAL_SURFING_PIKACHU_TITLE => set_pal_pikachus_beach(cpu),
        SET_PAL_SURFING_PIKACHU_MINIGAME => set_pal_pikachus_beach_title(cpu),
        i => panic!("Invalid SetPalFunctions index: {i}"),
    }
}

fn set_pal_battle_black(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x6781, 0x6621); // PalPacket_Black, BlkPacket_Battle
}

// uses PalPacket_Empty to build a packet based on mon IDs and health color
fn set_pal_battle(cpu: &mut Cpu) {
    log::debug!("set_pal_battle()");

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let player_species_index = if cpu.read_byte(wram::W_BATTLE_MON_SPECIES) == 0 {
        0
    } else {
        let idx = cpu.read_byte(wram::W_PLAYER_MON_NUMBER) as u16;
        cpu.read_byte(wram::W_PARTY_MON1 + (wram::W_PARTY_MON2 - wram::W_PARTY_MON1) * idx)
    };

    let player_palette_id = determine_palette_id(player_species_index);

    let enemy_species_index = cpu.read_byte(wram::W_ENEMY_MON_SPECIES2);
    let enemy_palette_id = determine_palette_id(enemy_species_index);

    let player_hp_palette_id = match cpu.read_byte(wram::W_PLAYER_HP_BAR_COLOR) {
        0 => PAL_GREENBAR,
        1 => PAL_YELLOWBAR,
        2 => PAL_REDBAR,
        n => panic!("Invalid player HP bar color {n}"),
    };

    let enemy_hp_palette_id = match cpu.read_byte(wram::W_ENEMY_HP_BAR_COLOR) {
        0 => PAL_GREENBAR,
        1 => PAL_YELLOWBAR,
        2 => PAL_REDBAR,
        n => panic!("Invalid enemy HP bar color {n}"),
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, player_hp_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 3, enemy_hp_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 5, player_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 7, enemy_palette_id);

    cpu.write_byte(wram::W_DEFAULT_PALETTE_COMMAND, SET_PAL_BATTLE);

    send_sgb_packets(cpu, wram::W_PAL_PACKET, 0x6621); // _, BlkPacket_Battle
}

fn set_pal_town_map(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x6791, 0x6611); // PalPacket_TownMap, BlkPacket_WholeScreen
}

// uses PalPacket_Empty to build a packet based the mon ID
fn set_pal_status_screen(cpu: &mut Cpu) {
    log::debug!("set_pal_status_screen()");

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let species = cpu.read_byte(wram::W_CUR_PARTY_SPECIES);

    let mon_pal = if species > pokemon_constants::NUM_POKEMON_INDEXES {
        // not pokemon
        PAL_GREENMON
    } else {
        determine_palette_id(species)
    };

    let hp_pal = match cpu.read_byte(wram::W_STATUS_SCREEN_HP_BAR_COLOR) {
        0 => PAL_GREENBAR,
        1 => PAL_YELLOWBAR,
        2 => PAL_REDBAR,
        n => panic!("Invalid HP bar color: {n}"),
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, hp_pal);
    cpu.write_byte(wram::W_PAL_PACKET + 3, mon_pal);

    send_sgb_packets(cpu, wram::W_PAL_PACKET, 0x6641); // _, BlkPacket_StatusScreen
}

fn set_pal_party_menu(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x6771, wram::W_PARTY_MENU_BLK_PACKET); // PalPacket_PartyMenu, _
}

fn set_pal_pokedex(cpu: &mut Cpu) {
    log::debug!("set_pal_pokedex()");

    for (i, byte) in PAL_PACKET_POKEDEX.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let species = cpu.read_byte(wram::W_CUR_PARTY_SPECIES);
    let mon_pal = determine_palette_id(species);

    cpu.write_byte(wram::W_PAL_PACKET + 3, mon_pal);

    send_sgb_packets(cpu, wram::W_PAL_PACKET, 0x6651); // _, BlkPacket_Pokedex
}

fn set_pal_slots(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x67b1, 0x6661); // PalPacket_Slots, BlkPacket_Slots
}

fn set_pal_title_screen(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x67c1, 0x6681); // PalPacket_Titlescreen, BlkPacket_Titlescreen
}

// used mostly for menus and the Oak intro
fn set_pal_generic(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x67e1, 0x6611); // PalPacket_Generic, BlkPacket_WholeScreen
}

fn set_pal_nidorino_intro(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x67f1, 0x66a1); // PalPacket_NidorinoIntro, BlkPacket_NidorinoIntro
}

fn set_pal_game_freak_intro(cpu: &mut Cpu) {
    cpu.write_byte(wram::W_DEFAULT_PALETTE_COMMAND, SET_PAL_GENERIC);
    send_sgb_packets(cpu, 0x6801, 0x6731); // PalPacket_GameFreakIntro, BlkPacket_GameFreakIntro
}

// uses PalPacket_Empty to build a packet based on the current map
fn set_pal_overworld(cpu: &mut Cpu) {
    log::debug!("set_pal_overworld()");

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let wram = cpu.borrow_wram();
    let cur_tileset = wram.cur_map_tileset();
    let cur_map = wram.cur_map();
    let last_map = wram.last_map();

    let pal = match cur_map {
        // some tilesets always use specific palette
        _ if cur_tileset == CEMETERY => PAL_GRAYMON,
        _ if cur_tileset == CAVERN => PAL_CAVE,

        // specific town or route
        PALLET_TOWN..=SAFFRON_CITY => cur_map + 1,
        ROUTE_1..=ROUTE_25 => PAL_ROUTE,
        CERULEAN_CAVE_2F..=CERULEAN_CAVE_1F => PAL_CAVE,
        LORELEIS_ROOM => PAL_PALLET,
        BRUNOS_ROOM => PAL_CAVE,
        TRADE_CENTER => PAL_GRAYMON,
        COLOSSEUM => PAL_GRAYMON,

        // town that current dungeon or building is located
        _ if (PALLET_TOWN..=SAFFRON_CITY).contains(&last_map) => last_map + 1,

        // fallback
        _ => PAL_ROUTE,
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, pal);

    cpu.write_byte(wram::W_DEFAULT_PALETTE_COMMAND, SET_PAL_OVERWORLD);

    send_sgb_packets(cpu, wram::W_PAL_PACKET, 0x6611); // _, BlkPacket_WholeScreen
}

// used when a Pokemon is the only thing on the screen
// such as evolution, trading and the Hall of Fame
fn set_pal_pokemon_whole_screen(cpu: &mut Cpu) {
    log::debug!("set_pal_pokemon_whole_screen({:02x})", cpu.c);

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let pal = if cpu.c != 0 {
        PAL_BLACK
    } else {
        let species = cpu.read_byte(wram::W_WHOLE_SCREEN_PALETTE_MON_SPECIES);
        determine_palette_id(species)
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, pal);

    send_sgb_packets(cpu, wram::W_PAL_PACKET, 0x6611); // _, BlkPacket_WholeScreen
}

fn set_pal_trainer_card(cpu: &mut Cpu) {
    const BLK_PACKET_TRAINER_CARD: u16 = 0x66f1;

    for i in 0..0x40 {
        let value = cpu.read_byte(BLK_PACKET_TRAINER_CARD + i);
        cpu.write_byte(wram::W_TRAINER_CARD_BLK_PACKET + i, value);
    }

    let mut addr = wram::W_TRAINER_CARD_BLK_PACKET + 2;
    let mut badges = cpu.read_byte(wram::W_OBTAINED_BADGES);

    for byte_count in BADGE_BLK_DATA_LENGTHS {
        let have_badge = badges & 1 != 0;
        badges >>= 1;

        if have_badge {
            // The player does have the badge, so skip past the badge's blk data.
            addr += byte_count as u16;
        } else {
            // The player doens't have the badge, so zero the badge's blk data.
            for _ in 0..byte_count {
                cpu.write_byte(addr, 0);
                addr += 1;
            }
        }
    }

    send_sgb_packets(cpu, 0x67d1, wram::W_TRAINER_CARD_BLK_PACKET); // PalPacket_TrainerCard, _
}

pub fn set_pal_pikachus_beach(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x6811, 0x6611); // PalPacket_PikachusBeach,  BlkPacket_WholeScreen
}

pub fn set_pal_pikachus_beach_title(cpu: &mut Cpu) {
    send_sgb_packets(cpu, 0x6821, 0x6751); // PalPacket_PikachusBeachTitle,  BlkPacket_PikachusBeachTitle
}

/// The length of the blk data of each badge on the Trainer Card.
///
/// The Rainbow Badge has 3 entries because of its many colors.
const BADGE_BLK_DATA_LENGTHS: [u8; NUM_BADGES] = [
    6,     // Boulder Badge
    6,     // Cascade Badge
    6,     // Thunder Badge
    6 * 3, // Rainbow Badge
    6,     // Soul Badge
    6,     // Marsh Badge
    6,     // Volcano Badge
    6,     // Earth Badge
];

pub fn determine_palette_id(species_index: u8) -> u8 {
    monster_palette(PokemonSpecies::from_index(species_index))
}

pub fn load_sgb(cpu: &mut Cpu) {
    // This function should only be called once
    assert_eq!(cpu.read_byte(wram::W_ON_SGB), 0x00);
    cpu.write_byte(wram::W_ON_SGB, 0x01);

    // ret
    cpu.pc = cpu.stack_pop();
}

pub fn send_sgb_packets(cpu: &mut Cpu, first_packet: u16, second_packet: u16) {
    log::trace!("send_sgb_packets()");

    if cpu.read_byte(hram::H_ON_CGB) == 0 {
        panic!("send_sgb_packets called on non-SGB device");
    }

    cpu.set_hl(first_packet);
    cpu.call(0x6346); // InitCGBPalettes

    cpu.set_hl(second_packet);
    cpu.call(0x6346); // InitCGBPalettes

    cpu.a = cpu.read_byte(hardware_constants::R_LCDC);
    cpu.a &= 1 << hardware_constants::R_LCDC_ENABLE;

    if cpu.a != 0 {
        cpu.call(0x3ddb); // Delay3
    }

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn init_cgb_palettes(cpu: &mut Cpu) {
    log::debug!("init_cgb_palettes()");

    let packet = cpu.hl();

    cpu.a = cpu.read_byte(packet) & 0xf8;

    if cpu.a == 0x20 {
        translate_pal_packet_to_bg_map_attributes(cpu);
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    for index in 0..NUM_ACTIVE_PALS {
        cpu.a = cpu.read_byte(packet + 1 + (index as u16 * 2));
        cpu.call(0x63fe); // GetCGBBasePalAddress
        let base = cpu.de();

        cpu.write_byte(CGB_BASE_PAL_POINTERS + (index as u16 * 2), base as u8);
        cpu.write_byte(
            CGB_BASE_PAL_POINTERS + (index as u16 * 2) + 1,
            (base >> 8) as u8,
        );

        cpu.a = CONVERT_BGP;
        cpu.set_de(base);
        cpu.call(0x640f); // DMGPalToCGBPal

        cpu.a = index;
        cpu.call(0x6470); // TransferCurBGPData

        cpu.a = CONVERT_OBP0;
        cpu.set_de(base);
        cpu.call(0x640f); // DMGPalToCGBPal

        cpu.a = index;
        cpu.call(0x64df); // TransferCurOBPData

        cpu.a = CONVERT_OBP1;
        cpu.set_de(base);
        cpu.call(0x640f); // DMGPalToCGBPal

        cpu.a = index + 4;
        cpu.call(0x64df); // TransferCurOBPData
    }

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn transfer_cur_bgp_data(cpu: &mut Cpu) {
    let pal_index = cpu.a as usize;

    log::trace!("transfer_cur_bgp_data({:02x})", pal_index);

    if (cpu.read_byte(R_LCDC) & (1 << R_LCDC_ENABLE)) != 0 {
        // mask for non-V-blank/non-H-blank STAT mode
        let mask = 0b10;

        // In case we're already in H-blank or V-blank, wait for it to end. This is a
        // precaution so that the transfer doesn't extend past the blanking period.
        while cpu.read_byte(hardware_constants::R_STAT) & mask == 0 {
            cpu.cycle(4);
        }

        while cpu.read_byte(hardware_constants::R_STAT) & mask != 0 {
            cpu.cycle(4);
        }
    }

    for i in 0..NUM_PAL_COLORS {
        let hi = cpu.read_byte(wram::W_CGB_PAL + (i as u16 * 2));
        let lo = cpu.read_byte(wram::W_CGB_PAL + (i as u16 * 2) + 1);

        let r = hi & 0x1F;
        let g = (hi >> 5) | ((lo & 0x3) << 3);
        let b = (lo >> 2) & 0x1F;

        cpu.gpu_set_bg_palette_color(pal_index, i as usize, [r, g, b]);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

/// Transfer a palette color while the LCD is enabled.
pub fn transfer_pal_color_lcd_enabled(cpu: &mut Cpu) {
    // In case we're already in H-blank or V-blank, wait for it to end. This is a
    // precaution so that the transfer doesn't extend past the blanking period.
    while (cpu.read_byte(hardware_constants::R_STAT) & cpu.b) == 0 {
        cpu.cycle(4);
    }

    // Wait for H-blank or V-blank to begin.
    while (cpu.read_byte(hardware_constants::R_STAT) & cpu.b) != 0 {
        cpu.cycle(4);
    }

    // fall through
    transfer_pal_color_lcd_disabled(cpu)
}

/// Transfer a palette color while the LCD is disabled.
pub fn transfer_pal_color_lcd_disabled(cpu: &mut Cpu) {
    let byte = cpu.read_byte(cpu.hl());
    cpu.write_byte(cpu.de(), byte);

    let byte = cpu.read_byte(cpu.hl() + 1);
    cpu.write_byte(cpu.de(), byte);

    cpu.set_hl(cpu.hl() + 2);

    cpu.pc = cpu.stack_pop(); // ret
}

/// translate the SGB pal packets into something usable for the CGB
pub fn translate_pal_packet_to_bg_map_attributes(cpu: &mut Cpu) {
    log::debug!(
        "translate_pal_packet_to_bg_map_attributes(0x{:04x})",
        cpu.hl()
    );

    if let Some(i) = PAL_PACKET_POINTERS.iter().position(|&ptr| ptr == cpu.hl()) {
        cpu.c = (PAL_PACKET_POINTERS.len() - i) as u8;
        log::debug!("LoadBGMapAttributes({:02x})", cpu.c);
        macros::farcall::farcall(cpu, 0x2f, 0x7450); // LoadBGMapAttributes
    }
}

const PAL_PACKET_POINTERS: [u16; 12] = [
    0x6611, // BlkPacket_WholeScreen
    0x6621, // BlkPacket_Battle
    0x6641, // BlkPacket_StatusScreen
    0x6651, // BlkPacket_Pokedex
    0x6661, // BlkPacket_Slots
    0x6681, // BlkPacket_Titlescreen
    0x66a1, // BlkPacket_NidorinoIntro
    wram::W_PARTY_MENU_BLK_PACKET,
    wram::W_TRAINER_CARD_BLK_PACKET,
    0x6731, // BlkPacket_GameFreakIntro
    wram::W_PAL_PACKET,
    0x6751, // BlkPacket_PikachusBeachTitle
];
