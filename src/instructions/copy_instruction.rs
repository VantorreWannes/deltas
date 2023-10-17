use super::InstructionItem;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct CopyInstruction {
    content: Vec<InstructionItem>,
}

impl CopyInstruction {
    pub fn new(content: Vec<InstructionItem>) -> Self {
        Self { content }
    }

}
