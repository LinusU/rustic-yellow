use crate::{
    cpu::Cpu,
    game::{
        constants::{
            gfx_constants::{CONVERT_BGP, CONVERT_OBP0, CONVERT_OBP1},
            hardware_constants::{self, R_LCDC, R_LCDC_ENABLE},
            palette_constants::{self, NUM_ACTIVE_PALS, NUM_PAL_COLORS, PAL_GREENMON},
            pokemon_constants,
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

    if pal_fn == palette_constants::SET_PAL_DEFAULT {
        pal_fn = cpu.read_byte(wram::W_DEFAULT_PALETTE_COMMAND);
    }

    if pal_fn == palette_constants::SET_PAL_PARTY_MENU_HP_BARS {
        cpu.call(0x618b); // UpdatePartyMenuBlkPacket
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    match pal_fn {
        palette_constants::SET_PAL_BATTLE_BLACK => set_pal_battle_black(cpu),
        palette_constants::SET_PAL_BATTLE => set_pal_battle(cpu),
        palette_constants::SET_PAL_TOWN_MAP => set_pal_town_map(cpu),
        palette_constants::SET_PAL_STATUS_SCREEN => set_pal_status_screen(cpu),
        palette_constants::SET_PAL_POKEDEX => set_pal_pokedex(cpu),
        palette_constants::SET_PAL_SLOTS => set_pal_slots(cpu),
        palette_constants::SET_PAL_TITLE_SCREEN => set_pal_title_screen(cpu),
        palette_constants::SET_PAL_NIDORINO_INTRO => set_pal_nidorino_intro(cpu),
        palette_constants::SET_PAL_GENERIC => set_pal_generic(cpu),
        palette_constants::SET_PAL_OVERWORLD => cpu.call(0x5fa5), // SetPal_Overworld
        palette_constants::SET_PAL_PARTY_MENU => set_pal_party_menu(cpu),
        palette_constants::SET_PAL_POKEMON_WHOLE_SCREEN => set_pal_pokemon_whole_screen(cpu),
        palette_constants::SET_PAL_GAME_FREAK_INTRO => set_pal_game_freak_intro(cpu),
        palette_constants::SET_PAL_TRAINER_CARD => cpu.call(0x6025), // SetPal_TrainerCard
        palette_constants::SET_PAL_SURFING_PIKACHU_TITLE => cpu.call(0x605d), // SetPal_PikachusBeach
        palette_constants::SET_PAL_SURFING_PIKACHU_MINIGAME => cpu.call(0x6064), // SetPal_PikachusBeachTitle
        i => panic!("Invalid SetPalFunctions index: {i}"),
    }

    cpu.call(0x6328); // SendSGBPackets

    cpu.pc = cpu.stack_pop(); // ret
}

fn set_pal_battle_black(cpu: &mut Cpu) {
    cpu.set_hl(0x6781); // PalPacket_Black
    cpu.set_de(0x6621); // BlkPacket_Battle
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
        0 => palette_constants::PAL_GREENBAR,
        1 => palette_constants::PAL_YELLOWBAR,
        2 => palette_constants::PAL_REDBAR,
        n => panic!("Invalid player HP bar color {n}"),
    };

    let enemy_hp_palette_id = match cpu.read_byte(wram::W_ENEMY_HP_BAR_COLOR) {
        0 => palette_constants::PAL_GREENBAR,
        1 => palette_constants::PAL_YELLOWBAR,
        2 => palette_constants::PAL_REDBAR,
        n => panic!("Invalid enemy HP bar color {n}"),
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, player_hp_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 3, enemy_hp_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 5, player_palette_id);
    cpu.write_byte(wram::W_PAL_PACKET + 7, enemy_palette_id);

    cpu.set_hl(wram::W_PAL_PACKET);

    cpu.set_de(0x6621); //BlkPacket_Battle
    cpu.a = palette_constants::SET_PAL_BATTLE;
    cpu.write_byte(wram::W_DEFAULT_PALETTE_COMMAND, cpu.a);
}

fn set_pal_town_map(cpu: &mut Cpu) {
    cpu.set_hl(0x6791); // PalPacket_TownMap
    cpu.set_de(0x6611); // BlkPacket_WholeScreen
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
        0 => palette_constants::PAL_GREENBAR,
        1 => palette_constants::PAL_YELLOWBAR,
        2 => palette_constants::PAL_REDBAR,
        n => panic!("Invalid HP bar color: {n}"),
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, hp_pal);
    cpu.write_byte(wram::W_PAL_PACKET + 3, mon_pal);

    cpu.set_hl(wram::W_PAL_PACKET);
    cpu.set_de(0x6641); // BlkPacket_StatusScreen
}

fn set_pal_party_menu(cpu: &mut Cpu) {
    cpu.set_hl(0x6771); // PalPacket_PartyMenu
    cpu.set_de(wram::W_PARTY_MENU_BLK_PACKET);
}

fn set_pal_pokedex(cpu: &mut Cpu) {
    log::debug!("set_pal_pokedex()");

    for (i, byte) in PAL_PACKET_POKEDEX.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let species = cpu.read_byte(wram::W_CUR_PARTY_SPECIES);
    let mon_pal = determine_palette_id(species);

    cpu.write_byte(wram::W_PAL_PACKET + 3, mon_pal);

    cpu.set_hl(wram::W_PAL_PACKET);
    cpu.set_de(0x6651); // BlkPacket_Pokedex
}

fn set_pal_slots(cpu: &mut Cpu) {
    cpu.set_hl(0x67b1); // PalPacket_Slots
    cpu.set_de(0x6661); // BlkPacket_Slots
}

fn set_pal_title_screen(cpu: &mut Cpu) {
    cpu.set_hl(0x67c1); // PalPacket_Titlescreen
    cpu.set_de(0x6681); // BlkPacket_Titlescreen
}

// used mostly for menus and the Oak intro
fn set_pal_generic(cpu: &mut Cpu) {
    cpu.set_hl(0x67e1); // PalPacket_Generic
    cpu.set_de(0x6611); // BlkPacket_WholeScreen
}

fn set_pal_nidorino_intro(cpu: &mut Cpu) {
    cpu.set_hl(0x67f1); // PalPacket_NidorinoIntro
    cpu.set_de(0x66a1); // BlkPacket_NidorinoIntro
}

fn set_pal_game_freak_intro(cpu: &mut Cpu) {
    cpu.set_hl(0x6801); // PalPacket_GameFreakIntro
    cpu.set_de(0x6731); // BlkPacket_GameFreakIntro

    cpu.write_byte(
        wram::W_DEFAULT_PALETTE_COMMAND,
        palette_constants::SET_PAL_GENERIC,
    );
}

// used when a Pokemon is the only thing on the screen
// such as evolution, trading and the Hall of Fame
fn set_pal_pokemon_whole_screen(cpu: &mut Cpu) {
    log::debug!("set_pal_pokemon_whole_screen({:02x})", cpu.c);

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    let pal = if cpu.c != 0 {
        palette_constants::PAL_BLACK
    } else {
        let species = cpu.read_byte(wram::W_WHOLE_SCREEN_PALETTE_MON_SPECIES);
        determine_palette_id(species)
    };

    cpu.write_byte(wram::W_PAL_PACKET + 1, pal);

    cpu.set_hl(wram::W_PAL_PACKET);
    cpu.set_de(0x6611); // BlkPacket_WholeScreen
}

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

/// Input:
/// - hl: Pointer to the first packet to be sent
/// - de: Pointer to the second packet to be sent
pub fn send_sgb_packets(cpu: &mut Cpu) {
    log::trace!("send_sgb_packets()");

    if cpu.read_byte(hram::H_ON_CGB) == 0 {
        panic!("send_sgb_packets called on non-SGB device");
    }

    let first_packet = cpu.hl();
    let second_packet = cpu.de();

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
