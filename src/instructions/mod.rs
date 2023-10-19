use std::error::Error;

mod add_instruction;
mod copy_instruction;
pub mod delta_instruction;
mod remove_instruction;

type InstructionItem = u8;
type InstructionLength = u8;

type Result<T> = std::result::Result<T, InstructionError>;
type InstructionItemIter<'a> = Peekable<Iter<'a, InstructionItem>>;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';

const NON_ZERO_MAX_COUNT_PERCENT: InstructionLength = 50;

use std::{iter::Peekable, slice::Iter};

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn treshold(&self) -> InstructionLength {
        ((self.len() as u32 * NON_ZERO_MAX_COUNT_PERCENT as u32) / 100u32) as InstructionLength
    }

    fn non_default_item_count(&self) -> Option<InstructionLength>;
}

pub trait InstructionContent {
    fn push(&mut self, content: InstructionItem) -> Result<()>;

    fn fill(
        &mut self,
        lcs: &mut InstructionItemIter,
        source: &mut InstructionItemIter,
        target: &mut InstructionItemIter,
    );
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
                std::any::type_name::<InstructionItem>()
            ),
        }
    }
}

impl Error for InstructionError {}
