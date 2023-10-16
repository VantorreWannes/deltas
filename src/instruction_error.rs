use std::{error::Error, fmt::Display};

use crate::instructions::{ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN, REMOVE_INSTRUCTION_SIGN};

pub type Result<T> = std::result::Result<T, InstructionError>;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionError {
    ContentOverflow,
    MissingSign,
    InvalidSign,
    MissingLength,
    MissingContent,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::ContentOverflow => {
                write!(f, "Content length cannot exceed {} bytes.", u8::MAX)
            }
            InstructionError::MissingSign => {
                write!(f, "Missing instruction sign byte.")
            }
            InstructionError::InvalidSign => {
                write!(
                    f,
                    "Invalid instruction sign. Please use {}, {} or {}",
                    REMOVE_INSTRUCTION_SIGN, ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN
                )
            }
            InstructionError::MissingLength => {
                write!(f, "Missing instruction length byte.")
            },
            InstructionError::MissingContent => {
                write!(f, "Missing instruction content.")
            },
        }
    }
}

impl Error for InstructionError {}
