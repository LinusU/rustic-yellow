// these values are copied to rSCX, rSCY, and rWY during V-blank
pub const H_SCX: u16 = 0xffae;
pub const H_SCY: u16 = 0xffaf;
pub const H_WY: u16 = 0xffb0;

pub const H_JOY_LAST: u16 = 0xffb1;
pub const H_JOY_RELEASED: u16 = 0xffb2;
pub const H_JOY_PRESSED: u16 = 0xffb3;
pub const H_JOY_HELD: u16 = 0xffb4;

/// is automatic background transfer during V-blank enabled? \
/// if nonzero, yes \
/// if zero, no
pub const H_AUTO_BG_TRANSFER_ENABLED: u16 = 0xffba;

/// V-blank sets this to 0 each time it runs.
/// So, by setting it to a nonzero value and waiting for it to become 0 again,
/// you can detect that the V-blank handler has run since then.
pub const H_VBLANK_OCCURRED: u16 = 0xffd6;

/// Controls which tiles are animated. \
/// `0` = no animations (breaks Surf) \
/// `1` = water tile `$14` is animated \
/// `2` = water tile `$14` and flower tile `$03` are animated
pub const H_TILE_ANIMATIONS: u16 = 0xffd7;
