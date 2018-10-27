use advent_of_code_2017::{run_day, Config};

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Advent of Code 2017 Solver in Rust")
        .version("0.1.0")
        .author("Michael Rutter <michael.john.rutter@gmail.com>")
        .about("Solves 2017 Advent of Code problems")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("NUMBER")
                .help("Set the day to solve")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("INPUT PATH").help("Set the input file as problem input"))
        .get_matches();

    run(matches).unwrap();
}

fn run(matches: ArgMatches) -> Result<(), String> {
    let config = Config {
        day: 24,
        stdin: false,
        path: Some(String::from("dd")),
    };

    run_day(config)
}
