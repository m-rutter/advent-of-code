//! AoC 2021 module
pub mod day01;
pub mod day02;
pub mod day03;

use anyhow::bail;
use crate::{Config, Solution};

pub fn run(config: &Config) -> anyhow::Result<Solution> {
    match config.day {
        1 => day01::run(&config.input),
        2 => day02::run(&config.input),
        3 => day03::run(&config.input),
        _ => bail!("not implemented yet"),
    }
}
