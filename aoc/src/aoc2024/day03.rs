use crate::{error::Result, Solution};
use once_cell::sync::Lazy;
use regex::Regex;

static PART_ONE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Error compiling regex"));

static PART_TWO_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?<mull>mul)\((\d{1,3}),(\d{1,3})\)|(?P<conditional>don't|do)")
        .expect("Error compiling regex")
});

enum State {
    Do,
    Dont,
}

pub fn run(input: &str) -> Result<Solution> {
    let part_one: u32 = PART_ONE_RE
        .captures_iter(input)
        .map(|mul| {
            let x: u32 = mul.get(1).unwrap().as_str().parse().unwrap();
            let y: u32 = mul.get(2).unwrap().as_str().parse().unwrap();

            x * y
        })
        .sum();

    let mut state = State::Do;
    let mut part_two = 0;

    for cap in PART_TWO_RE.captures_iter(input) {
        if let Some(_) = cap.name("mull") {
            if let State::Do = state {
                let x: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
                let y: u32 = cap.get(3).unwrap().as_str().parse().unwrap();

                part_two += x * y
            }
        }

        if let Some(m) = cap.name("conditional") {
            match m.as_str() {
                "do" => {
                    state = State::Do;
                }
                "don't" => {
                    state = State::Dont;
                }
                _ => {}
            }
        }
    }

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offical_example_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "161");
        assert_eq!(result.part_two, "161");
    }

    #[test]
    fn offical_example_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "161");
        assert_eq!(result.part_two, "48");
    }

    #[test]
    fn offical_input() {
        let input = include_str!("./input/day03");
        let result = run(input).unwrap();

        assert_eq!(result.part_one, "156388521");
        assert_eq!(result.part_two, "75920122");
    }
}
