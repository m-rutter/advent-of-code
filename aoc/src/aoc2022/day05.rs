use std::str::FromStr;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let (layout, procedures) = input.split_once("\n\n").ok_or_else(|| {
        anyhow::anyhow!("Expected layout and list of procedures sepereated by empty new line")
    })?;

    let mut slots = gen_initial_layout(layout);
    let mut slots2 = slots.clone();

    let procedures = procedures
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Procedure>>>()?;

    for procedure in procedures {
        for _ in 0..procedure.amount {
            let item = slots[procedure.from].pop().unwrap();
            slots[procedure.to].push(item);
        }
    }

    // for procedure in procedures {
    //     for _ in 0..procedure.amount {
    //         let item = slots[procedure.from].pop().unwrap();
    //         slots[procedure.to].push(item);
    //     }
    // }

    let part_one = slots
        .iter()
        .map(|col| col.last().unwrap())
        .collect::<String>();

    Ok(Solution {
        part_one: part_one,
        part_two: "".to_string(),
    })
}

fn gen_initial_layout(layout: &str) -> Vec<Vec<char>> {
    let (layout, slots) = layout.rsplit_once('\n').unwrap();
    let mut slots: Vec<Vec<char>> = vec![vec![]; slots.split_whitespace().count()];

    for line in layout.lines().rev() {
        for (index, item) in line.chars().chunks(4).into_iter().enumerate() {
            let item = item.into_iter().skip(1).take(1).next().unwrap();

            if !item.is_whitespace() {
                dbg!(index, &item);
                slots[index].push(item);
            }
        }
    }

    slots
}

#[derive(Debug)]
struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)")
        .expect("Error compiling regex")
});

impl FromStr for Procedure {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RE
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("expecting procedure description"))?;

        let amount: usize = captures["amount"].parse().unwrap();
        let from: usize = captures["from"].parse().unwrap();
        let to: usize = captures["to"].parse().unwrap();

        Ok(Procedure {
            amount: amount,
            from: from - 1,
            to: to - 1,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"    [D]     
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "CMZ");
        assert_eq!(solution.part_two, "MCD");
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day05");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "MQTPGLLDN");
        assert_eq!(solution.part_two, "");
    }
}
