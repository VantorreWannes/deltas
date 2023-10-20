use crate::{
    instructions::{
        add_instruction::AddInstruction, copy_instruction::CopyInstruction,
        delta_instruction::DeltaInstruction, remove_instruction::RemoveInstruction,
        InstructionContent, InstructionItemIter,
    },
    lcs::Lcs,
};

pub struct Patch {
    instructions: Vec<DeltaInstruction>,
}

impl Patch {
    pub fn new(source: &[u8], target: &[u8]) -> Self {
        let lcs = Lcs::new(source, target).subsequence();
        let mut lcs_iter = lcs.iter().peekable();
        let mut source_iter = source.iter().peekable();
        let mut target_iter = target.iter().peekable();
        Self {
            instructions: Self::create_instructions(
                &mut lcs_iter,
                &mut source_iter,
                &mut target_iter,
            ),
        }
    }

    fn create_instructions(
        lcs: &mut InstructionItemIter,
        source: &mut InstructionItemIter,
        target: &mut InstructionItemIter,
    ) -> Vec<DeltaInstruction> {
        let mut instructions: Vec<DeltaInstruction> = Vec::new();
        while lcs.peek().is_some() {
            if lcs.peek() != source.peek() {
                let mut instruction: DeltaInstruction = RemoveInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            } else if lcs.peek() != target.peek() {
                let mut instruction: DeltaInstruction = AddInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            } else {
                let mut instruction: DeltaInstruction = CopyInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            }
        }
        while lcs.peek() != source.peek() {
            let mut instruction: DeltaInstruction = RemoveInstruction::default().into();
            instruction.fill(lcs, source, target);
            instructions.push(instruction);
        }
        while lcs.peek() != target.peek() {
            let mut instruction: DeltaInstruction = AddInstruction::default().into();
            instruction.fill(lcs, source, target);
            instructions.push(instruction);
        }
        instructions
    }
}
