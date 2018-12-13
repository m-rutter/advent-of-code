use crate::{error, AoCSolution};
use pest::{self, Parser};
use pest_derive::Parser;

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

    println!("{:?}", claims);

    Ok(AoCSolution {
        part_one: String::new(),
        part_two: String::new(),
    })
}

fn parse(input: &str) -> Vec<Claim> {
    let mut claims = vec![];

    if let Ok(files) = Day03Parser::parse(Rule::file, input) {
        for file in files {
            for record in file.into_inner() {
                if let Rule::record = record.as_rule() {
                    let mut record_pair = record.into_inner();

                    // if the pest libary is to be believed, it is literately
                    // impossible for these to be Result::Err at this point
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
