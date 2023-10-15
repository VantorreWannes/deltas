use crate::instruction_error::{InstructionError, Result};

pub const MAX_INSTRUCTION_LENGTH: u8 = u8::MAX;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Remove { length: u8 },
    Add { content: Vec<u8> },
    Copy { content: Vec<u8> },
}

impl Instruction {
    pub fn len(&self) -> u8 {
        match self {
            Instruction::Remove { length } => *length,
            Instruction::Add { content } | Instruction::Copy { content } => content.len() as u8,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == u8::MIN
    }

    pub fn is_full(&self) -> bool {
        self.len() == MAX_INSTRUCTION_LENGTH
    }

    pub fn push(&mut self, byte: u8) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        match self {
            Instruction::Remove { length } => *length += 1,
            Instruction::Add { content } | Instruction::Copy { content } => content.push(byte),
        }
        Ok(())
    }

    fn sign(&self) -> u8 {
        match self {
            Instruction::Remove { .. } => REMOVE_INSTRUCTION_SIGN,
            Instruction::Add { .. } => ADD_INSTRUCTION_SIGN,
            Instruction::Copy { .. } => COPY_INSTRUCTION_SIGN,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::Remove { length } => {
                vec![REMOVE_INSTRUCTION_SIGN, *length]
            },
            Instruction::Add { content } | Instruction::Copy { content }  => {
                let mut bytes = vec![self.sign(), content.len() as u8];
                bytes.extend(content);
                bytes
            },
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
