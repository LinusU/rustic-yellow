/// V-blank sets this to 0 each time it runs.
/// So, by setting it to a nonzero value and waiting for it to become 0 again,
/// you can detect that the V-blank handler has run since then.
pub const H_VBLANK_OCCURRED: u16 = 0xffd6;
