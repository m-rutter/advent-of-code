use failure::Error;
use std::collections::HashMap;
use std::process;

use super::{AoCError, AoCSolution, Config};

pub fn run(config: &Config) -> Result<AoCSolution, AoCError> {
    let input = parser(&config.input).unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let (part_one, part_two) = relocate_until_repeat(&input);

    Ok(AoCSolution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

fn relocate_until_repeat(banks: &[u32]) -> (u32, u32) {
    let mut set = HashMap::new();
    let loop_size;
    set.insert(banks.to_vec(), 0);

    let mut banks = banks.to_vec();

    let mut cycles = 0;

    loop {
        cycles += 1;

        let max_bank_index = banks
            .iter()
            .max()
            .and_then(|max| {
                banks
                    .iter()
                    .enumerate()
                    .find(|&(_, bank_blocks)| bank_blocks == max)
            })
            .expect("There shouldn't an empty vector")
            .0;

        cycle(&mut banks, max_bank_index);

        if set.contains_key(&banks) {
            loop_size = cycles - set[&banks];

            break;
        }

        set.insert(banks.clone(), cycles);
    }

    (cycles, loop_size)
}

fn cycle(banks: &mut Vec<u32>, start: usize) {
    let length = banks.len();
    let mut blocks = banks[start];
    banks[start] = 0;
    let mut current = start;

    while blocks > 0 {
        current += 1;

        if current == length {
            current = 0
        };

        banks[current] += 1;
        blocks -= 1;
    }
}

fn parser(input: &str) -> Result<Vec<u32>, Error> {
    let banks = input
        .trim()
        .split_whitespace()
        .filter_map(|bank| bank.parse().ok())
        .collect();

    Ok(banks)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input");

        let config = Config {
            day: 1,
            input: input.to_string(),
        };

        let result = run(&config).unwrap();

        assert_eq!(result.part_one, "12841");
        assert_eq!(result.part_two, "8038");
    }

}
