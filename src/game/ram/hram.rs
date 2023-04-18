pub const H_JOY_LAST: u16 = 0xffb1;
pub const H_JOY_RELEASED: u16 = 0xffb2;
pub const H_JOY_PRESSED: u16 = 0xffb3;
pub const H_JOY_HELD: u16 = 0xffb4;

/// V-blank sets this to 0 each time it runs.
/// So, by setting it to a nonzero value and waiting for it to become 0 again,
/// you can detect that the V-blank handler has run since then.
pub const H_VBLANK_OCCURRED: u16 = 0xffd6;
