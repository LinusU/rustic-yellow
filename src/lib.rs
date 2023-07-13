#![allow(clippy::bool_to_int_with_if, clippy::identity_op)]

pub use crate::game::Game;
pub use crate::gpu::{SCREEN_H, SCREEN_W};
pub use crate::keypad::{KeyboardEvent, KeyboardKey};
pub use crate::save_state::PokemonSpecies;

pub(crate) mod cpu;
pub(crate) mod game;
mod gpu;
mod keypad;
mod mbc5;
mod mmu;
mod rom;
mod save_state;
mod saves;
mod serial;
mod sound;
mod sound2;
mod timer;
