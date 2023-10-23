use std::{iter::Peekable, slice::Iter};

use super::{
    InstructionBytes, InstructionContent, InstructionError, InstructionInfo, Result,
    ADD_INSTRUCTION_SIGN,
};

#[derive(Debug, PartialEq, Clone)]
pub struct AddInstruction {
    content: Vec<u8>,
}

impl AddInstruction {
    pub fn new(content: Vec<u8>) -> Self {
        assert!(
            content.len() <= u8::MAX.into(),
            "Instruction content exceeded 255 items",
        );
        Self { content }
    }
}

impl InstructionInfo for AddInstruction {
    fn len(&self) -> u8 {
        self.content.len() as u8
    }

    fn is_empty(&self) -> bool {
        self.len() == u8::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == u8::MAX
    }

    fn non_default_item_count(&self) -> Option<u8> {
        Some(self.content.iter().filter(|item| **item != 0).count() as u8)
    }
}

impl InstructionContent for AddInstruction {
    fn push(&mut self, content: u8) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.content.push(content);
        Ok(())
    }

    fn fill(
        &mut self,
        lcs: &mut Peekable<Iter<'_, u8>>,
        _: &mut Peekable<Iter<'_, u8>>,
        target: &mut Peekable<Iter<'_, u8>>,
    ) {
        while target.peek().is_some() && lcs.peek() != target.peek() && !self.is_full() {
            self.push(*target.next().unwrap()).unwrap();
        }
    }

    fn apply(&self, _: &mut Iter<'_, u8>, target: &mut Vec<u8>) {
        target.extend(self.content.iter());
    }
}

impl InstructionBytes for AddInstruction {
    fn byte_sign(&self) -> u8 {
        ADD_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        self.len() as usize + 2
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(self.byte_sign());
        bytes.push(self.len());
        bytes.extend(self.content.iter());
        bytes
    }

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self> {
        match bytes.next() {
            Some(&ADD_INSTRUCTION_SIGN) => (),
            Some(_) => return Err(InstructionError::InvalidSign),
            None => return Err(InstructionError::MissignSign),
        };

        let length = *bytes.next().ok_or(InstructionError::MissingLength)? as usize;
        let content: Vec<u8> = bytes.take(length).copied().collect();

        if content.len() < length {
            return Err(InstructionError::MissingContent);
        }

        Ok(Self { content })
    }
}

impl Default for AddInstruction {
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}

impl From<&AddInstruction> for Vec<u8> {
    fn from(value: &AddInstruction) -> Self {
        value.to_bytes()
    }
}

impl From<AddInstruction> for Vec<u8> {
    fn from(value: AddInstruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for AddInstruction {
    type Error = InstructionError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        AddInstruction::try_from_bytes(value)
    }
}

impl TryFrom<Peekable<Iter<'_, u8>>> for AddInstruction {
    type Error = InstructionError;

    fn try_from(mut value: Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        AddInstruction::try_from_bytes(&mut value)
    }
}

impl TryFrom<Vec<u8>> for AddInstruction {
    type Error = InstructionError;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        AddInstruction::try_from_bytes(&mut value.iter().peekable())
    }
}

impl TryFrom<&[u8]> for AddInstruction {
    type Error = InstructionError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        AddInstruction::try_from_bytes(&mut value.iter().peekable())
    }
}

#[cfg(test)]
mod add_instruction_tests {
    use crate::lcs::Lcs;

    use super::*;

    #[test]
    fn instruction_info() {
        let mut instruction = AddInstruction::new(vec![0; u8::MAX.try_into().unwrap()]);
        assert_eq!(instruction.len(), u8::MAX);
        assert!(instruction.is_full());

        instruction = AddInstruction::new(Vec::new());
        assert_eq!(instruction.len(), u8::MIN);
        assert!(instruction.is_empty());

        instruction = AddInstruction::default();
        assert_eq!(instruction.len(), u8::MIN);
        assert!(instruction.is_empty());
    }

    #[test]
    fn non_default_item_count() {
        let mut instruction = AddInstruction::default();
        for _ in 0..(u8::MAX / 2) {
            instruction.push(0).unwrap();
            assert_eq!(instruction.non_default_item_count().unwrap(), 0);
        }
        for i in 0..(u8::MAX / 2) {
            instruction.push(0 + 1).unwrap();
            assert_eq!(instruction.non_default_item_count().unwrap(), i + 1);
        }
    }

    #[test]
    fn instruction_content_push() {
        let mut instruction = AddInstruction::new(vec![0; (u8::MAX - 1).try_into().unwrap()]);
        assert!(instruction.push(0).is_ok());
        assert!(instruction
            .push(0)
            .is_err_and(|err| err == InstructionError::ContentOverflow));
    }

    fn fill_wrapper(source: &[u8], target: &[u8]) -> AddInstruction {
        let mut instruction = AddInstruction::default();
        let lcs = Lcs::new(source, target).subsequence();
        let mut lcs_iter = lcs.iter().peekable();
        let mut source_iter = source.iter().peekable();
        let mut target_iter = target.iter().peekable();
        instruction.fill(&mut lcs_iter, &mut source_iter, &mut target_iter);
        instruction
    }

    #[test]
    fn instruction_content_fill() {
        let instruction = fill_wrapper(b"", b"AAA");
        assert_eq!(instruction.len(), 3);
        let instruction = fill_wrapper(b"B", b"AAA");
        assert_eq!(instruction.len(), 3);
        let instruction = fill_wrapper(b"BBA", b"AAA");
        assert_eq!(instruction.len(), 0);
    }

    #[test]
    fn instruction_bytes_to_bytes() {
        let mut instruction = AddInstruction::new(vec![0; u8::MAX.try_into().unwrap()]);
        let mut bytes = vec![ADD_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        bytes.extend(instruction.content.iter());
        assert_eq!(instruction.to_bytes(), bytes);

        instruction = AddInstruction::default();
        bytes = vec![ADD_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        assert_eq!(instruction.to_bytes(), bytes);
    }

    #[test]
    fn instruction_bytes_try_from_bytes_ok() {
        let mut instruction = AddInstruction::new(vec![0; u8::MAX.try_into().unwrap()]);
        assert_eq!(
            AddInstruction::try_from_bytes(&mut instruction.to_bytes().iter().peekable()).unwrap(),
            instruction
        );

        instruction = AddInstruction::default();
        assert_eq!(
            AddInstruction::try_from_bytes(&mut instruction.to_bytes().iter().peekable()).unwrap(),
            instruction
        );
    }

    #[test]
    fn instruction_bytes_try_from_bytes_err() {
        let mut bytes = vec![];

        assert_eq!(
            AddInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::MissignSign
        );

        bytes = vec![0];
        assert_eq!(
            AddInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::InvalidSign
        );

        bytes = vec![ADD_INSTRUCTION_SIGN];
        bytes.extend(u8::MAX.to_be_bytes());
        bytes.append(&mut vec![0; u8::MAX as usize - 1]);
        assert_eq!(
            AddInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::MissingContent
        );
    }
}
