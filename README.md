# advent-of-code 🎄

[Advent of Code](https://adventofcode.com) (AoC) solutions for 2017, 2018, 2019,
2020, 2021, 2022 and a partial intcode implementation. All written in Rust as a
learning project. Does not have full coverage for all days as I find they end up
becoming too time consuming eventually.

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

| Day | 2017          | 2018          | 2019          | 2020          | 2021          | 2022          | 2023          | 2024          |
| --- | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
| 1   | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: |
| 2   | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: | :star: :star: |
| 3   | :star: :star: | :star: :star: |               |               | :star:        | :star: :star: | :star: :star: | :star: :star: |
| 4   | :star: :star: | :star: :star: |               |               |               | :star: :star: | :star: :star: |               |
| 5   | :star: :star: | :star: :star: |               |               |               | :star: :star: | :star: :star: |               |
| 6   | :star: :star: | :star: :star: |               |               |               | :star: :star: | :star: :star: | :star: :star: |
| 7   | :star:        |               |               |               |               |               | :star: :star: |               |
| 8   |               |               |               |               |               |               | :star: :star: |               |
| 9   |               |               |               |               |               |               | :star: :star: |               |
| 10  |               |               |               |               |               |               |               |               |
| 11  |               |               |               |               |               |               |               |               |
| 12  |               |               |               |               |               |               |               |               |
| 13  |               |               |               |               |               |               |               |               |
| 14  |               |               |               |               |               |               |               |               |
| 15  |               |               |               |               |               |               |               |               |
| 16  |               |               |               |               |               |               |               |               |               |
| 17  |               |               |               |               |               |               |               |               |
| 18  |               |               |               |               |               |               |               |               |
| 19  |               |               |               |               |               |               |               |               |
| 20  |               |               |               |               |               |               |               |               |
| 21  |               |               |               |               |               |               |               |               |
| 22  |               |               |               |               |               |               |               |               |
| 23  |               |               |               |               |               |               |               |               |
| 24  |               |               |               |               |               |               |               |               |
| 25  |               |               |               |               |               |               |               |               |
