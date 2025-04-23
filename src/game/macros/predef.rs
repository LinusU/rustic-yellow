#[rustfmt::skip]
macro_rules! predef_id {
    (CopyUncompressedPicToTilemap) => { 0x01 };
    (CopyDownscaledMonTiles) => { 0x05 };
    (HealParty) => { 0x07 };
    (ApplyOutOfBattlePoisonDamage) => { 0x13 };
    (LoadTilesetHeader) => { 0x19 };
    (GetQuantityOfItemInBag) => { 0x1c };
    (GetTileAndCoordsInFrontOfPlayer) => { 0x35 };
    (_RunPaletteCommand) => { 0x45 };
    (LoadSAV) => { 0x52 };
    (DrawHP) => { 0x5f  };
}

macro_rules! predef_call {
    ($cpu:ident, $id:ident) => {
        // LD A,u8
        $cpu.a = crate::game::macros::predef::predef_id!($id);
        $cpu.cycle(8);

        // CALL u16
        $cpu.cycle(24);
        $cpu.call(0x3eb4);
    };
}

macro_rules! predef_jump {
    ($cpu:ident, $id:ident) => {
        // LD A,u8
        $cpu.a = crate::game::macros::predef::predef_id!($id);
        $cpu.cycle(8);

        // JP u16
        $cpu.cycle(16);
        $cpu.jump(0x3eb4);
        return;
    };
}

pub(crate) use predef_call;
pub(crate) use predef_id;
pub(crate) use predef_jump;
