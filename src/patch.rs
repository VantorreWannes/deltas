use crate::{
    instructions::{
        add_instruction::AddInstruction, copy_instruction::CopyInstruction,
        delta_instruction::DeltaInstruction, remove_instruction::RemoveInstruction,
        InstructionBytes, InstructionContent, InstructionInfo, InstructionItemIter,
    },
    lcs::Lcs,
};

#[derive(Debug, Default, PartialEq, Clone)]
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
            if lcs.len() > source.len() || lcs.len() > target.len() {}
            if lcs.peek() != source.peek() && source.peek().is_some() {
                let mut instruction: DeltaInstruction = RemoveInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            } else if lcs.peek() != target.peek() && target.peek().is_some() {
                let mut instruction: DeltaInstruction = AddInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            } else {
                let mut instruction: DeltaInstruction = CopyInstruction::default().into();
                instruction.fill(lcs, source, target);
                instructions.push(instruction);
            }
        }
        while source.peek().is_some() {
            let mut instruction: DeltaInstruction = RemoveInstruction::default().into();
            instruction.fill(lcs, source, target);
            instructions.push(instruction);
        }
        while target.peek().is_some() {
            let mut instruction: DeltaInstruction = AddInstruction::default().into();
            instruction.fill(lcs, source, target);
            instructions.push(instruction);
        }
        instructions
    }

    pub fn apply(&self, source: &[u8]) -> Vec<u8> {
        todo!();
    }

    fn construct_target(&self, source: &[u8], target_length: usize) -> Vec<u8> {
        todo!();
    }

    fn target_lenth(&self) -> usize {
        self.instructions
            .iter()
            .fold(0usize, |mut acc, instruction| {
                match instruction {
                    DeltaInstruction::Remove(_) => (),
                    DeltaInstruction::Add(_) => acc += instruction.len() as usize,
                    DeltaInstruction::Copy(_) => {
                        acc += instruction.len() as usize
                            - instruction.non_default_item_count().unwrap() as usize
                    }
                };
                acc
            }) as usize
    }

    fn byte_length(&self) -> usize {
        self.instructions
            .iter()
            .map(|instruction| instruction.byte_length())
            .sum::<usize>()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        for instruction in self.instructions.iter() {
            bytes.extend(instruction.to_bytes());
        }
        bytes
    }
}

#[cfg(test)]
mod remove_instruction_tests {
    use super::*;

    #[test]
    fn new() {
        //SPECIAL BUG: dbg!(Patch::new(b"ABCCCC", b"AC"));
        assert_eq!(
            Patch::new(b"BBAAA", b"AAABBBAA").instructions,
            vec![
                AddInstruction::new(vec![65, 65, 65]).into(),
                CopyInstruction::new(vec![0, 0, 1, 0, 0,]).into(),
            ],
        );
    }

    #[test]
    fn target_length() {
        assert_eq!(Patch::new(b"AAAAAAAA", b"AAA").target_lenth(), 3);
        assert_eq!(
            Patch::new(b"AAAAAAAABBBCCC", b"BBBCCCAAA").target_lenth(),
            9
        );
        assert_eq!(Patch::new(b"AAAAAAAA", b"").target_lenth(), 0);
        assert_eq!(Patch::new(b"", b"AAA").target_lenth(), 3);
    }
}