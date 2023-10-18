use super::{InstructionLength, traits::{InstructionInfo, InstructionContent}, InstructionItem, error::InstructionError, Result};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RemoveInstruction {
    length: InstructionLength,
}

impl RemoveInstruction {
    pub fn new(length: InstructionLength) -> Self {
        Self { length }
    }
}


impl InstructionInfo for RemoveInstruction {
    fn len(&self) -> InstructionLength {
        self.length.try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }
}

impl InstructionContent for RemoveInstruction {
    fn push(&mut self, _: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.length += 1;
        Ok(())
    }
}

