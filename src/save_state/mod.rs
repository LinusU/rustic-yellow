use std::{
    io::{self, Read},
    path::PathBuf,
};

fn parse_string(data: &[u8], max_len: usize) -> String {
    let mut result = String::new();

    for ch in data.iter().take(max_len) {
        match ch {
            0x50 => break,
            0x80..=0x99 => result.push((b'A' + (ch - 0x80)) as char),
            0x9a => result.push('('),
            0x9b => result.push(')'),
            0x9c => result.push(':'),
            0x9d => result.push(';'),
            0x9e => result.push('['),
            0x9f => result.push(']'),
            0xa0..=0xb9 => result.push((b'a' + (ch - 0xa0)) as char),
            0xe0 => result.push('\''),
            0xe1 => result.push('𝔭'),
            0xe2 => result.push('𝔪'),
            0xe3 => result.push('-'),
            0xe6 => result.push('?'),
            0xe7 => result.push('!'),
            0xe8 => result.push('.'),
            0xef => result.push('♂'),
            0xf0 => result.push('¥'),
            0xf5 => result.push('♀'),
            0xf6..=0xff => result.push((b'0' + (ch - 0xf6)) as char),
            _ => panic!("Invalid character in poke string: {:02x}", ch),
        }
    }

    result
}

pub struct SaveState {
    data: [u8; 0x8000],
}

impl SaveState {
    pub fn new() -> SaveState {
        SaveState { data: [0; 0x8000] }
    }

    pub fn from_file(path: &PathBuf) -> io::Result<SaveState> {
        let mut file = std::fs::File::open(path)?;
        let mut data = [0; 0x8000];
        file.read_exact(&mut data)?;
        Ok(SaveState { data })
    }

    pub fn write_to_file(&self, path: &PathBuf) -> io::Result<()> {
        std::fs::write(path, &self.data)
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn player_name(&self) -> String {
        parse_string(&self.data[0x2598..], 11)
    }

    pub fn count_badges(&self) -> u32 {
        self.data[0x2602].count_ones()
    }

    pub fn count_owned_mons(&self) -> u32 {
        let mut result = 0;

        for addr in 0x25a3..=0x25b5 {
            result += self.data[addr].count_ones();
        }

        result
    }
}
