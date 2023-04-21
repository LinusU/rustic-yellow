# Rustic Yellow

Rustic Yellow is a project to re-implement the classic GameBoy Color game Pokemon Yellow in Rust. The goal of this project is to create a playable version of the game that runs on modern computers, while also serving as a learning experience for those interested in game development and emulation.

## Getting Started

To get started with Rustic Yellow, you'll need to have Rust installed on your computer. You can download Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can clone the Rustic Yellow repository:

```sh
git clone https://github.com/LinusU/rustic-yellow.git
cd rustic-yellow
```

## ROM File

Rustic Yellow requires a ROM file for Pokemon Yellow in order to build the project. The ROM file must have the SHA1 hash `cc7d03262ebfaf2f06772c1a480c7d9d5f4a38e1` and be named `rom_file.gb`. You can obtain a ROM file from various sources online, but please note that it may be illegal to download and use ROMs in some jurisdictions.

## Music

Rustic Yellow requires music in FLAC format from the "Pokémon Red & Pokémon Green: Super Music Collection" album. You can download the music from the following link:

https://archive.org/details/pkmn-rgby-soundtrack

Copy all of the FLAC files from the album, both from `Disc 1` and `Disc 2 (Yellow)`, into a directory named `music` in the project root.

## Running the Game

To run Rustic Yellow, make sure you have a ROM file named `rom_file.gb` with the correct SHA1 hash in the project directory. Then use the following command:

```sh
cargo run --release
```

## Contributing

Contributions to Rustic Yellow are welcome! If you'd like to contribute code or report a bug, please open an issue or pull request on the project's GitHub page: https://github.com/LinusU/rustic-yellow

## Acknowledgements

- [Mathijs van de Nes](https://github.com/mvdnes) for their work on [mvdnes/rboy](https://github.com/mvdnes/rboy), from which this project was bootstrapped
- [The pret team](https://github.com/orgs/pret/people) for their work on [pret/pokeyellow](https://github.com/pret/pokeyellow), which was used as a reference for the disassembly of the original Pokemon Yellow ROM
- The original developers of Pokemon Yellow, for creating such a wonderful game

## License

Rustic Yellow is licensed under the MIT License. See the LICENSE file for details.
