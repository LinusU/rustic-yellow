# Rustic Yellow

Rustic Yellow is a project to re-implement the classic GameBoy Color game Pokemon Yellow in Rust. The goal of this project is to create a playable version of the game that runs on modern computers, while also serving as a learning experience for those interested in game development and emulation.

## Features

- **High quality music** - The game uses the original music from the Pokemon Yellow soundtrack, but in FLAC format instead of the original GameBoy audio format.
- **Speedup game without affecting sounds** - Rustic Yellow plays all sounds using it's own sound engine, so the game emulation can be changed without affecting the music. Especially useful when speeding up the game.
- **Multiple save files** - The game supports multiple save files, so you can play the game with different teams or try out different strategies. "Continue" and "New Game" in the main menu has been reimplemented to support this.

<img width="200" alt="Main Menu" src="https://user-images.githubusercontent.com/189580/235338348-d04a743f-222c-499e-892c-2ab42717edcf.png" /><img width="200" alt="Continue" src="https://user-images.githubusercontent.com/189580/235338353-4d807a6d-1790-4659-9237-22034ef9f5cc.png" /><img width="200" alt="New Game" src="https://user-images.githubusercontent.com/189580/235338561-211e592a-9f5d-4936-b430-5b78ad3d746f.png"><img width="200" alt="Palette Town" src="https://user-images.githubusercontent.com/189580/235338567-363767ac-bea4-459e-8d80-cabac68e70f5.png" />

## Getting Started

To get started with Rustic Yellow, you'll need to have Rust installed on your computer. You can download Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can clone the Rustic Yellow repository:

```sh
git clone https://github.com/LinusU/rustic-yellow.git
cd rustic-yellow
```

## ROM Files

Rustic Yellow requires two ROM files in order to build the project: one for the original Pokemon Yellow game, and one for Pokemon Crystal from which some updated sprites are used. The ROM files must have the following SHA1 hashes:

- `pokeyellow.gbc` - `cc7d03262ebfaf2f06772c1a480c7d9d5f4a38e1`
- `pokecrystal.gbc` - `f4cd194bdee0d04ca4eac29e09b8e4e9d818c133`

You can obtain a ROM file from various sources online, but please note that it may be illegal to download and use ROMs in some jurisdictions.

## Music

Rustic Yellow requires music in FLAC format from the "Pokémon Red & Pokémon Green: Super Music Collection" album. You can download the music from the following link:

https://archive.org/details/pkmn-rgby-soundtrack

Copy all of the FLAC files from the album, both from `Disc 1` and `Disc 2 (Yellow)`, into a directory named `music` in the project root.

## Running the Game

Use the following command to build and run the game:

```sh
cargo run --release
```

## Packaging

I've added some basic support for packaging the game to a proper app using [Cargo bundle](https://github.com/burtonageo/cargo-bundle). Currently only macOS is supported, but it should be possible to add support for other platforms as well.

Running `cargo bundle` should produce an `.app` file with the game and the music bundled together. The `.app` file can be moved to the `Applications` folder and run from there.

## Contributing

Contributions to Rustic Yellow are welcome! Feel free to open an issue if you have any questions or suggestions. If you want to contribute code, it's probably a good idea to open an issue first to discuss the change you want to make, since it's still early days for this project.

## Acknowledgements

- [Mathijs van de Nes](https://github.com/mvdnes) for their work on [mvdnes/rboy](https://github.com/mvdnes/rboy), from which this project was bootstrapped
- [The pret team](https://github.com/orgs/pret/people) for their work on [pret/pokeyellow](https://github.com/pret/pokeyellow), which was used as a reference for the disassembly of the original Pokemon Yellow ROM
- The original developers of Pokemon Yellow, for creating such a wonderful game

## License

Rustic Yellow is licensed under the MIT License. See the LICENSE file for details.
