use super::errors::InstructionError;
use std::slice::Iter;

pub trait PushToInstruction {
    type Error: Into<InstructionError>;

    fn push(&mut self, byte: u8) -> Result<(), Self::Error>;
}

pub trait InstructionBytes {
    const INSTRUCTION_BYTE_SIGN: u8;
    const NUMBER_BYTES_LENGTH: usize;
    type Error: Into<InstructionError>;

    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &mut Iter<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
