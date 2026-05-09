use crate::game::constants::palette_constants::PAL_BROWNMON;

macro_rules! pal_set {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        [
            ((0xa << 3) + 1) as u8,
            $a,
            0,
            $b,
            0,
            $c,
            0,
            $d,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    };
}

pub const PAL_PACKET_EMPTY: [u8; 16] = pal_set!(0, 0, 0, 0);
pub const PAL_PACKET_POKEDEX: [u8; 16] = pal_set!(PAL_BROWNMON, 0, 0, 0);
