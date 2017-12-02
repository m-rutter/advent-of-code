extern crate failure;

use std::io::{self, Read};
use std::process;
use failure::Error;

fn main() {
    let parsed_input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = check_pairs(&parsed_input, 1);
    let part_two = check_pairs(&parsed_input, parsed_input.len() / 2);

    println!("Part one solution: {}", part_one);
    println!("Part two solution: {}", part_two);
}


fn parse_input() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let v: Vec<u32> = buff.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}

fn check_pairs(v: &Vec<u32>, distance: usize) -> u32 {
    let mut sum = 0;
    let length = v.len();

    for (index, elem) in v.iter().enumerate() {
        let mut pair_index = index + distance;

        if pair_index > length - 1 {
            pair_index = pair_index - length;
        }

        if &v[pair_index] == elem {
            sum = sum + elem;
        }
    }
    sum
}
