use super::{InstructionItem, InstructionLength, traits::{InstructionInfo, InstructionContent}, error::InstructionError, Result};


#[derive(Debug, PartialEq, Clone, Default)]
pub struct CopyInstruction {
    content: Vec<InstructionItem>,
}

impl CopyInstruction {
    pub fn new(content: Vec<InstructionItem>) -> Self {
        assert!(
            content.len() <= InstructionLength::MAX.try_into().unwrap(),
            "Instruction content exceeded {} items",
            InstructionLength::MAX
        );
        Self { content }
    }

}

impl InstructionInfo for CopyInstruction {
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

impl InstructionContent for CopyInstruction {
    fn push(&mut self, content: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.content.push(content);
        Ok(())
    }
}


