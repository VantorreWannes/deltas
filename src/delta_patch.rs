use crate::{delta_instruction::Instruction, delta_traits::LCS};

pub enum InstructionItem {
    Add(u8),
    Remove,
    Copy,
}

impl InstructionItem {
    pub fn matches(&self, instruction: &Instruction) -> bool {
        match (self, instruction) {
            (InstructionItem::Add(_), Instruction::Add { .. })
            | (InstructionItem::Remove, Instruction::Remove { .. })
            | (InstructionItem::Copy, Instruction::Copy { .. }) => true,
            _ => false,
        }
    }
}

pub struct DeltaPatch {
    instructions: Vec<Instruction>,
}

impl LCS for DeltaPatch {}

impl DeltaPatch {
    pub fn push_instruction_item(
        instructions: &mut Vec<Instruction>,
        instruction_item: InstructionItem,
    ) {
        if let Some(instruction) = instructions.last_mut() {
            if instruction_item.matches(instruction) {
                if instruction.push_instruction_item(&instruction_item).is_ok() {
                    return;
                }
            }
        }
        instructions.push(Instruction::from(&instruction_item));
    }

    pub fn new(source: &[u8], target: &[u8]) -> Self {
        Self {
            instructions: Self::construct_instructions(source, target, Self::lcs(source, target)),
        }
    }

    fn construct_instructions(source: &[u8], target: &[u8], lcs: Vec<u8>) -> Vec<Instruction> {
        let mut lcs = lcs.iter().peekable();
        let mut source = source.iter().peekable();
        let mut target = target.iter().peekable();

        let mut instructions: Vec<Instruction> = Vec::new();
        while source.peek().is_some() || target.peek().is_some() {
            if source.peek() != lcs.peek() {
                Self::push_instruction_item(&mut instructions, InstructionItem::Remove);
                source.next();
            } else if target.peek() != lcs.peek() {
                Self::push_instruction_item(
                    &mut instructions,
                    InstructionItem::Add(*target.next().unwrap()),
                );
            } else if source.peek().is_some() && target.peek().is_some() {
                Self::push_instruction_item(&mut instructions, InstructionItem::Copy);
                lcs.next();
                source.next();
                target.next();
            } else if source.peek().is_some() {
                Self::push_instruction_item(&mut instructions, InstructionItem::Remove);
                source.next();
            } else if target.peek().is_some() {
                Self::push_instruction_item(
                    &mut instructions,
                    InstructionItem::Add(*target.next().unwrap()),
                );
            }
        }
        instructions
    }
}

#[cfg(test)]
mod delta_patch_tests {
    use crate::{delta_instruction::Instruction, delta_patch::InstructionItem, delta_traits::LCS};

    use super::DeltaPatch;

    #[test]
    fn push_instruction_item() {
        let mut instructions = vec![];
        DeltaPatch::push_instruction_item(&mut instructions, InstructionItem::Add(b'A'));
        assert_eq!(
            instructions,
            vec![Instruction::Add {
                content: vec![b'A']
            }]
        );
        DeltaPatch::push_instruction_item(&mut instructions, InstructionItem::Add(b'A'));
        assert_eq!(
            instructions,
            vec![Instruction::Add {
                content: vec![b'A', b'A']
            }]
        );
        DeltaPatch::push_instruction_item(&mut instructions, InstructionItem::Copy);
        assert_eq!(
            instructions,
            vec![
                Instruction::Add {
                    content: vec![b'A', b'A']
                },
                Instruction::Copy { length: 1 }
            ]
        );
        DeltaPatch::push_instruction_item(&mut instructions, InstructionItem::Remove);
        assert_eq!(
            instructions,
            vec![
                Instruction::Add {
                    content: vec![b'A', b'A']
                },
                Instruction::Copy { length: 1 },
                Instruction::Remove { length: 1 }
            ]
        );
    }

    #[test]
    fn construct_instructions() {
        let source = b"AAXCCC";
        let target = b"BBBXCC";
        let lcs = DeltaPatch::lcs(source, target);
        assert_eq!(
            DeltaPatch::construct_instructions(source, target, lcs),
            vec![
                Instruction::Remove { length: 2 },
                Instruction::Add {
                    content: vec![66u8, 66u8, 66u8,],
                },
                Instruction::Copy { length: 3 },
                Instruction::Remove { length: 1 },
            ]
        );
    }
}
