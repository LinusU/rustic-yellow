macro_rules! pal_set {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        [
            ((0xa << 3) + 1) as u8,
            ($a & 0xff) as u8,
            (($a >> 8) & 0xff) as u8,
            ($b & 0xff) as u8,
            (($b >> 8) & 0xff) as u8,
            ($c & 0xff) as u8,
            (($c >> 8) & 0xff) as u8,
            ($d & 0xff) as u8,
            (($d >> 8) & 0xff) as u8,
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
