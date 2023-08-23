use std::{mem, slice::Iter};

use crate::{delta_instruction_error::ApplyToError, delta_instruction_traits::ApplyDeltaTo};

use super::{
    delta_instruction_error::{InstructionError, InstructionFromBytesError},
    delta_instruction_traits::ConvertBetweenBytes,
};

pub const ADD_INSTRUCTION_SIGN: u8 = b'+';
pub const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
pub const COPY_INSTRUCTION_SIGN: u8 = b'|';

#[derive(Debug, PartialEq)]
pub enum DeltaInstruction {
    Add { content: Vec<u8> },
    Remove { length: u8 },
    Copy { length: u8 },
}

impl DeltaInstruction {
    pub fn len(&self) -> u8 {
        match self {
            DeltaInstruction::Add { content } => content
                .len()
                .try_into()
                .expect("Content should be shorter then u8::MAX"),
            DeltaInstruction::Remove { length } => *length,
            DeltaInstruction::Copy { length } => *length,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == u8::MIN
    }

    pub fn is_full(&self) -> bool {
        self.len() == u8::MAX
    }

    pub fn push(&mut self, byte: &u8) -> Result<(), InstructionError> {
        if self.is_full() {
            return Err(InstructionError::MaxLengthReached);
        }
        match self {
            DeltaInstruction::Add { content } => {
                content.push(*byte);
                Ok(())
            }
            DeltaInstruction::Remove { length } | DeltaInstruction::Copy { length } => {
                *length += 1u8;
                Ok(())
            }
        }
    }
}

impl ConvertBetweenBytes for DeltaInstruction {
    type Error = InstructionFromBytesError;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(<usize>::from(self.len()) + 2);
        match self {
            DeltaInstruction::Add { content } => {
                bytes.push(ADD_INSTRUCTION_SIGN);
                bytes.push(self.len());
                bytes.extend(content);
                bytes
            }
            DeltaInstruction::Remove { length } => {
                bytes.push(REMOVE_INSTRUCTION_SIGN);
                bytes.push(self.len());
                bytes
            }
            DeltaInstruction::Copy { length } => {
                bytes.push(COPY_INSTRUCTION_SIGN);
                bytes.push(self.len());
                bytes
            }
        }
    }

    fn try_from_bytes(bytes: &mut Iter<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        match bytes.next() {
            Some(&ADD_INSTRUCTION_SIGN) => {
                let length = <usize>::from(
                    *bytes
                        .next()
                        .ok_or(InstructionFromBytesError::NoLengthFound)?,
                );
                let content: Vec<u8> = bytes.take(length).copied().collect();
                if content.len() != length {
                    return Err(InstructionFromBytesError::InvalidLength);
                }
                Ok(DeltaInstruction::Add { content })
            }
            Some(&REMOVE_INSTRUCTION_SIGN) => {
                let length = *bytes
                    .next()
                    .ok_or(InstructionFromBytesError::NoLengthFound)?;
                Ok(DeltaInstruction::Remove { length })
            }
            Some(&COPY_INSTRUCTION_SIGN) => {
                let length = *bytes
                    .next()
                    .ok_or(InstructionFromBytesError::NoLengthFound)?;
                Ok(DeltaInstruction::Copy { length })
            }
            Some(_) => Err(InstructionFromBytesError::InvalidSign),
            None => Err(InstructionFromBytesError::NoSignFound),
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
    type Error = InstructionFromBytesError;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value.iter())
    }
}

impl TryFrom<Vec<u8>> for DeltaInstruction {
    type Error = InstructionFromBytesError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value.iter())
    }
}

impl ApplyDeltaTo for DeltaInstruction {
    type Error = ApplyToError;

    fn apply_to(&self, source: &mut Iter<u8>) -> Result<Vec<u8>, Self::Error> {
        match self {
            DeltaInstruction::Add { content } => Ok(content.clone()),
            DeltaInstruction::Remove { length } => {
                let length = <usize>::from(*length);
                let content: Vec<u8> = source.take(length).copied().collect();
                if content.len() != length {
                    return Err(ApplyToError::InvalidSourceLength);
                }
                Ok(vec![])
            }
            DeltaInstruction::Copy { length } => {
                let length = <usize>::from(*length);
                let content: Vec<u8> = source.take(length).copied().collect();
                if content.len() != length {
                    return Err(ApplyToError::InvalidSourceLength);
                }
                Ok(content)
            }
        }
    }
}

#[cfg(test)]
mod delta_instruction_tests {
    use crate::{
        delta_instruction::{ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN, REMOVE_INSTRUCTION_SIGN},
        delta_instruction_error::{InstructionError, InstructionFromBytesError},
        delta_instruction_traits::ConvertBetweenBytes,
    };

    use super::DeltaInstruction;

    type Instruction = DeltaInstruction;

    #[test]
    fn len() {
        let mut max_instruction = Instruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        assert_eq!(max_instruction.len(), u8::MAX.into());

        max_instruction = Instruction::Remove {
            length: u8::MAX.into(),
        };
        assert_eq!(max_instruction.len(), u8::MAX.into());
    }

    #[test]
    fn is_full() {
        let add_instruction = DeltaInstruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        let remove_instruction = DeltaInstruction::Remove {
            length: u8::MAX.into(),
        };
        let copy_instruction = DeltaInstruction::Copy {
            length: u8::MAX.into(),
        };
        assert!(add_instruction.is_full());
        assert!(remove_instruction.is_full());
        assert!(copy_instruction.is_full());
    }

    #[test]
    fn to_bytes() {
        let add_instruction = DeltaInstruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        let remove_instruction = DeltaInstruction::Remove {
            length: u8::MAX.into(),
        };
        let copy_instruction = DeltaInstruction::Copy {
            length: u8::MAX.into(),
        };
        let mut add_instruction_bytes = vec![ADD_INSTRUCTION_SIGN, u8::MAX];
        add_instruction_bytes.append(&mut vec![0; u8::MAX.into()]);
        let mut remove_instruction_bytes = vec![REMOVE_INSTRUCTION_SIGN, u8::MAX];
        let mut copy_instruction_bytes = vec![COPY_INSTRUCTION_SIGN, u8::MAX];

        assert_eq!(add_instruction.to_bytes(), add_instruction_bytes);
        assert_eq!(remove_instruction.to_bytes(), remove_instruction_bytes);
        assert_eq!(copy_instruction.to_bytes(), copy_instruction_bytes);
    }

    #[test]
    fn add_instruction_from_bytes_ok() {
        let instruction = Instruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        let from_bytes_instruction =
            Instruction::try_from_bytes(&mut instruction.to_bytes().iter());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn remove_instruction_from_bytes_ok() {
        let instruction = Instruction::Remove {
            length: u8::MAX.into(),
        };
        let from_bytes_instruction =
            Instruction::try_from_bytes(&mut instruction.to_bytes().iter());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn copy_instruction_from_bytes_ok() {
        let instruction = Instruction::Copy {
            length: u8::MAX.into(),
        };
        let from_bytes_instruction =
            Instruction::try_from_bytes(&mut instruction.to_bytes().iter());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn from_bytes_no_sign_found_err() {
        let from_bytes_instruction = Instruction::try_from_bytes(&mut vec![].iter());
        assert!(from_bytes_instruction.is_err());
        assert_eq!(from_bytes_instruction.unwrap_err(), InstructionFromBytesError::NoSignFound);
    }

    #[test]
    fn from_bytes_invalid_sign_err() {
        let from_bytes_instruction = Instruction::try_from_bytes(&mut vec![b'A'].iter());
        assert!(from_bytes_instruction.is_err());
        assert_eq!(from_bytes_instruction.unwrap_err(), InstructionFromBytesError::InvalidSign);
    }

    #[test]
    fn from_bytes_no_length_found_err() {
        let from_bytes_instruction = Instruction::try_from_bytes(&mut vec![ADD_INSTRUCTION_SIGN].iter());
        assert!(from_bytes_instruction.is_err());
        assert_eq!(from_bytes_instruction.unwrap_err(), InstructionFromBytesError::NoLengthFound);
    }

    #[test]
    fn from_bytes_invalid_length_err() {
        let from_bytes_instruction = Instruction::try_from_bytes(&mut vec![ADD_INSTRUCTION_SIGN, 10, b'A'].iter());
        assert!(from_bytes_instruction.is_err());
        assert_eq!(from_bytes_instruction.unwrap_err(), InstructionFromBytesError::InvalidLength);
    }
}
