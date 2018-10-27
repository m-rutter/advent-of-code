use aoc_2017::{solve_day, Config};
use std::fs::File;
use std::io::{self, Read};

use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = App::new("Advent of Code 2017 CLI")
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
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("INPUT PATH")
                .help("Set the input file as problem input"),
        )
        .get_matches();

    run(&matches);
}

fn run(matches: &ArgMatches) {
    let day = matches.value_of("day").unwrap();
    let config = create_config(matches);

    match solve_day(&config) {
        Ok(solution) => {
            println!(
                "Solution to part 1 of day {} is: {}",
                day, solution.part_one
            );
            println!(
                "Solution to part 2 of day {} is: {}",
                day, solution.part_two
            );
        }
        Err(s) => {
            eprintln!("{}", s);
        }
    }
}

enum InputType {
    Stdin,
    InputPath(String),
}

fn create_config(matches: &ArgMatches) -> Config {
    let day = matches.value_of("day").unwrap();
    let day: u8 = day.parse().unwrap();

    let input_type = match matches.value_of("path") {
        Some(path) => InputType::InputPath(path.to_string()),
        None => InputType::Stdin,
    };

    let input = get_input_data(input_type);

    Config::new(day, input)
}

fn get_input_data(input_type: InputType) -> String {
    let mut buff = String::new();

    match input_type {
        InputType::Stdin => {
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buff).unwrap();
        }
        InputType::InputPath(path) => {
            let mut file = File::open(path).unwrap();
            file.read_to_string(&mut buff).unwrap();
        }
    }

    buff
}
