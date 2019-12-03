use crate::error;
use crate::Solution;
use std::convert::TryFrom;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let mut program = parse(input);

    if program.len() < 2 {
        Err(error::Error::msg(&format!(
            "Unsufficient intcode input: {:?}",
            program
        )))?;
    }
    program[1] = 12;
    program[2] = 2;

    let part_one = IntCodeExecutor::new(&program).execute()?;

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
    memory: Vec<usize>,
}

impl IntCodeExecutor {
    fn new(program: &[usize]) -> Self {
        IntCodeExecutor {
            memory: Vec::from(program),
        }
    }

    fn execute(mut self) -> error::AoCResult<usize> {
        let mut address: usize = 0;

        loop {
            if address > self.memory.len() - 1 {
                Err(error::Error::msg(&"No opcode provided"))?;
            }

            let op = Instruction::try_from(&self.memory[address..])?;

            match op {
                Instruction::Add(param1, param2, param3) => {
                    let value = if let (Some(x), Some(y)) =
                        (self.memory.get(param1), self.memory.get(param2))
                    {
                        x + y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = self.memory.get_mut(param3) {
                        *elem = value
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid index in op code for output (out of bounds): {}",
                            param3
                        )))?
                    };
                }
                Instruction::Multiply(param1, param2, param3) => {
                    let value = if let (Some(x), Some(y)) =
                        (self.memory.get(param1), self.memory.get(param2))
                    {
                        x * y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = self.memory.get_mut(param3) {
                        *elem = value
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid index in op code for output (out of bounds): {}",
                            param3
                        )))?
                    };
                }
                Instruction::Terminal => break Ok(self.memory[0]),
            }

            address += 4;
        }
    }
}

enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Terminal,
}

impl TryFrom<&[usize]> for Instruction {
    type Error = error::Error;
    fn try_from(ops: &[usize]) -> Result<Self, Self::Error> {
        if ops.len() == 0 {
            Err(error::Error::msg(&"No opcode provided"))?
        }

        Ok(match ops[0..1] {
            [1] | [2] => match ops[0..=3] {
                [1, param1, param2, param3] => Instruction::Add(param1, param2, param3),
                [2, param1, param2, param3] => Instruction::Multiply(param1, param2, param3),
                _ => Err(error::Error::msg(&format!(
                    "Unsufficient arugments for opcode \"{}\": {:?}",
                    ops[0], ops
                )))?,
            },
            [99] => Instruction::Terminal,
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
