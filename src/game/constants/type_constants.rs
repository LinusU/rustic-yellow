/// TypeNames indexes (see data/types/names)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    // Physical types
    Normal = 0x00,
    Fighting = 0x01,
    Flying = 0x02,
    Poison = 0x03,
    Ground = 0x04,
    Rock = 0x05,
    Bird = 0x06,
    Bug = 0x07,
    Ghost = 0x08,

    // Special types
    Fire = 0x14,
    Water = 0x15,
    Grass = 0x16,
    Electric = 0x17,
    Psychic = 0x18,
    Ice = 0x19,
    Dragon = 0x1a,
}
