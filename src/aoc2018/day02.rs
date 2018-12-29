use crate::{error, Solution};
use std::collections::HashMap;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let ids = parse(&input);

    Ok(Solution {
        part_one: compute_checksum(&ids).to_string(),
        part_two: find_common_char(&ids).into_iter().collect(),
    })
}

fn find_common_char(ids: &[&str]) -> Vec<char> {
    let mut common_chars = vec![];

    for (index, id) in ids.iter().enumerate() {
        for id_to_compare in ids.iter().skip(index + 1) {
            let difference_count =
                id.chars()
                    .zip(id_to_compare.chars())
                    .fold(0, |mut acc, (c1, c2)| {
                        if c1 != c2 {
                            acc += 1
                        }

                        acc
                    });

            if difference_count == 1 {
                for (c1, c2) in id.chars().zip(id_to_compare.chars()) {
                    if c1 == c2 {
                        common_chars.push(c1)
                    }
                }
                break;
            }
        }
    }

    common_chars
}

fn compute_checksum(ids: &[&str]) -> u32 {
    let twice_thrice = ids
        .iter()
        .map(|id| {
            let counts: HashMap<char, u32> = id.chars().fold(HashMap::new(), |mut acc, char| {
                *acc.entry(char).or_insert(0) += 1;
                acc
            });

            let twice = counts.values().any(|x| *x == 3);
            let thrice = counts.values().any(|x| *x == 2);

            (twice, thrice)
        })
        .fold((0, 0), |mut acc, (twice, thrice)| {
            if twice {
                acc.0 += 1
            }
            if thrice {
                acc.1 += 1
            }

            acc
        });

    twice_thrice.0 * twice_thrice.1
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}
