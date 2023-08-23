use std::{error::Error, fmt::Display};
use crate::delta_instruction_error::InstructionConvertBetweenBytesError;

#[derive(Debug, PartialEq)]
pub enum PatchError {
    InstructionError(InstructionConvertBetweenBytesError),
}

impl Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatchError::InstructionError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for PatchError {}

impl From<InstructionConvertBetweenBytesError> for PatchError {

    fn from(value: InstructionConvertBetweenBytesError) -> Self {
        Self::InstructionError(value)
    }
}