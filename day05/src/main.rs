extern crate failure;

use failure::Error;
use std::io::{self, Read};
use std::process;

fn main() {
    let input = parser().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    let part_one = steps_to_exit(&input, |_| 1);
    let part_two = steps_to_exit(&input, |item| if item >= 3 { -1 } else { 1 });

    println!("Solution to part one: {}", part_one);
    println!("Solution to part two: {}", part_two);
}


fn steps_to_exit<T>(v: &Vec<i32>, f: T) -> u32
where
    T: Fn(i32) -> i32,
{
    let mut v = v.clone();

    let max_position = v.len() as i32 - 1;
    let mut steps = 0;
    let mut position: usize = 0;

    loop {
        steps += 1;

        let next_position = v[position] + position as i32;
        v[position] += f(v[position]);

        if next_position > max_position || next_position < 0 {
            break;
        }

        position = next_position as usize;
    }
    steps
}

fn parser() -> Result<Vec<i32>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let jumps = buff.lines().filter_map(|line| line.parse().ok()).collect();

    Ok(jumps)
}
