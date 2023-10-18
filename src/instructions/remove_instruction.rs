use std::{iter::Peekable, slice::Iter};

use super::{
    InstructionError, InstructionBytes, InstructionContent, InstructionInfo,
    InstructionItem, InstructionLength, Result, REMOVE_INSTRUCTION_SIGN,
};

#[derive(Debug, PartialEq, Clone)]
pub struct RemoveInstruction {
    length: InstructionLength,
}

impl RemoveInstruction {
    pub fn new(length: InstructionLength) -> Self {
        Self { length }
    }
}

impl InstructionInfo for RemoveInstruction {
    fn len(&self) -> InstructionLength {
        self.length.try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }

    fn non_default_item_count(&self) -> Option<InstructionLength> {
        None
    }
}

impl InstructionContent for RemoveInstruction {
    fn push(&mut self, _: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.length += 1;
        Ok(())
    }
}

impl InstructionBytes for RemoveInstruction {
    fn byte_sign(&self) -> u8 {
        REMOVE_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(REMOVE_INSTRUCTION_SIGN);
        bytes.extend(self.len().to_be_bytes());
        bytes
    }

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self> {
        match bytes.next() {
            Some(&REMOVE_INSTRUCTION_SIGN) => (),
            Some(_) => return Err(InstructionError::InvalidSign),
            None => return Err(InstructionError::MissignSign),
        };

        if bytes.peek().is_none() {
            return Err(InstructionError::MissingLength);
        }

        let length_bytes: Vec<u8> = bytes
            .take(std::mem::size_of::<InstructionLength>())
            .copied()
            .collect();
        let length = InstructionLength::from_be_bytes(
            length_bytes
                .as_slice()
                .try_into()
                .map_err(|_| InstructionError::InvalidLength)?,
        );

        Ok(Self { length })
    }
}

impl Default for RemoveInstruction {
    fn default() -> Self {
        Self::new(InstructionLength::MIN)
    }
}

#[cfg(test)]
mod remove_instruction_tests {
    use super::*;

    #[test]
    fn instruction_info() {
        let mut instruction = RemoveInstruction::new(InstructionLength::MAX);
        assert_eq!(instruction.len(), InstructionLength::MAX);
        assert!(instruction.is_full());

        instruction = RemoveInstruction::new(InstructionLength::MIN);
        assert_eq!(instruction.len(), InstructionLength::MIN);
        assert!(instruction.is_empty());

        let default_instruction = RemoveInstruction::default();
        assert_eq!(default_instruction, instruction);
    }

    #[test]
    fn instruction_content() {
        let mut instruction = RemoveInstruction::new(InstructionLength::MAX - 1);
        assert!(instruction.push(InstructionItem::default()).is_ok());
        assert_eq!(
            instruction.push(InstructionItem::default()),
            Err(InstructionError::ContentOverflow)
        );
    }

    #[test]
    fn instruction_bytes_to_bytes() {
        let mut instruction = RemoveInstruction::new(InstructionLength::MAX);
        let mut bytes = vec![REMOVE_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        assert_eq!(instruction.to_bytes(), bytes);

        instruction = RemoveInstruction::default();
        bytes = vec![REMOVE_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        assert_eq!(instruction.to_bytes(), bytes);
    }

    #[test]
    fn instruction_bytes_try_from_bytes_ok() {
        let mut instruction = RemoveInstruction::new(InstructionLength::MAX);
        let mut bytes = instruction.to_bytes();
        assert_eq!(
            RemoveInstruction::try_from_bytes(&mut bytes.iter().peekable()),
            Ok(instruction)
        );

        instruction = RemoveInstruction::default();
        bytes = instruction.to_bytes();
        assert_eq!(
            RemoveInstruction::try_from_bytes(&mut bytes.iter().peekable()),
            Ok(instruction)
        );
    }

    #[test]
    fn instruction_bytes_try_from_bytes_err() {
        let mut bytes: Vec<u8> = vec![];
        assert_eq!(
            RemoveInstruction::try_from_bytes(&mut bytes.iter().peekable()),
            Err(InstructionError::MissignSign)
        );
        bytes = vec![InstructionItem::default()];
        assert_eq!(
            RemoveInstruction::try_from_bytes(&mut bytes.iter().peekable()),
            Err(InstructionError::InvalidSign)
        );
        bytes = vec![REMOVE_INSTRUCTION_SIGN];
        assert_eq!(
            RemoveInstruction::try_from_bytes(&mut bytes.iter().peekable()),
            Err(InstructionError::MissingLength)
        );
    }
}
