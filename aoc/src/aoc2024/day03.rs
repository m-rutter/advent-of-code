use crate::{error::Result, Solution};
use once_cell::sync::Lazy;
use regex::Regex;

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Error compiling regex"));

pub fn run(input: &str) -> Result<Solution> {
    let sum: u32 = RE
        .captures_iter(input)
        .map(|mul| {
            let x: u32 = mul.get(1).unwrap().as_str().parse().unwrap();
            let y: u32 = mul.get(2).unwrap().as_str().parse().unwrap();

            x * y
        })
        .sum();

    Ok(Solution {
        part_one: sum.to_string(),
        part_two: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offical_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "161");
    }

    #[test]
    fn offical_input() {
        let input = include_str!("./input/day03");
        let result = run(input).unwrap();

        assert_eq!(result.part_one, "156388521");
        assert_eq!(result.part_two, "");
    }
}
