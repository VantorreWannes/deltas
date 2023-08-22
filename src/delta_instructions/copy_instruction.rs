use std::{mem, slice::Iter};

use super::{traits::{PushToInstruction, InstructionBytes}, errors::CopyInstructionError};

pub type CopyInstructionlength = u8;

#[derive(Debug, PartialEq, Default)]
pub struct CopyInstruction {
    length: CopyInstructionlength,
}


impl CopyInstruction {
    pub fn new(length: impl Into<CopyInstructionlength>) -> CopyInstruction {
        Self { length: length.into() }
    }

    pub fn len(&self) -> CopyInstructionlength {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl PushToInstruction for CopyInstruction {
    type Error = CopyInstructionError;

    fn push(&mut self, _: u8) -> Result<(), Self::Error> {
        self.length = self.length.checked_add(1).ok_or(CopyInstructionError::MaxLengthReached)?;
        Ok(())
    }
}

impl InstructionBytes for CopyInstruction {
    
    type Error = CopyInstructionError;
    const INSTRUCTION_BYTE_SIGN: u8 = b'|';
    const NUMBER_BYTES_LENGTH: usize = mem::size_of::<CopyInstructionlength>();

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
                Some(byte) => Err(CopyInstructionError::InvalidSignByte(Some(*byte))),
                None => Err(CopyInstructionError::InvalidSignByte(None)),
            }?;
    
            match bytes.size_hint().0 {
                0 => Err(CopyInstructionError::InvalidLengthBytes(None)),
                number_bytes_length if number_bytes_length < Self::NUMBER_BYTES_LENGTH => Err(CopyInstructionError::InvalidLengthBytes(Some(number_bytes_length))),
                number_bytes_length => Ok(number_bytes_length),
            }?;
    
            let number_bytes: [u8; Self::NUMBER_BYTES_LENGTH] = bytes
                .take(Self::NUMBER_BYTES_LENGTH)
                .copied()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
    
            let length = CopyInstructionlength::from_be_bytes(number_bytes);
            Ok(Self::new(length))
    }
}


impl From<&CopyInstruction> for Vec<u8> {
    fn from(value: &CopyInstruction) -> Self {
        value.to_bytes()
    }
}

impl From<CopyInstruction> for Vec<u8> {
    fn from(value: CopyInstruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Iter<'_, u8>> for CopyInstruction {
    type Error = CopyInstructionError;

    fn try_from(value: &mut Iter<'_, u8>) -> Result<Self, Self::Error> {
       CopyInstruction::from_bytes(value)
    }
}

impl TryFrom<Iter<'_, u8>> for CopyInstruction {
    type Error = CopyInstructionError;

    fn try_from(mut value: Iter<'_, u8>) -> Result<Self, Self::Error> {
        CopyInstruction::from_bytes(&mut value)
    }
}


#[cfg(test)]
mod copy_instruction_tests {
    use super::*;

    #[test]
    fn new() {
        let length: CopyInstructionlength = CopyInstructionlength::MAX;
        let new_copy = CopyInstruction::new(length);
        assert_eq!(new_copy, CopyInstruction { length });
    }

    #[test]
    fn default() {
        let new_copy = CopyInstruction::new(0u8);
        assert_eq!(new_copy, CopyInstruction::default());
    }

    #[test]
    fn len() {
        let new_copy = CopyInstruction::new(10u8);
        assert_eq!(new_copy.len(), 10);
    }

    #[test]
    fn is_empty() {
        let mut default_copy = CopyInstruction::default();
        assert!(default_copy.is_empty());
        default_copy.push(b'A').unwrap();
        assert!(!default_copy.is_empty());
    }

    #[test]
    fn push() {
        let mut new_copy = CopyInstruction::new(CopyInstructionlength::MAX - 1);
        assert!(new_copy.push(0).is_ok());
        assert_eq!(new_copy.len(), CopyInstructionlength::MAX.try_into().unwrap());
        assert!(new_copy.push(0).is_err());
        assert_eq!(new_copy.len(), CopyInstructionlength::MAX.try_into().unwrap());
    }

    #[test]
    fn into_bytes() {
        let mut bytes = vec![CopyInstruction::INSTRUCTION_BYTE_SIGN];
        bytes.extend(CopyInstructionlength::MIN.to_be_bytes());
        let mut default_add = CopyInstruction::default();
        assert_eq!(default_add.to_bytes(), bytes);
        default_add.push(b'A').unwrap();
        bytes.resize(1, 0);
        bytes.extend(CopyInstructionlength::from(1u8).to_be_bytes());
        assert_eq!(default_add.to_bytes(),bytes);
    }

    #[test]
    fn from_bytes_ok() {
        let mut add_bytes = vec![CopyInstruction::INSTRUCTION_BYTE_SIGN];
        add_bytes.extend(CopyInstructionlength::default().to_be_bytes());
        let default_add = CopyInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_ok());
    }

    #[test]
    fn from_bytes_sign_err() {
        let copy_bytes = vec![];
        let default_copy = CopyInstruction::from_bytes(&mut copy_bytes.iter());
        assert!(default_copy.is_err());
        assert_eq!(
            default_copy.unwrap_err(),
            CopyInstructionError::InvalidSignByte(None)
        );
    }

    #[test]
    fn from_bytes_length_err() {
        let add_bytes = vec![CopyInstruction::INSTRUCTION_BYTE_SIGN];
        let default_add = CopyInstruction::from_bytes(&mut add_bytes.iter());
        assert!(default_add.is_err());
        assert_eq!(
            default_add.unwrap_err(),
            CopyInstructionError::InvalidLengthBytes(None)
        );
    }
}
