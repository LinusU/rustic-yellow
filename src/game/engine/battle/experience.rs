use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{MAX_LEVEL, NUM_STATS, TRANSFORMED},
            misc_constants::{FLAG_SET, FLAG_TEST},
            pikachu_emotion_constants::PIKAHAPPY_LEVELUP,
            serial_constants::LINK_STATE_BATTLING,
        },
        macros,
        ram::{hram, wram},
    },
};

pub fn gain_experience(cpu: &mut Cpu) {
    log::info!("gain_experience()");

    cpu.pc = 0x525f;

    // ld a, [wLinkState]
    cpu.a = cpu.read_byte(wram::W_LINK_STATE);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp LINK_STATE_BATTLING
    cpu.set_flag(CpuFlag::Z, cpu.a == LINK_STATE_BATTLING);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (LINK_STATE_BATTLING & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < LINK_STATE_BATTLING);
    cpu.pc += 1;
    cpu.cycle(4);

    // return if link battle
    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // call DivideExpDataByNumMonsGainingExp
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.stack_push(pc);
        divide_exp_data_by_num_mons_gaining_exp(cpu);
        assert_eq!(cpu.pc, pc);
    }

    // ld hl, wPartyMon1
    cpu.set_hl(wram::W_PARTY_MON1);
    cpu.pc += 3;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wWhichPokemon], a
    cpu.write_byte(wram::W_WHICH_POKEMON, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    gain_experience_party_mon_loop(cpu);
}

// loop over each mon and add gained exp
fn gain_experience_party_mon_loop(cpu: &mut Cpu) {
    cpu.pc = 0x526f;

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // is mon's HP 0?
    // or a, [hl]
    cpu.a |= cpu.read_byte(cpu.hl());
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(8);

    // if so, go to next mon
    // jp z, .nextMon
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return gain_experience_next_mon(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, wPartyGainExpFlags
    cpu.set_hl(wram::W_PARTY_GAIN_EXP_FLAGS);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, FLAG_TEST
    cpu.b = FLAG_TEST;
    cpu.pc += 2;
    cpu.cycle(8);

    // predef FlagActionPredef
    macros::predef::predef_call!(cpu, FlagActionPredef);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // is mon's gain exp flag set?
    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // if mon's gain exp flag not set, go to next mon
    // jp z, .nextMon
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return gain_experience_next_mon(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld de, (wPartyMon1HPExp + 1) - (wPartyMon1HP + 1)
    cpu.set_de((wram::W_PARTY_MON1_HP_EXP + 1) - (wram::W_PARTY_MON1_HP + 1));
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, wEnemyMonBaseStats
    cpu.set_hl(wram::W_ENEMY_MON_BASE_STATS);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld c, NUM_STATS
    cpu.c = NUM_STATS;
    cpu.pc += 2;
    cpu.cycle(8);

    gain_experience_gain_stat_exp_loop(cpu);
}

fn gain_experience_gain_stat_exp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5295;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // enemy mon base stat
    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // stat exp
    // ld a, [de]
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);

    // add enemy mon base state to stat exp
    // add b
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.b & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.b as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.b);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // jr nc, .nextBaseStat
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return gain_experience_next_base_stat(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // if there was a carry, increment the upper byte
    // dec de
    cpu.set_de(cpu.de().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [de]
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jump if the value overflowed
    // jr z, .maxStatExp
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_max_stat_exp(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // jr .nextBaseStat
    cpu.cycle(12);
    gain_experience_next_base_stat(cpu)
}

// if the upper byte also overflowed, then we have hit the max stat exp
fn gain_experience_max_stat_exp(cpu: &mut Cpu) {
    cpu.pc = 0x52a5;

    // ld a, $ff; a is 0 from previous check
    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    gain_experience_next_base_stat(cpu);
}

fn gain_experience_next_base_stat(cpu: &mut Cpu) {
    cpu.pc = 0x52a9;

    // dec c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x00);
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .statExpDone
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_stat_exp_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // jr .gainStatExpLoop
    cpu.cycle(12);
    gain_experience_gain_stat_exp_loop(cpu)
}

fn gain_experience_stat_exp_done(cpu: &mut Cpu) {
    cpu.pc = 0x52b0;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hMultiplicand], a
    cpu.write_byte(hram::H_MULTIPLICAND, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [hMultiplicand + 1], a
    cpu.write_byte(hram::H_MULTIPLICAND + 1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [wEnemyMonBaseExp]
    cpu.a = cpu.read_byte(wram::W_ENEMY_MON_BASE_EXP);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hMultiplicand + 2], a
    cpu.write_byte(hram::H_MULTIPLICAND + 2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [wEnemyMonLevel]
    cpu.a = cpu.read_byte(wram::W_ENEMY_MON_LEVEL);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh [hMultiplier], a
    cpu.write_byte(hram::H_MULTIPLIER, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call Multiply
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x38a5); // Multiply
        cpu.pc = pc;
    }

    // ld a, 7
    cpu.a = 7;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hDivisor], a
    cpu.write_byte(hram::H_DIVISOR, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld b, 4
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);

    // call Divide
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x38b2); // Divide
        cpu.pc = pc;
    }

    // ld hl, wPartyMon1OTID - (wPartyMon1DVs - 1)
    cpu.set_hl(wram::W_PARTY_MON1_OTID.wrapping_sub(wram::W_PARTY_MON1_DVS - 1));
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // party mon OTID
    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wPlayerID]
    cpu.a = cpu.read_byte(wram::W_PLAYER_ID);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .tradedMon
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_traded_mon(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wPlayerID + 1]
    cpu.a = cpu.read_byte(wram::W_PLAYER_ID + 1);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, 0
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .next
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_next(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    gain_experience_traded_mon(cpu);
}

fn gain_experience_traded_mon(cpu: &mut Cpu) {
    cpu.pc = 0x52e0;

    // traded mon exp boost
    // call BoostExp
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54ae); // BoostExp
        cpu.pc = pc;
    }

    // ld a, 1
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);

    gain_experience_next(cpu);
}

fn gain_experience_next(cpu: &mut Cpu) {
    cpu.pc = 0x52e5;

    // ld [wGainBoostedExp], a
    cpu.write_byte(wram::W_GAIN_BOOSTED_EXP, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wIsInBattle]
    cpu.a = cpu.read_byte(wram::W_IS_IN_BATTLE);
    cpu.pc += 3;
    cpu.cycle(16);

    // is it a trainer battle?
    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // if so, boost exp
    // call nz, BoostExp
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54ae); // BoostExp
        cpu.pc = pc;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // add the gained exp to the party mon's exp
    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ldh a, [hQuotient + 3]
    cpu.a = cpu.read_byte(hram::H_QUOTIENT + 3);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [wExpAmountGained + 1], a
    cpu.write_byte(wram::W_EXP_AMOUNT_GAINED + 1, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // add b
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.b & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.b as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.b);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ldh a, [hQuotient + 2]
    cpu.a = cpu.read_byte(hram::H_QUOTIENT + 2);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [wExpAmountGained], a
    cpu.write_byte(wram::W_EXP_AMOUNT_GAINED, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // adc b
    {
        let carry = if cpu.flag(CpuFlag::C) { 1 } else { 0 };
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.b & 0x0f) + carry > 0x0f);
        cpu.set_flag(
            CpuFlag::C,
            (cpu.a as u16) + (cpu.b as u16) + (carry as u16) > 0xff,
        );
        cpu.a = cpu.a.wrapping_add(cpu.b).wrapping_add(carry);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);
    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // jr nc, .noCarry
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return gain_experience_no_carry(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // dec hl
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc [hl]
    {
        let addr = cpu.hl();
        let value = cpu.read_byte(addr);
        cpu.write_byte(addr, value.wrapping_add(1));
        cpu.set_flag(CpuFlag::Z, value == 0xff);
        cpu.set_flag(CpuFlag::H, (value & 0x0f) == 0x0f);
        cpu.set_flag(CpuFlag::N, false);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    gain_experience_no_carry(cpu);
}

fn gain_experience_no_carry(cpu: &mut Cpu) {
    cpu.pc = 0x5307;

    // calculate exp for the mon at max level, and cap the exp at that value
    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, 0
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, wPartySpecies
    cpu.set_hl(wram::W_PARTY_SPECIES);
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wCurSpecies], a
    cpu.write_byte(wram::W_CUR_SPECIES, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // call GetMonHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x132f); // GetMonHeader
        cpu.pc = pc;
    }

    // ld d, MAX_LEVEL
    cpu.d = MAX_LEVEL;
    cpu.pc += 2;
    cpu.cycle(8);

    // get max exp
    // callfar CalcExperience
    macros::farcall::callfar(cpu, 0x16, 0x4dc0);

    // compare max exp with current exp
    // ldh a, [hExperience]
    cpu.a = cpu.read_byte(hram::H_EXPERIENCE);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh a, [hExperience + 1]
    cpu.a = cpu.read_byte(hram::H_EXPERIENCE + 1);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh a, [hExperience + 2]
    cpu.a = cpu.read_byte(hram::H_EXPERIENCE + 2);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld d, a
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, [hld]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // sub a, d
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.d & 0x0f));
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.d);
    cpu.a = cpu.a.wrapping_sub(cpu.d);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hld]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // sbc c
    {
        let carry = if cpu.flag(CpuFlag::C) { 1 } else { 0 };
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.c & 0x0f) + carry);
        cpu.set_flag(CpuFlag::C, (cpu.a as u16) < (cpu.c as u16) + (carry as u16));
        cpu.a = cpu.a.wrapping_sub(cpu.c).wrapping_sub(carry);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // sbc b
    {
        let carry = if cpu.flag(CpuFlag::C) { 1 } else { 0 };
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f) + carry);
        cpu.set_flag(CpuFlag::C, (cpu.a as u16) < (cpu.b as u16) + (carry as u16));
        cpu.a = cpu.a.wrapping_sub(cpu.b).wrapping_sub(carry);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr c, .next2
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return gain_experience_next2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // the mon's exp is greater than the max exp, so overwrite it with the max exp
    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, d
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // dec hl
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    gain_experience_next2(cpu);
}

fn gain_experience_next2(cpu: &mut Cpu) {
    cpu.pc = 0x533d;

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld hl, wPartyMonNicks
    cpu.set_hl(wram::W_PARTY_MON_NICKS);
    cpu.pc += 3;
    cpu.cycle(12);

    // call GetPartyMonName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x139a); // GetPartyMonName
        cpu.pc = pc;
    }

    // ld hl, GainedText
    cpu.set_hl(0x54c6);
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3c36); // PrintText
        cpu.pc = pc;
    }

    // PLAYER_PARTY_DATA
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMonDataLocation], a
    cpu.write_byte(wram::W_MON_DATA_LOCATION, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // call LoadMonData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1132); // LoadMonData
        cpu.pc = pc;
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld bc, wPartyMon1Level - wPartyMon1Exp
    cpu.set_bc(wram::W_PARTY_MON1_LEVEL - wram::W_PARTY_MON1_EXP);
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // farcall CalcLevelFromExperience
    macros::farcall::farcall(cpu, 0x16, 0x4d99);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // current level
    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // cp d
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.d);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.d & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.d);
    cpu.pc += 1;
    cpu.cycle(4);

    // if level didn't change, go to next mon
    // jp z, .nextMon
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return gain_experience_next_mon(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld a, [wCurEnemyLevel]
    cpu.a = cpu.read_byte(wram::W_CUR_ENEMY_LEVEL);
    cpu.pc += 3;
    cpu.cycle(16);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, d
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wCurEnemyLevel], a
    cpu.write_byte(wram::W_CUR_ENEMY_LEVEL, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld bc, wPartyMon1Species - wPartyMon1Level
    cpu.set_bc(wram::W_PARTY_MON1_SPECIES.wrapping_sub(wram::W_PARTY_MON1_LEVEL));
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [wCurSpecies], a
    cpu.write_byte(wram::W_CUR_SPECIES, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wPokedexNum], a
    cpu.write_byte(wram::W_POKEDEX_NUM, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // call GetMonHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x132f); // GetMonHeader
        cpu.pc = pc;
    }

    // ld bc, (wPartyMon1MaxHP + 1) - wPartyMon1Species
    cpu.set_bc((wram::W_PARTY_MON1_MAX_HP + 1) - wram::W_PARTY_MON1_SPECIES);
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [hld]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // push max HP (from before levelling up)
    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld bc, (wPartyMon1HPExp - 1) - wPartyMon1MaxHP
    cpu.set_bc((wram::W_PARTY_MON1_HP_EXP - 1).wrapping_sub(wram::W_PARTY_MON1_MAX_HP));
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // consider stat exp when calculating stats
    // ld b, $1
    cpu.b = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // call CalcStats
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x392b); // CalcStats
        cpu.pc = pc;
    }

    // pop max HP (from before levelling up)
    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, [hld]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // sub a, c
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.c & 0x0f));
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.c);
    cpu.a = cpu.a.wrapping_sub(cpu.c);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // sbc b
    {
        let carry = if cpu.flag(CpuFlag::C) { 1 } else { 0 };
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f) + carry);
        cpu.set_flag(CpuFlag::C, (cpu.a as u16) < (cpu.b as u16) + (carry as u16));
        cpu.a = cpu.a.wrapping_sub(cpu.b).wrapping_sub(carry);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // bc = difference between old max HP and new max HP after levelling
    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld de, (wPartyMon1HP + 1) - wPartyMon1MaxHP
    let e = cpu.read_byte(cpu.pc + 1);
    let d = cpu.read_byte(cpu.pc + 2);
    cpu.set_de((wram::W_PARTY_MON1_HP + 1).wrapping_sub(wram::W_PARTY_MON1_MAX_HP));
    assert_eq!(cpu.d, d);
    assert_eq!(cpu.e, e);
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // add to the current HP the amount of max HP gained when levelling
    // wPartyMon1HP + 1
    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // add c
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.c & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.c as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.c);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // wPartyMon1HP + 1
    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // adc b
    {
        let carry = if cpu.flag(CpuFlag::C) { 1 } else { 0 };
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.b & 0x0f) + carry > 0x0f);
        cpu.set_flag(
            CpuFlag::C,
            (cpu.a as u16) + (cpu.b as u16) + (carry as u16) > 0xff,
        );
        cpu.a = cpu.a.wrapping_add(cpu.b).wrapping_add(carry);
    }
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);
    // wPartyMon1HP
    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wPlayerMonNumber]
    cpu.a = cpu.read_byte(wram::W_PLAYER_MON_NUMBER);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // is the current mon in battle?
    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .printGrewLevelText
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_print_grew_level_text(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // current mon is in battle
    // ld de, wBattleMonHP
    cpu.set_de(wram::W_BATTLE_MON_HP);
    cpu.pc += 3;
    cpu.cycle(12);

    // copy party mon HP to battle mon HP
    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // copy other stats from party mon to battle mon
    // ld bc, wPartyMon1Level - (wPartyMon1HP + 1)
    cpu.set_bc(wram::W_PARTY_MON1_LEVEL - (wram::W_PARTY_MON1_HP + 1));
    cpu.pc += 3;
    cpu.cycle(12);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld de, wBattleMonLevel
    cpu.set_de(wram::W_BATTLE_MON_LEVEL);
    cpu.pc += 3;
    cpu.cycle(12);

    // size of stats
    // ld bc, 1 + NUM_STATS * 2
    cpu.set_bc(1 + (NUM_STATS as u16) * 2);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x00b1); // CopyData
        cpu.pc = pc;
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, [wPlayerBattleStatus3]
    cpu.a = cpu.read_byte(wram::W_PLAYER_BATTLE_STATUS3);
    cpu.pc += 3;
    cpu.cycle(16);

    // bit TRANSFORMED, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << TRANSFORMED)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .recalcStatChanges
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_recalc_stat_changes(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // the mon is not transformed, so update the unmodified stats
    // ld de, wPlayerMonUnmodifiedLevel
    cpu.set_de(wram::W_PLAYER_MON_UNMODIFIED_LEVEL);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, 1 + NUM_STATS * 2
    cpu.set_bc(1 + (NUM_STATS as u16) * 2);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x00b1); // CopyData
        cpu.pc = pc;
    }

    gain_experience_recalc_stat_changes(cpu);
}

fn gain_experience_recalc_stat_changes(cpu: &mut Cpu) {
    cpu.pc = 0x53d7;

    // battle mon
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wCalculateWhoseStats], a
    cpu.write_byte(wram::W_CALCULATE_WHOSE_STATS, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld hl, CalculateModifiedStats
    cpu.set_hl(0x6f25); // CalculateModifiedStats
    cpu.pc += 3;
    cpu.cycle(12);

    // call CallBattleCore
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54c1); // CallBattleCore
        cpu.pc = pc;
    }

    // ld hl, ApplyBurnAndParalysisPenaltiesToPlayer
    cpu.set_hl(0x6ea6); // ApplyBurnAndParalysisPenaltiesToPlayer
    cpu.pc += 3;
    cpu.cycle(12);

    // call CallBattleCore
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54c1); // CallBattleCore
        cpu.pc = pc;
    }

    // ld hl, ApplyBadgeStatBoosts
    cpu.set_hl(0x6fa5); // ApplyBadgeStatBoosts
    cpu.pc += 3;
    cpu.cycle(12);

    // call CallBattleCore
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54c1); // CallBattleCore
        cpu.pc = pc;
    }

    // ld hl, DrawPlayerHUDAndHPBar
    cpu.set_hl(0x4e25); // DrawPlayerHUDAndHPBar
    cpu.pc += 3;
    cpu.cycle(12);

    // call CallBattleCore
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54c1); // CallBattleCore
        cpu.pc = pc;
    }

    // ld hl, PrintEmptyString
    cpu.set_hl(0x7020); // PrintEmptyString
    cpu.pc += 3;
    cpu.cycle(12);

    // call CallBattleCore
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x54c1); // CallBattleCore
        cpu.pc = pc;
    }

    // call SaveScreenTilesToBuffer1
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x370f); // SaveScreenTilesToBuffer1
        cpu.pc = pc;
    }

    gain_experience_print_grew_level_text(cpu);
}

fn gain_experience_print_grew_level_text(cpu: &mut Cpu) {
    cpu.pc = 0x53fc;

    // callabd_ModifyPikachuHappiness PIKAHAPPY_LEVELUP
    macros::farcall::callabd_modify_pikachu_happiness(cpu, PIKAHAPPY_LEVELUP);

    // ld hl, GrewLevelText
    cpu.set_hl(0x54f1); // GrewLevelText
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3c36); // PrintText
        cpu.pc = pc;
    }

    // PLAYER_PARTY_DATA
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMonDataLocation], a
    cpu.write_byte(wram::W_MON_DATA_LOCATION, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // call LoadMonData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1132); // LoadMonData
        cpu.pc = pc;
    }

    // ld d, $1
    cpu.d = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // callfar PrintStatsBox
    macros::farcall::callfar(cpu, 0x04, 0x568a);

    // call WaitForTextScrollButtonPress
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3852); // WaitForTextScrollButtonPress
        cpu.pc = pc;
    }

    // call LoadScreenTilesFromBuffer1
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x371b); // LoadScreenTilesFromBuffer1
        cpu.pc = pc;
    }

    // PLAYER_PARTY_DATA
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMonDataLocation], a
    cpu.write_byte(wram::W_MON_DATA_LOCATION, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wCurSpecies]
    cpu.a = cpu.read_byte(wram::W_CUR_SPECIES);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wPokedexNum], a
    cpu.write_byte(wram::W_POKEDEX_NUM, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // predef LearnMoveFromLevelUp
    macros::predef::predef_call!(cpu, LearnMoveFromLevelUp);

    // ld hl, wCanEvolveFlags
    cpu.set_hl(wram::W_CAN_EVOLVE_FLAGS);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, FLAG_SET
    cpu.b = FLAG_SET;
    cpu.pc += 2;
    cpu.cycle(8);

    // predef FlagActionPredef
    macros::predef::predef_call!(cpu, FlagActionPredef);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wCurEnemyLevel], a
    cpu.write_byte(wram::W_CUR_ENEMY_LEVEL, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    gain_experience_next_mon(cpu);
}

fn gain_experience_next_mon(cpu: &mut Cpu) {
    cpu.pc = 0x5445;

    // ld a, [wPartyCount]
    cpu.a = cpu.borrow_wram().party().len() as u8;
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wWhichPokemon]
    cpu.a = cpu.read_byte(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(16);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return gain_experience_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld [wWhichPokemon], a
    cpu.write_byte(wram::W_WHICH_POKEMON, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld bc, wPartyMon2 - wPartyMon1
    cpu.set_bc(wram::W_PARTY_MON2 - wram::W_PARTY_MON1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld hl, wPartyMon1
    cpu.set_hl(wram::W_PARTY_MON1);
    cpu.pc += 3;
    cpu.cycle(12);

    // call AddNTimes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3a74); // AddNTimes
        cpu.pc = pc;
    }

    // jp .partyMonLoop
    cpu.cycle(16);
    gain_experience_party_mon_loop(cpu)
}

fn gain_experience_done(cpu: &mut Cpu) {
    cpu.pc = 0x545f;

    // ld hl, wPartyGainExpFlags
    cpu.set_hl(wram::W_PARTY_GAIN_EXP_FLAGS);
    cpu.pc += 3;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // clear gain exp flags
    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wPlayerMonNumber]
    cpu.a = cpu.read_byte(wram::W_PLAYER_MON_NUMBER);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, FLAG_SET
    cpu.b = FLAG_SET;
    cpu.pc += 2;
    cpu.cycle(8);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // set the gain exp flag for the mon that is currently out
    // predef FlagActionPredef
    macros::predef::predef_call!(cpu, FlagActionPredef);

    // ld hl, wPartyFoughtCurrentEnemyFlags
    cpu.set_hl(wram::W_PARTY_FOUGHT_CURRENT_ENEMY_FLAGS);
    cpu.pc += 3;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // set the fought current enemy flag for the mon that is currently out
    // predef_jump FlagActionPredef
    macros::predef::predef_call!(cpu, FlagActionPredef);
    cpu.pc = cpu.stack_pop();
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
