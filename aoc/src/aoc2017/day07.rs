use crate::{error, Solution};
use pest::{self, Parser};
use std::collections::{HashMap, HashSet};

mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "aoc2017/day07.pest"]
    pub struct Day07Parser;
}

#[derive(Debug)]
struct Node {
    #[allow(unused)]
    id: String,
    #[allow(unused)]
    weight: u32,
    children: Option<Vec<String>>,
}

/// Compute the solution to day 7 of AoC 2017
pub fn run(input: &str) -> error::AoCResult<Solution> {
    let nodes = parser(&input)?;

    if nodes.is_empty() {
        Err(error::ErrorKind::InputParse)?
    }

    let _root = find_root_node(&nodes);

    unimplemented!()
}

fn find_root_node(nodes: &HashMap<String, Node>) -> String {
    let mut names: HashSet<&str> = nodes.keys().map(|key| key.as_str()).collect();

    for node in nodes.values() {
        if let Some(children) = &node.children {
            for child in children.iter() {
                names.remove(child.as_str());
            }
        }
    }

    names
        .into_iter()
        .nth(0)
        .expect("There should be at least one node")
        .to_string()
}

fn parser(input: &str) -> error::AoCResult<HashMap<String, Node>> {
    let mut nodes = HashMap::new();

    let file = parser::Day07Parser::parse(parser::Rule::file, input)?
        .next()
        .ok_or_else(|| error::Error::msg(&"Unable to parse any records"))?;

    for record in file.into_inner() {
        if let parser::Rule::node = record.as_rule() {
            let mut id = "".to_string();
            let mut weight = 0;
            let mut children = None;
            for value in record.into_inner() {
                match value.as_rule() {
                    parser::Rule::id => {
                        id = value.as_str().to_string();
                    }
                    parser::Rule::weight => {
                        weight = value.as_str().parse()?;
                    }
                    parser::Rule::children => {
                        children = Some(
                            value
                                .into_inner()
                                .map(|child| child.as_str().to_string())
                                .collect(),
                        )
                    }
                    _ => {}
                }
            }
            nodes.insert(
                id.clone(),
                Node {
                    id: id,
                    weight: weight,
                    children: children,
                },
            );
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    // TODO: Solve test and put the offical answer here
    // use super::*;

    // #[test]
    // fn matches_offical_result() {
    //     let input = include_str!("./input/day07");

    //     let config = Config {
    //         day: 1,
    //         input: input.to_string(),
    //     };

    //     let _result = run(&config.input).unwrap();
    // }
}
