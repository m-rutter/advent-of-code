use failure::Error;
use std::collections::HashMap;
use std::io::{self, Read};
use std::process;

pub fn run() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let (part_one, part_two) = relocate_until_repeat(&input);

    println!("Part one solution is: {}", part_one);
    println!("Part two solution is: {}", part_two);
}

fn relocate_until_repeat(banks: &Vec<u32>) -> (u32, u32) {
    let mut set = HashMap::new();
    let loop_size;
    set.insert(banks.clone(), 0);

    let mut banks = banks.clone();

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
            loop_size = cycles - set.get(&banks).unwrap();

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

fn parser() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let banks = buff
        .trim()
        .split_whitespace()
        .filter_map(|bank| bank.parse().ok())
        .collect();

    Ok(banks)
}
