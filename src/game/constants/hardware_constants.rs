// MBC1
pub const MBC1_SRAM_ENABLE: u16 = 0x0000;
pub const MBC1_ROM_BANK: u16 = 0x2000;
pub const MBC1_SRAM_BANK: u16 = 0x4000;
pub const MBC1_SRAM_BANKING_MODE: u16 = 0x6000;

pub const SRAM_DISABLE: u8 = 0x00;
pub const SRAM_ENABLE: u8 = 0x0a;

// Hardware registers
/// LCD Control (R/W)
pub const R_LCDC: u16 = 0xff40;
pub const R_LCDC_ENABLE: u8 = 7;

/// BG Palette Data (R/W) - Non CGB Mode Only
pub const R_BGP: u16 = 0xff47;
/// Object Palette 0 Data (R/W) - Non CGB Mode Only
pub const R_OBP0: u16 = 0xff48;
/// Object Palette 1 Data (R/W) - Non CGB Mode Only
pub const R_OBP1: u16 = 0xff49;

/// Window Y Position (R/W)
pub const R_WY: u16 = 0xff4a;

/// CGB Mode Only - Background Palette Index
pub const R_BGPI: u16 = 0xff68;
