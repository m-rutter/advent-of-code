extern crate failure;

use std::io::{self, Read};
use std::process;
use failure::Error;

fn main() {
    let mut parsed_input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    if parsed_input.len() >= 3 {
        let last = parsed_input[parsed_input.len() - 1];
        parsed_input.insert(0, last);
    }

    let mut sum = 0;

    let mut p = parsed_input.iter().peekable();

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

fn parse_input() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let v: Vec<u32> = buff.chars().filter_map(|c| c.to_digit(10)).collect();

    Ok(v)
}
