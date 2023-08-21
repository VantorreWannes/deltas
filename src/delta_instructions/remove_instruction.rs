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

    pub fn len(&self) -> u8 {
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
