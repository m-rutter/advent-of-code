use crate::{error, Solution};
use std::collections::HashSet;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let v: Vec<char> = input.chars().collect();

    Ok(Solution {
        part_one: reduce_polymer(v.iter()).to_string(),
        part_two: optimial_polymer_length(&v).to_string(),
    })
}

fn reduce_polymer<'a>(input: impl Iterator<Item = &'a char>) -> u32 {
    let mut v: Vec<char> = Vec::new();

    for ch1 in input {
        match v.last() {
            None => v.push(*ch1),
            Some(ch2) => {
                if ch1.to_ascii_lowercase() == ch2.to_ascii_lowercase() && ch1 != ch2 {
                    v.pop();
                } else {
                    v.push(*ch1)
                }
            }
        }
    }

    v.len() as u32
}

fn optimial_polymer_length(input: &[char]) -> u32 {
    let char_set: HashSet<char> = input.iter().map(|c| c.to_ascii_lowercase()).collect();

    char_set
        .iter()
        .map(|c1| {
            let v = input.iter().filter(|c2| *c1 != c2.to_ascii_lowercase());

            reduce_polymer(v)
        })
        .min()
        .unwrap() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day05_2018_original_examples() {
        let input = "dabAcCaCBAcCcaDA";

        let v: Vec<char> = input.chars().collect();

        assert_eq!(reduce_polymer(v.iter()), 10);

        assert_eq!(optimial_polymer_length(&v), 4);
    }
}
