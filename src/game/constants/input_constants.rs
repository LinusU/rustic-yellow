// joypad buttons
pub const BIT_A_BUTTON: u8 = 0;
pub const BIT_B_BUTTON: u8 = 1;
pub const BIT_SELECT: u8 = 2;
pub const BIT_START: u8 = 3;
pub const BIT_D_RIGHT: u8 = 4;
pub const BIT_D_LEFT: u8 = 5;
pub const BIT_D_UP: u8 = 6;
pub const BIT_D_DOWN: u8 = 7;

pub const NO_INPUT: u8 = 0;
pub const A_BUTTON: u8 = 1 << BIT_A_BUTTON;
pub const B_BUTTON: u8 = 1 << BIT_B_BUTTON;
pub const SELECT: u8 = 1 << BIT_SELECT;
pub const START: u8 = 1 << BIT_START;
pub const D_RIGHT: u8 = 1 << BIT_D_RIGHT;
pub const D_LEFT: u8 = 1 << BIT_D_LEFT;
pub const D_UP: u8 = 1 << BIT_D_UP;
pub const D_DOWN: u8 = 1 << BIT_D_DOWN;
