use advent_of_code::{solve_day, Config};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Advent of Code Cli", about = "Solves Advent of Code problems")]
struct Opt {
    /// Set the day to solve
    #[structopt(short = "d", long = "day")]
    day: u8,
    /// Set the year
    #[structopt(short = "y", long = "year")]
    year: u16,
    /// Set the input file as problem input
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let config = create_config(&opt).unwrap_or_else(|err| {
        eprintln!("Error reading input: {}", err);
        process::exit(1);
    });

    let solution = solve_day(&config).unwrap_or_else(|err| {
        eprintln!("Error when attempting to solve day: {}", err);
        process::exit(1);
    });

    println!(
        "Solution to part 1 of day {} is: \n{}",
        config.day, solution.part_one
    );
    println!(
        "Solution to part 2 of day {} is: \n{}",
        config.day, solution.part_two
    );
}

fn create_config(opt: &Opt) -> io::Result<Config> {
    let input = read_input_data(&opt.path)?;

    Ok(Config::new(opt.year, opt.day, input))
}

fn read_input_data(input_type: &Option<PathBuf>) -> io::Result<String> {
    let mut buff = String::new();

    match input_type {
        Some(path) => {
            let mut file = File::open(path)?;
            file.read_to_string(&mut buff)?;
        }
        None => {
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buff)?;
        }
    }

    Ok(buff)
}
