use super::{InstructionLength, InstructionItem, Result};

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;
}

pub trait InstructionContent {

    fn push(&mut self, content: InstructionItem) -> Result<()>;

}