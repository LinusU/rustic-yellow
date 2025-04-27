use crate::{
    cpu::Cpu,
    game::{
        constants::{
            gfx_constants::{CONVERT_BGP, CONVERT_OBP0, CONVERT_OBP1},
            hardware_constants,
            palette_constants::{self, NUM_ACTIVE_PALS},
        },
        data::sgb::sgb_packets::PAL_PACKET_EMPTY,
        ram::{hram, wram},
    },
};

const CGB_BASE_PAL_POINTERS: u16 = 0xdee1;

// uses PalPacket_Empty to build a packet based on mon IDs and health color
pub fn set_pal_battle(cpu: &mut Cpu) {
    log::debug!("set_pal_battle()");

    for (i, byte) in PAL_PACKET_EMPTY.iter().enumerate() {
        cpu.write_byte(wram::W_PAL_PACKET + i as u16, *byte);
    }

    if cpu.read_byte(wram::W_BATTLE_MON_SPECIES) == 0 {
        cpu.set_hl(wram::W_BATTLE_MON_SPECIES);
    } else {
        let idx = cpu.read_byte(wram::W_PLAYER_MON_NUMBER) as u16;
        cpu.set_hl(wram::W_PARTY_MON1 + (wram::W_PARTY_MON2 - wram::W_PARTY_MON1) * idx);
    }

    cpu.call(0x6093); // DeterminePaletteID
    let player_palette_id = cpu.a;

    cpu.set_hl(wram::W_ENEMY_MON_SPECIES2);
    cpu.call(0x6093); // DeterminePaletteID
    let enemy_palette_id = cpu.a;

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

    cpu.pc = cpu.stack_pop(); // ret
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
        cpu.call(0x65be); // TranslatePalPacketToBGMapAttributes
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
