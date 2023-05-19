use crate::{
    cpu::Cpu, game::engine::items::item_effects::switch_to_non_full_box, save_state::BoxView,
};

const WRAM_BOX_DATA_START: usize = 0x1a7f;

pub fn hook_give_pokemon_next_end(cpu: &mut Cpu) {
    // If the current box is full, switch to a non-full box
    if BoxView::new(&cpu.mmu.wram[WRAM_BOX_DATA_START..]).full() {
        let _ = switch_to_non_full_box(cpu);
    }

    // ret
    cpu.pc = cpu.stack_pop();
}
