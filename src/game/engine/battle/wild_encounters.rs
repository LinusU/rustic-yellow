use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            map_constants::FIRST_INDOOR_MAP, text_constants::TEXT_REPEL_WORE_OFF,
            tileset_constants::FOREST,
        },
        data::wild::probabilities::WILD_MON_ENCOUNTER_SLOT_CHANCES,
        macros,
        ram::{hram, wram},
    },
};

fn try_do_wild_encounter_cant_encounter(cpu: &mut Cpu) {
    cpu.set_flag(CpuFlag::Z, false);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc = cpu.stack_pop(); // ret
    log::trace!("try_do_wild_encounter() == false");
}

fn try_do_wild_encounter_will_encounter(cpu: &mut Cpu) {
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc = cpu.stack_pop(); // ret
    log::debug!("try_do_wild_encounter() == true");
}

/// try to initiate a wild pokemon encounter \
/// returns success in Z
pub fn try_do_wild_encounter(cpu: &mut Cpu) {
    if cpu.read_byte(wram::W_NPC_MOVEMENT_SCRIPT_POINTER_TABLE_NUM) != 0 {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    if cpu.read_byte(wram::W_D736) != 0 {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    macros::farcall::callfar(cpu, 0x03, 0x41e6); // IsPlayerStandingOnDoorTileOrWarpTile

    if cpu.flag(CpuFlag::C) {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    // callfar IsPlayerJustOutsideMap
    macros::farcall::callfar(cpu, 0x3a, 0x476c);

    if cpu.flag(CpuFlag::Z) {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    match cpu.read_byte(wram::W_REPEL_REMAINING_STEPS) {
        0 => {}
        1 => return try_do_wild_encounter_last_repel_step(cpu),
        n => cpu.write_byte(wram::W_REPEL_REMAINING_STEPS, n - 1),
    }

    // determine if wild pokemon can appear in the half-block we're standing in
    // is the bottom left tile (8,9) of the half-block we're standing in a grass/water tile?
    // note that by using the bottom left tile, this prevents the "left-shore" tiles from generating grass encounters
    cpu.c = cpu.read_byte(macros::coords::coord!(8, 9));

    if cpu.c == cpu.read_byte(wram::W_GRASS_TILE) {
        let rate = cpu.read_byte(wram::W_GRASS_RATE);
        return try_do_wild_encounter_can_encounter(cpu, rate);
    }

    // in all tilesets with a water tile, 0x14 is its id
    if cpu.c == 0x14 {
        let rate = cpu.read_byte(wram::W_WATER_RATE);
        return try_do_wild_encounter_can_encounter(cpu, rate);
    }

    // even if not in grass/water, standing anywhere we can encounter pokemon
    // so long as the map is "indoor" and has wild pokemon defined.
    // ...as long as it's not Viridian Forest or Safari Zone.
    if cpu.borrow_wram().cur_map() < FIRST_INDOOR_MAP {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    // Viridian Forest/Safari Zone
    if cpu.read_byte(wram::W_CUR_MAP_TILESET) == FOREST {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    let rate = cpu.read_byte(wram::W_GRASS_RATE);
    try_do_wild_encounter_can_encounter(cpu, rate)
}

fn try_do_wild_encounter_can_encounter(cpu: &mut Cpu, encounter_rate: u8) {
    // compare encounter chance with a random number to determine if there will be an encounter
    if cpu.read_byte(hram::H_RANDOM_ADD) >= encounter_rate {
        return try_do_wild_encounter_cant_encounter(cpu);
    }

    let rand = cpu.read_byte(hram::H_RANDOM_SUB);

    for (slot_chance, double_slot_number) in WILD_MON_ENCOUNTER_SLOT_CHANCES {
        if slot_chance >= rand {
            return try_do_wild_encounter_got_encounter_slot(cpu, double_slot_number);
        }
    }

    unreachable!()
}

fn try_do_wild_encounter_got_encounter_slot(cpu: &mut Cpu, double_slot_number: u8) {
    log::debug!(
        "try_do_wild_encounter_got_encounter_slot({})",
        double_slot_number,
    );

    // determine which wild pokemon (grass or water) can appear in the half-block we're standing in
    // is the bottom left tile (8,9) of the half-block we're standing in a water tile?
    // else, it's treated as a grass tile by default
    let mons_data = if cpu.read_byte(macros::coords::coord!(8, 9)) == 0x14 {
        wram::W_WATER_MONS
    } else {
        wram::W_GRASS_MONS
    };

    let level = cpu.read_byte(mons_data + (double_slot_number as u16));
    let species = cpu.read_byte(mons_data + (double_slot_number as u16) + 1);

    cpu.write_byte(wram::W_CUR_ENEMY_LVL, level);
    cpu.write_byte(wram::W_CF91, species);
    cpu.write_byte(wram::W_ENEMY_MON_SPECIES2, species);

    if cpu.read_byte(wram::W_REPEL_REMAINING_STEPS) == 0 {
        return try_do_wild_encounter_will_encounter(cpu);
    }

    // repel prevents encounters if the leading party mon's level is higher than the wild mon
    if cpu.read_byte(wram::W_CUR_ENEMY_LVL) < cpu.read_byte(wram::W_PARTY_MON1_LEVEL) {
        try_do_wild_encounter_cant_encounter(cpu)
    } else {
        try_do_wild_encounter_will_encounter(cpu)
    }
}

fn try_do_wild_encounter_last_repel_step(cpu: &mut Cpu) {
    cpu.write_byte(wram::W_REPEL_REMAINING_STEPS, 0);

    cpu.write_byte(hram::H_SPRITE_INDEX_OR_TEXT_ID, TEXT_REPEL_WORE_OFF);
    cpu.call(0x3c29); // EnableAutoTextBoxDrawing
    cpu.call(0x2817); // DisplayTextID

    try_do_wild_encounter_cant_encounter(cpu);
}
