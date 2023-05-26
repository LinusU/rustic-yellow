use crate::{
    cpu::Cpu,
    game::ram::wram,
    save_state::{BoxId, BoxView, BoxViewMut, BoxedPokemon},
};

const WRAM_BOX_DATA_START: usize = 0x1a7f;

pub fn switch_to_non_full_box(cpu: &mut Cpu) -> Result<(), ()> {
    log::debug!("Switching to non-full box");
    let current_box = cpu.read_byte(wram::W_CURRENT_BOX_NUM);

    let initialized = (current_box >> 7) != 0;
    let current_box_number = current_box & 0x7f;

    let sram = cpu.borrow_sram_mut();

    // If this is the first time changing boxes, we first reset all boxes
    if !initialized {
        log::info!("Resetting all boxes");
        sram.box_mut(BoxId::Box1).clear();
        sram.box_mut(BoxId::Box2).clear();
        sram.box_mut(BoxId::Box3).clear();
        sram.box_mut(BoxId::Box4).clear();
        sram.box_mut(BoxId::Box5).clear();
        sram.box_mut(BoxId::Box6).clear();
        sram.box_mut(BoxId::Box7).clear();
        sram.box_mut(BoxId::Box8).clear();
        sram.box_mut(BoxId::Box9).clear();
        sram.box_mut(BoxId::Box10).clear();
        sram.box_mut(BoxId::Box11).clear();
        sram.box_mut(BoxId::Box12).clear();
    }

    // Find non-full box
    for test in 0..12 {
        if test == current_box_number {
            continue;
        }

        let box_id = match test {
            0 => BoxId::Box1,
            1 => BoxId::Box2,
            2 => BoxId::Box3,
            3 => BoxId::Box4,
            4 => BoxId::Box5,
            5 => BoxId::Box6,
            6 => BoxId::Box7,
            7 => BoxId::Box8,
            8 => BoxId::Box9,
            9 => BoxId::Box10,
            10 => BoxId::Box11,
            11 => BoxId::Box12,
            _ => unreachable!(),
        };

        if !sram.r#box(box_id).full() {
            log::debug!("Switching to Box{}", test + 1);

            const BOX_BYTE_SIZE: usize = (wram::W_BOX_DATA_END - wram::W_BOX_DATA_START) as usize;
            log::debug!("BOX_BYTE_SIZE: {}", BOX_BYTE_SIZE);

            let new_box_sram_addr = box_id.sram_offset();
            log::debug!("new_box_sram_addr: {:x}", new_box_sram_addr);

            let old_box_sram_addr = match current_box_number {
                0 => BoxId::Box1.sram_offset(),
                1 => BoxId::Box2.sram_offset(),
                2 => BoxId::Box3.sram_offset(),
                3 => BoxId::Box4.sram_offset(),
                4 => BoxId::Box5.sram_offset(),
                5 => BoxId::Box6.sram_offset(),
                6 => BoxId::Box7.sram_offset(),
                7 => BoxId::Box8.sram_offset(),
                8 => BoxId::Box9.sram_offset(),
                9 => BoxId::Box10.sram_offset(),
                10 => BoxId::Box11.sram_offset(),
                11 => BoxId::Box12.sram_offset(),
                _ => unreachable!(),
            };
            log::debug!("old_box_sram_addr: {:x}", old_box_sram_addr);

            // Copy current box from WRAM to SRAM
            log::debug!("Copying current box from WRAM to SRAM");
            for i in 0..BOX_BYTE_SIZE {
                let value = cpu.mmu.wram[WRAM_BOX_DATA_START + i];
                cpu.borrow_sram_mut().set_byte(old_box_sram_addr + i, value);
            }

            // Copy new box from SRAM to WRAM
            log::debug!("Copying new box from SRAM to WRAM");
            for i in 0..BOX_BYTE_SIZE {
                let value = cpu.borrow_sram().byte(new_box_sram_addr + i);
                cpu.mmu.wram[WRAM_BOX_DATA_START + i] = value;
            }

            // Clear new box in SRAM
            log::debug!("Clearing new box in SRAM");
            cpu.borrow_sram_mut().box_mut(box_id).clear();

            cpu.write_byte(wram::W_CURRENT_BOX_NUM, test | 0b1000_0000);

            return Ok(());
        }
    }

    log::warn!("No non-full box found");

    Err(())
}

pub fn hook_send_new_mon_to_box_end(cpu: &mut Cpu) {
    log::info!("A pokemon has been sent to the box");

    // If the current box is full, switch to a non-full box
    if BoxView::new(&cpu.mmu.wram[WRAM_BOX_DATA_START..]).full() {
        let _ = switch_to_non_full_box(cpu);
    }

    // ret
    cpu.pc = cpu.stack_pop();
}

pub fn add_pokemon_to_box(cpu: &mut Cpu, pokemon: BoxedPokemon) -> Result<(), ()> {
    let mut current = BoxViewMut::new(&mut cpu.mmu.wram[WRAM_BOX_DATA_START..]);

    if !current.full() {
        current.push(pokemon);

        if current.full() {
            let _ = switch_to_non_full_box(cpu);
        }
    } else {
        switch_to_non_full_box(cpu)?;

        let mut current = BoxViewMut::new(&mut cpu.mmu.wram[WRAM_BOX_DATA_START..]);
        current.push(pokemon);
    }

    Ok(())
}
