use crate::{error, AoCSolution};
use failure::{format_err, Error};
use pest::Parser;
use std::collections::HashSet;
use std::str::FromStr;

pub fn run(input: &str) -> error::AoCResult<AoCSolution> {
    let conditionals = parse(&input);

    Ok(AoCSolution {
        part_one: compute_sequence(&conditionals),
        part_two: String::new(),
    })
}

fn parse(s: &str) -> Vec<Conditional> {
    s.lines()
        .filter_map(|line| Conditional::from_str(line.trim()).ok())
        .collect()
}

fn compute_sequence(conditionals: &[Conditional]) -> String {
    let mut completed_steps: HashSet<char> = HashSet::new();

    unimplemented!()
}

mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "aoc2018/day07.pest"]
    pub struct Day07Parser;
}

struct Conditional {
    antecedent: char,
    consequent: char,
}

impl FromStr for Conditional {
    type Err = Error;

    fn from_str(s: &str) -> Result<Conditional, Error> {
        let mut pair = parser::Day07Parser::parse(parser::Rule::conditional, s)?
            .next()
            .ok_or_else(|| format_err!("No conditional in line"))?
            .into_inner();

        let antecedent = pair
            .next()
            .expect("should be impossible")
            .as_str()
            .chars()
            .next()
            .unwrap();

        let consequent = pair
            .next()
            .expect("should be impossible")
            .as_str()
            .chars()
            .next()
            .unwrap();

        Ok(Conditional {
            antecedent: antecedent,
            consequent: consequent,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn original_example_day07_2018() {
        let input = r#"
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
        "#;

        let conditionals = parse(&input);

        assert_eq!(conditionals.len(), 7);
        assert_eq!(conditionals[0].antecedent, 'C');
        assert_eq!(conditionals[0].consequent, 'A');
        assert_eq!(conditionals[6].antecedent, 'F');
        assert_eq!(conditionals[6].consequent, 'E');

        assert_eq!(compute_sequence(&conditionals), "CABDFE");
    }
}
