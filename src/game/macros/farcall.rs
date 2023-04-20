#[rustfmt::skip]
macro_rules! bank {
    (LoadEDTile) => { 0x01 };
    (LoadMonPartySpriteGfx) => { 0x1c };
    (SaveSAVtoSRAM) => { 0x1c };
}

#[rustfmt::skip]
macro_rules! function {
    (LoadEDTile) => { 0x64cb };
    (LoadMonPartySpriteGfx) => { 0x57f9 };
    (SaveSAVtoSRAM) => { 0x7b91 };
}

macro_rules! farcall {
    ($cpu:ident, $id:ident) => {
        // LD A,u8
        $cpu.b = crate::game::macros::farcall::bank!($id);
        $cpu.cycle(8);

        // LD HL,u16
        $cpu.set_hl(crate::game::macros::farcall::function!($id));
        $cpu.cycle(12);

        // CALL u16
        $cpu.cycle(24);
        $cpu.call(0x3e84); // Bankswitch
    };
}

pub(crate) use bank;
pub(crate) use farcall;
pub(crate) use function;
