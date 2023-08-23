use std::{mem, slice::Iter};

use super::{
    delta_instruction_error::{InstructionConvertBetweenBytesError, InstructionError},
    delta_instruction_traits::ConvertBetweenBytes,
};

pub type AddInstructionLength = u16;
pub type RemoveInstructionLength = u16;
pub type CopyInstructionLength = u16;

pub const ADD_INSTRUCTION_SIGN: u8 = b'+';
pub const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
pub const COPY_INSTRUCTION_SIGN: u8 = b'|';

pub const ADD_INSTRUCTION_LENGTH_BYTE_LENGTH: usize = mem::size_of::<AddInstructionLength>();
pub const REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH: usize = mem::size_of::<RemoveInstructionLength>();
pub const COPY_INSTRUCTION_LENGTH_BYTE_LENGTH: usize = mem::size_of::<CopyInstructionLength>();

#[derive(Debug, PartialEq)]
pub enum DeltaInstruction {
    Add { content: Vec<u8> },
    Remove { length: RemoveInstructionLength },
    Copy { length: CopyInstructionLength },
}

impl DeltaInstruction {
    pub fn len(&self) -> usize {
        match self {
            DeltaInstruction::Add { content } => content.len(),
            DeltaInstruction::Remove { length } => <usize>::try_from(*length).unwrap(),
            DeltaInstruction::Copy { length } => <usize>::try_from(*length).unwrap(),
        }
    }

    pub fn push(&mut self, byte: &u8) -> Result<(), InstructionError> {
        match self {
            DeltaInstruction::Add { content } => {
                if content.len() >= AddInstructionLength::MAX.try_into().unwrap() {
                    return Err(InstructionError::MaxLengthReached);
                }
                content.push(*byte);
                Ok(())
            }
            DeltaInstruction::Remove { length } | DeltaInstruction::Copy { length } => {
                *length = length
                    .checked_add(1)
                    .ok_or(InstructionError::MaxLengthReached)?;
                Ok(())
            }
        }
    }
}

impl ConvertBetweenBytes for DeltaInstruction {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            DeltaInstruction::Add { content } => {
                let length = content.len();
                let mut bytes: Vec<u8> =
                    Vec::with_capacity(length + ADD_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
                bytes.push(ADD_INSTRUCTION_SIGN);
                bytes.extend(
                    AddInstructionLength::try_from(length)
                        .expect(
                            "length is constructed to be shorter then AddInstructionLength::MAX",
                        )
                        .to_be_bytes(),
                );
                bytes.extend(content);
                bytes
            }
            DeltaInstruction::Remove { length } => {
                let mut bytes: Vec<u8> =
                    Vec::with_capacity(REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
                bytes.push(REMOVE_INSTRUCTION_SIGN);
                bytes.extend(length.to_be_bytes());
                bytes
            }
            DeltaInstruction::Copy { length } => {
                let mut bytes: Vec<u8> =
                    Vec::with_capacity(COPY_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
                bytes.push(COPY_INSTRUCTION_SIGN);
                bytes.extend(length.to_be_bytes());
                bytes
            }
        }
    }

    fn try_from_bytes(bytes: &mut Iter<u8>) -> Result<Self, InstructionConvertBetweenBytesError>
    where
        Self: Sized,
    {
        match bytes.next() {
            Some(&ADD_INSTRUCTION_SIGN) => {
                let length_bytes_vec = bytes
                    .take(ADD_INSTRUCTION_LENGTH_BYTE_LENGTH)
                    .copied()
                    .collect::<Vec<u8>>();
                let length_bytes =
                    <[u8; ADD_INSTRUCTION_LENGTH_BYTE_LENGTH]>::try_from(length_bytes_vec)
                        .map_err(|_| {
                            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount
                        })?;
                let length = <AddInstructionLength>::from_be_bytes(length_bytes);
                let content = bytes
                    .take(length.try_into().unwrap())
                    .copied()
                    .collect::<Vec<u8>>();
                if content.len() != length.try_into().unwrap() {
                    return Err(InstructionConvertBetweenBytesError::IncorrrectContentLength);
                }
                Ok(DeltaInstruction::Add { content })
            }
            Some(&REMOVE_INSTRUCTION_SIGN) => {
                let length_bytes_vec = bytes
                    .take(REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH)
                    .copied()
                    .collect::<Vec<u8>>();
                let length_bytes =
                    <[u8; REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH]>::try_from(length_bytes_vec)
                        .map_err(|_| {
                            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount
                        })?;
                let length = <RemoveInstructionLength>::from_be_bytes(length_bytes);
                Ok(DeltaInstruction::Remove { length })
            }
            Some(&COPY_INSTRUCTION_SIGN) => {
                let length_bytes_vec = bytes
                    .take(COPY_INSTRUCTION_LENGTH_BYTE_LENGTH)
                    .copied()
                    .collect::<Vec<u8>>();
                let length_bytes =
                    <[u8; COPY_INSTRUCTION_LENGTH_BYTE_LENGTH]>::try_from(length_bytes_vec)
                        .map_err(|_| {
                            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount
                        })?;
                let length = <CopyInstructionLength>::from_be_bytes(length_bytes);
                Ok(DeltaInstruction::Copy { length })
            }
            Some(_) => Err(InstructionConvertBetweenBytesError::InvalidSign),
            None => Err(InstructionConvertBetweenBytesError::NoSignByteFound),
        }
    }
}

impl From<&DeltaInstruction> for Vec<u8> {
    fn from(value: &DeltaInstruction) -> Self {
        value.to_bytes()
    }
}

impl From<DeltaInstruction> for Vec<u8> {
    fn from(value: DeltaInstruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Vec<u8>> for DeltaInstruction {
    type Error = InstructionConvertBetweenBytesError;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value.iter())
    }
}

impl TryFrom<Vec<u8>> for DeltaInstruction {
    type Error = InstructionConvertBetweenBytesError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value.iter())
    }
}

