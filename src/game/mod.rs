#![allow(dead_code)]

use std::{
    ops::{Generator, GeneratorState},
    pin::Pin, cell::RefCell, rc::Rc,
};

use crate::{cpu::Cpu, AudioPlayer, KeypadKey, mmu::Mmu, op::run_rom_code};

mod constants;
mod engine;
mod home;
mod ram;

pub struct Game {
    // cpu: Cpu,
    foo: Pin<Box<dyn Generator<Yield = u32, Return = Cpu>>>,
    mmu: Rc<RefCell<Mmu>>,
    cycles: u64,
}

#[macro_export]
macro_rules! yield_from {
    ($generator_ctor:expr) => {{
        let mut generator = $generator_ctor;
        loop {
            match std::pin::Pin::new(&mut generator).resume(()) {
                std::ops::GeneratorState::Yielded(cycles) => yield cycles,
                std::ops::GeneratorState::Complete(_) => break,
            }
        }
    }};
}

impl Game {
    pub fn new(player: Box<dyn AudioPlayer>) -> Self {
        let rom = include_bytes!("../../rom_file.gb").to_vec();

        assert_eq!(rom[0x143], 0x80);
        assert_eq!(rom[0x147], 0x1b);
        assert_eq!(rom[0x149], 0x03);

        let mmu = Rc::new(RefCell::new(Mmu::new(rom, player)));

        let mut cpu = Cpu::new(mmu.clone());
        let mut cycles = 0;

        // let mut generator = home::start::start(&mut cpu);

        // while let GeneratorState::Yielded(ticks) = Pin::new(&mut generator).resume(()) {
        //     cycles += ticks as u64;
        // }

        // drop(generator);

        eprintln!("We have ported the first {cycles} cycles to Rust!");

        // Self { cpu, mmu, cycles }
        Self { foo: Box::pin(run_rom_code(cpu)), mmu, cycles }
    }

    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    pub fn do_cycle(&mut self) {
        match Pin::new(&mut self.foo).resume(()) {
            GeneratorState::Yielded(ticks) => self.cycles += (ticks * 4) as u64,
            GeneratorState::Complete(_) => panic!("Generator is complete"),
        }

        // self.cycles += self.cpu.do_cycle() as u64;
    }

    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        let result = self.mmu.borrow().gpu.updated;
        self.mmu.borrow_mut().gpu.updated = false;
        result
    }

    pub fn get_gpu_data(&self) -> Vec<u8> {
        self.mmu.borrow().gpu.data.clone()
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        self.mmu.borrow_mut().keypad.keyup(key)
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        self.mmu.borrow_mut().keypad.keydown(key)
    }

    pub fn sync_audio(&mut self) {
        self.mmu.borrow_mut().sound.sync()
    }
}

#[cfg(test)]
mod tests {
    use crate::{SCREEN_H, SCREEN_W};
    use image::{ColorType, DynamicImage, GenericImageView, RgbImage};

    use super::*;

    struct NullAudioPlayer {}

    impl AudioPlayer for NullAudioPlayer {
        fn play(&mut self, _: &[f32], _: &[f32]) {}
        fn samples_rate(&self) -> u32 {
            44100
        }
        fn underflowed(&self) -> bool {
            false
        }
    }

    fn dump_image(game: &Game) -> DynamicImage {
        DynamicImage::ImageRgb8(
            RgbImage::from_raw(
                SCREEN_W as u32,
                SCREEN_H as u32,
                game.get_gpu_data(),
            )
            .unwrap(),
        )
    }

    #[allow(dead_code)]
    fn save_image(game: &Game, path: &str) {
        image::save_buffer(
            path,
            &game.get_gpu_data(),
            SCREEN_W as u32,
            SCREEN_H as u32,
            ColorType::Rgb8,
        )
        .unwrap();

        eprintln!("Saved image to {path}, don't forget to run ImageOptim before committing");
    }

    fn assert_matches(game: &Game, path: &str) {
        let actual = dump_image(game);
        let expected = image::open(path).unwrap();

        assert!(Iterator::eq(actual.pixels(), expected.pixels()));
    }

    #[test]
    fn test_new() {
        let mut game = Game::new(Box::new(NullAudioPlayer {}));

        while game.cycles() < 10_000_000 {
            game.do_cycle();
        }

        assert_eq!(game.cycles(), 10_000_000);
        assert_matches(&game, "snapshots/10_000_000.png");

        while game.cycles() < 100_000_000 {
            game.do_cycle();
        }

        assert_eq!(game.cycles(), 100_000_000);
        assert_matches(&game, "snapshots/100_000_000.png");

        while game.cycles() < 200_000_000 {
            game.do_cycle();
        }

        assert_eq!(game.cycles(), 200_000_000);
        assert_matches(&game, "snapshots/200_000_000.png");

        // save_image(&game, "snapshots/200_000_000.png");
    }
}
