use super::InstructionItem;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct CopyInstruction {
    exceptions: Vec<InstructionItem>,
}

impl CopyInstruction {
    pub fn new(exceptions: Vec<InstructionItem>) -> Self {
        Self { exceptions }
    }

}
