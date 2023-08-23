use std::{iter::Peekable, slice::Iter};

use crate::delta_instruction::RemoveInstructionLength;

use super::delta_instruction::DeltaInstruction;

#[derive(Debug, PartialEq)]
pub struct DeltaPatch {
    instructions: Vec<DeltaInstruction>,
}

impl DeltaPatch {
    fn lcs(source: &[u8], target: &[u8]) -> Vec<u8> {
        let s_len = source.len();
        let t_len = target.len();

        let mut table = vec![vec![0; t_len + 1]; s_len + 1];

        for i in 0..=s_len {
            for j in 0..=t_len {
                if i == 0 || j == 0 {
                    table[i][j] = 0
                } else if source[i - 1] == target[j - 1] {
                    table[i][j] = table[i - 1][j - 1] + 1
                } else {
                    table[i][j] = table[i - 1][j].max(table[i][j - 1])
                }
            }
        }

        let mut index = table[s_len][t_len];
        let mut lcs = vec![0; index + 1];
        lcs[index] = 0;

        let mut i = s_len;
        let mut j = t_len;
        while i > 0 && j > 0 {
            if source[i - 1] == target[j - 1] {
                lcs[index - 1] = source[i - 1];
                i -= 1;
                j -= 1;
                index -= 1
            } else if table[i - 1][j] > table[i][j - 1] {
                i -= 1
            } else {
                j -= 1
            }
        }
        lcs.resize(table[s_len][t_len], 0);

        lcs
    }

    pub fn new(source: &[u8], target: &[u8]) -> Self {
        Self {
            instructions: Self::construct_instructions(source, target, Self::lcs(source, target)),
        }
    }

    fn construct_instructions(
        source: &[u8],
        target_iter: &[u8],
        lcs_iter: Vec<u8>,
    ) -> Vec<DeltaInstruction> {
        let mut source = source.iter().peekable();
        let mut target = target_iter.iter().peekable();
        let mut lcs = lcs_iter.iter().peekable();

        let mut instructions: Vec<DeltaInstruction> = Vec::new();

        while source.peek().is_some() || target.peek().is_some() {
            let source_peek = source.peek();
            let target_peek = target.peek();
            let lcs_peek = lcs.peek();

            if source_peek.is_some() && lcs_peek.is_some() && source_peek != lcs_peek {
                let remove_command = Self::construct_remove_instruction(&mut source, &mut lcs);
                instructions.push(remove_command);
            } else if target_peek.is_some() && lcs_peek.is_some() && target_peek != lcs_peek {
                let add_command = Self::construct_add_instruction(&mut target, &mut lcs);
                instructions.push(add_command);
            } else if source_peek.is_some()
                && target_peek.is_some()
                && lcs_peek.is_some()
                && source_peek == target_peek
            {
                let copy_command =
                    Self::construct_copy_instruction(&mut source, &mut target, &mut lcs);
                instructions.push(copy_command);
            } else if source_peek.is_some() {
                let remove_instruction = Self::soft_construct_remove_instruction(&mut source);
                instructions.push(remove_instruction);
            } else if target_peek.is_some() {
                let add_instruction = Self::soft_construct_add_instruction(&mut target);
                instructions.push(add_instruction);
            }
        }
        instructions
    }

    fn construct_remove_instruction(
        source: &mut Peekable<Iter<u8>>,
        lcs: &mut Peekable<Iter<u8>>,
    ) -> DeltaInstruction {
        let mut remove_instruction = DeltaInstruction::Remove { length: 0 };
        while source.peek().is_some() && lcs.peek().is_some() && source.peek() != lcs.peek() {
            if remove_instruction.push(source.next().unwrap()).is_err() {
                break;
            };
        }
        remove_instruction
    }

    fn construct_add_instruction(
        target: &mut Peekable<Iter<u8>>,
        lcs: &mut Peekable<Iter<u8>>,
    ) -> DeltaInstruction {
        let mut add_instruction = DeltaInstruction::Add { content: vec![] };
        while target.peek().is_some() && lcs.peek().is_some() && target.peek() != lcs.peek() {
            if add_instruction.push(target.next().unwrap()).is_err() {
                break;
            };
        }
        add_instruction
    }

    fn construct_copy_instruction(
        source: &mut Peekable<Iter<u8>>,
        target: &mut Peekable<Iter<u8>>,
        lcs: &mut Peekable<Iter<u8>>,
    ) -> DeltaInstruction {
        let mut copy_instruction = DeltaInstruction::Copy { length: 0 };
        while source.peek().is_some()
            && target.peek().is_some()
            && lcs.peek().is_some()
            && source.peek() == target.peek()
        {
            if copy_instruction.push(lcs.next().unwrap()).is_err() {
                break;
            };
            source.next();
            target.next();
        }
        copy_instruction
    }

    fn soft_construct_remove_instruction(source: &mut Peekable<Iter<u8>>) -> DeltaInstruction {
        let mut remove_instruction = DeltaInstruction::Remove { length: 0 };
        while source.peek().is_some() {
            if remove_instruction.push(source.next().unwrap()).is_err() {
                break;
            };
        }
        remove_instruction
    }

    fn soft_construct_add_instruction(target: &mut Peekable<Iter<u8>>) -> DeltaInstruction {
        let mut add_instruction = DeltaInstruction::Add { content: vec![] };
        while target.peek().is_some() {
            if add_instruction.push(target.next().unwrap()).is_err() {
                break;
            };
        }
        add_instruction
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

#[cfg(test)]
mod delta_patch_tests {
    use crate::delta_instruction::DeltaInstruction;

    use super::DeltaPatch;

    #[test]
    fn construct_remove_instruction() {
        let source = b"AAAX";
        let target = b"BBBX";
        let lcs = DeltaPatch::lcs(source, target);
        assert_eq!(
            DeltaPatch::construct_remove_instruction(
                &mut source.iter().peekable(),
                &mut lcs.iter().peekable()
            ),
            DeltaInstruction::Remove { length: 3 }
        );
    }

    #[test]
    fn construct_add_instruction() {
        let source = b"AAAX";
        let target = b"BBBX";
        let lcs = DeltaPatch::lcs(source, target);
        assert_eq!(
            DeltaPatch::construct_add_instruction(
                &mut target.iter().peekable(),
                &mut lcs.iter().peekable()
            ),
            DeltaInstruction::Add {
                content: vec![b'B', b'B', b'B']
            }
        );
    }

    #[test]
    fn construct_copy_instruction() {
        let source = b"AAA";
        let target = b"AAA";
        let lcs = DeltaPatch::lcs(source, target);
        assert_eq!(
            DeltaPatch::construct_copy_instruction(
                &mut source.iter().peekable(),
                &mut target.iter().peekable(),
                &mut lcs.iter().peekable()
            ),
            DeltaInstruction::Copy { length: 3 }
        );
    }

    #[test]
    fn soft_construct_remove_instruction() {
        let source = b"AAA";
        assert_eq!(
            DeltaPatch::soft_construct_remove_instruction(&mut source.iter().peekable()),
            DeltaInstruction::Remove { length: 3 }
        );
    }

    #[test]
    fn soft_construct_add_instruction() {
        let target = b"BBB";
        assert_eq!(
            DeltaPatch::soft_construct_add_instruction(&mut target.iter().peekable()),
            DeltaInstruction::Add {
                content: vec![b'B', b'B', b'B']
            }
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
                DeltaInstruction::Remove { length: 2 },
                DeltaInstruction::Add {
                    content: vec![66u8, 66u8, 66u8,],
                },
                DeltaInstruction::Copy { length: 3 },
                DeltaInstruction::Remove { length: 1 },
            ]
        );
    }
}
