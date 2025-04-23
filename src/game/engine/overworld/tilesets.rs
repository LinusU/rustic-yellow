use crate::{
    cpu::Cpu,
    game::{
        data::tilesets::dungeon_tilesets::DUNGEON_TILESETS,
        home::{overworld::load_destination_warp_position, predef::get_predef_registers},
    },
};

/// Input: (from predef registers) \
/// hl = pointer to warp data
pub fn load_tileset_header(cpu: &mut Cpu) {
    log::debug!("load_tileset_header()");

    // call GetPredefRegisters
    cpu.stack_push(0x0001);
    get_predef_registers(cpu);

    let warp_data = cpu.hl();

    let cur_map_tileset = cpu.borrow_wram().cur_map_tileset();

    cpu.set_hl(0x4558 + ((cur_map_tileset as u16) * 12)); // Tilesets

    cpu.set_de(0xd52a); // wTilesetBank
    cpu.set_bc(0xb);
    cpu.call(0x00b1); // CopyData

    assert_eq!(cpu.hl(), 0x4558 + ((cur_map_tileset as u16) * 12) + 0xb);

    let tile_animations = cpu.read_byte(cpu.hl());
    cpu.borrow_wram_mut().set_tile_animations(tile_animations);
    cpu.borrow_wram_mut().set_moving_bg_tiles_counter1(0);

    cpu.set_hl(warp_data);

    let cur_map_tileset = cpu.borrow_wram().cur_map_tileset();

    if !DUNGEON_TILESETS.contains(&cur_map_tileset) {
        let previous_tileset = cpu.borrow_wram().previous_tileset();

        if cur_map_tileset == previous_tileset {
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }
    }

    let warp_id = cpu.borrow_wram().destination_warp_id();

    if warp_id == 0xff {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    load_destination_warp_position(cpu, warp_id, warp_data);

    let y_block_coord = cpu.borrow_wram().y_coord() & 1;
    cpu.borrow_wram_mut().set_y_block_coord(y_block_coord);

    let x_block_coord = cpu.borrow_wram().x_coord() & 1;
    cpu.borrow_wram_mut().set_x_block_coord(x_block_coord);

    cpu.pc = cpu.stack_pop(); // ret
}
