use anyhow::{Context, Result};
use advent_of_code::{solve_day, Config};
use clap::Parser;
use std::fs::File;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[command(name = "Advent of Code Cli", about = "Solves Advent of Code problems")]
struct Opt {
    /// Set the day to solve
    #[arg(short = 'd', long = "day")]
    day: u8,
    /// Set the year
    #[arg(short = 'y', long = "year")]
    year: u16,
    /// Set the input file as problem input
    #[arg(short = 'p', long = "path")]
    path: Option<PathBuf>,
}

fn main() {
    let opt = Opt::parse();

    if let Err(err) = run(&opt) {
        // anyhow's alternate Display flattens the context chain: "context: cause"
        if io::stderr().is_terminal() {
            eprintln!("\x1b[31merror:\x1b[0m {err:#}");
        } else {
            eprintln!("error: {err:#}");
        }
        process::exit(1);
    }
}

fn run(opt: &Opt) -> Result<()> {
    let config = create_config(opt)?;

    let solution = solve_day(&config)
        .with_context(|| format!("failed to solve year {} day {}", opt.year, opt.day))?;

    println!(
        "Solution to part 1 of day {} is: \n{}",
        config.day, solution.part_one
    );
    println!(
        "Solution to part 2 of day {} is: \n{}",
        config.day, solution.part_two
    );

    Ok(())
}

fn create_config(opt: &Opt) -> Result<Config> {
    let input = read_input_data(&opt.path)?;

    Ok(Config::new(opt.year, opt.day, input))
}

fn read_input_data(input_type: &Option<PathBuf>) -> Result<String> {
    let mut buff = String::new();

    match input_type {
        Some(path) => {
            let mut file = File::open(path)
                .with_context(|| format!("failed to open {}", path.display()))?;
            file.read_to_string(&mut buff)
                .with_context(|| format!("failed to read {}", path.display()))?;
        }
        None => {
            let mut stdin = io::stdin();
            stdin
                .read_to_string(&mut buff)
                .context("failed to read input from stdin")?;
        }
    }

    Ok(buff)
}
