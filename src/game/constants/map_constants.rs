pub const PALLET_TOWN: u8 = 0x00;
pub const INDIGO_PLATEAU: u8 = 0x09;
pub const SAFFRON_CITY: u8 = 0x0a;

pub const ROUTE_1: u8 = 0x0c;
pub const ROUTE_17: u8 = 0x1c;
pub const ROUTE_23: u8 = 0x22;
pub const ROUTE_25: u8 = 0x24;

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

pub const SAFARI_ZONE_NORTH_REST_HOUSE: u8 = 0xe1;
pub const CERULEAN_CAVE_2F: u8 = 0xe2;

pub const CERULEAN_CAVE_1F: u8 = 0xe4;

pub const TRADE_CENTER: u8 = 0xef;
pub const COLOSSEUM: u8 = 0xf0;

pub const SAFARI_ZONE_EAST: u8 = 0xd9;
pub const SAFARI_ZONE_CENTER_REST_HOUSE: u8 = 0xdd;

pub const LORELEIS_ROOM: u8 = 0xf5;
pub const BRUNOS_ROOM: u8 = 0xf6;

/// Indoor maps, such as houses, use this as the Map ID in their exit warps.
///
/// This map ID takes the player back to the last outdoor map they were on, stored in `wLastMap`.
pub const LAST_MAP: u8 = 0xff;
