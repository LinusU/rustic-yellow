pub const ESTABLISH_CONNECTION_WITH_INTERNAL_CLOCK: u8 = 0x01;
pub const ESTABLISH_CONNECTION_WITH_EXTERNAL_CLOCK: u8 = 0x02;

pub const USING_EXTERNAL_CLOCK: u8 = 0x01;
pub const USING_INTERNAL_CLOCK: u8 = 0x02;
pub const CONNECTION_NOT_ESTABLISHED: u8 = 0xff;

/// not using link
pub const LINK_STATE_NONE: u8 = 0x00;
/// in a link battle
pub const LINK_STATE_BATTLING: u8 = 0x04;
