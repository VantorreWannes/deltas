use std::{iter::Peekable, slice::Iter};

use crate::{
    instructions::{
        add_instruction::AddInstruction, copy_instruction::CopyInstruction,
        delta_instruction::DeltaInstruction, remove_instruction::RemoveInstruction,
        InstructionBytes, InstructionContent, InstructionError, InstructionInfo, Result,
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
        lcs: &mut Peekable<Iter<'_, u8>>,
        source: &mut Peekable<Iter<'_, u8>>,
        target: &mut Peekable<Iter<'_, u8>>,
    ) -> Vec<DeltaInstruction> {
        let mut instructions: Vec<DeltaInstruction> = Vec::new();
        while lcs.peek().is_some() {
            debug_assert!(lcs.len() <= source.len() && lcs.len() <= target.len());
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

    pub fn apply(&self, source: &[u8]) -> Option<Vec<u8>> {
        let mut source_iter = source.iter();
        self.construct_target(&mut source_iter)
    }

    fn construct_target(&self, source: &mut Iter<'_, u8>) -> Option<Vec<u8>> {
        if source.len() != self.source_lenth() {
            return None;
        }
        let mut target: Vec<u8> = Vec::with_capacity(self.target_length());
        for instruction in self.instructions.iter() {
            instruction.apply(source, &mut target)
        }
        Some(target)
    }

    fn target_length(&self) -> usize {
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
            })
    }

    fn source_lenth(&self) -> usize {
        self.instructions
            .iter()
            .fold(0usize, |mut acc, instruction| {
                match instruction {
                    DeltaInstruction::Remove(_) => acc += instruction.len() as usize,
                    DeltaInstruction::Add(_) => (),
                    DeltaInstruction::Copy(_) => acc += instruction.len() as usize,
                };
                acc
            })
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

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut bytes_iter = bytes.iter().peekable();
        let mut instructions: Vec<DeltaInstruction> = Vec::new();
        while bytes_iter.peek().is_some() {
            instructions.push(DeltaInstruction::try_from_bytes(&mut bytes_iter)?);
        }
        Ok(Self { instructions })
    }
}

impl From<&Patch> for Vec<u8> {
    fn from(patch: &Patch) -> Self {
        patch.to_bytes()
    }
}

impl From<Patch> for Vec<u8> {
    fn from(patch: Patch) -> Self {
        patch.to_bytes()
    }
}

impl TryFrom<&[u8]> for Patch {
    type Error = InstructionError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        Patch::try_from_bytes(value)
    }
}

impl TryFrom<Vec<u8>> for Patch {
    type Error = InstructionError;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Patch::try_from_bytes(&value)
    }
}

#[cfg(test)]
mod remove_instruction_tests {
    use std::fs;

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
        assert_eq!(Patch::new(b"AAAAAAAA", b"AAA").target_length(), 3);
        assert_eq!(
            Patch::new(b"AAAAAAAABBBCCC", b"BBBCCCAAA").target_length(),
            9
        );
        assert_eq!(Patch::new(b"AAAAAAAA", b"").target_length(), 0);
        assert_eq!(Patch::new(b"", b"AAA").target_length(), 3);
    }

    #[test]
    fn source_length() {
        assert_eq!(Patch::new(b"AAA", b"AAA").source_lenth(), 3);
        assert_eq!(Patch::new(b"", b"AAA").source_lenth(), 0);
        assert_eq!(Patch::new(b"AAA", b"").source_lenth(), 3);
        assert_eq!(Patch::new(b"AAA", b"BAABBCCCAAA").source_lenth(), 3);
    }

    #[test]
    fn apply() {
        assert_eq!(Patch::new(b"", b"AAA").apply(b""), Some(b"AAA".to_vec()));
        assert_eq!(Patch::new(b"AAA", b"").apply(b"AAA"), Some(b"".to_vec()));
        let source_phrases = vec![
            b"The quick brown fox jumps over the lazy dog.".to_vec(),
            b"Rust is a systems programming language.".to_vec(),
            b"12345".to_vec(),
            b"OpenAI's GPT-3 is a language model.".to_vec(),
            b"Delta encoding is efficient for data compression.".to_vec(),
            b"Unit testing is crucial for software development.".to_vec(),
            b"Markdown is a lightweight markup language.".to_vec(),
            b"C# is used for developing Windows applications.".to_vec(),
            b"Python is known for its simplicity and readability.".to_vec(),
            b"Binary files can be challenging to diff.".to_vec(),
        ];
        let target_phrases = vec![
            b"A slow red cat leaps over a sleepy dog.".to_vec(),
            b"C++ is a high-level programming language.".to_vec(),
            b"67890".to_vec(),
            b"OpenAI's GPT-4 is an AI language model.".to_vec(),
            b"Run-length encoding is effective for data compression.".to_vec(),
            b"Integration testing is vital for software development.".to_vec(),
            b"HTML is a versatile markup language.".to_vec(),
            b"Java is utilized for building Android applications.".to_vec(),
            b"JavaScript is praised for its flexibility and ease of use.".to_vec(),
            b"Text files are easy to compare, unlike binary files.".to_vec(),
        ];
        for (source, target) in source_phrases.iter().zip(target_phrases.iter()) {
            assert_eq!(&Patch::new(&source, &target).apply(source).unwrap(), target);
        }
    }

    #[test]
    fn try_from_bytes() {
        let source = fs::read("files/source.txt").unwrap();
        let target = fs::read("files/target.txt").unwrap();
        let patch = Patch::new(&source, &target);
        let patch_bytes = patch.to_bytes();
        let constructed_patch = Patch::try_from_bytes(&patch_bytes).unwrap();
        assert_eq!(patch, constructed_patch);
    }
}
