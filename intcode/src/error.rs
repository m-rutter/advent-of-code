use thiserror::Error;

use crate::intcode::Instruction;

type Address = usize;

#[derive(Error, Debug)]

pub enum Error {
    #[error("Out of bounds read of op code at address: {0}")]
    OutOfBoundsOpCodeRead(Address),

    #[error("Out of bounds write at address: {0}")]
    OutOfBoundsWrite(Address),

    #[error("Out of bounds reads for op: {0:?}")]
    OutOfBoundsOpParamsRead(Instruction),

    #[error("Out of bound write for op: {0:?}")]
    OutOfBoundsOpParamsWrite(Instruction),

    #[error("Unsufficient arugments for opcode: {0}")]
    ParseErrorOutOfBoundsArguments(usize),

    #[error("Unrecognised op code: {0}")]
    ParseErrorUnsupportOpCode(usize),

    #[error("No op code provided, memory might not be initialised")]
    ParseErrorNoOpCodeProvided,
}

pub type IntCodeResult<T> = std::result::Result<T, Error>;
