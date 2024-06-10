use crate::{
    cpu::Cpu,
    game::{data::wild::grass_water::WildPokemonData, ram::wram},
    rom::ROM,
};

const WILD_DATA_POINTERS: u16 = 0x4b95;

pub fn load_wild_data(cpu: &mut Cpu) {
    log::debug!("load_wild_data()");

    // get wild data for current map
    let cur_map = cpu.borrow_wram().cur_map();
    let pointer = WILD_DATA_POINTERS + (cur_map as u16) * 2;

    // Pointers are stored in little-endian format
    let target = u16::from_le_bytes([cpu.read_byte(pointer), cpu.read_byte(pointer + 1)]);

    // Wild Pokemon data is stored in bank 3
    let offset = (3 * 0x4000) | ((target as usize) & 0x3FFF);
    let data = WildPokemonData::from_bytes(&ROM[offset..]);

    log::debug!("grass_rate = {}", data.grass_encounter_rate());
    cpu.write_byte(wram::W_GRASS_RATE, data.grass_encounter_rate());

    log::debug!("water_rate = {}", data.water_encounter_rate());
    cpu.write_byte(wram::W_WATER_RATE, data.water_encounter_rate());

    if let Some(grass) = data.grass {
        for (idx, (species, level)) in grass.mons.iter().enumerate() {
            log::debug!("grass_mon[{}] = {} lvl {}", idx, species.name(), level);

            let species = species.into_index();

            cpu.write_byte(wram::W_GRASS_MONS + (idx as u16) * 2, *level);
            cpu.write_byte(wram::W_GRASS_MONS + (idx as u16) * 2 + 1, species);
        }
    }

    if let Some(water) = data.water {
        for (idx, (species, level)) in water.mons.iter().enumerate() {
            log::debug!("water_mon[{}] = {} lvl {}", idx, species.name(), level);

            let species = species.into_index();

            cpu.write_byte(wram::W_WATER_MONS + (idx as u16) * 2, *level);
            cpu.write_byte(wram::W_WATER_MONS + (idx as u16) * 2 + 1, species);
        }
    }

    cpu.pc = cpu.stack_pop(); // ret
}
