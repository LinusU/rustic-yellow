use crate::cpu::Cpu;

/// This function is used to wait a short period after printing a letter to the
/// screen unless the player presses the A/B button or the delay is turned off
/// through the [`W_D730`](crate::game::ram::wram::W_D730) or
/// [`W_LETTER_PRINTING_DELAY_FLAGS`](crate::game::ram::wram::W_LETTER_PRINTING_DELAY_FLAGS) flags.
pub fn print_letter_delay(cpu: &mut Cpu) {
    cpu.call(0x38c8); // PrintLetterDelay
}
