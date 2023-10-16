use crate::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    content: Vec<Instruction>,
}

impl Patch {
    pub fn new(source: &[u8], target: &[u8]) -> Self {
        todo!();
    }

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
