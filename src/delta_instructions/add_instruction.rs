use super::{
    errors::AddInstructionError,
    traits::{InstructionBytes, PushToInstruction},
};
use std::{mem, slice::Iter};

pub type AddInstructionlength = u8;

#[derive(Debug, PartialEq)]
pub struct AddInstruction {
    content: Vec<u8>,
}

impl AddInstruction {
    pub fn new(content: &[u8]) -> Result<AddInstruction, AddInstructionError> {
        if content.len() > AddInstructionlength::MAX.into() {
            return Err(AddInstructionError::MaxLengthReached);
        }
        Ok(Self {
            content: content.into(),
        })
    }

    pub fn len(&self) -> u8 {
        debug_assert!(self.content.len() <= AddInstructionlength::MAX.into());
        self.content.len().try_into().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl PushToInstruction for AddInstruction {
    type Error = AddInstructionError;

    fn push(&mut self, byte: u8) -> Result<(), Self::Error> {
        if self.content.len() >= AddInstructionlength::MAX.into() {
            return Err(AddInstructionError::MaxLengthReached);
        }
        self.content.push(byte);
        Ok(())
    }
}

impl Default for AddInstruction {
    fn default() -> Self {
        return Self { content: vec![] };
    }
}

impl InstructionBytes for AddInstruction {
    type Error = AddInstructionError;
    const INSTRUCTION_BYTE_SIGN: u8 = b'+';
    const NUMBER_BYTES_LENGTH: usize = mem::size_of::<AddInstructionlength>();

    fn to_bytes(&self) -> Vec<u8> {
        debug_assert!(self.content.len() <= AddInstructionlength::MAX.into());
        let mut bytes: Vec<u8> = Vec::with_capacity(self.content.len() + 2);
        bytes.push(Self::INSTRUCTION_BYTE_SIGN);
        bytes.extend((self.content.len() as AddInstructionlength).to_be_bytes());
        bytes.extend(self.content.iter());
        bytes
    }

    fn from_bytes(bytes: &mut Iter<u8>) -> Result<Self, AddInstructionError>
    where
        Self: Sized,
    {
        let sign = bytes
            .next()
            .ok_or(AddInstructionError::InvalidSignByte(b' '))?;
        if sign != &b'+' {
            return Err(AddInstructionError::InvalidSignByte(*sign));
        }

        if bytes.size_hint().0 < Self::NUMBER_BYTES_LENGTH {
            return Err(AddInstructionError::InvalidLengthBytes(bytes.size_hint().0));
        }

        let number_bytes: [u8; Self::NUMBER_BYTES_LENGTH] = bytes
            .take(Self::NUMBER_BYTES_LENGTH)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let length = AddInstructionlength::from_be_bytes(number_bytes).into();
        let content: Vec<u8> = bytes.take(length).copied().collect();
        if content.len() < length {
            return Err(AddInstructionError::MissingByteContent(
                content.len(),
                length.try_into().unwrap(),
            ));
        }
        Self::new(&content)
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

impl TryFrom<&mut Iter<'_, u8>> for AddInstruction {
    type Error = AddInstructionError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
        AddInstruction::from_bytes(value)
    }
}

impl TryFrom<Iter<'_, u8>> for AddInstruction {
    type Error = AddInstructionError;

    fn try_from(mut value: Iter<'_, u8>) -> Result<Self, Self::Error> {
        AddInstruction::from_bytes(&mut value)
    }
}

#[cfg(test)]
mod add_instruction_tests {
    use super::*;

    #[test]
    fn new() {
        let content: Vec<u8> = vec![0, 1, 2, 3];
        let new_add = AddInstruction::new(&content);
        assert!(new_add.is_ok());
        assert_eq!(new_add.unwrap(), AddInstruction { content });

        let content: Vec<u8> = vec![0; AddInstructionlength::MAX as usize + 1];
        let new_add = AddInstruction::new(&content);
        assert!(new_add.is_err());
        assert_eq!(new_add.unwrap_err(), AddInstructionError::MaxLengthReached);
    }

    #[test]
    fn default() {
        let new_add = AddInstruction::new(&vec![]).unwrap();
        assert_eq!(new_add, AddInstruction::default());
    }

    #[test]
    fn len() {
        let new_add = AddInstruction::new(&vec![1, 1, 1]).unwrap();
        assert_eq!(new_add.len(), 3);
    }

    #[test]
    fn is_empty() {
        let mut default_add = AddInstruction::default();
        assert!(default_add.is_empty());
        default_add.push(b'A').unwrap();
        assert!(!default_add.is_empty());
    }

    #[test]
    fn push() {
        let mut new_add =
            AddInstruction::new(&vec![0; <usize>::from(AddInstructionlength::MAX) - 1]).unwrap();
        assert!(new_add.push(0).is_ok());
        assert_eq!(new_add.len(), AddInstructionlength::MAX.into());
        assert!(new_add.push(0).is_err());
        assert_eq!(new_add.len(), AddInstructionlength::MAX.into());
    }

    #[test]
    fn into_bytes() {
        let mut default_add = AddInstruction::default();
        assert_eq!(
            default_add.to_bytes(),
            vec![AddInstruction::INSTRUCTION_BYTE_SIGN, 0]
        );
        default_add.push(b'A').unwrap();
        assert_eq!(
            default_add.to_bytes(),
            vec![AddInstruction::INSTRUCTION_BYTE_SIGN, 1, b'A']
        );
    }

    #[test]
    fn from_bytes_ok() {
        let mut add_bytes = vec![AddInstruction::INSTRUCTION_BYTE_SIGN];
        add_bytes.extend(AddInstructionlength::default().to_be_bytes());
        let default_add = AddInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_ok());
    }

    #[test]
    fn from_bytes_sign_err() {
        let mut add_bytes = vec![b' '];
        add_bytes.extend(AddInstructionlength::default().to_be_bytes());
        let default_add = AddInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_err());
        assert_eq!(
            default_add.unwrap_err(),
            AddInstructionError::InvalidSignByte(b' ')
        );
    }

    #[test]
    fn from_bytes_length_err() {
        let add_bytes = vec![AddInstruction::INSTRUCTION_BYTE_SIGN];
        let default_add = AddInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_err());
        assert_eq!(
            default_add.unwrap_err(),
            AddInstructionError::InvalidLengthBytes(0)
        );
    }

    #[test]
    fn from_bytes_content_err() {
        let mut add_bytes = vec![AddInstruction::INSTRUCTION_BYTE_SIGN];
        let length: AddInstructionlength = 1;
        add_bytes.extend(length.to_be_bytes());
        let default_add = AddInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_err());
        assert_eq!(
            default_add.unwrap_err(),
            AddInstructionError::MissingByteContent(0, length)
        );
    }
}
