use std::fmt::{Debug, Display, Write};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PokeString(Vec<u8>);

impl PokeString {
    pub fn from_bytes(data: &[u8], max_len: usize) -> PokeString {
        let result = data
            .iter()
            .copied()
            .take(max_len)
            .take_while(|&b| b != 0x50)
            .collect();

        PokeString(result)
    }

    pub fn iter(&self) -> std::iter::Copied<std::slice::Iter<'_, u8>> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a PokeString {
    type Item = u8;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl Display for PokeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in &self.0 {
            match ch {
                0x4a => f.write_str("𝔭𝔪")?,
                0x50 => unreachable!(),
                0x54 => f.write_str("POKé")?,
                0x56 => f.write_str("……")?,
                0x5b => f.write_str("PC")?,
                0x5c => f.write_str("TM")?,
                0x5d => f.write_str("TRAINER")?,
                0x5e => f.write_str("ROCKET")?,

                0x69 => f.write_char('𝐕')?,
                0x6a => f.write_char('𝐒')?,
                0x6d => f.write_char(':')?,
                0x70 => f.write_char('‘')?,
                0x71 => f.write_char('’')?,
                0x72 => f.write_char('“')?,
                0x73 => f.write_char('”')?,
                0x74 => f.write_char('·')?,
                0x75 => f.write_char('…')?,

                0x79 => f.write_char('┌')?,
                0x7a => f.write_char('─')?,
                0x7b => f.write_char('┐')?,
                0x7c => f.write_char('│')?,
                0x7d => f.write_char('└')?,
                0x7e => f.write_char('┘')?,
                0x7f => f.write_char(' ')?,

                0x80..=0x99 => f.write_char((b'A' + (ch - 0x80)) as char)?,

                0x9a => f.write_char('(')?,
                0x9b => f.write_char(')')?,
                0x9c => f.write_char(':')?,
                0x9d => f.write_char(';')?,
                0x9e => f.write_char('[')?,
                0x9f => f.write_char(']')?,

                0xa0..=0xb9 => f.write_char((b'a' + (ch - 0xa0)) as char)?,

                0xba => f.write_char('é')?,
                0xbb => f.write_str("'d")?,
                0xbc => f.write_str("'l")?,
                0xbd => f.write_str("'s")?,
                0xbe => f.write_str("'t")?,
                0xbf => f.write_str("'v")?,

                0xe0 => f.write_char('\'')?,
                0xe1 => f.write_char('𝔭')?,
                0xe2 => f.write_char('𝔪')?,
                0xe3 => f.write_char('-')?,

                0xe4 => f.write_str("'r")?,
                0xe5 => f.write_str("'m")?,

                0xe6 => f.write_char('?')?,
                0xe7 => f.write_char('!')?,
                0xe8 => f.write_char('.')?,

                0xec => f.write_char('▷')?,
                0xed => f.write_char('▶')?,
                0xee => f.write_char('▼')?,
                0xef => f.write_char('♂')?,
                0xf0 => f.write_char('¥')?,
                0xf1 => f.write_char('×')?,
                0xf2 => f.write_char('.')?,
                0xf3 => f.write_char('/')?,
                0xf4 => f.write_char(',')?,
                0xf5 => f.write_char('♀')?,

                0xf6..=0xff => f.write_char((b'0' + (ch - 0xf6)) as char)?,

                _ => f.write_char('�')?,
            }
        }

        Ok(())
    }
}

impl Debug for PokeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PokeString {:?}", format!("{}", self))
    }
}
