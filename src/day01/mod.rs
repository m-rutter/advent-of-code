use failure::Error;
use std::io::{self, Read};
use std::process;

pub fn run() {
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
    let str = include_str!("input");

    let v: Vec<u32> = str.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}

fn circular_match_and_sum(v: &[u32], offset: usize) -> u32 {
    v.iter()
        .cycle()
        .skip(offset)
        .zip(v.iter())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None }) // if blocks are expressions? awesome!
        .sum()
}
