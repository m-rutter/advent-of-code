use crate::error::{Error, IntCodeResult};

pub type Cursor = usize;
pub type Value = isize;

/// Intcode executor. Will exceute intcode programs
#[derive(Debug, Default)]
pub struct IntCodeExecutor {
    memory: Vec<Value>,
    cursor: Cursor,
}

impl IntCodeExecutor {
    /// Creates new Intcode executor for input program
    pub fn new(program: &[Value]) -> Self {
        IntCodeExecutor {
            memory: Vec::from(program),
            ..Default::default()
        }
    }

    /// Modify Intcode program in place within an executor
    pub fn modify_with_address(&mut self, address: Cursor, value: Value) -> IntCodeResult<()> {
        if let Some(elem) = self.memory.get_mut(address) {
            *elem = value;
        } else {
            Err(Error::OutOfBoundsWrite(address))?
        }

        Ok(())
    }

    /// Consume Intcode executor to get the result
    pub fn execute(mut self) -> IntCodeResult<Value> {
        loop {
            if self.cursor > self.memory.len() - 1 {
                Err(Error::OutOfBoundsOpCodeRead(self.cursor))?;
            }

            let slice = &self.memory[self.cursor..];

            let op = slice.try_into()?;

            if let Some(res) = self.execute_op(op)? {
                return Ok(res);
            }
        }
    }

    fn execute_op(&mut self, op: Instruction) -> IntCodeResult<Option<Value>> {
        match op {
            Instruction::Add(param1, param2, param3) => {
                let value = if let (Some(x), Some(y)) =
                    (self.memory.get(param1), self.memory.get(param2))
                {
                    x + y
                } else {
                    Err(Error::OutOfBoundsOpParamsRead(op.clone()))?
                };

                if let Some(elem) = self.memory.get_mut(param3) {
                    *elem = value
                } else {
                    Err(Error::OutOfBoundsOpParamsWrite(op.clone()))?
                };

                self.cursor += 4;
                Ok(None)
            }
            Instruction::Multiply(param1, param2, param3) => {
                let value = if let (Some(x), Some(y)) =
                    (self.memory.get(param1), self.memory.get(param2))
                {
                    x * y
                } else {
                    Err(Error::OutOfBoundsOpParamsRead(op.clone()))?
                };

                if let Some(elem) = self.memory.get_mut(param3) {
                    *elem = value
                } else {
                    Err(Error::OutOfBoundsOpParamsWrite(op.clone()))?
                };

                self.cursor += 4;
                Ok(None)
            }
            Instruction::Terminal => Ok(Some(self.memory[0])),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Cursor, Cursor, Cursor),
    Multiply(Cursor, Cursor, Cursor),
    Terminal,
}

impl TryFrom<&[Value]> for Instruction {
    type Error = Error;
    fn try_from(ops: &[Value]) -> IntCodeResult<Self> {
        Ok(match ops {
            [1, param1, param2, param3, ..] => {
                Instruction::Add(*param1 as Cursor, *param2 as Cursor, *param3 as Cursor)
            }
            [2, param1, param2, param3, ..] => {
                Instruction::Multiply(*param1 as Cursor, *param2 as Cursor, *param3 as Cursor)
            }
            [99, ..] => Instruction::Terminal,
            [1, ..] | [2, ..] => Err(Error::ParseErrorOutOfBoundsArguments(ops[0]))?,
            [code, ..] => Err(Error::ParseErrorUnsupportOpCode(*code))?,
            [] => Err(Error::ParseErrorNoOpCodeProvided)?,
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

    #[test]
    #[ignore = "todo"]
    fn original_examples3() {
        let ops = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(code, 43210);
    }

    #[test]
    #[ignore = "todo"]
    fn original_examples4() {
        let ops = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(code, 54321);
    }

    #[test]
    #[ignore = "todo"]
    fn original_examples5() {
        let ops = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let code = IntCodeExecutor::new(&ops)
            .execute()
            .expect("Error unwrapping op");

        assert_eq!(code, 65210);
    }
}
