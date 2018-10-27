mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub struct Config {
    pub day: u8,
    pub stdin: bool,
    pub path: Option<String>,
}

pub fn run_day(config: Config) -> Result<(), String> {
    match config.day {
        1 => day01::run(config),
        2 => day02::run(config),
        3 => day03::run(config),
        4 => day04::run(config),
        5 => day05::run(config),
        6 => day06::run(config),
        7 => day07::run(config),
        _ => return Err(String::from("That day is not yet supported")),
    }

    Ok(())
}
