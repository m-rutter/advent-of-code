use crate::{error, AoCSolution};
use pest::{self, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "aoc2018/day03.pest"]
struct Day03Parser;

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    height: u32,
    width: u32,
}

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let claims = parse(&input);

    Ok(AoCSolution {
        part_one: count_overlapping_claims(&claims).to_string(),
        part_two: String::new(),
    })
}

fn count_overlapping_claims(claims: &[Claim]) -> u32 {
    let mut cloth: HashMap<(u32, u32), u32> = HashMap::new();
    let mut count = 0;

    for claim in claims.iter() {
        let x_range = claim.left..(claim.width + claim.left);
        let y_range = claim.top..(claim.height + claim.top);

        for x in x_range {
            for y in y_range.clone() {
                let value = cloth.entry((x, y)).or_insert(0);

                *value += 1;

                if *value == 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse(input: &str) -> Vec<Claim> {
    let mut claims = vec![];

    if let Ok(files) = Day03Parser::parse(Rule::file, input) {
        for file in files {
            for record in file.into_inner() {
                if let Rule::record = record.as_rule() {
                    let mut record_pair = record.into_inner();

                    // if the pest libary is to be believed, it is impossible
                    // for these Results to be Result::Err at this point
                    claims.push(Claim {
                        id: record_pair.next().unwrap().as_str().parse().unwrap(),
                        left: record_pair.next().unwrap().as_str().parse().unwrap(),
                        top: record_pair.next().unwrap().as_str().parse().unwrap(),
                        height: record_pair.next().unwrap().as_str().parse().unwrap(),
                        width: record_pair.next().unwrap().as_str().parse().unwrap(),
                    });
                }
            }
        }
    }

    claims
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_result_original_example() {
        let s = r#"
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
        "#;

        let c = parse(s);

        assert_eq!(count_overlapping_claims(&c), 4);
    }
}
