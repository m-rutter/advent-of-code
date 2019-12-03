use std::convert::TryFrom;

use crate::error;
use crate::Solution;

pub fn run(input: &str) -> error::AoCResult<Solution> {
    let program = parse(input);

    Ok(Solution {
        part_one: part_one(&program)?.to_string(),
        part_two: part_two(&program)?.to_string(),
    })
}

fn part_one(program: &[usize]) -> error::AoCResult<usize> {
    let mut executor = IntCodeExecutor::new(&program);
    executor.modify_with_address(1, 12)?;
    executor.modify_with_address(2, 2)?;

    executor.execute()
}

fn part_two(program: &[usize]) -> error::AoCResult<usize> {
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

struct IntCodeExecutor {
    memory: Vec<usize>,
}

impl IntCodeExecutor {
    fn new(program: &[usize]) -> Self {
        IntCodeExecutor {
            memory: Vec::from(program),
        }
    }

    fn modify_with_address(&mut self, address: usize, value: usize) -> error::AoCResult<()> {
        if let Some(elem) = self.memory.get_mut(address) {
            *elem = value;
        } else {
            Err(error::Error::msg(&format!(
                "Error attempting to modify intcode memory at address (out of bounds): {}",
                address
            )))?
        }

        Ok(())
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

                    address += 4;
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

                    address += 4;
                }
                Instruction::Terminal => break Ok(self.memory[0]),
            }
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

        Ok(match ops[0] {
            1 | 2 => {
                if ops.len() < 4 {
                    Err(error::Error::msg(&format!(
                        "Unsufficient arugments for opcode \"{}\": {:?}",
                        ops[0],
                        ops[1..]
                    )))?
                }

                match ops[0..4] {
                    [1, param1, param2, param3] => Instruction::Add(param1, param2, param3),
                    [2, param1, param2, param3] => Instruction::Multiply(param1, param2, param3),
                    _ => unreachable!(),
                }
            }
            99 => Instruction::Terminal,
            code => Err(error::Error::msg(&format!(
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
        assert_eq!(result.part_two, "6472");
    }
}
