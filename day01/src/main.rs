extern crate failure;

use std::io::{self, Read};
use std::env;
use std::process;
use failure::Error;

fn main() {
    let parsed_input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let mut args = env::args();
    args.next();

    match args.next() {
        Some(arg) => if arg == "2" {
            part_two(parsed_input);
        } else {
            part_one(parsed_input);
        },
        None => {
            part_one(parsed_input);
        }
    }
}

fn part_one(mut v: Vec<u32>) {
    if v.len() >= 3 {
        let last = v[v.len() - 1];
        v.insert(0, last);
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

    println!("{:?}", sum);
}

fn part_two(v: Vec<u32>) {
    let mut sum = 0;
    let length = v.len();
    let distance = length / 2;

    for (index, elem) in v.iter().enumerate() {
        let mut pair_index = index + distance;

        if pair_index > length - 1 {
            pair_index = pair_index - length;
        }

        if &v[pair_index] == elem {
            sum = sum + elem;
        }
    }

    println!("{}", sum);
}

fn parse_input() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let v: Vec<u32> = buff.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}
