use super::{remove_instruction::RemoveInstruction, add_instruction::AddInstruction, copy_instruction::CopyInstruction};

#[derive(Debug, PartialEq, Clone)]
pub enum DeltaInstruction {
    Remove(RemoveInstruction),
    Add(AddInstruction),
    Copy(CopyInstruction),
}
