pub const INDIGO_PLATEAU: u8 = 0x09;
pub const ROUTE_17: u8 = 0x1c;
pub const ROUTE_23: u8 = 0x22;

pub const FIRST_INDOOR_MAP: u8 = 0x25;
pub const OAKS_LAB: u8 = 0x28;

pub const ROCK_TUNNEL_1F: u8 = 0x52;

pub const SS_ANNE_3F: u8 = 0x61;

pub const POKEMON_TOWER_1F: u8 = 0x8e;
pub const POKEMON_TOWER_7F: u8 = 0x94;

pub const CINNABAR_GYM: u8 = 0xa6;

pub const ROCKET_HIDEOUT_B1F: u8 = 0xc7;
pub const ROCKET_HIDEOUT_B2F: u8 = 0xc8;

pub const ROCKET_HIDEOUT_B4F: u8 = 0xca;

pub const SAFARI_ZONE_EAST: u8 = 0xd9;
pub const SAFARI_ZONE_CENTER_REST_HOUSE: u8 = 0xdd;

/// Indoor maps, such as houses, use this as the Map ID in their exit warps.
///
/// This map ID takes the player back to the last outdoor map they were on, stored in `wLastMap`.
pub const LAST_MAP: u8 = 0xff;
