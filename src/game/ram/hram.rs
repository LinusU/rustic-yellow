// these values are copied to rSCX, rSCY, and rWY during V-blank
pub const H_SCX: u16 = 0xffae;
pub const H_SCY: u16 = 0xffaf;

pub const H_LOADED_ROM_BANK: u16 = 0xffb8;

/// Controls which tiles are animated.
/// 0 = no animations (breaks Surf)
/// 1 = water tile $14 is animated
/// 2 = water tile $14 and flower tile $03 are animated
pub const H_TILE_ANIMATIONS: u16 = 0xffd7;

pub const H_GBC: u16 = 0xfffe;
