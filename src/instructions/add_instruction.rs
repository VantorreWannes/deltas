use std::{iter::Peekable, slice::Iter};

use super::{
    error::InstructionError,
    traits::{InstructionBytes, InstructionContent, InstructionInfo},
    InstructionItem, InstructionLength, Result, ADD_INSTRUCTION_SIGN,
};

#[derive(Debug, PartialEq, Clone)]
pub struct AddInstruction {
    content: Vec<InstructionItem>,
}

impl AddInstruction {
    pub fn new(content: Vec<InstructionItem>) -> Self {
        assert!(
            content.len() <= InstructionLength::MAX.try_into().unwrap(),
            "Instruction content exceeded {} items",
            InstructionLength::MAX
        );
        Self { content }
    }
}

impl InstructionInfo for AddInstruction {
    fn len(&self) -> InstructionLength {
        self.content.len().try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }
}

impl InstructionContent for AddInstruction {
    fn push(&mut self, content: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.content.push(content);
        Ok(())
    }
}

impl InstructionBytes for AddInstruction {
    fn byte_sign() -> u8 {
        ADD_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(ADD_INSTRUCTION_SIGN);
        bytes.extend(self.len().to_be_bytes());
        bytes.extend(self.content.iter());
        bytes
    }

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self> {
        match bytes.next() {
            Some(&ADD_INSTRUCTION_SIGN) => (),
            Some(_) => return Err(InstructionError::InvalidSign),
            None => return Err(InstructionError::MissignSign),
        };
        let length: InstructionLength = {
            let length_bytes: Vec<u8> = bytes
                .take(std::mem::size_of::<InstructionLength>())
                .copied()
                .collect();
            InstructionLength::from_be_bytes(length_bytes.as_slice().try_into().unwrap())
        };

        let content: Vec<InstructionItem> = {
            let content_bytes: Vec<u8> = bytes
                .take(
                    length
                        .try_into()
                        .map_err(|_| InstructionError::MissingLength)?,
                )
                .copied()
                .collect();
            content_bytes
                .chunks_exact(std::mem::size_of::<InstructionItem>())
                .map(|chunk| InstructionItem::from_be_bytes(chunk.try_into().unwrap()))
                .collect()
        };
        if content.len() < length as usize {
            return Err(InstructionError::MissingContent);
        };
        Ok(Self { content })
    }
}

impl Default for AddInstruction {
    fn default() -> Self {
        Self::new(vec![
            InstructionItem::default();
            InstructionLength::MIN.try_into().unwrap()
        ])
    }
}

#[cfg(test)]
mod add_instruction_tests {
    use super::*;

    #[test]
    fn instruction_info() {
        let mut instruction = AddInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
        assert_eq!(instruction.len(), InstructionLength::MAX);
        assert!(instruction.is_full());

        instruction = AddInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MIN.try_into().unwrap()
        ]);
        assert_eq!(instruction.len(), InstructionLength::MIN);
        assert!(instruction.is_empty());

        instruction = AddInstruction::default();
        assert_eq!(instruction.len(), InstructionLength::MIN);
        assert!(instruction.is_empty());
    }

    #[test]
    fn instruction_content() {
        let mut instruction =
            AddInstruction::new(vec![
                InstructionItem::default();
                (InstructionLength::MAX - 1).try_into().unwrap()
            ]);
        assert!(instruction.push(InstructionItem::default()).is_ok());
        assert!(instruction
            .push(InstructionItem::default())
            .is_err_and(|err| err == InstructionError::ContentOverflow));
    }

    #[test]
    fn instruction_bytes_to_bytes() {
        let mut instruction = AddInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
        let mut bytes = vec![AddInstruction::byte_sign()];
        bytes.extend(instruction.len().to_be_bytes());
        bytes.extend(instruction.content.iter());
        assert_eq!(instruction.to_bytes(), bytes);

        instruction = AddInstruction::default();
        bytes = vec![AddInstruction::byte_sign()];
        bytes.extend(instruction.len().to_be_bytes());
        assert_eq!(instruction.to_bytes(), bytes);
    }

    #[test]
    fn instruction_bytes_try_from_bytes_ok() {
        let mut instruction = AddInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
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

        bytes = vec![AddInstruction::byte_sign(), InstructionLength::MAX];
        bytes.append(&mut vec![
            InstructionItem::default();
            InstructionLength::MAX as usize - 1
        ]);
        assert_eq!(
            AddInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::MissingContent
        );
    }
}
