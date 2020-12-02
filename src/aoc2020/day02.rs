use std::convert::TryFrom;

use crate::error;
use crate::Solution;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct PasswordWithPolicy {
    password: String,
    policy: Policy,
}

#[derive(Debug)]
struct Policy {
    param1: usize,
    param2: usize,
    char: char,
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<char>[a-z]):\s(?P<password>[a-z]+)")
        .expect("Error compiling regex")
});

impl PasswordWithPolicy {
    fn pass_sled(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|char| *char == self.policy.char)
            .count();

        count >= self.policy.param1 && count <= self.policy.param2
    }

    fn pass_toboggan(&self) -> bool {
        let char1_match = self
            .password
            .chars()
            .nth(self.policy.param1 - 1)
            .map_or_else(|| false, |char| char == self.policy.char);

        let char2_match = self
            .password
            .chars()
            .nth(self.policy.param2 - 1)
            .map_or_else(|| false, |char| char == self.policy.char);

        dbg!(self, char1_match, char2_match);

        char1_match ^ char2_match
    }
}

impl TryFrom<&str> for PasswordWithPolicy {
    type Error = error::Error;

    fn try_from(s: &str) -> error::AoCResult<PasswordWithPolicy> {
        let captures = RE.captures(s).unwrap();

        let password = &captures["password"];
        let policy_char = &captures["char"];
        let min = &captures["min"];
        let max = &captures["max"];

        Ok(PasswordWithPolicy {
            password: password.to_string(),
            policy: Policy {
                char: policy_char.chars().next().unwrap(),
                param1: min.parse().unwrap(),
                param2: max.parse().unwrap(),
            },
        })
    }
}

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let passwords_with_policy: Vec<_> = input
        .lines()
        .filter_map(|line| PasswordWithPolicy::try_from(line).ok())
        .collect();

    let pass_count_seld = passwords_with_policy
        .iter()
        .filter(|password| password.pass_sled())
        .count();

    let pass_count_toboggan = passwords_with_policy
        .iter()
        .filter(|password| password.pass_toboggan())
        .count();

    Ok(Solution {
        part_one: pass_count_seld.to_string(),
        part_two: pass_count_toboggan.to_string(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn original_example() {
        let input = r#"1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc"#;

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "2");
        assert_eq!(result.part_two, "1");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "396");
        assert_eq!(result.part_two, "428");
    }
}
