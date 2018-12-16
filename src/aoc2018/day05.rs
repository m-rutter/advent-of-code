use crate::{error, AoCSolution};
use std::collections::HashSet;

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    Ok(AoCSolution {
        part_one: reduce_polymer(&input).len().to_string(),
        part_two: optimial_polymer_length(&input).to_string(),
    })
}

fn reduce_polymer(input: &str) -> String {
    let mut current_lenth = input.len();
    let mut new_length = 0;
    let mut s = String::from(input);

    while current_lenth != new_length {
        current_lenth = new_length;

        let mut new_string = String::new();
        {
            let mut peakable = s.chars().peekable();

            loop {
                let current = peakable.next();
                let next = peakable.peek();

                if let Some(current_char) = current {
                    if let Some(next_char) = next {
                        if current_char.to_ascii_lowercase() == next_char.to_ascii_lowercase()
                            && current_char != *next_char
                        {
                            peakable.next();
                            continue;
                        }
                    }
                    new_string.push(current_char);
                } else {
                    break;
                }
            }
        }
        new_length = new_string.len();
        s = new_string;
    }

    s
}

fn optimial_polymer_length(input: &str) -> u32 {
    let char_set: HashSet<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    char_set
        .iter()
        .map(|c1| {
            let s: String = input
                .chars()
                .filter(|c2| *c1 != c2.to_ascii_lowercase())
                .collect();

            reduce_polymer(&s).len()
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

        assert_eq!(reduce_polymer(&input), "dabCBAcaDA");

        assert_eq!(optimial_polymer_length(&input), 4);
    }
}
