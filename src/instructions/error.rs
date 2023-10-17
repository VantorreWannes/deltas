use std::error::Error;

use crate::instructions::InstructionLength;

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionError {
    ContentOverflow,
}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::ContentOverflow => {
                write!(f, "Cannot exceed {} amount of bytes in an instruction", InstructionLength::MAX)
            }
        }
    }
}

impl Error for InstructionError {}