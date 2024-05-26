/// There are 10 slots for wild pokemon, and this is the table that defines how common each of
/// those 10 slots is. A random number is generated and then the first byte of each pair in this
/// table is compared against that random number. If the random number is less than or equal
/// to the first byte, then that slot is chosen.  The second byte is double the slot number.
pub const WILD_MON_ENCOUNTER_SLOT_CHANCES: [(u8, u8); 10] = [
    (50, 0x00),  // 51/256 = 19.9% chance of slot 0
    (101, 0x02), // 51/256 = 19.9% chance of slot 1
    (140, 0x04), // 39/256 = 15.2% chance of slot 2
    (165, 0x06), // 25/256 =  9.8% chance of slot 3
    (190, 0x08), // 25/256 =  9.8% chance of slot 4
    (215, 0x0A), // 25/256 =  9.8% chance of slot 5
    (228, 0x0C), // 13/256 =  5.1% chance of slot 6
    (241, 0x0E), // 13/256 =  5.1% chance of slot 7
    (252, 0x10), // 11/256 =  4.3% chance of slot 8
    (255, 0x12), //  3/256 =  1.2% chance of slot 9
];
