extern crate failure;

use std::io::{self, Read};
use std::process;
use failure::Error;

fn main() {
    let parsed_input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });



    part_one(&parsed_input);
    let part_two = check_pairs(&parsed_input, &parsed_input.len() / 2);

    println!("Part two solution: {}", part_two);
}

fn part_one(v: &Vec<u32>) {
    let mut v = v.clone();
    let length = v.len();

    if length >= 3 {
        if let Some(&a) = v.last() {
            v.insert(0, a);
        }
    }

    let mut sum = 0;

    let mut p = v.iter().peekable();

    loop {
        let num = match p.next() {
            Some(n) => n,
            None => break,
        };

        match p.peek() {
            Some(&next_num) => if num == next_num {
                sum = num + sum;
            },
            None => break,
        }
    }

    println!("Part one solution: {}", sum);
}

fn check_pairs(v: &Vec<u32>, look_ahead: usize) -> u32 {
    let mut sum = 0;
    let length = v.len();

    for (index, elem) in v.iter().enumerate() {
        let pair_index = index as i32 + look_ahead as i32;
        let pair_index = (pair_index - length as i32).abs() as usize;

        if &v[pair_index] == elem {
            sum = sum + elem;
        }
    }

    sum
}

fn parse_input() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let v: Vec<u32> = buff.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}
