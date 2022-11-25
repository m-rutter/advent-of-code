use crate::{
    error::{ErrorKind, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let lines = input.trim().lines().map(|line| line.trim());
    let line_len = lines.clone().next().map_or_else(|| 0, |line| line.len());

    let mut counts: Vec<(u32, u32)> = vec![(0, 0); line_len];

    for line in input.trim().lines().map(|line| line.trim()) {
        for (index, char) in line.chars().enumerate() {
            match char {
                '0' => counts[index].0 += 1,
                '1' => counts[index].1 += 1,
                _ => Err(ErrorKind::InputParse)?,
            };
        }
    }

    let gamma = u32::from_str_radix(
        &counts
            .iter()
            .map(|(noughts, ones)| if noughts > ones { '0' } else { '1' })
            .collect::<String>(),
        2,
    )?;

    let epsilon = u32::from_str_radix(
        &counts
            .iter()
            .map(|(noughts, ones)| if noughts < ones { '0' } else { '1' })
            .collect::<String>(),
        2,
    )?;

    Ok(Solution {
        part_one: (gamma * epsilon).to_string(),
        part_two: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn original_examples() {
        let input = r"
	00100
	11110
	10110
	10111
	10101
	01111
	00111
	11100
	10000
	11001
	00010
	01010   
	    ";

        let res = run(input).unwrap();

        assert_eq!(res.part_one, "198");
        assert_eq!(res.part_two, "230");
    }

    #[test]
    #[ignore]
    fn matches_offical_result() {
        let input = include_str!("./input/day03");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "2724524");
        assert_eq!(result.part_two, "1855892637");
    }
}
