use self::error::InstructionError;

mod add_instruction;
mod copy_instruction;
pub mod error;
pub mod instruction;
mod remove_instruction;

type InstructionItem = u8;
type InstructionLength = u8;

type Result<T> = std::result::Result<T, InstructionError>;

const REMOVE_INSTRUCTION_SIGN: InstructionItem = b'-';
const ADD_INSTRUCTION_SIGN: InstructionItem = b'+';
const COPY_INSTRUCTION_SIGN: InstructionItem = b'|';

use std::{iter::Peekable, slice::Iter};

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn non_default_item_count(&self) -> Option<InstructionLength>;
}

pub trait InstructionContent {
    fn push(&mut self, content: InstructionItem) -> Result<()>;
}
pub trait InstructionBytes {
    fn byte_sign() -> u8;

    fn byte_length(&self) -> usize;

    fn to_bytes(&self) -> Vec<u8>;

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self>
    where
        Self: Sized;
}
