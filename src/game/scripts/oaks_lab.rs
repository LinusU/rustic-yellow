use crate::{
    cpu::Cpu,
    game::{constants::item_constants, ram::wram},
};

pub fn oaks_lab_text18(cpu: &mut Cpu) {
    cpu.write_byte(wram::W_PLAYER_STARTER, cpu.starter.into_index());
    cpu.write_byte(wram::W_D11E, cpu.starter.into_index());

    {
        let source = cpu.starter.name();
        let target = wram::W_NAME_BUFFER;

        for (idx, byte) in source.iter().enumerate() {
            cpu.write_byte(target + (idx as u16), byte);
        }

        for idx in source.len()..=10 {
            cpu.write_byte(target + (idx as u16), 0x50);
        }
    }

    cpu.write_byte(
        wram::W_DO_NOT_WAIT_FOR_BUTTON_PRESS_AFTER_DISPLAYING_TEXT,
        1,
    );

    // PrintText OaksLabOakGivesText
    cpu.set_hl(0x4b85);
    cpu.call(0x3c36);

    // PrintText OaksLabRecievedText
    cpu.set_hl(0x4b8a);
    cpu.call(0x3c36);

    cpu.write_byte(wram::W_MON_DATA_LOCATION, 0);
    cpu.write_byte(wram::W_CUR_ENEMY_LVL, 5);

    // ld [wd11e], STARTER_PIKACHU
    cpu.write_byte(wram::W_D11E, cpu.starter.into_index());

    // ld [wCurPartySpecies], STARTER_PIKACHU
    cpu.write_byte(wram::W_CUR_PARTY_SPECIES, cpu.starter.into_index());

    // call AddPartyMon
    cpu.call(0x391c);

    // ld [wPartyMon1CatchRate], LIGHT_BALL_GSC
    cpu.write_byte(0xd171, item_constants::LIGHT_BALL_GSC);

    // call DisablePikachuOverworldSpriteDrawing
    cpu.call(0x152d);

    // SetEvent EVENT_GOT_STARTER
    {
        let value = cpu.read_byte(wram::W_EVENT_FLAGS + 4);
        cpu.write_byte(wram::W_EVENT_FLAGS + 4, value | (1 << 2));
    }

    // set 3, [wd72e]
    cpu.borrow_wram_mut()
        .set_has_received_pokemon_from_oak(true);

    // jp TextScriptEnd
    cpu.jump(0x23d2);
}
