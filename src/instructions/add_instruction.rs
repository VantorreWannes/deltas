use super::{InstructionItem, traits::InstructionInfo, InstructionLength, MIN_INSTRUCTION_LENGTH};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct AddInstruction {
    content: Vec<InstructionItem>,
}

impl AddInstruction {
    pub fn new(content: Vec<InstructionItem>) -> Self {
        Self { content }
    }
}

impl InstructionInfo for AddInstruction {
    fn len(&self) -> InstructionLength {
        self.content.len().try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }
}

