use thiserror::Error;

use crate::intcode::{Cursor, Instruction, Value};

#[derive(Error, Debug)]

pub enum Error {
    #[error("Out of bounds read of op code at address: {0}")]
    OutOfBoundsOpCodeRead(Cursor),

    #[error("Out of bounds write at address: {0}")]
    OutOfBoundsWrite(Cursor),

    #[error("Out of bounds reads for op: {0:?}")]
    OutOfBoundsOpParamsRead(Instruction),

    #[error("Out of bound write for op: {0:?}")]
    OutOfBoundsOpParamsWrite(Instruction),

    #[error("Unsufficient arugments for opcode: {0}")]
    ParseErrorOutOfBoundsArguments(Value),

    #[error("Unrecognised op code: {0}")]
    ParseErrorUnsupportOpCode(Value),

    #[error("No op code provided, memory might not be initialised")]
    ParseErrorNoOpCodeProvided,
}

pub type IntCodeResult<T> = std::result::Result<T, Error>;
