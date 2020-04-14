# rust-series-example1

## Game Of Life

Example sorce code accompanying the blog article miniseries: Rust in a Nutshel, you can find at: https://www.softax.pl/blog

## Installation

To compile and run code you need to instal Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
additional details may be viewed at: <https://www.rust-lang.org/tools/install>

## Usage

In order to run simulation you can simply write on terminal:
```
cargo run -- -g 50 -s 20 -t 0.3
```
or 
```bash
cargo run -- --help
```
for usage information:

```bash
Game of life 

USAGE:
    game_of_life --grid-size <GRID SIZE> --steps <STEPS NUMBER> --threshold <THRESHOLD>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --grid-size <GRID SIZE>    Size of rectangular grid on which Conway's game of life unfolds
    -s, --steps <STEPS NUMBER>     Number of simulation steps
    -t, --threshold <THRESHOLD>    Random initialization threshold
```

## Results
Simulation results are stored as png files in output directory. If you want to make animated gif from those files install gifski tool:

```bash
cargo install gifski
```
and then 

```bash
gifski -o game_of_life.gif output/game_of_life*.png
```

# License

[![CC 4.0][cc-image]][cc-url]

&copy; [Softax](http://softax.pl)

[cc-url]: http://creativecommons.org/licenses/by/4.0/
[cc-image]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg?style=flat-square
