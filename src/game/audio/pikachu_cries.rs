use crate::{rom::ROM, sound2::Sfx};

pub struct PikachuCry {
    data: &'static [u8],
    pos: usize,
}

impl PikachuCry {
    pub fn new(id: u8) -> Self {
        assert!(id <= 41);

        const TABLE: usize = 0x0f008e;

        let offset = TABLE + (id as usize) * 3;

        let bank = ROM[offset] as usize;
        let addr = (ROM[offset + 1] as usize) | ((ROM[offset + 2] as usize) << 8);

        let offset = (bank * 0x4000) + (addr & 0x3fff);
        let length = (ROM[offset] as usize) | ((ROM[offset + 1] as usize) << 8);

        let start = offset + 2;
        let end = start + length;

        Self {
            data: &ROM[start..end],
            pos: 0,
        }
    }
}

impl Iterator for PikachuCry {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_pos = self.pos >> 3;

        if byte_pos >= self.data.len() {
            return None;
        }

        let byte = self.data[byte_pos];

        let bit_pos = 7 - (self.pos & 0x7);
        let bit = (byte >> bit_pos) & 0x1;

        self.pos += 1;

        Some(if bit == 0 { -0.2 } else { 0.2 })
    }
}

impl rodio::Source for PikachuCry {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        22050
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f64(
            (self.data.len() as f64) * 8.0 / (self.sample_rate() as f64),
        ))
    }
}

impl Sfx<PikachuCry> for PikachuCry {
    fn open(self) -> PikachuCry {
        self
    }
}
