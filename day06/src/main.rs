extern crate failure;

use failure::Error;
use std::io::{self, Read};
use std::process;


fn main() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    println!("{:?}", input)
}


fn parser() -> Result<Vec<u32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let banks = buff.split_whitespace()
        .filter_map(|bank| bank.parse().ok())
        .collect();

    Ok(banks)
}
