use std::{slice::Iter, iter::Peekable};

use super::{InstructionLength, InstructionItem, Result};

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;
}

pub trait InstructionContent {

    fn push(&mut self, content: InstructionItem) -> Result<()>;

}
pub trait InstructionBytes {

    fn byte_length(&self) -> usize;

    fn to_bytes(&self) -> Vec<u8>;

    fn try_from_bytes(bytes: Peekable<Iter<'_, u8>>) -> Self where Self: Sized;

}