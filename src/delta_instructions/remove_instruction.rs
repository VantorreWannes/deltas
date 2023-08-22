use std::{mem, slice::Iter};

use super::{traits::{PushToInstruction, InstructionBytes}, errors::RemoveInstructionError};

pub type RemoveInstructionlength = u8;

#[derive(Debug, PartialEq, Default)]
pub struct RemoveInstruction {
    length: RemoveInstructionlength,
}


impl RemoveInstruction {
    pub fn new(length: impl Into<RemoveInstructionlength>) -> RemoveInstruction {
        Self { length: length.into() }
    }

    pub fn len(&self) -> RemoveInstructionlength {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl PushToInstruction for RemoveInstruction {
    type Error = RemoveInstructionError;

    fn push(&mut self, _: u8) -> Result<(), Self::Error> {
        self.length = self.length.checked_add(1).ok_or(RemoveInstructionError::MaxLengthReached)?;
        Ok(())
    }
}

impl InstructionBytes for RemoveInstruction {
    
    type Error = RemoveInstructionError;
    const INSTRUCTION_BYTE_SIGN: u8 = b'-';
    const NUMBER_BYTES_LENGTH: usize = mem::size_of::<RemoveInstructionlength>();

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(1 + Self::NUMBER_BYTES_LENGTH);
        bytes.push(Self::INSTRUCTION_BYTE_SIGN);
        bytes.extend(self.length.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &mut std::slice::Iter<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized {
            match bytes.next() {
                Some(&Self::INSTRUCTION_BYTE_SIGN) => Ok(()),
                Some(byte) => Err(RemoveInstructionError::InvalidSignByte(Some(*byte))),
                None => Err(RemoveInstructionError::InvalidSignByte(None)),
            }?;
    
            match bytes.size_hint().0 {
                0 => Err(RemoveInstructionError::InvalidLengthBytes(None)),
                number_bytes_length if number_bytes_length < Self::NUMBER_BYTES_LENGTH => Err(RemoveInstructionError::InvalidLengthBytes(Some(number_bytes_length))),
                number_bytes_length => Ok(number_bytes_length),
            }?;
    
            let number_bytes: [u8; Self::NUMBER_BYTES_LENGTH] = bytes
                .take(Self::NUMBER_BYTES_LENGTH)
                .copied()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
    
            let length = RemoveInstructionlength::from_be_bytes(number_bytes);
            Ok(Self::new(length))
    }
}


impl From<&RemoveInstruction> for Vec<u8> {
    fn from(value: &RemoveInstruction) -> Self {
        value.to_bytes()
    }
}

impl From<RemoveInstruction> for Vec<u8> {
    fn from(value: RemoveInstruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Iter<'_, u8>> for RemoveInstruction {
    type Error = RemoveInstructionError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
       RemoveInstruction::from_bytes(value)
    }
}

impl TryFrom<Iter<'_, u8>> for RemoveInstruction {
    type Error = RemoveInstructionError;

    fn try_from(mut value: Iter<'_, u8>) -> Result<Self, Self::Error> {
        RemoveInstruction::from_bytes(&mut value)
    }
}


#[cfg(test)]
mod remove_instruction_tests {
    use super::*;

    #[test]
    fn new() {
        let length: RemoveInstructionlength = RemoveInstructionlength::MAX;
        let new_remove = RemoveInstruction::new(length);
        assert_eq!(new_remove, RemoveInstruction { length });
    }

    #[test]
    fn default() {
        let new_remove = RemoveInstruction::new(0u8);
        assert_eq!(new_remove, RemoveInstruction::default());
    }

    #[test]
    fn len() {
        let new_remove = RemoveInstruction::new(10u8);
        assert_eq!(new_remove.len(), 10);
    }

    #[test]
    fn is_empty() {
        let mut default_remove = RemoveInstruction::default();
        assert!(default_remove.is_empty());
        default_remove.push(b'A').unwrap();
        assert!(!default_remove.is_empty());
    }

    #[test]
    fn push() {
        let mut new_remove = RemoveInstruction::new(RemoveInstructionlength::MAX - 1);
        assert!(new_remove.push(0).is_ok());
        assert_eq!(new_remove.len(), RemoveInstructionlength::MAX.try_into().unwrap());
        assert!(new_remove.push(0).is_err());
        assert_eq!(new_remove.len(), RemoveInstructionlength::MAX.try_into().unwrap());
    }

    #[test]
    fn into_bytes() {
        let mut bytes = vec![RemoveInstruction::INSTRUCTION_BYTE_SIGN];
        bytes.extend(RemoveInstructionlength::MIN.to_be_bytes());
        let mut default_remove = RemoveInstruction::default();
        assert_eq!(default_remove.to_bytes(), bytes);
        default_remove.push(b'A').unwrap();
        bytes.resize(1, 0);
        bytes.extend(RemoveInstructionlength::from(1u8).to_be_bytes());
        assert_eq!(default_remove.to_bytes(),bytes);
    }

    #[test]
    fn from_bytes_ok() {
        let mut remove_bytes = vec![RemoveInstruction::INSTRUCTION_BYTE_SIGN];
        remove_bytes.extend(RemoveInstructionlength::default().to_be_bytes());
        let default_remove = RemoveInstruction::from_bytes(&mut remove_bytes.iter());
        assert!(default_remove.is_ok());
    }

    #[test]
    fn from_bytes_sign_err() {
        let remove_bytes = vec![];
        let default_remove = RemoveInstruction::from_bytes(&mut remove_bytes.iter());
        assert!(default_remove.is_err());
        assert_eq!(
            default_remove.unwrap_err(),
            RemoveInstructionError::InvalidSignByte(None)
        );
    }

    #[test]
    fn from_bytes_length_err() {
        let remove_bytes = vec![RemoveInstruction::INSTRUCTION_BYTE_SIGN];
        let default_remove = RemoveInstruction::from_bytes(&mut remove_bytes.iter());
        assert!(default_remove.is_err());
        assert_eq!(
            default_remove.unwrap_err(),
            RemoveInstructionError::InvalidLengthBytes(None)
        );
    }
}
