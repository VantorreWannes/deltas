use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    MaxLengthReached,
    InstructionFromBytesError(InstructionFromBytesError),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::MaxLengthReached => write!(
                f,
                "DeltaInstructions can't be filled up past a length of 255. (u8::MAX)"
            ),
            InstructionError::InstructionFromBytesError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for InstructionError {}

#[derive(Debug, PartialEq)]
pub enum InstructionFromBytesError {
    NoSignFound,
    InvalidSign,
    NoLengthFound,
    InvalidLength,
}

impl Display for InstructionFromBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionFromBytesError::NoSignFound => write!(f, "Iterator is empty. No bytes left to represent a DeltaInstruction sign."),
            InstructionFromBytesError::InvalidSign => write!(f, "Couldn't recognise the current byte as a valid DeltaInstruction sign."),
            InstructionFromBytesError::NoLengthFound => write!(f, "Iterator is empty. No bytes left to represent a DeltaInstruction length value."),
            InstructionFromBytesError::InvalidLength => write!(f, "DeltaInstruction's length value did not match with the remaining length of the iterator."),
        }
    }
}

impl Error for InstructionFromBytesError {}

impl From<InstructionFromBytesError> for InstructionError {
    fn from(value: InstructionFromBytesError) -> Self {
        InstructionError::InstructionFromBytesError(value)
    }
}


#[derive(Debug, PartialEq)]
pub enum ApplyToError {
    InvalidSourceLength,
}

impl Display for ApplyToError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyToError::InvalidSourceLength => write!(f, "Source Iterator ended before DeltaInstruction could extract the needed bytes."),
        }
    }
}

impl Error for ApplyToError {}

#[cfg(test)]
mod instruction_error_tests {}

