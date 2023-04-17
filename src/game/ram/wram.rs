pub const W_TILE_MAP: u16 = 0xc3a0;

/// bit 0: If 0, limit the delay to 1 frame. Note that this has no effect if
///        the delay has been disabled entirely through bit 1 of this variable
///        or bit 6 of wd730.
/// bit 1: If 0, no delay.
pub const W_LETTER_PRINTING_DELAY_FLAGS: u16 = 0xd357;

pub const W_PRINTER_SETTINGS: u16 = 0xd497;

/// bit 7 = battle animation
///   0: On
///   1: Off
/// bit 6 = battle style
///   0: Shift
///   1: Set
/// bits 0-3 = text speed (number of frames to delay after printing a letter)
///   1: Fast
///   3: Medium
///   5: Slow
pub const W_OPTIONS: u16 = 0xd354;
