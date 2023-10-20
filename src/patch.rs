use crate::{
    instructions::{delta_instruction::DeltaInstruction, InstructionItemIter},
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
        todo!();
    }
}
