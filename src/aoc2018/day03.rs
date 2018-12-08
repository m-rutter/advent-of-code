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

    unimplemented!()
}

fn parse(input: &str) -> Vec<Claim> {
    let mut claims = vec![];

    let file = Day03Parser::parse(Rule::file, input)
        .expect("unable to parse input")
        .next()
        .unwrap();

    for record in file.into_inner() {
        if let Rule::record = record.as_rule() {
            let mut record = record.into_inner();
            claims.push(Claim {
                id: record.next().unwrap().as_str().parse().unwrap(),
                left: record.next().unwrap().as_str().parse().unwrap(),
                top: record.next().unwrap().as_str().parse().unwrap(),
                height: record.next().unwrap().as_str().parse().unwrap(),
                width: record.next().unwrap().as_str().parse().unwrap(),
            });
        }
    }

    claims
}
