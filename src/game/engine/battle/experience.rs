use crate::{
    cpu::Cpu,
    game::{
        constants::{
            battle_constants::{MAX_LEVEL, NUM_STATS, TRANSFORMED},
            misc_constants::FLAG_SET,
            pikachu_emotion_constants::PIKAHAPPY_LEVELUP,
            serial_constants::LINK_STATE_BATTLING,
        },
        macros,
        ram::{hram, wram},
    },
};

pub fn gain_experience(cpu: &mut Cpu) {
    log::info!("gain_experience()");

    // return if link battle
    if cpu.read_byte(wram::W_LINK_STATE) == LINK_STATE_BATTLING {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    divide_exp_data_by_num_mons_gaining_exp(cpu);

    cpu.set_hl(wram::W_PARTY_MON1);
    cpu.write_byte(wram::W_WHICH_POKEMON, 0);

    let party_len = cpu.borrow_wram().party().len();

    // loop over each mon and add gained exp
    for party_pokemon_index in 0..party_len {
        cpu.write_byte(wram::W_WHICH_POKEMON, party_pokemon_index as u8);
        let pokemon = cpu.borrow_wram().party().get(party_pokemon_index).unwrap();

        if pokemon.hp == 0 {
            continue;
        }

        let bitfield = cpu.read_byte(wram::W_PARTY_GAIN_EXP_FLAGS);
        let mon_gains_exp = (bitfield & (1 << party_pokemon_index)) != 0;

        // if mon's gain exp flag not set, go to next mon
        if !mon_gains_exp {
            continue;
        }

        log::debug!(
            "Adding experience to party member {} ({})",
            party_pokemon_index,
            pokemon.nickname.unwrap_or(pokemon.species.name())
        );

        let mon_start = cpu.hl();

        for i in 0..(NUM_STATS as u16) {
            let enemy_mon_base_stat = cpu.read_byte(wram::W_ENEMY_MON_BASE_STATS + i);

            let stat_exp_hi = cpu.read_byte(mon_start + 0x11 + (i * 2));
            let stat_exp_lo = cpu.read_byte(mon_start + 0x11 + (i * 2) + 1);

            let stat_exp = u16::from_be_bytes([stat_exp_hi, stat_exp_lo]);
            let stat_exp = stat_exp.saturating_add(enemy_mon_base_stat as u16);

            let [stat_exp_hi, stat_exp_lo] = stat_exp.to_be_bytes();

            cpu.write_byte(mon_start + 0x11 + (i * 2), stat_exp_hi);
            cpu.write_byte(mon_start + 0x11 + (i * 2) + 1, stat_exp_lo);
        }

        // GainExperience.statExpDone
        cpu.pc = 0x52b0;

        let mut exp_gained = {
            let base_exp = cpu.read_byte(wram::W_ENEMY_MON_BASE_EXP) as u16;
            let level = cpu.read_byte(wram::W_ENEMY_MON_LEVEL) as u16;

            base_exp * level / 7
        };

        // party mon OTID
        let traded_mon = cpu.borrow_wram().player_id() != pokemon.ot_id;

        if traded_mon {
            log::debug!("Traded mon, multiplying experience by 1.5");
            exp_gained += exp_gained >> 1; // Multiply by 1.5
        }

        // GainExperience.next
        cpu.pc = 0x52e5;

        cpu.write_byte(wram::W_GAIN_BOOSTED_EXP, if traded_mon { 1 } else { 0 });

        // is it a trainer battle? if so, boost exp
        if cpu.read_byte(wram::W_IS_IN_BATTLE) != 1 {
            log::debug!("Trainer battle, multiplying experience by 1.5");
            exp_gained += exp_gained >> 1; // Multiply by 1.5
        }

        log::debug!("Experience gained: {}", exp_gained);

        // add the gained exp to the party mon's exp
        cpu.write_byte(wram::W_EXP_AMOUNT_GAINED, (exp_gained >> 8) as u8);
        cpu.write_byte(wram::W_EXP_AMOUNT_GAINED + 1, (exp_gained & 0xff) as u8);

        let new_exp = pokemon.exp + (exp_gained as u32);
        cpu.write_byte(mon_start + 0x0e, ((new_exp >> 16) & 0xff) as u8);
        cpu.write_byte(mon_start + 0x0f, ((new_exp >> 8) & 0xff) as u8);
        cpu.write_byte(mon_start + 0x10, (new_exp & 0xff) as u8);

        // GainExperience.noCarry
        cpu.pc = 0x5307;

        // calculate exp for the mon at max level, and cap the exp at that value
        cpu.write_byte(wram::W_CUR_SPECIES, pokemon.species.into_index());
        cpu.call(0x132f); // GetMonHeader

        // get max exp
        cpu.d = MAX_LEVEL;
        macros::farcall::callfar(cpu, 0x16, 0x4dc0); // CalcExperience

        let max_exp = u32::from_be_bytes([
            0,
            cpu.read_byte(hram::H_EXPERIENCE),
            cpu.read_byte(hram::H_EXPERIENCE + 1),
            cpu.read_byte(hram::H_EXPERIENCE + 2),
        ]);

        // compare max exp with current exp
        if new_exp > max_exp {
            // the mon's exp is greater than the max exp, so overwrite it with the max exp
            cpu.write_byte(mon_start + 0x0e, ((max_exp >> 16) & 0xff) as u8);
            cpu.write_byte(mon_start + 0x0f, ((max_exp >> 8) & 0xff) as u8);
            cpu.write_byte(mon_start + 0x10, (max_exp & 0xff) as u8);
        }

        // GainExperience.next2
        cpu.pc = 0x533d;

        cpu.a = party_pokemon_index as u8;
        cpu.set_hl(wram::W_PARTY_MON_NICKS);
        cpu.call(0x139a); // GetPartyMonName

        cpu.set_hl(0x54c6); // GainedText
        cpu.call(0x3c36); // PrintText

        // PLAYER_PARTY_DATA
        cpu.a = 0;
        cpu.write_byte(wram::W_MON_DATA_LOCATION, 0);
        cpu.call(0x1132); // LoadMonData

        cpu.set_hl(mon_start + 0x21); // This HL is unused?
        macros::farcall::farcall(cpu, 0x16, 0x4d99); // CalcLevelFromExperience
        let calculated_level = cpu.d;

        // if level didn't change, go to next mon
        if pokemon.level == calculated_level {
            continue;
        }

        log::debug!(
            "Pokemon is leveling up from level {} to level {}",
            pokemon.level,
            calculated_level
        );

        let saved_enemy_level = cpu.read_byte(wram::W_CUR_ENEMY_LEVEL);

        cpu.write_byte(wram::W_CUR_ENEMY_LEVEL, calculated_level);
        cpu.write_byte(mon_start + 0x21, calculated_level);

        cpu.write_byte(wram::W_CUR_SPECIES, pokemon.species.into_index());
        cpu.write_byte(wram::W_POKEDEX_NUM, pokemon.species.into_index());
        cpu.call(0x132f); // GetMonHeader

        cpu.b = 0x1; // input: consider stat exp when calculating stats
        cpu.set_hl(mon_start + 0x10); // input: base ptr to stat exp values
        cpu.set_de(mon_start + 0x22); // output
        cpu.call(0x392b); // CalcStats

        let new_max_hp = u16::from_be_bytes([
            cpu.read_byte(mon_start + 0x22),
            cpu.read_byte(mon_start + 0x23),
        ]);

        // difference between old max HP and new max HP after levelling
        cpu.set_bc(new_max_hp - pokemon.max_hp);

        // add to the current HP the amount of max HP gained when levelling
        {
            // Increment low byte
            cpu.a = cpu.read_byte(mon_start + 0x2);
            let carry = (cpu.a as u16) + (cpu.c as u16) > 0xff;
            cpu.a = cpu.a.wrapping_add(cpu.c);
            cpu.write_byte(mon_start + 0x2, cpu.a);

            // Increment high byte
            cpu.a = cpu.read_byte(mon_start + 0x1);
            cpu.a += cpu.b + if carry { 1 } else { 0 };
            cpu.write_byte(mon_start + 0x1, cpu.a);
        }

        cpu.b = cpu.read_byte(wram::W_PLAYER_MON_NUMBER);
        cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);

        // is the current mon in battle?
        if cpu.a == cpu.b {
            // copy party mon HP to battle mon HP
            cpu.a = cpu.read_byte(mon_start + 0x1);
            cpu.write_byte(wram::W_BATTLE_MON_HP, cpu.a);
            cpu.a = cpu.read_byte(mon_start + 0x2);
            cpu.write_byte(wram::W_BATTLE_MON_HP + 1, cpu.a);

            // copy other stats from party mon to battle mon
            cpu.set_bc(1 + (NUM_STATS as u16) * 2); // size of stats
            cpu.set_hl(mon_start + 0x21);
            cpu.set_de(wram::W_BATTLE_MON_LEVEL);
            cpu.call(0x00b1); // CopyData

            if (cpu.read_byte(wram::W_PLAYER_BATTLE_STATUS3) & (1 << TRANSFORMED)) == 0 {
                // the mon is not transformed, so update the unmodified stats
                cpu.set_bc(1 + (NUM_STATS as u16) * 2);
                cpu.set_hl(mon_start + 0x21);
                cpu.set_de(wram::W_PLAYER_MON_UNMODIFIED_LEVEL);
                cpu.call(0x00b1); // CopyData
            }

            // GainExperience.recalcStatChanges
            cpu.pc = 0x53d7;

            // battle mon
            cpu.write_byte(wram::W_CALCULATE_WHOSE_STATS, 0);
            cpu.set_hl(0x6f25); // CalculateModifiedStats
            cpu.call(0x54c1); // CallBattleCore

            cpu.set_hl(0x6ea6); // ApplyBurnAndParalysisPenaltiesToPlayer
            cpu.call(0x54c1); // CallBattleCore

            cpu.set_hl(0x6fa5); // ApplyBadgeStatBoosts
            cpu.call(0x54c1); // CallBattleCore

            cpu.set_hl(0x4e25); // DrawPlayerHUDAndHPBar
            cpu.call(0x54c1); // CallBattleCore

            cpu.set_hl(0x7020); // PrintEmptyString
            cpu.call(0x54c1); // CallBattleCore

            cpu.call(0x370f); // SaveScreenTilesToBuffer1
        }

        // GainExperience.printGrewLevelText
        cpu.pc = 0x53fc;

        // callabd_ModifyPikachuHappiness PIKAHAPPY_LEVELUP
        macros::farcall::callabd_modify_pikachu_happiness(cpu, PIKAHAPPY_LEVELUP);

        cpu.set_hl(0x54f1); // GrewLevelText
        cpu.call(0x3c36); // PrintText

        cpu.write_byte(wram::W_MON_DATA_LOCATION, 0); // PLAYER_PARTY_DATA
        cpu.call(0x1132); // LoadMonData

        cpu.d = 0x1;
        macros::farcall::callfar(cpu, 0x04, 0x568a); // PrintStatsBox

        cpu.call(0x3852); // WaitForTextScrollButtonPress
        cpu.call(0x371b); // LoadScreenTilesFromBuffer1

        cpu.write_byte(wram::W_MON_DATA_LOCATION, 0); // PLAYER_PARTY_DATA
        cpu.a = cpu.read_byte(wram::W_CUR_SPECIES);
        cpu.write_byte(wram::W_POKEDEX_NUM, cpu.a);
        macros::predef::predef_call!(cpu, LearnMoveFromLevelUp); // LearnMoveFromLevelUp

        cpu.set_hl(wram::W_CAN_EVOLVE_FLAGS);
        cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
        cpu.c = cpu.read_byte(wram::W_WHICH_POKEMON);
        cpu.b = FLAG_SET;
        macros::predef::predef_call!(cpu, FlagActionPredef);

        cpu.write_byte(wram::W_CUR_ENEMY_LEVEL, saved_enemy_level);
    }

    // GainExperience.done
    cpu.pc = 0x545f;

    // clear gain exp flags
    cpu.write_byte(wram::W_PARTY_GAIN_EXP_FLAGS, 0);

    // set the gain exp flag for the mon that is currently out
    cpu.b = FLAG_SET;
    cpu.c = cpu.read_byte(wram::W_PLAYER_MON_NUMBER);
    cpu.set_hl(wram::W_PARTY_GAIN_EXP_FLAGS);
    macros::predef::predef_call!(cpu, FlagActionPredef);

    cpu.write_byte(wram::W_PARTY_FOUGHT_CURRENT_ENEMY_FLAGS, 0);

    // set the fought current enemy flag for the mon that is currently out
    cpu.b = FLAG_SET;
    cpu.c = cpu.read_byte(wram::W_PLAYER_MON_NUMBER);
    cpu.set_hl(wram::W_PARTY_FOUGHT_CURRENT_ENEMY_FLAGS);
    macros::predef::predef_call!(cpu, FlagActionPredef);

    cpu.pc = cpu.stack_pop(); // ret
}

/// divide enemy base stats, catch rate, and base exp by the number of mons gaining exp
fn divide_exp_data_by_num_mons_gaining_exp(cpu: &mut Cpu) {
    let mons_gaining_exp = cpu.read_byte(wram::W_PARTY_GAIN_EXP_FLAGS).count_ones() as u8;

    // return if only one mon is gaining exp
    if mons_gaining_exp <= 1 {
        return;
    }

    for addr in wram::W_ENEMY_MON_BASE_STATS..=wram::W_ENEMY_MON_BASE_EXP {
        let input = cpu.read_byte(addr);
        let output = input / mons_gaining_exp;

        cpu.write_byte(addr, output);
    }
}
