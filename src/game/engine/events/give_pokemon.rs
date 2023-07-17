use crate::{cpu::Cpu, game::engine::items::item_effects::switch_to_non_full_box};

pub fn hook_give_pokemon_next_end(cpu: &mut Cpu) {
    // If the current box is full, switch to a non-full box
    if cpu.borrow_wram().r#box().full() {
        let _ = switch_to_non_full_box(cpu);
    }

    // ret
    cpu.pc = cpu.stack_pop();
}
