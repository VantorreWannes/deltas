use std::error::Error;

pub mod add_instruction;
pub mod copy_instruction;
pub mod delta_instruction;
pub mod remove_instruction;

type InstructionLength = u8;

pub type Result<T> = std::result::Result<T, InstructionError>;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';

const NON_ZERO_MAX_COUNT_PERCENT: u8 = 100;

use std::{iter::Peekable, slice::Iter};

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn threshold(&self) -> InstructionLength {
        ((self.len() as u32 * NON_ZERO_MAX_COUNT_PERCENT as u32) / 100u32) as InstructionLength
    }

    fn non_default_item_count(&self) -> Option<InstructionLength>;
}

pub trait InstructionContent {
    fn push(&mut self, content: u8) -> Result<()>;

    fn fill(
        &mut self,
        lcs: &mut Peekable<Iter<'_, u8>>,
        source: &mut Peekable<Iter<'_, u8>>,
        target: &mut Peekable<Iter<'_, u8>>,
    );

    fn apply(&self, source: &mut Iter<'_, u8>, target: &mut Vec<u8>);
}
pub trait InstructionBytes {
    fn byte_sign(&self) -> u8;

    fn byte_length(&self) -> usize;

    fn to_bytes(&self) -> Vec<u8>;

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self>
    where
        Self: Sized;
}

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
                std::any::type_name::<u8>()
            ),
        }
    }
}

impl Error for InstructionError {}

#[cfg(test)]
mod instruction_mod_tests {
    use super::*;

    fn threshold(
        len: InstructionLength,
        non_zero_max_count_percent: InstructionLength,
    ) -> InstructionLength {
        ((len as f32 * non_zero_max_count_percent as f32) / 100f32) as InstructionLength
    }

    #[test]
    fn instruction_info() {
        for len in 0..=InstructionLength::MAX {
            assert_eq!(threshold(len, NON_ZERO_MAX_COUNT_PERCENT), len);
        }
        //dbg!(threshold(1000, 10));
    }
}
