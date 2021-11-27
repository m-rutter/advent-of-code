# advent-of-code

[Advent of Code](https://adventofcode.com) (AoC) solutions for 2017, 2018, 2019,
2020, and a partial intcode implementation. All written in Rust as a learning
project. Does not have full coverage for all days as I find they end up becoming
too time consuming eventually.

## Install CLI

To use and install the cli tool that uses the library with `cargo`, clone the
repo and run `cargo install --path .` in the project directory. If cargo was
installed using `rustup` the resulting binary ought to be in your PATH as
`aoc-cli`. Uninstall with `cargo uninstall aoc-cli`.

## CLI Usage

aoc-cli:

```
Advent of Code Cli 0.1.0
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

## Solution Coverage

| Day | 2017 | 2018 | 2019 | 2020 | 2021 |
| --- | ---- | ---- | ---- | ---- | ---- |
| 1   | [x]  | [x]  | [x]  | [x]  | [ ]  |
| 2   | [x]  | [x]  | [x]  | [x]  | [ ]  |
| 3   | [x]  | [x]  | [x]  | [ ]  | [ ]  |
| 4   | [x]  | [x]  | [x]  | [ ]  | [ ]  |
| 5   | [x]  | [x]  | [x]  | [ ]  | [ ]  |
| 6   | [x]  | [x]  | [ ]  | [ ]  | [ ]  |
| 7   | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 8   | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 9   | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 10  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 11  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 12  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 13  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 14  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 15  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 16  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 17  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 18  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 19  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 20  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 21  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 22  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 23  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 24  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
| 25  | [ ]  | [ ]  | [ ]  | [ ]  | [ ]  |
