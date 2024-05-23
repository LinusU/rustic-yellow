use crate::{
    cpu::Cpu,
    game::{home::predef::get_predef_registers, ram::wram},
};

/// In: b = item ID
/// Out: b = how many of that item are in the bag
pub fn get_quantity_of_item_in_bag(cpu: &mut Cpu) {
    // call GetPredefRegisters
    cpu.stack_push(0x0001);
    get_predef_registers(cpu);

    let item_id = cpu.b;
    let mut pos = wram::W_NUM_BAG_ITEMS;

    let item_count = loop {
        pos += 1;

        // end of list
        if cpu.read_byte(pos) == 0xff {
            break 0;
        }

        // item found
        if cpu.read_byte(pos) == item_id {
            break cpu.read_byte(pos + 1);
        }

        pos += 1;
    };

    cpu.b = item_count;
    cpu.pc = cpu.stack_pop(); // ret

    log::debug!(
        "get_quantity_of_item_in_bag({:#04x}) == {}",
        item_id,
        item_count
    );
}
