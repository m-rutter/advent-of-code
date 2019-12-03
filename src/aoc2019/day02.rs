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

    let part_one = IntCodeExecutor::new(&ops).execute()?;

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

struct IntCodeExecutor {
    registers: Vec<usize>,
}

impl IntCodeExecutor {
    fn new(registers: &[usize]) -> Self {
        IntCodeExecutor {
            registers: Vec::from(registers),
        }
    }

    fn execute(mut self) -> error::AoCResult<usize> {
        let mut cursor: usize = 0;

        loop {
            if cursor > self.registers.len() - 1 {
                Err(error::Error::msg(&"No opcode provided"))?;
            }

            let op = Op::try_from(&self.registers[cursor..])?;

            match op {
                Op::Add(param1, param2, output) => {
                    let value = if let (Some(x), Some(y)) =
                        (self.registers.get(param1), self.registers.get(param2))
                    {
                        x + y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = self.registers.get_mut(output) {
                        *elem = value
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid index in op code for output (out of bounds): {}",
                            output
                        )))?
                    };
                }
                Op::Multiply(param1, param2, output) => {
                    let value = if let (Some(x), Some(y)) =
                        (self.registers.get(param1), self.registers.get(param2))
                    {
                        x * y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = self.registers.get_mut(output) {
                        *elem = value
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid index in op code for output (out of bounds): {}",
                            output
                        )))?
                    };
                }
                Op::Terminal => break Ok(self.registers[0]),
            }

            cursor += 4;
        }
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

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(30, code);
    }

    #[test]
    fn original_examples2() {
        let input2 = "1,9,10,3,2,3,11,0,99,30,40,50";

        let ops2 = parse(input2);

        let code2 = IntCodeExecutor::new(&ops2)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(3500, code2);
    }

    #[test]
    fn matches_offical_result() {
        let input = include_str!("./input/day02");

        let result = run(&input).unwrap();

        assert_eq!(result.part_one, "3716250");
    }
}
