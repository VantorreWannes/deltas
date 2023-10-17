use super::InstructionLength;

pub trait InstructionInfo {
    fn len(&self) -> InstructionLength;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;
}
