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

    part_one(&input);
    part_two(&input);
}

fn part_one(v: &Vec<Vec<u32>>) {
    let mut checksum = 0;

    for row in v.iter() {
        let max = match row.iter().max() {
            Some(&num) => num,
            None => 0,
        };

        let min = match row.iter().min() {
            Some(&num) => num,
            None => 0,
        };

        let difference = max - min;

        checksum = checksum + difference;
    }

    println!("Part one solution: {}", checksum);
}

fn part_two(v: &Vec<Vec<u32>>) {
    let mut sum_of_even_divisons = 0;

    v.iter().map(|row| row);

    for row in v.iter() {
        for (row_elem_index, row_elem) in row.iter().enumerate() {
            for row_elem_pair in row[row_elem_index + 1..].iter() {
                if row_elem % row_elem_pair == 0 {
                    sum_of_even_divisons = sum_of_even_divisons + row_elem / row_elem_pair;
                } else if row_elem_pair % row_elem == 0 {
                    sum_of_even_divisons = sum_of_even_divisons + row_elem_pair / row_elem;
                }
            }
        }
    }


    println!("Part two solution: {}", sum_of_even_divisons);
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
