use crate::error;
use crate::Solution;
use intcode::intcode::IntCodeExecutor;

pub fn run(input: &str) -> error::Result<Solution> {
    let program = parse(input);

    Ok(Solution {
        part_one: part_one(&program)?.to_string(),
        part_two: part_two(&program)?.to_string(),
    })
}

fn part_one(program: &[usize]) -> error::Result<usize> {
    let mut executor = IntCodeExecutor::new(&program);
    executor.modify_with_address(1, 12)?;
    executor.modify_with_address(2, 2)?;

    Ok(match executor.execute() {
        Ok(res) => res,
        Err(e) => Err(e)?,
    })
}

fn part_two(program: &[usize]) -> error::Result<usize> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut executor = IntCodeExecutor::new(program);
            executor.modify_with_address(1, noun)?;
            executor.modify_with_address(2, verb)?;

            let output = executor.execute()?;

            if output == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(error::Error::msg(
        &"Could not find a correct noun and verb combination",
    ))
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|code| code.trim().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "3716250");
        assert_eq!(result.part_two, "6472");
    }
}
