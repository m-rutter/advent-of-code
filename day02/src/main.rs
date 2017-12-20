extern crate failure;
extern crate itertools;

use failure::Error;

use std::io::{self, Read};
use std::process;
use itertools::Itertools;

fn main() {
    let input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let checksum = gen_checksum(&input);
    let sum_of_even_divisons = users_are_odd(&input);


    println!("Part one solution: {}", checksum);
    println!("Part two solution: {}", sum_of_even_divisons);
}

fn gen_checksum(v: &[Vec<u32>]) -> u32 {
    v.iter()
        .map(|row| {
            let max = match row.iter().max() {
                Some(&num) => num,
                None => 0,
            };

            let min = match row.iter().min() {
                Some(&num) => num,
                None => 0,
            };

            max - min
        })
        .sum()
}

fn users_are_odd(v: &[Vec<u32>]) -> u32 {
    v.iter()
        .map(|inner_v| {
            inner_v
                .iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    let sum = if a % b == 0 { a / b } else { 0 };

                    if b % a == 0 {
                        b / a + sum
                    } else {
                        sum
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn parse_input() -> Result<Vec<Vec<u32>>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let table: Vec<Vec<u32>> = buff.lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|elem| elem.parse().ok())
                .collect()
        })
        .collect();

    Ok(table)
}
