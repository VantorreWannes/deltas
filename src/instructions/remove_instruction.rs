use super::InstructionLength;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RemoveInstruction {
    length: InstructionLength,
}

impl RemoveInstruction {
    pub fn new(length: InstructionLength) -> Self {
        Self { length }
    }

    pub fn len(&self) -> InstructionLength {
        self.length
    }
}