use crate::error;
use crate::Solution;
use std::convert::TryFrom;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let mut ops = parse(input);

    if ops.len() < 2 {
        Err(error::Error::msg(&format!(
            "Unsufficient intcode input: {:?}",
            ops
        )))?;
    }
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
    let mut registers = Vec::from(ops);
    let mut cursor: usize = 0;

    loop {
        if cursor > registers.len() - 1 {
            Err(error::Error::msg(&"No opcode provided"))?;
        }

        let op = Op::try_from(&registers[cursor..])?;

        match op {
            Op::Add(param1, param2, output) => {
                let value = registers[param1] + registers[param2];
                registers[output] = value;
            }
            Op::Multiply(param1, param2, output) => {
                let value = registers[param1] * registers[param2];
                registers[output] = value;
            }
            Op::Terminal => break Ok(registers[0]),
        }

        cursor += 4;
    }
}

enum Op {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Terminal,
}

impl TryFrom<&[usize]> for Op {
    type Error = error::Error;
    fn try_from(op: &[usize]) -> Result<Self, Self::Error> {
        if op.len() == 0 {
            Err(error::Error::msg(&"No opcode provided"))?
        }

        Ok(match op[0..1] {
            [1] | [2] => match op[0..=3] {
                [1, param1, param2, output] => Op::Add(param1, param2, output),
                [2, param1, param2, output] => Op::Multiply(param1, param2, output),
                _ => Err(error::Error::msg(&format!(
                    "Unsufficient arugments for opcode \"{}\": {:?}",
                    op[0], op
                )))?,
            },
            [99] => Op::Terminal,
            [code] => Err(error::Error::msg(&format!(
                "Unrecognised op code: {}",
                code
            )))?,
            _ => Err(error::Error::msg(&"No opcode provided"))?,
        })
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
