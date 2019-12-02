use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let mut ops = parse(input);

    ops[1] = 12;
    ops[2] = 2;

    let part_one = execute_intcode(&mut ops)?;

    Ok(Solution {
        part_one: part_one.to_string(),
        part_two: String::new(),
    })
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .filter_map(|code| code.trim().parse().ok())
        .collect()
}

fn execute_intcode(ops: &[usize]) -> error::AoCResult<usize> {
    let mut ops = Vec::from(ops);
    let mut cursor: usize = 0;

    loop {
        match ops[cursor] {
            1 => {
                let value = ops[ops[cursor + 1]] + ops[ops[cursor + 2]];
                let reg = ops[ops[cursor + 3]];
                ops[reg] = value;
            }
            2 => {
                let value = ops[ops[cursor + 1]] * ops[ops[cursor + 2]];
                let reg = ops[ops[cursor + 3]];
                ops[reg] = value;
            }
            99 => return Ok(ops[0]),
            _ => Err(error::ErrorKind::InputParse)?,
        }

        cursor += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_examples1() {
        let input = "1,1,1,4,99,5,6,0,99";

        let ops = parse(input);

        let code = execute_intcode(&ops).expect("Error unwrapping op");

        assert_eq!(30, code);
    }

    #[test]
    fn original_examples2() {
        let input2 = "1,9,10,3,2,3,11,0,99,30,40,50";

        let ops2 = parse(input2);

        let code2 = execute_intcode(&ops2).expect("Error unwrapping op");

        assert_eq!(3500, code2);
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let x = 2 >= 3;

        let result = run(&input).unwrap();
    }
}
