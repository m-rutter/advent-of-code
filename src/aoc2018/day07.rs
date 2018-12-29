use crate::{error, Solution};
use pest::Parser;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let conditionals = parse(&input);

    if conditionals.is_empty() {
        Err(error::Error::msg(&"No conditionals parsed from input"))?
    }

    Ok(Solution {
        part_one: compute_sequence(&conditionals),
        part_two: String::new(),
    })
}

fn parse(s: &str) -> Vec<Dependency> {
    s.lines()
        .filter_map(|line| Dependency::from_str(line.trim()).ok())
        .collect()
}

fn compute_sequence(dependencies: &[Dependency]) -> String {
    let sequence: VecDeque<char> = VecDeque::new();
    let mut nodes_without_incoming_edges: Vec<_> = nodes_without_incoming_edges(&dependencies)
        .into_iter()
        .collect();

    nodes_without_incoming_edges.sort();

    sequence.iter().collect()
}

fn nodes_without_incoming_edges(dependencies: &[Dependency]) -> HashSet<char> {
    let antecedent_set: HashSet<char> = dependencies
        .into_iter()
        .map(|dependency| dependency.antecedent)
        .collect();

    let consequent_set: HashSet<char> = dependencies
        .into_iter()
        .map(|dependency| dependency.consequent)
        .collect();

    antecedent_set
        .difference(&consequent_set)
        .cloned()
        .collect()
}

mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "aoc2018/day07.pest"]
    pub struct Day07Parser;
}

struct Dependency {
    antecedent: char,
    consequent: char,
}

impl FromStr for Dependency {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Dependency, error::Error> {
        let mut pair = parser::Day07Parser::parse(parser::Rule::conditional, s)?
            .next()
            .ok_or_else(|| error::Error::msg(&"No conditional in line"))?
            .into_inner();

        let antecedent = pair
            .next()
            .expect("should be impossible")
            .as_str()
            .chars()
            .next()
            .expect("should be impossible");

        let consequent = pair
            .next()
            .expect("should be impossible")
            .as_str()
            .chars()
            .next()
            .expect("should be impossible");

        Ok(Dependency {
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

        let dependencies = parse(&input);

        assert_eq!(dependencies.len(), 7);
        assert_eq!(dependencies[0].antecedent, 'C');
        assert_eq!(dependencies[0].consequent, 'A');
        assert_eq!(dependencies[6].antecedent, 'F');
        assert_eq!(dependencies[6].consequent, 'E');

        let nodes_without_incoming_edges = nodes_without_incoming_edges(&dependencies);

        assert!(nodes_without_incoming_edges.contains(&'C'));
        assert!(!nodes_without_incoming_edges.contains(&'A'));

        // assert_eq!(compute_sequence(&conditionals), "CABDFE");
    }
}
