pub struct BCD {
    bytes: [u8; 3],
}

impl BCD {
    pub fn new(bytes: [u8; 3]) -> Self {
        fn is_valid_bcd(byte: u8) -> bool {
            (byte & 0x0F) <= 9 && (byte >> 4) <= 9
        }

        assert!(
            is_valid_bcd(bytes[0]) && is_valid_bcd(bytes[1]) && is_valid_bcd(bytes[2]),
            "Value out of range for BCD"
        );

        BCD { bytes }
    }

    pub fn from_u32(value: u32) -> Self {
        assert!(value <= 999999, "Value out of range for BCD");

        let bytes = [
            ((value / 100000 % 10) as u8) << 4 | ((value / 10000 % 10) as u8),
            ((value / 1000 % 10) as u8) << 4 | ((value / 100 % 10) as u8),
            ((value / 10 % 10) as u8) << 4 | ((value % 10) as u8),
        ];

        BCD { bytes }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.bytes[0] >> 4) as u32) * 100000
            + ((self.bytes[0] & 0x0F) as u32) * 10000
            + ((self.bytes[1] >> 4) as u32) * 1000
            + ((self.bytes[1] & 0x0F) as u32) * 100
            + ((self.bytes[2] >> 4) as u32) * 10
            + (self.bytes[2] & 0x0F) as u32
    }
}

impl Into<[u8; 3]> for BCD {
    fn into(self) -> [u8; 3] {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::BCD;

    #[test]
    fn test_new_valid_bcd() {
        let bcd = BCD::new([0x12, 0x34, 0x56]);
        assert_eq!(bcd.bytes, [0x12, 0x34, 0x56]);
    }

    #[test]
    #[should_panic(expected = "Value out of range for BCD")]
    fn test_new_invalid_bcd() {
        BCD::new([0x1A, 0x34, 0x56]); // 0x1A is not a valid BCD
    }

    #[test]
    fn test_from_u32() {
        let bcd = BCD::from_u32(123456);
        assert_eq!(bcd.bytes, [0x12, 0x34, 0x56]);

        let bcd = BCD::from_u32(0);
        assert_eq!(bcd.bytes, [0x00, 0x00, 0x00]);

        let bcd = BCD::from_u32(999999);
        assert_eq!(bcd.bytes, [0x99, 0x99, 0x99]);
    }

    #[test]
    #[should_panic(expected = "Value out of range for BCD")]
    fn test_from_u32_out_of_range() {
        BCD::from_u32(1000000); // Value is out of range for 3-byte BCD
    }

    #[test]
    fn test_to_u32() {
        let bcd = BCD::new([0x12, 0x34, 0x56]);
        assert_eq!(bcd.to_u32(), 123456);

        let bcd = BCD::new([0x00, 0x00, 0x00]);
        assert_eq!(bcd.to_u32(), 0);

        let bcd = BCD::new([0x99, 0x99, 0x99]);
        assert_eq!(bcd.to_u32(), 999999);
    }
}
