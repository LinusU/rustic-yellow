use crate::{cpu::Cpu, game::ram::hram};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoypadLowSensitivityMode {
    GetNewlyPressedButtonsOnly,
    GetCurrentlyPressedButtonsAtLowSampleRateWithDelay,
    SameAs2ButReportNoButtonsAsPressedIfAOrBIsHeldDown,
}

/// this function is used when lower button sensitivity is wanted (e.g. menus)
///
/// RETURNS: pressed buttons in usual format
///
/// There are essentially three modes of operation:
///
/// 1. Get newly pressed buttons only
/// 2. Get currently pressed buttons at low sample rate with delay \
///    If the user holds down buttons for more than half a second,
///    report buttons as being pressed up to 12 times per second thereafter. \
///    If the user holds down buttons for less than half a second,
///    report only one button press.
/// 3. Same as 2, but report no buttons as pressed if A or B is held down.
pub fn joypad_low_sensitivity(cpu: &mut Cpu, mode: JoypadLowSensitivityMode) -> u8 {
    match mode {
        JoypadLowSensitivityMode::GetNewlyPressedButtonsOnly => {
            cpu.write_byte(hram::H_JOY_7, 0);
        }
        JoypadLowSensitivityMode::GetCurrentlyPressedButtonsAtLowSampleRateWithDelay => {
            cpu.write_byte(hram::H_JOY_7, 1);
            cpu.write_byte(hram::H_JOY_6, 1);
        }
        JoypadLowSensitivityMode::SameAs2ButReportNoButtonsAsPressedIfAOrBIsHeldDown => {
            cpu.write_byte(hram::H_JOY_7, 1);
            cpu.write_byte(hram::H_JOY_6, 0);
        }
    }

    cpu.call(0x381e); // JoypadLowSensitivity

    cpu.read_byte(hram::H_JOY_5)
}
