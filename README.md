# advent-of-code
[Advent of Code](https://adventofcode.com) (AoC) solutions library with solutions for 2017, 2018, and 2019. All written in Rust as a learning project. Does not have full coverage for all days. There is also a simple cli tool that uses it called `aoc-cli` and a wasm version of the lib for use in node or browsers created with [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [wasm-pack](https://github.com/rustwasm/wasm-pack) called `aoc-wasm` that can be directly imported as an npm module from `aoc-wasm/pkg`.

## Install CLI

To use and install the cli tool that uses the library with `cargo`, clone the repo and run `cargo install --path .` in the project directory. If cargo was installed using `rustup` the resulting binary ought to be in your PATH as `aoc-cli`. Uninstall with `cargo uninstall aoc-cli`.

## CLI Usage

aoc-cli:
```
Advent of Code Cli 0.1.0
Michael Rutter <michael.john.rutter@gmail.com>
Solves Advent of Code problems

USAGE:
    aoc-cli [OPTIONS] --day <day> --year <year>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <day>      Set the day to solve
    -p, --path <path>    Set the input file as problem input
    -y, --year <year>    Set the year
```

### Examples

#### Reading input from a file
```sh
aoc-cli --day 1 --year 2017 --path ./input
```

#### Using stdin
```sh
cat ./input | aoc-cli --day 1 --year 2017
```
