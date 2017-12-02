extern crate failure;

use failure::Error;

use std::io::{self, Read};
use std::process;

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
    let mut v = v.clone();

    let mut sum_of_even_divisons = 0;

    for row in v.iter() {
        for (row_elem_index, row_elem) in row.iter().enumerate() {
            for row_elem_pair in row[row_elem_index + 1..].iter() {
                println!("{}, {}", row_elem, row_elem_pair);
                if row_elem % row_elem_pair == 0 {
                    sum_of_even_divisons = sum_of_even_divisons + row_elem / row_elem_pair;
                }
            }
        }
    }

    for row in v.iter_mut() {
        row.reverse();
        for (row_elem_index, row_elem) in row.iter().enumerate() {
            for row_elem_pair in row[row_elem_index + 1..].iter() {
                println!("{}, {}", row_elem, row_elem_pair);
                if row_elem % row_elem_pair == 0 {
                    sum_of_even_divisons = sum_of_even_divisons + row_elem / row_elem_pair;
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

    let lines: Vec<&str> = buff.lines().collect();

    let mut v: Vec<Vec<u32>> = Vec::new();

    for line in lines.iter() {
        let mut row: Vec<u32> = Vec::new();

        for sub in line.split_whitespace() {
            match sub.parse() {
                Ok(num) => row.push(num),
                _ => continue,
            };
        }

        v.push(row);
    }

    Ok(v)
}
