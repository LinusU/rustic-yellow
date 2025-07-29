use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            event_constants::EVENT_GOT_POKEDEX,
            hardware_constants::{
                self, START_TRANSFER_EXTERNAL_CLOCK, START_TRANSFER_INTERNAL_CLOCK,
            },
            music_constants::SFX_SAVE,
            ram_constants::BIT_LINK_CONNECTED,
            serial_constants::{
                CONNECTION_NOT_ESTABLISHED, ESTABLISH_CONNECTION_WITH_EXTERNAL_CLOCK,
                ESTABLISH_CONNECTION_WITH_INTERNAL_CLOCK, USING_EXTERNAL_CLOCK,
                USING_INTERNAL_CLOCK,
            },
        },
        macros,
        ram::{hram, wram},
    },
};

pub fn cable_club_npc(cpu: &mut Cpu) {
    cpu.pc = 0x7035;

    // ld hl, CableClubNPCWelcomeText
    cpu.set_hl(0x718d); // CableClubNPCWelcomeText
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

    // call CheckPikachuFollowingPlayer
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x154a); // CheckPikachuFollowingPlayer
        cpu.pc = pc;
    }

    // jr nz, .asm_7048
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_asm_7048(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // CheckEvent EVENT_GOT_POKEDEX
    macros::scripts::events::check_event(cpu, EVENT_GOT_POKEDEX);

    // jp nz, .receivedPokedex
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        cable_club_npc_received_pokedex(cpu);
        return;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // if the player hasn't received the pokedex
    cable_club_npc_asm_7048(cpu);
}

fn cable_club_npc_asm_7048(cpu: &mut Cpu) {
    cpu.pc = 0x7048;

    // ld c, 60
    cpu.c = 60;
    cpu.pc += 2;
    cpu.cycle(8);

    // call DelayFrames
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x372f); // DelayFrames
        cpu.pc = pc;
    }

    // ld hl, CableClubNPCMakingPreparationsText
    cpu.set_hl(0x71a7); // CableClubNPCMakingPreparationsText
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

    // jp .didNotConnect
    cpu.cycle(16);
    cable_club_npc_did_not_connect(cpu);
}

fn cable_club_npc_received_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x7056;

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wMenuJoypadPollCount], a
    cpu.borrow_wram_mut().set_menu_joypad_poll_count(1);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, 90
    cpu.a = 90;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wLinkTimeoutCounter], a
    let link_timeout_counter = cpu.a;
    cpu.borrow_wram_mut()
        .set_link_timeout_counter(link_timeout_counter);
    cpu.pc += 3;
    cpu.cycle(16);

    cable_club_npc_establish_connection_loop(cpu);
}

fn cable_club_npc_establish_connection_loop(cpu: &mut Cpu) {
    cpu.pc = 0x7060;

    // ldh a, [hSerialConnectionStatus]
    cpu.a = cpu.read_byte(hram::H_SERIAL_CONNECTION_STATUS);
    cpu.pc += 2;
    cpu.cycle(12);

    // cp USING_INTERNAL_CLOCK
    cpu.set_flag(CpuFlag::Z, cpu.a == USING_INTERNAL_CLOCK);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (USING_INTERNAL_CLOCK & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < USING_INTERNAL_CLOCK);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .establishedConnection
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_established_connection(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp USING_EXTERNAL_CLOCK
    cpu.set_flag(CpuFlag::Z, cpu.a == USING_EXTERNAL_CLOCK);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (USING_EXTERNAL_CLOCK & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < USING_EXTERNAL_CLOCK);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .establishedConnection
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_established_connection(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, CONNECTION_NOT_ESTABLISHED
    cpu.a = CONNECTION_NOT_ESTABLISHED;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hSerialConnectionStatus], a
    cpu.write_byte(hram::H_SERIAL_CONNECTION_STATUS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, ESTABLISH_CONNECTION_WITH_EXTERNAL_CLOCK
    cpu.a = ESTABLISH_CONNECTION_WITH_EXTERNAL_CLOCK;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hSerialReceiveData], a
    cpu.write_byte(hram::H_SERIAL_RECEIVE_DATA, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, START_TRANSFER_EXTERNAL_CLOCK
    cpu.a = START_TRANSFER_EXTERNAL_CLOCK;
    cpu.pc += 2;
    cpu.cycle(8);

    // This vc_hook causes the Virtual Console to set [hSerialConnectionStatus] to
    // USING_INTERNAL_CLOCK, which allows the player to proceed past the link
    // receptionist's "Please wait." It assumes that hSerialConnectionStatus is at
    // its original address.
    // vc_hook Link_fake_connection_status
    // vc_assert hSerialConnectionStatus == $ffaa, "hSerialConnectionStatus is no longer located at 00:ffaa"
    // vc_assert USING_INTERNAL_CLOCK == $02, "USING_INTERNAL_CLOCK is no longer equal to $02."
    cpu.write_byte(0xffaa, 0x02);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [wLinkTimeoutCounter]
    cpu.a = cpu.borrow_wram().link_timeout_counter();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wLinkTimeoutCounter], a
    let link_timeout_counter = cpu.a;
    cpu.borrow_wram_mut()
        .set_link_timeout_counter(link_timeout_counter);
    cpu.pc += 3;
    cpu.cycle(16);

    // jr z, .failedToEstablishConnection
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_failed_to_establish_connection(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, ESTABLISH_CONNECTION_WITH_INTERNAL_CLOCK
    cpu.a = ESTABLISH_CONNECTION_WITH_INTERNAL_CLOCK;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, START_TRANSFER_INTERNAL_CLOCK
    cpu.a = START_TRANSFER_INTERNAL_CLOCK;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1e64); // DelayFrame
        cpu.pc = pc;
    }

    // jr .establishConnectionLoop
    cpu.cycle(12);
    cable_club_npc_establish_connection_loop(cpu)
}

fn cable_club_npc_established_connection(cpu: &mut Cpu) {
    cpu.pc = 0x708f;

    // call Serial_SendZeroByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2149); // Serial_SendZeroByte
        cpu.pc = pc;
    }

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1e64); // DelayFrame
        cpu.pc = pc;
    }

    // call Serial_SendZeroByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2149); // Serial_SendZeroByte
        cpu.pc = pc;
    }

    // ld c, 50
    cpu.c = 50;
    cpu.pc += 2;
    cpu.cycle(8);

    // call DelayFrames
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x372f); // DelayFrames
        cpu.pc = pc;
    }

    // ld hl, CableClubNPCPleaseApplyHereHaveToSaveText
    cpu.set_hl(0x7192); // CableClubNPCPleaseApplyHereHaveToSaveText
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

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMenuJoypadPollCount], a
    let menu_joypad_poll_count = cpu.a;
    cpu.borrow_wram_mut()
        .set_menu_joypad_poll_count(menu_joypad_poll_count);
    cpu.pc += 3;
    cpu.cycle(16);

    // call YesNoChoice
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x35ef); // YesNoChoice
        cpu.pc = pc;
    }

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wMenuJoypadPollCount], a
    let menu_joypad_poll_count = cpu.a;
    cpu.borrow_wram_mut()
        .set_menu_joypad_poll_count(menu_joypad_poll_count);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wCurrentMenuItem]
    cpu.a = cpu.borrow_wram().current_menu_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .choseNo
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_chose_no(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // vc_hook Wireless_TryQuickSave_block_input

    // callfar SaveSAVtoSRAM
    macros::farcall::callfar(cpu, 0x1c, 0x7b91);

    // call WaitForSoundToFinish
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x373e); // WaitForSoundToFinish
        cpu.pc = pc;
    }

    // ld a, SFX_SAVE
    cpu.a = SFX_SAVE;
    cpu.pc += 2;
    cpu.cycle(8);

    // call PlaySoundWaitForCurrent
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3736); // PlaySoundWaitForCurrent
        cpu.pc = pc;
    }

    // ld hl, CableClubNPCPleaseWaitText
    cpu.set_hl(0x7197); // CableClubNPCPleaseWaitText
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

    // ld hl, wUnknownSerialCounter
    cpu.set_hl(wram::W_UNKNOWN_SERIAL_COUNTER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, $3
    cpu.a = 0x3;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

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

    // ldh [hSerialReceivedNewData], a
    cpu.write_byte(hram::H_SERIAL_RECEIVED_NEW_DATA, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [wSerialExchangeNybbleSendData], a
    let serial_exchange_nybble_send_data = cpu.a;
    cpu.borrow_wram_mut()
        .set_serial_exchange_nybble_send_data(serial_exchange_nybble_send_data);
    cpu.pc += 3;
    cpu.cycle(16);

    // vc_hook Wireless_prompt

    // call Serial_SyncAndExchangeNybble
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x20db); // Serial_SyncAndExchangeNybble
        cpu.pc = pc;
    }

    // vc_hook Wireless_net_recheck

    // ld hl, wUnknownSerialCounter
    cpu.set_hl(wram::W_UNKNOWN_SERIAL_COUNTER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .connected
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_connected(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .connected
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_connected(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld b, 10
    cpu.b = 10;
    cpu.pc += 2;
    cpu.cycle(8);

    cable_club_npc_sync_loop(cpu);
}

fn cable_club_npc_sync_loop(cpu: &mut Cpu) {
    cpu.pc = 0x70e8;

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1e64); // DelayFrame
        cpu.pc = pc;
    }

    // call Serial_SendZeroByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2149); // Serial_SendZeroByte
        cpu.pc = pc;
    }

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .syncLoop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return cable_club_npc_sync_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call CloseLinkConnection
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x71ac); // CloseLinkConnection
        cpu.pc = pc;
    }

    // ld hl, CableClubNPCLinkClosedBecauseOfInactivityText
    cpu.set_hl(0x719d); // CableClubNPCLinkClosedBecauseOfInactivityText
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

    // jr .didNotConnect
    cpu.cycle(12);
    cable_club_npc_did_not_connect(cpu)
}

fn cable_club_npc_failed_to_establish_connection(cpu: &mut Cpu) {
    cpu.pc = 0x70fc;

    // ld hl, CableClubNPCAreaReservedFor2FriendsLinkedByCableText
    cpu.set_hl(0x7188); // CableClubNPCAreaReservedFor2FriendsLinkedByCableText
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

    // jr .didNotConnect
    cpu.cycle(12);
    cable_club_npc_did_not_connect(cpu)
}

fn cable_club_npc_chose_no(cpu: &mut Cpu) {
    cpu.pc = 0x7104;

    // call CloseLinkConnection
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x71ac); // CloseLinkConnection
        cpu.pc = pc;
    }

    // ld hl, CableClubNPCPleaseComeAgainText
    cpu.set_hl(0x71a2); // CableClubNPCPleaseComeAgainText
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

    cable_club_npc_did_not_connect(cpu);
}

fn cable_club_npc_did_not_connect(cpu: &mut Cpu) {
    cpu.pc = 0x710d;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, wUnknownSerialCounter
    cpu.set_hl(wram::W_UNKNOWN_SERIAL_COUNTER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld hl, wStatusFlags4
    cpu.set_hl(wram::W_STATUS_FLAGS_4);
    cpu.pc += 3;
    cpu.cycle(12);

    // res BIT_LINK_CONNECTED, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << BIT_LINK_CONNECTED));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMenuJoypadPollCount], a
    let menu_joypad_poll_count = cpu.a;
    cpu.borrow_wram_mut()
        .set_menu_joypad_poll_count(menu_joypad_poll_count);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn cable_club_npc_connected(cpu: &mut Cpu) {
    cpu.pc = 0x711d;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wLetterPrintingDelayFlags]
    cpu.a = cpu.borrow_wram().letter_printing_delay_flags();
    cpu.pc += 3;
    cpu.cycle(16);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // callfar LinkMenu
    macros::farcall::callfar(cpu, 0x3d, 0x580c);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wLetterPrintingDelayFlags], a
    let letter_printing_delay_flags = cpu.a;
    cpu.borrow_wram_mut()
        .set_letter_printing_delay_flags(letter_printing_delay_flags);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
