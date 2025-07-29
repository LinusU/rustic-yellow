use pokemon_synthesizer::gen2::SoundIterator;
use rodio::Source;

use crate::{rom::CRYSTAL_ROM, sound2::Sfx as SfxTrait};

pub const EXP_BAR: CrystalSfx = CrystalSfx::new(0x3c, 0x5653);

#[derive(Debug, Clone, Copy)]
pub struct CrystalSfx {
    bank: u8,
    addr: u16,
    pitch: i16,
    length: u16,
}

impl CrystalSfx {
    const fn new(bank: u8, addr: u16) -> Self {
        Self {
            bank,
            addr,
            pitch: 0,
            length: 0x100,
        }
    }
}

pub struct SynthesizerSource<'a>(SoundIterator<'a>);

impl<'a> SynthesizerSource<'a> {
    pub fn new(source: SoundIterator<'a>) -> SynthesizerSource<'a> {
        SynthesizerSource(source)
    }
}

impl Iterator for SynthesizerSource<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Source for SynthesizerSource<'_> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.0.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl SfxTrait<SynthesizerSource<'static>> for CrystalSfx {
    fn open(self) -> SynthesizerSource<'static> {
        SynthesizerSource::new(
            pokemon_synthesizer::gen2::synthesis(
                CRYSTAL_ROM,
                self.bank,
                self.addr,
                self.pitch,
                self.length,
            )
            .iter(),
        )
    }
}
