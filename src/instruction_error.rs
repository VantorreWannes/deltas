use std::{fmt::Display, error::Error};

pub type Result<T> = std::result::Result<T, InstructionError>;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionError {
    ContentOverflow,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::ContentOverflow => {
                write!(f, "Content length cannot exceed {} bytes.", u8::MAX)
            }
        }
    }
}

impl Error for InstructionError {}
