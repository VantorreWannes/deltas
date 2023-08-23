use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    MaxLengthReached,
    ConvertBetweenBytesError(InstructionConvertBetweenBytesError),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::MaxLengthReached => write!(f, "Reached the max content length for this instruction type. Cannot fill it any further."),
            InstructionError::ConvertBetweenBytesError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for InstructionError {}

#[derive(Debug, PartialEq)]
pub enum InstructionConvertBetweenBytesError {
    NoSignByteFound,
    InvalidSign,
    IncorrectLengthByteAmount,
    IncorrrectContentLength,
}

impl Display for InstructionConvertBetweenBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionConvertBetweenBytesError::NoSignByteFound => write!(f, "The bytes iterator is empty."),
            InstructionConvertBetweenBytesError::InvalidSign => write!(f, "The first byte of the iterator was not a valid Instruction identifier byte."),
            InstructionConvertBetweenBytesError::IncorrectLengthByteAmount => write!(f, "There weren't enough correct length indicator bytes in the iterator to construct a valid InstructionLength value."),
            InstructionConvertBetweenBytesError::IncorrrectContentLength => write!(f, "The decoded InstructionLength and content length did not match."),
        }
    }
}

impl Error for InstructionConvertBetweenBytesError {}

impl From<InstructionConvertBetweenBytesError> for InstructionError {
    fn from(value: InstructionConvertBetweenBytesError) -> Self {
        InstructionError::ConvertBetweenBytesError(value)
    }
}

#[cfg(test)]
mod instruction_error_tests {
    use super::*;

    #[test]
    fn into_instruction_bytes_eror() {
        assert_eq!(
            <InstructionError>::from(InstructionConvertBetweenBytesError::IncorrectLengthByteAmount),
            InstructionError::ConvertBetweenBytesError(
                InstructionConvertBetweenBytesError::IncorrectLengthByteAmount
            )
        );

    }
}
