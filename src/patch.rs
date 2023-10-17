use crate::{instructions::Instruction, lcs::Lcs};

pub const ERROR_PERCENT_TRESHOLD: usize = 50;

#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    content: Vec<Instruction>,
}

impl Patch {
    pub fn new(source: &[u8], target: &[u8]) -> Self {
        let mut content: Vec<Instruction> = Vec::new();
        let lcs = Lcs::new(source, target).subsequence();
        let (mut source_index, mut target_index, mut lcs_index) = (0, 0, 0);
        while source_index < source.len() && target_index < target.len() {
            if let Some(instruction) = content.last_mut().filter(|instruction| {
                instruction.copy_error().is_some_and(|error| {
                    ((instruction.len() as usize * ERROR_PERCENT_TRESHOLD) / 100) > error as usize
                })
            }) {
                //Copy
                let adjusted_difference = target[target_index].wrapping_sub(source[source_index]);
                if !instruction.is_full() {
                    instruction.push(adjusted_difference).unwrap();
                } else {
                    content.push(Instruction::Copy {
                        content: vec![adjusted_difference],
                        error: 1,
                    });
                }
                source_index += 1;
                target_index += 1;
            } else if source[source_index] != lcs[lcs_index] {
                // Remove
                if let Some(instruction) = content
                    .last_mut()
                    .filter(|instruction| !instruction.is_full() && instruction.is_remove())
                {
                    instruction.push(source[source_index]).unwrap();
                } else {
                    content.push(Instruction::Remove { length: 1 });
                }
                source_index += 1;
            } else if target[target_index] != lcs[lcs_index] {
                //Add
                if let Some(instruction) = content
                    .last_mut()
                    .filter(|instruction| !instruction.is_full() && instruction.is_add())
                {
                    instruction.push(target[target_index]).unwrap();
                } else {
                    content.push(Instruction::Add {
                        content: vec![target[target_index]],
                    });
                }
                target_index += 1;
            }
        }
        if source.len() > source_index {
            if let Some(instruction) = content
                .last_mut()
                .filter(|instruction| !instruction.is_full() && instruction.is_remove())
            {
                instruction.push(source[source_index]).unwrap();
            } else {
                content.push(Instruction::Remove { length: 1 });
            }
        } else if target.len() > target_index {
            if let Some(instruction) = content
                .last_mut()
                .filter(|instruction| !instruction.is_full() && instruction.is_add())
            {
                instruction.push(target[target_index]).unwrap();
            } else {
                content.push(Instruction::Add {
                    content: vec![target[target_index]],
                });
            }
        } else if source[source_index] == lcs[lcs_index] && target[target_index] == lcs[lcs_index] {
            //Copy
            if let Some(instruction) = content
                .last_mut()
                .filter(|instruction| instruction.is_copy() && !instruction.is_full())
            {
                instruction.push(lcs[lcs_index]).unwrap();
            } else {
                content.push(Instruction::Copy {
                    content: vec![lcs[lcs_index]],
                    error: 0,
                });
            }
            source_index += 1;
            target_index += 1;
            lcs_index += 1;
        }

        Self { content }
    }

    fn byte_len(&self) -> usize {
        self.content
            .iter()
            .map(|instruction| match instruction {
                Instruction::Remove { length: _ } => 2,
                Instruction::Add { content } | Instruction::Copy { content, .. } => {
                    content.len() + 2
                }
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

        instruction = Instruction::Add {
            content: vec![0; MAX_INSTRUCTION_LENGTH.into()],
        };
        patch.content = vec![instruction.clone()];

        assert_eq!(patch.to_bytes(), instruction.to_bytes());

        instruction = Instruction::Copy {
            content: vec![0; MAX_INSTRUCTION_LENGTH.into()],
            error: 0,
        };
        patch.content = vec![instruction.clone()];

        assert_eq!(patch.to_bytes(), instruction.to_bytes());
    }

    #[test]
    fn new() {
        dbg!(Patch::new(&vec![0; 256], &vec![1; 256]));
    }
}
