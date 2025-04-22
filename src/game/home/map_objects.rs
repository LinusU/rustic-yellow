use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{move_constants::MoveId, pokemon_data_constants::PARTY_LENGTH},
        macros,
        ram::wram,
    },
    PokemonSpecies,
};

/// given an item_id in b
/// set zero flag if item isn't in player's bag
/// else reset zero flag
/// related to Pokémon Tower and ghosts
pub fn is_item_in_bag(cpu: &mut Cpu) {
    let item_id = cpu.b;

    macros::predef::predef_call!(cpu, GetQuantityOfItemInBag);
    log::debug!("is_item_in_bag({:#04x}) == {}", item_id, cpu.b != 0);

    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.pc = cpu.stack_pop(); // ret
}

/// set bit 6 of wd472 if true \
/// also calls Func_3467, which is a bankswitch to IsStarterPikachuInOurParty
pub fn is_surfing_pikachu_in_party(cpu: &mut Cpu) {
    cpu.pc = 0x342a;

    // ld a, [wd472]
    cpu.a = cpu.read_byte(wram::W_D472);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, $3f
    cpu.a &= 0x3f;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wd472], a
    cpu.write_byte(wram::W_D472, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld hl, wPartyMon1
    cpu.set_hl(wram::W_PARTY_MON1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld c, PARTY_LENGTH
    cpu.c = PARTY_LENGTH;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld b, SURF
    cpu.b = MoveId::Surf as u8;
    cpu.pc += 2;
    cpu.cycle(8);

    is_surfing_pikachu_in_party_loop(cpu);
}

fn is_surfing_pikachu_in_party_loop(cpu: &mut Cpu) {
    cpu.pc = 0x3439;

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // cp STARTER_PIKACHU
    cpu.set_flag(CpuFlag::Z, cpu.a == PokemonSpecies::Pikachu.into_index());
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (PokemonSpecies::Pikachu.into_index() & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < PokemonSpecies::Pikachu.into_index());
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .notPikachu
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_not_pikachu(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld de, $8
    cpu.set_de(0x8);
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

    // does pikachu have surf as one of its moves
    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .hasSurf
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_has_surf(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .hasSurf
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_has_surf(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .hasSurf
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_has_surf(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .noSurf
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_no_surf(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    is_surfing_pikachu_in_party_has_surf(cpu);
}

fn is_surfing_pikachu_in_party_has_surf(cpu: &mut Cpu) {
    cpu.pc = 0x3453;

    // ld a, [wd472]
    cpu.a = cpu.read_byte(wram::W_D472);
    cpu.pc += 3;
    cpu.cycle(16);

    // set 6, a
    cpu.a |= 1 << 6;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wd472], a
    cpu.write_byte(wram::W_D472, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    is_surfing_pikachu_in_party_no_surf(cpu);
}

fn is_surfing_pikachu_in_party_no_surf(cpu: &mut Cpu) {
    cpu.pc = 0x345b;

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    is_surfing_pikachu_in_party_not_pikachu(cpu);
}

fn is_surfing_pikachu_in_party_not_pikachu(cpu: &mut Cpu) {
    cpu.pc = 0x345c;

    // ld de, wPartyMon2 - wPartyMon1
    cpu.set_de(wram::W_PARTY_MON2 - wram::W_PARTY_MON1);
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

    // dec c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x00);
    cpu.c = cpu.c.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return is_surfing_pikachu_in_party_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call Func_3467
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.stack_push(pc);
        cpu.cycle(24);
        func_3467(cpu);
        cpu.pc = pc;
    }

    log::trace!(
        "is_surfing_pikachu_in_party() == {}",
        cpu.read_byte(wram::W_D472) & (1 << 6)
    );

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

pub fn func_3467(cpu: &mut Cpu) {
    cpu.pc = 0x3467;

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // callfar IsStarterPikachuInOurParty
    macros::farcall::callfar(cpu, 0x3f, 0x4db8);

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

    // ret nc
    if !cpu.flag(CpuFlag::C) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [wd472]
    cpu.a = cpu.read_byte(wram::W_D472);
    cpu.pc += 3;
    cpu.cycle(16);

    // set 7, a
    cpu.a |= 1 << 7;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wd472], a
    cpu.write_byte(wram::W_D472, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
