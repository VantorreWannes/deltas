use super::{remove_instruction::RemoveInstruction, add_instruction::AddInstruction, copy_instruction::CopyInstruction, InstructionInfo};

#[derive(Debug, PartialEq, Clone)]
pub enum DeltaInstruction {
    Remove(RemoveInstruction),
    Add(AddInstruction),
    Copy(CopyInstruction),
}

impl InstructionInfo for DeltaInstruction {
    fn len(&self) -> super::InstructionLength {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.len(),
            DeltaInstruction::Add(instruction) => instruction.len(),
            DeltaInstruction::Copy(instruction) => instruction.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.is_empty(),
            DeltaInstruction::Add(instruction) => instruction.is_empty(),
            DeltaInstruction::Copy(instruction) => instruction.is_empty(),
        }
    }

    fn is_full(&self) -> bool {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.is_full(),
            DeltaInstruction::Add(instruction) => instruction.is_full(),
            DeltaInstruction::Copy(instruction) => instruction.is_full(),
        }
    }

    fn non_default_item_count(&self) -> Option<super::InstructionLength> {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.non_default_item_count(),
            DeltaInstruction::Add(instruction) => instruction.non_default_item_count(),
            DeltaInstruction::Copy(instruction) => instruction.non_default_item_count(),
        }
    }
}