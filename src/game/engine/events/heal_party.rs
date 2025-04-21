use crate::{
    cpu::Cpu,
    game::{constants::battle_constants, data::moves::moves::MOVES, ram::wram},
};

const PARTY_MON_BYTES: u16 = 0xd196 - 0xd16a; // wPartyMon2 - wPartyMon1
const PARTY_MON_START: u16 = 0xd16a; // wPartyMons

const PARTY_HP_OFFSET: u16 = 0xd16b - 0xd16a; // wPartyMon1HP - wPartyMon1
const PARTY_STATUS_OFFSET: u16 = 0xd16e - 0xd16a; // wPartyMon1Status - wPartyMon1
const PARTY_PP_OFFSET: u16 = 0xd187 - 0xd16a; // wPartyMon1PP - wPartyMon1

/// Restore HP and PP
pub fn heal_party(cpu: &mut Cpu) {
    log::info!("heal_party()");

    for i in 0.. {
        if cpu.borrow_wram().party().get(i as usize).is_none() {
            break;
        }

        // Set status to 0
        cpu.write_byte(
            PARTY_MON_START + (i * PARTY_MON_BYTES) + PARTY_STATUS_OFFSET,
            0,
        );

        // Reset move PPs
        for move_idx in 0..battle_constants::NUM_MOVES {
            let move_id =
                cpu.borrow_wram().party().get(i as usize).unwrap().moves[move_idx as usize];

            if move_id == 0 {
                break;
            }

            let old_pp = cpu.borrow_wram().party().get(i as usize).unwrap().pp[move_idx as usize];
            let new_pp = (old_pp & 0xc0) + MOVES[(move_id as usize) - 1].pp;

            cpu.write_byte(
                PARTY_MON_START + (i * PARTY_MON_BYTES) + PARTY_PP_OFFSET + (move_idx as u16),
                new_pp,
            );
        }

        // Reset HP
        let hp = cpu.borrow_wram().party().get(i as usize).unwrap().max_hp;
        let hp = hp.to_le_bytes();

        cpu.write_byte(
            PARTY_MON_START + (i * PARTY_MON_BYTES) + PARTY_HP_OFFSET,
            hp[0],
        );
        cpu.write_byte(
            PARTY_MON_START + (i * PARTY_MON_BYTES) + PARTY_HP_OFFSET + 1,
            hp[1],
        );
    }

    cpu.write_byte(wram::W_D11E, 0);

    let party_size = cpu.borrow_wram().party().len() as u8;

    for i in 0..party_size {
        cpu.write_byte(wram::W_WHICH_POKEMON, i);
        cpu.call(0x654a); // RestoreBonusPP
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
