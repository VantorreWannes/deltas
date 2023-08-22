use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::{
    add_instruction::{AddInstruction, AddInstructionlength},
    copy_instruction::{CopyInstruction, CopyInstructionlength},
    remove_instruction::{RemoveInstruction, RemoveInstructionlength},
    traits::InstructionBytes,
};

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    MaxLengthReached(usize),
    InvalidSignByte(Option<u8>, u8),
    InvalidLengthBytes(Option<usize>, usize),
    MissingByteContent(usize, usize),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InstructionError::MaxLengthReached(max_content_length) => write!(f, "Can't fill this delta instruction type past {max_content_length} values. Try creating a new instruction instead."),
            InstructionError::InvalidSignByte(found_sign, expected_sign) => write!(f, "Expected this instruction sign: {expected_sign}. But found this instead: {}.", match found_sign {
                Some(sign) => format!("{}", sign),
                None => "nothing".to_string(),
            }),
            InstructionError::InvalidLengthBytes(found_number_length, expected_number_length) => write!(f, "Couldn't construct the instruction content length indicator with {} bytes. (Needed {expected_number_length}.)", match found_number_length {
                Some(length) => format!("{}", length),
                None => "no".to_string(),
            }),
            InstructionError::MissingByteContent(found_content_length, expected_content_length) => write!(f, "Expected an instruction content length of {expected_content_length} but found it was {found_content_length} long instead."),
        }
    }
}

impl Error for InstructionError {}

#[derive(Debug, PartialEq)]
pub enum AddInstructionError {
    MaxLengthReached,
    InvalidSignByte(Option<u8>),
    InvalidLengthBytes(Option<usize>),
    MissingByteContent(usize, AddInstructionlength),
}

impl Display for AddInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AddInstructionError::MaxLengthReached => write!(f, "Can't fill this delta instruction type past {} values. Try creating a new instruction instead.", AddInstructionlength::MAX),
            AddInstructionError::InvalidSignByte(found_sign) => write!(f, "Expected this instruction sign: b'+'. But found something this instead: {}.", match found_sign {
                Some(sign) => format!("{}", sign),
                None => "nothing".to_string(),
            }),
            AddInstructionError::InvalidLengthBytes(found_number_length) => write!(f, "Couldn't construct the instruction content length indicator with {} bytes. (Needed {}.)",match found_number_length {
                Some(length) => format!("{}", length),
                None => "no".to_string(),
            }, std::mem::size_of::<AddInstructionlength>()),
            AddInstructionError::MissingByteContent(found_content_length, expected_content_length) => write!(f, "Expected an instruction content length of {expected_content_length} but found it was {found_content_length} long instead."),
        }
    }
}

impl Error for AddInstructionError {}

impl From<AddInstructionError> for InstructionError {
    fn from(value: AddInstructionError) -> Self {
        match value {
            AddInstructionError::MaxLengthReached => {
                InstructionError::MaxLengthReached(AddInstructionlength::MAX.try_into().unwrap())
            }
            AddInstructionError::InvalidSignByte(found_sign) => {
                InstructionError::InvalidSignByte(found_sign, AddInstruction::INSTRUCTION_BYTE_SIGN)
            }
            AddInstructionError::InvalidLengthBytes(found_number_length) => {
                InstructionError::InvalidLengthBytes(
                    found_number_length,
                    std::mem::size_of::<AddInstructionlength>(),
                )
            }
            AddInstructionError::MissingByteContent(
                found_content_length,
                expected_content_length,
            ) => InstructionError::MissingByteContent(
                found_content_length,
                expected_content_length.try_into().unwrap(),
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveInstructionError {
    MaxLengthReached,
    InvalidSignByte(Option<u8>),
    InvalidLengthBytes(Option<usize>),
}

impl Display for RemoveInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RemoveInstructionError::MaxLengthReached => write!(f, "Can't fill this delta instruction type past {} values. Try creating a new instruction instead.", RemoveInstructionlength::MAX),
            RemoveInstructionError::InvalidSignByte(found_sign) => write!(f, "Expected this instruction sign: {}. But found something this instead: {}.", b'-', match found_sign {
                Some(sign) => format!("{}", sign),
                None => "nothing".to_string(),
            }),
            RemoveInstructionError::InvalidLengthBytes(found_number_length) => write!(f, "Couldn't construct the instruction content length indicator with {} bytes. (Needed {}.)",match found_number_length {
                Some(length) => format!("{}", length),
                None => "no".to_string(),
            }, std::mem::size_of::<RemoveInstructionlength>()),
        }
    }
}

impl Error for RemoveInstructionError {}

impl From<RemoveInstructionError> for InstructionError {
    fn from(value: RemoveInstructionError) -> Self {
        match value {
            RemoveInstructionError::MaxLengthReached => {
                InstructionError::MaxLengthReached(RemoveInstructionlength::MAX.try_into().unwrap())
            }
            RemoveInstructionError::InvalidSignByte(found_sign) => {
                InstructionError::InvalidSignByte(
                    found_sign,
                    RemoveInstruction::INSTRUCTION_BYTE_SIGN,
                )
            }
            RemoveInstructionError::InvalidLengthBytes(found_number_length) => {
                InstructionError::InvalidLengthBytes(
                    found_number_length,
                    std::mem::size_of::<RemoveInstructionlength>(),
                )
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CopyInstructionError {
    MaxLengthReached,
    InvalidSignByte(Option<u8>),
    InvalidLengthBytes(Option<usize>),
}

impl Display for CopyInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CopyInstructionError::MaxLengthReached => write!(f, "Can't fill this delta instruction type past {} values. Try creating a new instruction instead.", CopyInstructionlength::MAX),
            CopyInstructionError::InvalidSignByte(found_sign) => write!(f, "Expected this instruction sign: {}. But found something this instead: {}.", b'-', match found_sign {
                Some(sign) => format!("{}", sign),
                None => "nothing".to_string(),
            }),
            CopyInstructionError::InvalidLengthBytes(found_number_length) => write!(f, "Couldn't construct the instruction content length indicator with {} bytes. (Needed {}.)",match found_number_length {
                Some(length) => format!("{}", length),
                None => "no".to_string(),
            }, std::mem::size_of::<RemoveInstructionlength>()),
        }
    }
}

impl Error for CopyInstructionError {}

impl From<CopyInstructionError> for InstructionError {
    fn from(value: CopyInstructionError) -> Self {
        match value {
            CopyInstructionError::MaxLengthReached => {
                InstructionError::MaxLengthReached(CopyInstructionlength::MAX.try_into().unwrap())
            }
            CopyInstructionError::InvalidSignByte(found_sign) => InstructionError::InvalidSignByte(
                found_sign,
                CopyInstruction::INSTRUCTION_BYTE_SIGN,
            ),
            CopyInstructionError::InvalidLengthBytes(found_number_length) => {
                InstructionError::InvalidLengthBytes(
                    found_number_length,
                    std::mem::size_of::<CopyInstructionlength>(),
                )
            }
        }
    }
}

#[cfg(test)]
mod instruction_error_tests {
    use super::*;

    #[test]
    fn from_add_error() {
        assert_eq!(
            InstructionError::from(AddInstructionError::MaxLengthReached),
            InstructionError::MaxLengthReached(AddInstructionlength::MAX.try_into().unwrap())
        );
        assert_eq!(
            InstructionError::from(AddInstructionError::InvalidSignByte(Some(b'A'))),
            InstructionError::InvalidSignByte(Some(b'A'), AddInstruction::INSTRUCTION_BYTE_SIGN)
        );
        assert_eq!(
            InstructionError::from(AddInstructionError::InvalidLengthBytes(None)),
            InstructionError::InvalidLengthBytes(None, std::mem::size_of::<AddInstructionlength>())
        );
        assert_eq!(
            InstructionError::from(AddInstructionError::MissingByteContent(0, 1)),
            InstructionError::MissingByteContent(0, 1)
        );
    }

    #[test]
    fn from_remove_error() {
        assert_eq!(
            InstructionError::from(RemoveInstructionError::MaxLengthReached),
            InstructionError::MaxLengthReached(RemoveInstructionlength::MAX.try_into().unwrap())
        );
        assert_eq!(
            InstructionError::from(RemoveInstructionError::InvalidSignByte(Some(b'A'))),
            InstructionError::InvalidSignByte(Some(b'A'), RemoveInstruction::INSTRUCTION_BYTE_SIGN)
        );
        assert_eq!(
            InstructionError::from(RemoveInstructionError::InvalidLengthBytes(None)),
            InstructionError::InvalidLengthBytes(
                None,
                std::mem::size_of::<RemoveInstructionlength>()
            )
        );
    }

    #[test]
    fn from_copy_error() {
        assert_eq!(
            InstructionError::from(CopyInstructionError::MaxLengthReached),
            InstructionError::MaxLengthReached(CopyInstructionlength::MAX.try_into().unwrap())
        );
        assert_eq!(
            InstructionError::from(CopyInstructionError::InvalidSignByte(Some(b'A'))),
            InstructionError::InvalidSignByte(Some(b'A'), CopyInstruction::INSTRUCTION_BYTE_SIGN)
        );
        assert_eq!(
            InstructionError::from(CopyInstructionError::InvalidLengthBytes(None)),
            InstructionError::InvalidLengthBytes(
                None,
                std::mem::size_of::<CopyInstructionlength>()
            )
        );
    }
}
