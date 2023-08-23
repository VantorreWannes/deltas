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

    pub fn is_empty(&self) -> bool {
        match self {
            DeltaInstruction::Add { content } => content.is_empty(),
            DeltaInstruction::Remove { length } | DeltaInstruction::Copy { length } => *length == 0,
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

#[cfg(test)]
mod delta_instruction_tests {
    use crate::{
        delta_instruction_error::{InstructionConvertBetweenBytesError, InstructionError},
        delta_instruction_traits::ConvertBetweenBytes,
    };

    use super::{
        AddInstructionLength, CopyInstructionLength, DeltaInstruction, RemoveInstructionLength,
        ADD_INSTRUCTION_LENGTH_BYTE_LENGTH, ADD_INSTRUCTION_SIGN,
        COPY_INSTRUCTION_LENGTH_BYTE_LENGTH, COPY_INSTRUCTION_SIGN,
        REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH, REMOVE_INSTRUCTION_SIGN,
    };

    type Instruction = DeltaInstruction;

    #[test]
    fn len() {
        let mut min_instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MIN.try_into().unwrap()],
        };
        let mut max_instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MAX.try_into().unwrap()],
        };
        assert_eq!(
            min_instruction.len(),
            AddInstructionLength::MIN.try_into().unwrap()
        );
        assert_eq!(
            max_instruction.len(),
            AddInstructionLength::MAX.try_into().unwrap()
        );

        min_instruction = Instruction::Remove {
            length: RemoveInstructionLength::MIN.try_into().unwrap(),
        };
        max_instruction = Instruction::Remove {
            length: RemoveInstructionLength::MAX.try_into().unwrap(),
        };
        assert_eq!(
            min_instruction.len(),
            RemoveInstructionLength::MIN.try_into().unwrap()
        );
        assert_eq!(
            max_instruction.len(),
            RemoveInstructionLength::MAX.try_into().unwrap()
        );
        assert_eq!(
            min_instruction.len(),
            CopyInstructionLength::MIN.try_into().unwrap()
        );
        assert_eq!(
            max_instruction.len(),
            CopyInstructionLength::MAX.try_into().unwrap()
        );
    }

    #[test]
    fn push() {
        let mut instruction = Instruction::Add {
            content: vec![0; <usize>::try_from(AddInstructionLength::MAX).unwrap() - 1],
        };
        assert!(instruction.push(&b'A').is_ok());
        assert_eq!(
            instruction.push(&b'A').unwrap_err(),
            InstructionError::MaxLengthReached
        );

        instruction = Instruction::Remove {
            length: RemoveInstructionLength::MAX - 1,
        };
        assert!(instruction.push(&b'A').is_ok());
        assert_eq!(
            instruction.push(&b'A').unwrap_err(),
            InstructionError::MaxLengthReached
        );

        instruction = Instruction::Copy {
            length: CopyInstructionLength::MAX - 1,
        };
        assert!(instruction.push(&b'A').is_ok());
        assert_eq!(
            instruction.push(&b'A').unwrap_err(),
            InstructionError::MaxLengthReached
        );
    }

    #[test]
    fn add_instruction_to_bytes() {
        let instruction_length = AddInstructionLength::MAX;
        let instruction_content_bytes = vec![0; instruction_length.try_into().unwrap()];
        let instruction_length_bytes = AddInstructionLength::to_be_bytes(instruction_length);
        let mut instruction_bytes = Vec::with_capacity(
            <usize>::try_from(instruction_length).unwrap() + ADD_INSTRUCTION_LENGTH_BYTE_LENGTH + 1,
        );
        instruction_bytes.push(ADD_INSTRUCTION_SIGN);
        instruction_bytes.extend(instruction_length_bytes);
        instruction_bytes.extend(instruction_content_bytes.iter());
        let instruction = Instruction::Add {
            content: instruction_content_bytes,
        };
        assert_eq!(instruction.to_bytes(), instruction_bytes);
    }

    #[test]
    fn remove_instruction_to_bytes() {
        let instruction_length = RemoveInstructionLength::MAX;
        let instruction_length_bytes =
            RemoveInstructionLength::to_be_bytes(instruction_length.clone());
        let mut instruction_bytes = Vec::with_capacity(REMOVE_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
        instruction_bytes.push(REMOVE_INSTRUCTION_SIGN);
        instruction_bytes.extend(instruction_length_bytes);
        let instruction = Instruction::Remove {
            length: instruction_length,
        };
        assert_eq!(instruction.to_bytes(), instruction_bytes);
    }

    #[test]
    fn copy_instruction_to_bytes() {
        let instruction_length = CopyInstructionLength::MAX;
        let instruction_length_bytes =
            CopyInstructionLength::to_be_bytes(instruction_length.clone());
        let mut instruction_bytes = Vec::with_capacity(COPY_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
        instruction_bytes.push(COPY_INSTRUCTION_SIGN);
        instruction_bytes.extend(instruction_length_bytes);
        let instruction = Instruction::Copy {
            length: instruction_length,
        };
        assert_eq!(instruction.to_bytes(), instruction_bytes);
    }

    #[test]
    fn add_instruction_from_bytes_ok() {
        let instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MAX.try_into().unwrap()],
        };
        let instruction_bytes = instruction.to_bytes();
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert!(from_bytes_instruction.is_ok());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn add_instruction_from_bytes_incorrect_length_byte_amount_err() {
        let instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MAX.try_into().unwrap()],
        };
        let mut instruction_bytes = instruction.to_bytes();
        instruction_bytes.drain(1..ADD_INSTRUCTION_LENGTH_BYTE_LENGTH + 1);
        instruction_bytes.insert(1, 0);
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert!(from_bytes_instruction.is_ok());
        assert_eq!(
            from_bytes_instruction.unwrap(),
            Instruction::Add { content: vec![] }
        );
    }

    #[test]
    fn add_instruction_from_bytes_incorrect_content_length_err() {
        let instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MAX.try_into().unwrap()],
        };
        let mut instruction_bytes = instruction.to_bytes();
        instruction_bytes.pop();
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert_eq!(
            from_bytes_instruction.unwrap_err(),
            InstructionConvertBetweenBytesError::IncorrrectContentLength,
        );
    }

    #[test]
    fn remove_instruction_from_bytes_ok() {
        let instruction = Instruction::Remove {
            length: RemoveInstructionLength::MAX.try_into().unwrap(),
        };
        let instruction_bytes = instruction.to_bytes();
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert!(from_bytes_instruction.is_ok());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn remove_instruction_from_bytes_incorrect_length_byte_amount_err() {
        let instruction_bytes = vec![REMOVE_INSTRUCTION_SIGN];
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert_eq!(
            from_bytes_instruction.unwrap_err(),
            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount,
        );
    }

    #[test]
    fn copy_instruction_from_bytes_ok() {
        let instruction = Instruction::Copy {
            length: CopyInstructionLength::MAX.try_into().unwrap(),
        };
        let instruction_bytes = instruction.to_bytes();
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        dbg!(&from_bytes_instruction);
        assert!(from_bytes_instruction.is_ok());
        assert_eq!(from_bytes_instruction.unwrap(), instruction);
    }

    #[test]
    fn copy_instruction_from_bytes_incorrect_length_byte_amount_err() {
        let instruction_bytes = vec![COPY_INSTRUCTION_SIGN];
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert_eq!(
            from_bytes_instruction.unwrap_err(),
            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount,
        );
    }

    #[test]
    fn instruction_from_bytes_no_sign_err() {
        let from_bytes_instruction = Instruction::try_from_bytes(&mut vec![].iter());
        assert_eq!(
            from_bytes_instruction.unwrap_err(),
            InstructionConvertBetweenBytesError::NoSignByteFound
        );
    }

    #[test]
    fn instruction_from_bytes_invalid_sign_err() {
        let instruction = Instruction::Add {
            content: vec![0; AddInstructionLength::MAX.try_into().unwrap()],
        };
        let mut instruction_bytes = instruction.to_bytes();
        instruction_bytes.remove(0);
        instruction_bytes.insert(0, 0);
        let from_bytes_instruction = Instruction::try_from_bytes(&mut instruction_bytes.iter());
        assert_eq!(
            from_bytes_instruction.unwrap_err(),
            InstructionConvertBetweenBytesError::InvalidSign,
        );
    }
}
