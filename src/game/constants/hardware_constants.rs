// memory map
pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const SRAM_BEGIN: u16 = 0xa000;
pub const SRAM_END: u16 = 0xc000;
pub const WRAM0_BEGIN: u16 = 0xc000;
pub const WRAM0_END: u16 = 0xd000;
pub const WRAM1_BEGIN: u16 = 0xd000;
pub const WRAM1_END: u16 = 0xe000;
// hardware registers $ff00-$ff80 (see below)
pub const HRAM_BEGIN: u16 = 0xff80;
pub const HRAM_END: u16 = 0xffff;

pub const LY_VBLANK: u8 = 145;

/// Joypad (R/W)
pub const R_JOYP: u16 = 0xff00;

/// Serial transfer data (R/W)
pub const R_SB: u16 = 0xff01;

/// Serial Transfer Control (R/W)
pub const R_SC: u16 = 0xff02;
pub const R_SC_ON: u8 = 7;
pub const R_SC_CGB: u8 = 1;
pub const R_SC_CLOCK: u8 = 0;

/// Divider Register (R/W)
pub const R_DIV: u16 = 0xff04;

/// Timer counter (R/W)
pub const R_TIMA: u16 = 0xff05;

/// Timer Modulo (R/W)
pub const R_TMA: u16 = 0xff06;

/// Timer Control (R/W)
pub const R_TAC: u16 = 0xff07;
pub const R_TAC_ON: u8 = 2;
pub const R_TAC_4096_HZ: u8 = 0;
pub const R_TAC_262144_HZ: u8 = 1;
pub const R_TAC_65536_HZ: u8 = 2;
pub const R_TAC_16384_HZ: u8 = 3;

/// Interrupt Flag (R/W)
pub const R_IF: u16 = 0xff0f;

/// Channel 1 Sweep register (R/W)
pub const R_NR10: u16 = 0xff10;

/// Channel 1 Sound length/Wave pattern duty (R/W)
pub const R_NR11: u16 = 0xff11;

/// Channel 1 Volume Envelope (R/W)
pub const R_NR12: u16 = 0xff12;

/// Channel 1 Frequency lo (Write Only)
pub const R_NR13: u16 = 0xff13;

/// Channel 1 Frequency hi (R/W)
pub const R_NR14: u16 = 0xff14;

/// Channel 2 Sound Length/Wave Pattern Duty (R/W)
pub const R_NR21: u16 = 0xff16;

/// Channel 2 Volume Envelope (R/W)
pub const R_NR22: u16 = 0xff17;

/// Channel 2 Frequency lo data (W)
pub const R_NR23: u16 = 0xff18;

/// Channel 2 Frequency hi data (R/W)
pub const R_NR24: u16 = 0xff19;

/// Channel 3 Sound on/off (R/W)
pub const R_NR30: u16 = 0xff1a;

/// Channel 3 Sound Length
pub const R_NR31: u16 = 0xff1b;

/// Channel 3 Select output level (R/W)
pub const R_NR32: u16 = 0xff1c;

/// Channel 3 Frequency's lower data (W)
pub const R_NR33: u16 = 0xff1d;

/// Channel 3 Frequency's higher data (R/W)
pub const R_NR34: u16 = 0xff1e;

/// Channel 4 Sound Length (R/W)
pub const R_NR41: u16 = 0xff20;

/// Channel 4 Volume Envelope (R/W)
pub const R_NR42: u16 = 0xff21;

/// Channel 4 Polynomial Counter (R/W)
pub const R_NR43: u16 = 0xff22;

/// Channel 4 Counter/consecutive; Initial (R/W)
pub const R_NR44: u16 = 0xff23;

/// Channel control / ON-OFF / Volume (R/W)
pub const R_NR50: u16 = 0xff24;

/// Selection of Sound output terminal (R/W)
pub const R_NR51: u16 = 0xff25;

/// Sound on/off
pub const R_NR52: u16 = 0xff26;

pub const R_WAVE_0: u16 = 0xff30;
pub const R_WAVE_1: u16 = 0xff31;
pub const R_WAVE_2: u16 = 0xff32;
pub const R_WAVE_3: u16 = 0xff33;
pub const R_WAVE_4: u16 = 0xff34;
pub const R_WAVE_5: u16 = 0xff35;
pub const R_WAVE_6: u16 = 0xff36;
pub const R_WAVE_7: u16 = 0xff37;
pub const R_WAVE_8: u16 = 0xff38;
pub const R_WAVE_9: u16 = 0xff39;
pub const R_WAVE_A: u16 = 0xff3a;
pub const R_WAVE_B: u16 = 0xff3b;
pub const R_WAVE_C: u16 = 0xff3c;
pub const R_WAVE_D: u16 = 0xff3d;
pub const R_WAVE_E: u16 = 0xff3e;
pub const R_WAVE_F: u16 = 0xff3f;

/// LCD Control (R/W)
pub const R_LCDC: u16 = 0xff40;
pub const R_LCDC_ENABLE: u8 = 7;
pub const R_LCDC_ENABLE_MASK: u8 = 1 << R_LCDC_ENABLE;

/// LCDC Status (R/W)
pub const R_STAT: u16 = 0xff41;

/// Scroll Y (R/W)
pub const R_SCY: u16 = 0xff42;

/// Scroll X (R/W)
pub const R_SCX: u16 = 0xff43;

/// LCDC Y-Coordinate (R)
pub const R_LY: u16 = 0xff44;

/// LY Compare (R/W)
pub const R_LYC: u16 = 0xff45;

/// DMA Transfer and Start Address (W)
pub const R_DMA: u16 = 0xff46;

/// BG Palette Data (R/W) - Non CGB Mode Only
pub const R_BGP: u16 = 0xff47;

/// Object Palette 0 Data (R/W) - Non CGB Mode Only
pub const R_OBP0: u16 = 0xff48;

/// Object Palette 1 Data (R/W) - Non CGB Mode Only
pub const R_OBP1: u16 = 0xff49;

/// Window Y Position (R/W)
pub const R_WY: u16 = 0xff4a;

/// Window X Position minus 7 (R/W)
pub const R_WX: u16 = 0xff4b;

/// CGB Mode Only - Prepare Speed Switch
pub const R_KEY1: u16 = 0xff4d;

/// CGB Mode Only - VRAM Bank
pub const R_VBK: u16 = 0xff4f;

/// CGB Mode Only - New DMA Source, High
pub const R_HDMA1: u16 = 0xff51;

/// CGB Mode Only - New DMA Source, Low
pub const R_HDMA2: u16 = 0xff52;

/// CGB Mode Only - New DMA Destination, High
pub const R_HDMA3: u16 = 0xff53;

/// CGB Mode Only - New DMA Destination, Low
pub const R_HDMA4: u16 = 0xff54;

/// CGB Mode Only - New DMA Length/Mode/Start
pub const R_HDMA5: u16 = 0xff55;

/// CGB Mode Only - Infrared Communications Port
pub const R_RP: u16 = 0xff56;

/// CGB Mode Only - Background Palette Index
pub const R_BGPI: u16 = 0xff68;

/// CGB Mode Only - Background Palette Data
pub const R_BGPD: u16 = 0xff69;

/// CGB Mode Only - Sprite Palette Index
pub const R_OBPI: u16 = 0xff6a;

/// CGB Mode Only - Sprite Palette Data
pub const R_OBPD: u16 = 0xff6b;

/// CGB Mode Only - Object Priority Mode
pub const R_OPRI: u16 = 0xff6c;

/// CGB Mode Only - WRAM Bank
pub const R_SVBK: u16 = 0xff70;

/// Channels 1 & 2 Amplitude (R)
pub const R_PCM12: u16 = 0xff76;

/// Channels 3 & 4 Amplitude (R)
pub const R_PCM34: u16 = 0xff77;

/// Interrupt Enable (R/W)
pub const R_IE: u16 = 0xffff;
