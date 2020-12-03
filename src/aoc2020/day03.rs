use crate::{error, Solution};

pub fn run(_input: &str) -> error::AoCResult<Solution> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_example() {
        let input = r"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
        "
        .trim();

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "");
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day03");

        let result = run(input).unwrap();

        assert_eq!(result.part_one, "");
    }
}
