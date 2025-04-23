//! {stat}_(UP|DOWN)(1|2) means that the move raises the user's (or lowers the target's) corresponding stat modifier by 1 (or 2) stages \
//! {status condition}_SIDE_EFFECT means that the move has a side chance of causing that condition \
//! {status condition}_EFFECT means that the move causes the status condition every time it hits the target

pub const NO_ADDITIONAL_EFFECT: u8 = 0x00;
/// unused
pub const EFFECT_01: u8 = 0x01;
pub const POISON_SIDE_EFFECT1: u8 = 0x02;
pub const DRAIN_HP_EFFECT: u8 = 0x03;
pub const BURN_SIDE_EFFECT1: u8 = 0x04;
pub const FREEZE_SIDE_EFFECT: u8 = 0x05;
pub const PARALYZE_SIDE_EFFECT1: u8 = 0x06;
/// Explosion, Self Destruct
pub const EXPLODE_EFFECT: u8 = 0x07;
pub const DREAM_EATER_EFFECT: u8 = 0x08;
pub const MIRROR_MOVE_EFFECT: u8 = 0x09;
pub const ATTACK_UP1_EFFECT: u8 = 0x0a;
pub const DEFENSE_UP1_EFFECT: u8 = 0x0b;
pub const SPEED_UP1_EFFECT: u8 = 0x0c;
pub const SPECIAL_UP1_EFFECT: u8 = 0x0d;
pub const ACCURACY_UP1_EFFECT: u8 = 0x0e;
pub const EVASION_UP1_EFFECT: u8 = 0x0f;
pub const PAY_DAY_EFFECT: u8 = 0x10;
pub const SWIFT_EFFECT: u8 = 0x11;
pub const ATTACK_DOWN1_EFFECT: u8 = 0x12;
pub const DEFENSE_DOWN1_EFFECT: u8 = 0x13;
pub const SPEED_DOWN1_EFFECT: u8 = 0x14;
pub const SPECIAL_DOWN1_EFFECT: u8 = 0x15;
pub const ACCURACY_DOWN1_EFFECT: u8 = 0x16;
pub const EVASION_DOWN1_EFFECT: u8 = 0x17;
pub const CONVERSION_EFFECT: u8 = 0x18;
pub const HAZE_EFFECT: u8 = 0x19;
pub const BIDE_EFFECT: u8 = 0x1a;
pub const THRASH_PETAL_DANCE_EFFECT: u8 = 0x1b;
pub const SWITCH_AND_TELEPORT_EFFECT: u8 = 0x1c;
pub const TWO_TO_FIVE_ATTACKS_EFFECT: u8 = 0x1d;
/// unused
pub const EFFECT_1E: u8 = 0x1e;
pub const FLINCH_SIDE_EFFECT1: u8 = 0x1f;
pub const SLEEP_EFFECT: u8 = 0x20;
pub const POISON_SIDE_EFFECT2: u8 = 0x21;
pub const BURN_SIDE_EFFECT2: u8 = 0x22;
pub const UNUSED_EFFECT_23: u8 = 0x23;
pub const PARALYZE_SIDE_EFFECT2: u8 = 0x24;
pub const FLINCH_SIDE_EFFECT2: u8 = 0x25;
/// moves like Horn Drill
pub const OHKO_EFFECT: u8 = 0x26;
/// moves like Solar Beam
pub const CHARGE_EFFECT: u8 = 0x27;
pub const SUPER_FANG_EFFECT: u8 = 0x28;
/// Seismic Toss, Night Shade, Sonic Boom, Dragon Rage, Psywave
pub const SPECIAL_DAMAGE_EFFECT: u8 = 0x29;
/// moves like Wrap
pub const TRAPPING_EFFECT: u8 = 0x2a;
pub const FLY_EFFECT: u8 = 0x2b;
pub const ATTACK_TWICE_EFFECT: u8 = 0x2c;
/// Jump Kick and Hi Jump Kick effect
pub const JUMP_KICK_EFFECT: u8 = 0x2d;
pub const MIST_EFFECT: u8 = 0x2e;
pub const FOCUS_ENERGY_EFFECT: u8 = 0x2f;
/// moves like Double Edge
pub const RECOIL_EFFECT: u8 = 0x30;
/// Confuse Ray, Supersonic (not the move Confusion)
pub const CONFUSION_EFFECT: u8 = 0x31;
pub const ATTACK_UP2_EFFECT: u8 = 0x32;
pub const DEFENSE_UP2_EFFECT: u8 = 0x33;
pub const SPEED_UP2_EFFECT: u8 = 0x34;
pub const SPECIAL_UP2_EFFECT: u8 = 0x35;
pub const ACCURACY_UP2_EFFECT: u8 = 0x36;
pub const EVASION_UP2_EFFECT: u8 = 0x37;
/// Recover, Softboiled, Rest
pub const HEAL_EFFECT: u8 = 0x38;
pub const TRANSFORM_EFFECT: u8 = 0x39;
pub const ATTACK_DOWN2_EFFECT: u8 = 0x3a;
pub const DEFENSE_DOWN2_EFFECT: u8 = 0x3b;
pub const SPEED_DOWN2_EFFECT: u8 = 0x3c;
pub const SPECIAL_DOWN2_EFFECT: u8 = 0x3d;
pub const ACCURACY_DOWN2_EFFECT: u8 = 0x3e;
pub const EVASION_DOWN2_EFFECT: u8 = 0x3f;
pub const LIGHT_SCREEN_EFFECT: u8 = 0x40;
pub const REFLECT_EFFECT: u8 = 0x41;
pub const POISON_EFFECT: u8 = 0x42;
pub const PARALYZE_EFFECT: u8 = 0x43;
pub const ATTACK_DOWN_SIDE_EFFECT: u8 = 0x44;
pub const DEFENSE_DOWN_SIDE_EFFECT: u8 = 0x45;
pub const SPEED_DOWN_SIDE_EFFECT: u8 = 0x46;
pub const SPECIAL_DOWN_SIDE_EFFECT: u8 = 0x47;
pub const CONFUSION_SIDE_EFFECT: u8 = 0x4c;
pub const TWINEEDLE_EFFECT: u8 = 0x4d;
pub const SUBSTITUTE_EFFECT: u8 = 0x4f;
pub const HYPER_BEAM_EFFECT: u8 = 0x50;
pub const RAGE_EFFECT: u8 = 0x51;
pub const MIMIC_EFFECT: u8 = 0x52;
pub const METRONOME_EFFECT: u8 = 0x53;
pub const LEECH_SEED_EFFECT: u8 = 0x54;
pub const SPLASH_EFFECT: u8 = 0x55;
pub const DISABLE_EFFECT: u8 = 0x56;

pub const NUM_MOVE_EFFECTS: usize = 86;
