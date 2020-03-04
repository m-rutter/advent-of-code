use std::convert::TryFrom;

use crate::error;

/// Intcode executor. Will exceute intcode programs
pub struct IntCodeExecutor {
    memory: Vec<usize>,
}

impl IntCodeExecutor {
    /// Creates new Intcode executor for input program
    pub fn new(program: &[usize]) -> Self {
        IntCodeExecutor {
            memory: Vec::from(program),
        }
    }

    /// Modify Intcode program in place within an executor
    pub fn modify_with_address(&mut self, address: usize, value: usize) -> error::AoCResult<()> {
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

    /// Consume Intcode executor to get the result
    pub fn execute(self) -> error::AoCResult<usize> {
        let mut memory = self.memory;
        let mut address: usize = 0;

        loop {
            if address > memory.len() - 1 {
                Err(error::Error::msg(&"No opcode provided"))?;
            }

            let op = Instruction::try_from(&memory[address..])?;

            match op {
                Instruction::Add(param1, param2, param3) => {
                    let value = if let (Some(x), Some(y)) = (memory.get(param1), memory.get(param2))
                    {
                        x + y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = memory.get_mut(param3) {
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
                    let value = if let (Some(x), Some(y)) = (memory.get(param1), memory.get(param2))
                    {
                        x * y
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid indexes in op code for parameters (out of bounds): {}, {}",
                            param1, param2,
                        )))?
                    };

                    if let Some(elem) = memory.get_mut(param3) {
                        *elem = value
                    } else {
                        Err(error::Error::msg(&format!(
                            "Invalid index in op code for output (out of bounds): {}",
                            param3
                        )))?
                    };

                    address += 4;
                }
                Instruction::Terminal => break Ok(memory[0]),
            }
        }
    }
}

pub enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Terminal,
}

impl TryFrom<&[usize]> for Instruction {
    type Error = error::Error;
    fn try_from(ops: &[usize]) -> Result<Self, Self::Error> {
        Ok(match ops {
            [1, param1, param2, param3, ..] => Instruction::Add(*param1, *param2, *param3),
            [2, param1, param2, param3, ..] => Instruction::Multiply(*param1, *param2, *param3),
            [99, ..] => Instruction::Terminal,
            [1 | 2, ..] => Err(error::Error::msg(&format!(
                "Unsufficient arugments for opcode \"{}\": {:?}",
                ops[0], ops
            )))?,
            [code, ..] => Err(error::Error::msg(&format!(
                "Unrecognised op code: {}",
                code
            )))?,
            [] => Err(error::Error::msg(&"No opcode provided"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_examples1() {
        let ops = [1, 1, 1, 4, 99, 5, 6, 0, 99];

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(30, code);
    }

    #[test]
    fn original_examples2() {
        let ops = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(3500, code);
    }
}
