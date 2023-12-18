use num::integer::lcm;
use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let network = Network::from_str(input)?;

    let steps = network.count_steps(Mode::Mortal)?;
    let ghost_steps = network.count_ghost_steps()?;

    Ok(Solution {
        part_one: steps.to_string(),
        part_two: ghost_steps.to_string(),
    })
}

static NODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<label>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap());

#[derive(Debug)]
struct Network {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode<'a> {
    Mortal,
    Ghost(&'a str),
}

impl Network {
    fn count_ghost_steps(&self) -> Result<usize> {
        Ok(self
            .nodes
            .iter()
            .filter(|(label, _)| label.ends_with('A'))
            .map(|(label, _)| label.to_owned())
            .map(|start| self.count_steps(Mode::Ghost(&start)).unwrap())
            .fold(1, |acc, step| lcm(acc, step)))
    }

    fn count_steps(&self, mode: Mode) -> Result<usize> {
        let mut iter = self.instructions.iter().cycle().enumerate();

        let mut current_node = if let Mode::Ghost(start) = mode {
            self.nodes.get(start).unwrap()
        } else {
            self.nodes.get("AAA").unwrap()
        };

        loop {
            let (step, direction) = iter.next().unwrap();

            if mode == Mode::Mortal {
                if current_node.label == "ZZZ" {
                    return Ok(step);
                }
            } else {
                if current_node.label.ends_with('Z') {
                    return Ok(step);
                }
            }

            match direction {
                Direction::Left => {
                    current_node = self.nodes.get(&current_node.left).unwrap();
                }
                Direction::Right => {
                    current_node = self.nodes.get(&current_node.right).unwrap();
                }
            }
        }
    }
}

impl FromStr for Network {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut instructions = Vec::new();
        let mut nodes = HashMap::new();

        let sections = s.trim().split("\n\n").collect::<Vec<_>>();

        if sections.len() != 2 {
            Err(anyhow!("invalid input, expected 2 sections"))?;
        }

        for c in sections[0].trim().chars() {
            instructions.push(Direction::try_from(&c)?);
        }

        for line in sections[1].lines() {
            let node = Node::from_str(line.trim())?;

            nodes.insert(node.label.clone(), node);
        }

        Ok(Network {
            instructions,
            nodes,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<&char> for Direction {
    type Error = AoCError;

    fn try_from(s: &char) -> Result<Self> {
        Ok(match s {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Err(anyhow!("invalid direction {s}"))?,
        })
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let captures = NODE_RE
            .captures(s)
            .ok_or_else(|| anyhow!("Failed to parse node: {s}"))?;

        let label = captures
            .name("label")
            .expect("named group called label")
            .as_str();
        let left = captures
            .name("left")
            .expect("named group called left")
            .as_str();

        let right = captures
            .name("right")
            .expect("named group called right")
            .as_str();

        Ok(Node {
            label: label.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offical_example_1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let network = Network::from_str(input).unwrap();

        let count = network.count_steps(Mode::Mortal).unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_offical_example_2() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let network = Network::from_str(input).unwrap();

        let count = network.count_steps(Mode::Mortal).unwrap();

        assert_eq!(count, 6);
    }

    #[test]
    fn test_offical_example_3() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let network = Network::from_str(input).unwrap();

        let count = network.count_ghost_steps().unwrap();

        assert_eq!(count, 6);
    }

    #[test]
    fn test_offical_input() {
        let input = include_str!("./input/day08");

        let solution = run(&input).unwrap();

        assert_eq!(solution.part_one, "20777");
        assert_eq!(solution.part_two, "13289612809129");
    }
}
