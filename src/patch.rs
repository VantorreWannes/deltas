use crate::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    content: Vec<Instruction>,
}

impl Patch {
    pub fn new(source: &[u8], target: &[u8]) -> Self {
        todo!();
    }

    fn byte_len(&self) -> usize {
        self.content
            .iter()
            .map(|instruction| match instruction {
                Instruction::Remove { length: _ } => 2,
                Instruction::Add { content } | Instruction::Copy { content } => content.len() + 2,
            })
            .sum()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_len());
        for instruction in self.content.iter() {
            bytes.append(&mut instruction.into());
        }
        bytes
    }
}

#[cfg(test)]
mod patch_tests {
    use crate::instructions::MAX_INSTRUCTION_LENGTH;

    use super::*;

    #[test]
    fn byte_len() {
        let mut patch = Patch {
            content: vec![Instruction::Remove {
                length: MAX_INSTRUCTION_LENGTH,
            }],
        };
        assert_eq!(patch.byte_len(), 2);

        patch.content = vec![Instruction::Add {
            content: vec![0; MAX_INSTRUCTION_LENGTH.into()],
        }];

        assert_eq!(patch.byte_len(), 2 + MAX_INSTRUCTION_LENGTH as usize);
    }

    #[test]
    fn to_bytes() {
        let mut instruction = Instruction::Remove {
            length: MAX_INSTRUCTION_LENGTH,
        };
        let mut patch = Patch {
            content: vec![instruction.clone()],
        };

        assert_eq!(patch.to_bytes(), instruction.to_bytes());

        instruction = Instruction::Add { content: vec![0; MAX_INSTRUCTION_LENGTH.into()] }; 
        patch.content = vec![instruction.clone()];

        assert_eq!(patch.to_bytes(), instruction.to_bytes());

        instruction = Instruction::Copy { content: vec![0; MAX_INSTRUCTION_LENGTH.into()] }; 
        patch.content = vec![instruction.clone()];

        assert_eq!(patch.to_bytes(), instruction.to_bytes());
    }
}
