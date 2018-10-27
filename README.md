# advent-of-code-2017
[Advent of Code 2017](https://adventofcode.com/2017) solutions library crate and a simple cli tool that uses it. All written in Rust as a learning project.

## Install CLI

To use and install the cli tool that uses the library with `cargo`, clone the repo and run `cargo install --path .` in the project directory. If cargo was installed using `rustup` the resulting binary ought to be in your PATH as `aoc-cli`. Uninstall with `cargo uninstall aoc-cli`.

## CLI Usage

aoc-cli:
```
Advent of Code 2017 CLI 0.1.0
Michael Rutter <michael.john.rutter@gmail.com>
Solves 2017 Advent of Code problems

USAGE:
    aoc-cli [OPTIONS] --day <NUMBER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <NUMBER>         Set the day to solve
    -p, --path <INPUT PATH>    Set the input file as problem input
```

### Examples

#### Reading input from a file
```sh
aoc-cli --day 1 --path ./input
```

#### Using stdin
```sh
cat ./input | aoc-cli --day 1
```

