use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let expenses: Vec<u32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    let mut pairs: Option<(u32, u32)> = None;

    'outer: for (index, x) in expenses.iter().enumerate() {
        for y in expenses[index..].iter() {
            if x + y == 2020 {
                pairs = Some((*x, *y));
                break 'outer;
            }
        }
    }

    let mut tri: Option<(u32, u32, u32)> = None;
    'outer2: for (index, x) in expenses.iter().enumerate() {
        for y in expenses[index..].iter() {
            for z in expenses[index + 1..].iter() {
                if x + y + z == 2020 {
                    tri = Some((*x, *y, *z));
                    break 'outer2;
                }
            }
        }
    }

    let pairs = pairs.unwrap();
    let tri = tri.unwrap();

    Ok(Solution {
        part_one: (pairs.0 * pairs.1).to_string(),
        part_two: (tri.0 * tri.1 * tri.2).to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_example() {
        let input = r"
        1721
        979
        366
        299
        675
        1456
        "
        .trim();

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "514579");
        assert_eq!(result.part_two, "241861950");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day01");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "1010884");
        assert_eq!(result.part_two, "253928438");
    }
}
