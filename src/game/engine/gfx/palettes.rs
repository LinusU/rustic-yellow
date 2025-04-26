use crate::{
    cpu::Cpu,
    game::{constants::palette_constants, data::sgb::sgb_packets::PAL_PACKET_EMPTY, ram::wram},
};

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
