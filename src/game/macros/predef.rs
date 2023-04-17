#[rustfmt::skip]
macro_rules! predef_id {
    (_RunPaletteCommand) => { 0x45 };
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

pub(crate) use predef_id;
pub(crate) use predef_jump;
