use super::{InstructionItem, traits::{InstructionInfo, InstructionBytes, InstructionContent}, InstructionLength, Result, ADD_INSTRUCTION_SIGN, error::InstructionError};

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

impl InstructionContent for AddInstruction {
    fn push(&mut self, content: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.content.push(content);
        Ok(())
    }
}

impl InstructionBytes for AddInstruction {
    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + 2
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(ADD_INSTRUCTION_SIGN);
        bytes.extend(self.len().to_be_bytes());
        bytes.extend(self.content.iter());
        bytes
    }

    fn try_from_bytes(bytes: std::iter::Peekable<std::slice::Iter<'_, u8>>) -> Result<Self> {
        todo!()
    }
}

