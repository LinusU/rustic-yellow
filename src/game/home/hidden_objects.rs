use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::input_constants::BIT_A_BUTTON,
        macros,
        ram::{hram, wram},
    },
};

pub fn check_for_hidden_object_or_bookshelf_or_card_key_door(cpu: &mut Cpu) {
    log::info!("check_for_hidden_object_or_bookshelf_or_card_key_door");

    cpu.pc = 0x3ef9;

    // ldh a, [hLoadedROMBank]
    cpu.a = cpu.borrow_wram().loaded_rom_bank();
    cpu.pc += 2;
    cpu.cycle(12);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ldh a, [hJoyHeld]
    cpu.a = cpu.read_byte(hram::H_JOY_HELD);
    cpu.pc += 2;
    cpu.cycle(12);

    // bit BIT_A_BUTTON, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << BIT_A_BUTTON)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .nothingFound
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return check_for_hidden_object_or_bookshelf_or_card_key_door_nothing_found(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // A button is pressed
    // ld a, BANK(CheckForHiddenObject)
    cpu.a = 0x3c;
    cpu.pc += 2;
    cpu.cycle(8);

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // call CheckForHiddenObject
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x65f8); // CheckForHiddenObject
        cpu.pc = pc;
    }

    // ldh a, [hDidntFindAnyHiddenObject]
    cpu.a = cpu.read_byte(hram::H_DIDNT_FIND_ANY_HIDDEN_OBJECT);
    cpu.pc += 2;
    cpu.cycle(12);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .hiddenObjectNotFound
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return check_for_hidden_object_or_bookshelf_or_card_key_door_hidden_object_not_found(cpu);
    } else {
        cpu.pc += 2;
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

    // ldh [hItemAlreadyFound], a
    cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [wHiddenObjectFunctionRomBank]
    cpu.a = cpu.read_byte(wram::W_HIDDEN_OBJECT_FUNCTION_ROM_BANK);
    cpu.pc += 3;
    cpu.cycle(16);

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // call JumpToAddress
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e98); // JumpToAddress
        cpu.pc = pc;
    }

    // ldh a, [hItemAlreadyFound]
    cpu.a = cpu.read_byte(hram::H_ITEM_ALREADY_FOUND);
    cpu.pc += 2;
    cpu.cycle(12);

    // jr .done
    cpu.cycle(12);
    check_for_hidden_object_or_bookshelf_or_card_key_door_done(cpu)
}

fn check_for_hidden_object_or_bookshelf_or_card_key_door_hidden_object_not_found(cpu: &mut Cpu) {
    log::info!("check_for_hidden_object_or_bookshelf_or_card_key_door_hidden_object_not_found");

    cpu.pc = 0x3f1f;

    // predef GetTileAndCoordsInFrontOfPlayer
    macros::predef::predef_call!(cpu, GetTileAndCoordsInFrontOfPlayer);

    // farcall PrintBookshelfText
    macros::farcall::farcall(cpu, 0x03, 0x79de);

    // ldh a, [hInteractedWithBookshelf]
    cpu.a = cpu.read_byte(hram::H_INTERACTED_WITH_BOOKSHELF);
    cpu.pc += 2;
    cpu.cycle(12);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return check_for_hidden_object_or_bookshelf_or_card_key_door_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    check_for_hidden_object_or_bookshelf_or_card_key_door_nothing_found(cpu)
}

fn check_for_hidden_object_or_bookshelf_or_card_key_door_nothing_found(cpu: &mut Cpu) {
    log::info!("check_for_hidden_object_or_bookshelf_or_card_key_door_nothing_found");

    cpu.pc = 0x3f31;

    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    check_for_hidden_object_or_bookshelf_or_card_key_door_done(cpu)
}

fn check_for_hidden_object_or_bookshelf_or_card_key_door_done(cpu: &mut Cpu) {
    log::info!("check_for_hidden_object_or_bookshelf_or_card_key_door_done");

    cpu.pc = 0x3f33;

    // ldh [hItemAlreadyFound], a
    cpu.write_byte(hram::H_ITEM_ALREADY_FOUND, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // call BankswitchCommon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3e7e); // BankswitchCommon
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
