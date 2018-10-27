use failure::Error;
use std::process;

use super::{AoCSolution, Config};

pub fn run(config: &Config) -> AoCSolution {
    let input = parser(&config.input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = steps_to_exit(&input, |_| 1);
    let part_two = steps_to_exit(&input, |item| if item >= 3 { -1 } else { 1 });

    AoCSolution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    }
}

fn steps_to_exit<T>(jumps: &Vec<i32>, change_jump: T) -> u32
where
    T: Fn(i32) -> i32,
{
    let mut jumps = jumps.clone();

    let max_position = jumps.len() as i32 - 1;
    let mut steps = 0;
    let mut position: usize = 0;

    loop {
        steps += 1;

        let next_position = jumps[position] + position as i32;
        jumps[position] += change_jump(jumps[position]);

        if next_position > max_position || next_position < 0 {
            break;
        }

        position = next_position as usize;
    }
    steps
}

fn parser(input: &str) -> Result<Vec<i32>, Error> {
    let jumps = input.lines().filter_map(|line| line.parse().ok()).collect();

    Ok(jumps)
}

#[test]
fn matches_offical_result() {
    let input = include_str!("./input");

    let config = Config {
        day: 1,
        input: input.to_string(),
    };

    let result = run(&config);

    assert_eq!(result.part_one, "325922");
    assert_eq!(result.part_two, "24490906");
}
