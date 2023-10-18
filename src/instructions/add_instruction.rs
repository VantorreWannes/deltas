use super::{InstructionItem, traits::{InstructionInfo, InstructionBytes}, InstructionLength, MIN_INSTRUCTION_LENGTH};

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

impl InstructionBytes for AddInstruction {
    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + 2
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn try_from_bytes(bytes: std::iter::Peekable<std::slice::Iter<'_, u8>>) -> Self where Self: Sized {
        todo!()
    }
}

