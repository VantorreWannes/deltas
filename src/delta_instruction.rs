use std::slice::Iter;

use crate::{delta_instruction_error::ApplyToError, delta_instruction_traits::ApplyDeltaTo};

use super::{
    delta_instruction_error::{InstructionError, InstructionFromBytesError},
    delta_instruction_traits::ConvertBetweenBytes,
};

pub const ADD_INSTRUCTION_SIGN: u8 = b'+';
pub const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
pub const COPY_INSTRUCTION_SIGN: u8 = b'|';

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add { content: Vec<u8> },
    Remove { length: u8 },
    Copy { length: u8 },
}

impl Instruction {
    pub fn len(&self) -> u8 {
        match self {
            Instruction::Add { content } => content
                .len()
                .try_into()
                .expect("Content should be shorter then u8::MAX"),
            Instruction::Remove { length } => *length,
            Instruction::Copy { length } => *length,
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
            Instruction::Add { content } => {
                content.push(*byte);
                Ok(())
            }
            Instruction::Remove { length } | Instruction::Copy { length } => {
                *length += 1u8;
                Ok(())
            }
        }
    }
}

impl ConvertBetweenBytes for Instruction {
    type Error = InstructionFromBytesError;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(<usize>::from(self.len()) + 2);
        match self {
            Instruction::Add { content } => {
                bytes.push(ADD_INSTRUCTION_SIGN);
                bytes.push(self.len());
                bytes.extend(content);
                bytes
            }
            Instruction::Remove { length: _ } => {
                bytes.push(REMOVE_INSTRUCTION_SIGN);
                bytes.push(self.len());
                bytes
            }
            Instruction::Copy { length: _ } => {
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
                Ok(Instruction::Add { content })
            }
            Some(&REMOVE_INSTRUCTION_SIGN) => {
                let length = *bytes
                    .next()
                    .ok_or(InstructionFromBytesError::NoLengthFound)?;
                Ok(Instruction::Remove { length })
            }
            Some(&COPY_INSTRUCTION_SIGN) => {
                let length = *bytes
                    .next()
                    .ok_or(InstructionFromBytesError::NoLengthFound)?;
                Ok(Instruction::Copy { length })
            }
            Some(_) => Err(InstructionFromBytesError::InvalidSign),
            None => Err(InstructionFromBytesError::NoSignFound),
        }
    }
}

impl From<&Instruction> for Vec<u8> {
    fn from(value: &Instruction) -> Self {
        value.to_bytes()
    }
}

impl From<Instruction> for Vec<u8> {
    fn from(value: Instruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Vec<u8>> for Instruction {
    type Error = InstructionFromBytesError;

    fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
        Instruction::try_from_bytes(&mut value.iter())
    }
}

impl TryFrom<Vec<u8>> for Instruction {
    type Error = InstructionFromBytesError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Instruction::try_from_bytes(&mut value.iter())
    }
}

impl ApplyDeltaTo for Instruction {
    type Error = ApplyToError;

    fn apply_to(&self, source: &mut Iter<u8>) -> Result<Vec<u8>, Self::Error> {
        match self {
            Instruction::Add { content } => Ok(content.clone()),
            Instruction::Remove { length } => {
                let length = <usize>::from(*length);
                let content: Vec<u8> = source.take(length).copied().collect();
                if content.len() != length {
                    return Err(ApplyToError::InvalidSourceLength);
                }
                Ok(vec![])
            }
            Instruction::Copy { length } => {
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
        delta_instruction_error::InstructionFromBytesError,
        delta_instruction_traits::{ConvertBetweenBytes, ApplyDeltaTo},
    };

    use super::Instruction;

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
        let add_instruction = Instruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        let remove_instruction = Instruction::Remove {
            length: u8::MAX.into(),
        };
        let copy_instruction = Instruction::Copy {
            length: u8::MAX.into(),
        };
        assert!(add_instruction.is_full());
        assert!(remove_instruction.is_full());
        assert!(copy_instruction.is_full());
    }

    #[test]
    fn to_bytes() {
        let add_instruction = Instruction::Add {
            content: vec![0; u8::MAX.into()],
        };
        let remove_instruction = Instruction::Remove {
            length: u8::MAX.into(),
        };
        let copy_instruction = Instruction::Copy {
            length: u8::MAX.into(),
        };
        let mut add_instruction_bytes = vec![ADD_INSTRUCTION_SIGN, u8::MAX];
        add_instruction_bytes.append(&mut vec![0; u8::MAX.into()]);
        let remove_instruction_bytes = vec![REMOVE_INSTRUCTION_SIGN, u8::MAX];
        let copy_instruction_bytes = vec![COPY_INSTRUCTION_SIGN, u8::MAX];

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

    #[test]
    fn apply_to() {
        let add_instruction = Instruction::Add { content: vec![b'A'; u8::MAX.into()] };
        let add_applied_bytes = add_instruction.apply_to(&mut vec![].iter());

        let remove_instruction = Instruction::Remove { length: u8::MAX };
        let remove_applied_bytes = remove_instruction.apply_to(&mut vec![b'A'; u8::MAX.into()].iter());
        
        let copy_instruction = Instruction::Copy { length: u8::MAX };
        let copy_applied_bytes = copy_instruction.apply_to(&mut vec![b'A'; u8::MAX.into()].iter());

        assert_eq!(add_applied_bytes.unwrap(), vec![b'A'; u8::MAX.into()]);
        assert_eq!(remove_applied_bytes.unwrap(), vec![]);
        assert_eq!(copy_applied_bytes.unwrap(), vec![b'A'; u8::MAX.into()]);
    }
}
