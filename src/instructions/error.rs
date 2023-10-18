use std::error::Error;

use crate::instructions::InstructionLength;

use super::{
    InstructionItem, ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN, REMOVE_INSTRUCTION_SIGN,
};

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionError {
    ContentOverflow,
    MissignSign,
    InvalidSign,
    MissingLength,
    InvalidLength,
    MissingContent,
    InvalidContent,
}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::ContentOverflow => {
                write!(
                    f,
                    "Cannot exceed {} amount of bytes in an instruction",
                    InstructionLength::MAX
                )
            }
            InstructionError::MissignSign => write!(f, "No instruction sign found"),
            InstructionError::InvalidSign => write!(
                f,
                "Instruction sign didn't match: {}, {} or {}",
                REMOVE_INSTRUCTION_SIGN, ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN
            ),
            InstructionError::MissingLength => write!(f, "No length value found"),
            InstructionError::MissingContent => {
                write!(f, "Not enough bytes found to match the given length")
            }
            InstructionError::InvalidLength => write!(
                f,
                "Not enough bytes found to create a length of type {}",
                std::any::type_name::<InstructionLength>()
            ),
            InstructionError::InvalidContent => write!(
                f,
                "Not enough bytes found to create an item item of type {}",
                std::any::type_name::<InstructionItem>()
            ),
        }
    }
}

impl Error for InstructionError {}
