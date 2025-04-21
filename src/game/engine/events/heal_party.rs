use crate::{
    cpu::{Cpu, CpuFlag},
    game::{constants::battle_constants, ram::wram},
};

/// Restore HP and PP
pub fn heal_party(cpu: &mut Cpu) {
    log::info!("heal_party()");

    cpu.pc = 0x752b;

    // ld hl, wPartySpecies
    cpu.set_hl(0xd163);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wPartyMon1HP
    cpu.set_de(0xd16b);
    cpu.pc += 3;
    cpu.cycle(12);

    heal_party_healmon(cpu);
}

fn heal_party_healmon(cpu: &mut Cpu) {
    cpu.pc = 0x7531;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // cp $ff
    cpu.set_flag(CpuFlag::Z, cpu.a == 0xff);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0xff & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0xff);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return heal_party_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, wPartyMon1Status - wPartyMon1HP
    cpu.set_hl(0xd16e - 0xd16b);
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

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // A Pokémon has 4 moves
    // ld b, NUM_MOVES
    cpu.b = battle_constants::NUM_MOVES;
    cpu.pc += 2;
    cpu.cycle(8);

    heal_party_pp(cpu);
}

fn heal_party_pp(cpu: &mut Cpu) {
    cpu.pc = 0x7541;

    // ld hl, wPartyMon1Moves - wPartyMon1HP
    cpu.set_hl(0xd172 - 0xd16b);
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

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .nextmove
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return heal_party_nextmove(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, wPartyMon1PP - wPartyMon1HP
    cpu.set_hl(0xd187 - 0xd16b);
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

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, Moves
    cpu.set_hl(0x4000);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, MOVE_LENGTH
    cpu.set_bc(battle_constants::MOVE_LENGTH as u16);
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

    // ld de, wcd6d
    cpu.set_de(0xcd6d);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, BANK(Moves)
    cpu.a = 0x0e;
    cpu.pc += 2;
    cpu.cycle(8);

    // call FarCopyData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x009d); // FarCopyData
        cpu.pc = pc;
    }

    // PP is byte 5 of move data
    // ld a, [wcd6d + 5]
    cpu.a = cpu.read_byte(0xcd6d + 5);
    cpu.pc += 3;
    cpu.cycle(16);

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
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

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // and a, $c0
    cpu.a &= 0xc0;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // add b
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.b & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.b as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.b);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
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

    heal_party_nextmove(cpu);
}

fn heal_party_nextmove(cpu: &mut Cpu) {
    cpu.pc = 0x7571;

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .pp
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return heal_party_pp(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld hl, wPartyMon1MaxHP - wPartyMon1HP
    cpu.set_hl(0xd18c - 0xd16b);
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

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
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

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld bc, wPartyMon2 - wPartyMon1
    cpu.set_bc(0xd196 - 0xd16a);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld h, d
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld l, e
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

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

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // jr .healmon
    cpu.cycle(12);
    heal_party_healmon(cpu)
}

fn heal_party_done(cpu: &mut Cpu) {
    cpu.pc = 0x758c;

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

    // ld [wd11e], a
    cpu.write_byte(wram::W_D11E, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wPartyCount]
    cpu.a = cpu.borrow_wram().party().len() as u8;
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    heal_party_ppup(cpu);
}

fn heal_party_ppup(cpu: &mut Cpu) {
    cpu.pc = 0x7597;

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // call RestoreBonusPP
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x654a); // RestoreBonusPP
        cpu.pc = pc;
    }

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld hl, wWhichPokemon
    cpu.set_hl(wram::W_WHICH_POKEMON);
    cpu.pc += 3;
    cpu.cycle(12);

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

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .ppup
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return heal_party_ppup(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
