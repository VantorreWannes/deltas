use std::{iter::Peekable, slice::Iter};

use super::{
    InstructionError, InstructionBytes, InstructionContent, InstructionInfo,
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

    fn non_default_item_count(&self) -> Option<InstructionLength> {
        Some(
            self.content
                .iter()
                .filter(|item| **item != InstructionItem::default())
                .count() as InstructionLength,
        )
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
    fn byte_sign(&self) -> u8 {
        ADD_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(self.byte_sign());
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

        let content_bytes: Vec<u8> = bytes.take(length.try_into().unwrap()).copied().collect();

        let content: Result<Vec<InstructionItem>> = content_bytes
            .chunks(std::mem::size_of::<InstructionItem>())
            .map(|chunk: &[u8]| -> Result<InstructionItem> {
                Ok(InstructionItem::from_be_bytes(
                    chunk
                        .try_into()
                        .map_err(|_| InstructionError::InvalidContent)?,
                ))
            })
            .collect();

        let content = content?;

        if content.len() < length as usize {
            return Err(InstructionError::MissingContent);
        }

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

        bytes = vec![ADD_INSTRUCTION_SIGN, InstructionLength::MAX];
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
