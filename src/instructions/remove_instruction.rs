use super::{InstructionLength, traits::{InstructionInfo, InstructionContent, InstructionBytes}, InstructionItem, error::InstructionError, Result, REMOVE_INSTRUCTION_SIGN};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RemoveInstruction {
    length: InstructionLength,
}

impl RemoveInstruction {
    pub fn new(length: InstructionLength) -> Self {
        Self { length }
    }
}


impl InstructionInfo for RemoveInstruction {
    fn len(&self) -> InstructionLength {
        self.length.try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }
}

impl InstructionContent for RemoveInstruction {
    fn push(&mut self, _: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.length += 1;
        Ok(())
    }
}

impl InstructionBytes for RemoveInstruction {

    fn byte_sign() -> u8 {
        REMOVE_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(RemoveInstruction::byte_sign());
        bytes.extend(self.len().to_be_bytes());
        bytes
    }

    fn try_from_bytes(bytes: &mut  std::iter::Peekable<std::slice::Iter<'_, u8>>) -> Result<Self> where Self: Sized {
        match bytes.next() {
            Some(&REMOVE_INSTRUCTION_SIGN) => (),
            Some(_) => return Err(InstructionError::InvalidSign),
            None => return Err(InstructionError::MissignSign),
        };
        let length: InstructionLength = {
            let length_bytes: Vec<u8> = bytes
                .take(std::mem::size_of::<InstructionLength>())
                .copied()
                .collect();
            InstructionLength::from_be_bytes(length_bytes.as_slice().try_into().map_err(|_| InstructionError::MissingLength)?)
        };
        Ok(Self { length })
    }
}


#[cfg(test)]
mod remove_instruction_tests {
    use super::*;

    #[test]
    fn instruction_info() {
        todo!();
    }

    #[test]
    fn instruction_content() {
        todo!();
    }

    #[test]
    fn instruction_bytes_try_from_bytes_ok() {
       todo!();
    }

    #[test]
    fn instruction_bytes_try_from_bytes_err() {
        todo!();
    }
}


