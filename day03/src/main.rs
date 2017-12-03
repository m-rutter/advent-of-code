extern crate failure;

use std::io::{self, Read};
use std::process;
use failure::Error;

fn main() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });
}

fn parser() -> Result<u64, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let num = buff.parse()?;

    Ok(num)
}
