use super::input_constants;

pub const SPRITE_FACING_DOWN: u8 = 0x00;
pub const SPRITE_FACING_UP: u8 = 0x04;
pub const SPRITE_FACING_LEFT: u8 = 0x08;
pub const SPRITE_FACING_RIGHT: u8 = 0x0c;

const PLAYER_DIR_BIT_RIGHT: u8 = 0;
const PLAYER_DIR_BIT_LEFT: u8 = 1;
const PLAYER_DIR_BIT_DOWN: u8 = 2;
const PLAYER_DIR_BIT_UP: u8 = 3;

const PLAYER_DIR_RIGHT: u8 = 1 << PLAYER_DIR_BIT_RIGHT;
const PLAYER_DIR_LEFT: u8 = 1 << PLAYER_DIR_BIT_LEFT;
const PLAYER_DIR_DOWN: u8 = 1 << PLAYER_DIR_BIT_DOWN;
const PLAYER_DIR_UP: u8 = 1 << PLAYER_DIR_BIT_UP;

#[repr(u8)]
pub enum PlayerDirection {
    Right = PLAYER_DIR_RIGHT,
    Left = PLAYER_DIR_LEFT,
    Down = PLAYER_DIR_DOWN,
    Up = PLAYER_DIR_UP,
}

impl PlayerDirection {
    pub fn from_joy_held(joy_held: u8) -> Option<Self> {
        if (joy_held & input_constants::D_DOWN) != 0 {
            Some(PlayerDirection::Down)
        } else if (joy_held & input_constants::D_UP) != 0 {
            Some(PlayerDirection::Up)
        } else if (joy_held & input_constants::D_LEFT) != 0 {
            Some(PlayerDirection::Left)
        } else if (joy_held & input_constants::D_RIGHT) != 0 {
            Some(PlayerDirection::Right)
        } else {
            None
        }
    }

    pub fn dx(&self) -> i8 {
        match self {
            PlayerDirection::Right => 1,
            PlayerDirection::Left => -1,
            _ => 0,
        }
    }

    pub fn dy(&self) -> i8 {
        match self {
            PlayerDirection::Down => 1,
            PlayerDirection::Up => -1,
            _ => 0,
        }
    }
}
