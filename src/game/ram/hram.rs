pub const H_BASE_TILE_ID: u16 = 0xff8b;
pub const H_SPRITE_INDEX_OR_TEXT_ID: u16 = 0xff8c;

pub const H_LOAD_SPRITE_TEMP1: u16 = 0xff8d;
pub const H_LOAD_SPRITE_TEMP2: u16 = 0xff8e;

pub const H_QUOTIENT: u16 = 0xff95;
pub const H_EXPERIENCE: u16 = 0xff96;
pub const H_MULTIPLICAND: u16 = 0xff96;
pub const H_DIVISOR: u16 = 0xff99;
pub const H_MULTIPLIER: u16 = 0xff99;

// these values are copied to rSCX, rSCY, and rWY during V-blank
pub const H_SCX: u16 = 0xffae;
pub const H_SCY: u16 = 0xffaf;
pub const H_WY: u16 = 0xffb0;

pub const H_JOY_LAST: u16 = 0xffb1;
pub const H_JOY_RELEASED: u16 = 0xffb2;
pub const H_JOY_PRESSED: u16 = 0xffb3;
pub const H_JOY_HELD: u16 = 0xffb4;
pub const H_JOY_5: u16 = 0xffb5;

pub const H_LOADED_ROM_BANK: u16 = 0xffb8;

/// is automatic background transfer during V-blank enabled? \
/// if nonzero, yes \
/// if zero, no
pub const H_AUTO_BG_TRANSFER_ENABLED: u16 = 0xffba;

/// the destination address of the automatic background transfer \
/// size: 2 bytes
pub const H_AUTO_BG_TRANSFER_DEST: u16 = 0xffbc;

pub const H_RANDOM_ADD: u16 = 0xffd3;
pub const H_RANDOM_SUB: u16 = 0xffd4;

/// V-blank sets this to 0 each time it runs.
/// So, by setting it to a nonzero value and waiting for it to become 0 again,
/// you can detect that the V-blank handler has run since then.
pub const H_VBLANK_OCCURRED: u16 = 0xffd6;

/// Controls which tiles are animated. \
/// `0` = no animations (breaks Surf) \
/// `1` = water tile `$14` is animated \
/// `2` = water tile `$14` and flower tile `$03` are animated
pub const H_TILE_ANIMATIONS: u16 = 0xffd7;

pub const H_INTERACTED_WITH_BOOKSHELF: u16 = 0xffdb;

pub const H_START_TILE_ID: u16 = 0xffe1;

pub const H_ITEM_ALREADY_FOUND: u16 = 0xffeb;

pub const H_DIDNT_FIND_ANY_HIDDEN_OBJECT: u16 = 0xffee;
