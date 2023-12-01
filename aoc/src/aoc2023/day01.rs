use anyhow::anyhow;
use strum::IntoEnumIterator;

use crate::{error::Result, Solution};

pub fn run(input: &str) -> Result<Solution> {
    let lines: Vec<&str> = input.trim().lines().map(|line| line.trim()).collect();

    let part_one = get_calibrations(&lines, false)?;
    let part_two = get_calibrations(&lines, true)?;

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

#[derive(strum::IntoStaticStr, strum::EnumIter, Clone, Copy, Debug)]
#[strum(serialize_all = "lowercase")]
enum Numbers {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<Numbers> for u32 {
    fn from(value: Numbers) -> Self {
        match value {
            Numbers::One => 1,
            Numbers::Two => 2,
            Numbers::Three => 3,
            Numbers::Four => 4,
            Numbers::Five => 5,
            Numbers::Six => 6,
            Numbers::Seven => 7,
            Numbers::Eight => 8,
            Numbers::Nine => 9,
        }
    }
}

fn get_calibrations(lines: &[&str], with_words: bool) -> Result<u32> {
    let mut calibrations: Vec<u32> = Vec::with_capacity(lines.len());

    for &line in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for offset in 0..line.len() {
            let s = &line[offset..];

            for num_word in Numbers::iter() {
                let digit: u32 = num_word.into();

                let start_with_digit = s.starts_with(&digit.to_string());

                if start_with_digit {
                    if first.is_none() {
                        first = Some(digit);
                    } else {
                        last = Some(digit);
                    }
                }

                if with_words {
                    let word: &str = num_word.into();
                    let start_with_word = s.starts_with(word);

                    if start_with_word {
                        if first.is_none() {
                            first = Some(digit);
                        } else {
                            last = Some(digit);
                        }
                    }
                }
            }
        }

        let first = first.ok_or_else(|| anyhow!("missing first digit in line: {line}"))?;
        let last = last.unwrap_or(first);

        let mut calibration = first.to_string();
        calibration.push_str(&last.to_string());
        let calibration: u32 = calibration.parse()?;

        calibrations.push(calibration);
    }

    Ok(calibrations.iter().sum())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let lines = example
            .trim()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>();

        let solution = get_calibrations(&lines, false).unwrap();

        assert_eq!(solution, 142);

        let example = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let lines = example
            .trim()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>();

        let solution = get_calibrations(&lines, true).unwrap();

        assert_eq!(solution, 281);
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "53921");

        assert_eq!(solution.part_two, "54676");
    }
}
