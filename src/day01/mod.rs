use super::{AoCError, AoCSolution};

/// Compute the solution to day 1 of AoC 2017
pub fn run(input: &str) -> Result<AoCSolution, AoCError> {
    let input = parser(&input);

    let part_one = circular_match_and_sum(&input, 1);
    let part_two = circular_match_and_sum(&input, input.len() / 2);

    Ok(AoCSolution {
        part_one: part_one.to_string(),
        part_two: part_two.to_string(),
    })
}

fn parser(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn circular_match_and_sum(v: &[u32], offset: usize) -> u32 {
    v.iter()
        .cycle()
        .skip(offset)
        .zip(v.iter())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None }) // if blocks are expressions? awesome!
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "1228");
        assert_eq!(result.part_two, "1238");
    }
}
