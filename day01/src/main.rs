extern crate failure;

use std::io::{self, Read};
use std::process;
use failure::Error;

fn main() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = circular_match_and_sum(&input, 1);
    let part_two = circular_match_and_sum(&input, input.len() / 2);

    println!("Part one solution: {}", part_one);
    println!("Part two solution: {}", part_two);
}

fn parser() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let v: Vec<u32> = buff.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}

fn circular_match_and_sum(v: &Vec<u32>, offset: usize) -> u32 {
    let mut sum = 0;
    let length = v.len();

    for (index, elem) in v.iter().enumerate() {
        let mut pair_index = index + offset;

        if pair_index > length - 1 {
            pair_index = pair_index - length;
        }

        if &v[pair_index] == elem {
            sum = sum + elem;
        }
    }
    sum
}
