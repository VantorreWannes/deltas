use std::{iter::Peekable, slice::Iter};

use super::{InstructionItem, InstructionLength, traits::{InstructionInfo, InstructionContent, InstructionBytes}, error::InstructionError, Result, COPY_INSTRUCTION_SIGN};


#[derive(Debug, PartialEq, Clone)]
pub struct CopyInstruction {
    content: Vec<InstructionItem>,
}

impl CopyInstruction {
    pub fn new(content: Vec<InstructionItem>) -> Self {
        assert!(
            content.len() <= InstructionLength::MAX.try_into().unwrap(),
            "Instruction content exceeded {} items",
            InstructionLength::MAX
        );
        Self { content }
    }

}

impl InstructionInfo for CopyInstruction {
    fn len(&self) -> InstructionLength {
        self.content.len().try_into().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.len() == InstructionLength::MIN
    }

    fn is_full(&self) -> bool {
        self.len() == InstructionLength::MAX
    }
}

impl InstructionContent for CopyInstruction {
    fn push(&mut self, content: InstructionItem) -> Result<()> {
        if self.is_full() {
            return Err(InstructionError::ContentOverflow);
        }
        self.content.push(content);
        Ok(())
    }
}

impl InstructionBytes for CopyInstruction {
    fn byte_sign() -> u8 {
        COPY_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
            bytes.push(CopyInstruction::byte_sign());
            bytes.extend(self.len().to_be_bytes());
            bytes.extend(self.content.iter());
            bytes
    }

    fn try_from_bytes(bytes: &mut Peekable<Iter<'_, u8>>) -> Result<Self> {
        match bytes.next() {
            Some(&COPY_INSTRUCTION_SIGN) => (),
            Some(_) => return Err(InstructionError::InvalidSign),
            None => return Err(InstructionError::MissignSign),
        };

        if bytes.peek().is_none() {
            return Err(InstructionError::MissingLength);
        }

        let length_bytes: Vec<u8> = bytes
            .take(std::mem::size_of::<InstructionLength>())
            .copied()
            .collect();
        let length = InstructionLength::from_be_bytes(length_bytes.as_slice().try_into().map_err(|_| InstructionError::InvalidLength)?);

        let content_bytes: Vec<u8> = bytes
            .take(
                length.try_into().unwrap()
            )
            .copied()
            .collect();

        let content: Result<Vec<InstructionItem>> = content_bytes
            .chunks(std::mem::size_of::<InstructionItem>())
            .map(|chunk: &[u8]| -> Result<InstructionItem> {
                Ok(InstructionItem::from_be_bytes(
                    chunk
                        .try_into()
                        .map_err(|_| InstructionError::InvalidContent)?,
                ))
            })
            .collect();

        let content = content?;

        if content.len() < length as usize {
            return Err(InstructionError::MissingContent);
        }

        Ok(Self { content })
    }
}


impl Default for CopyInstruction {

    fn default() -> Self {
        Self { content: vec![InstructionItem::default(); InstructionLength::MIN.try_into().unwrap()] }
    }
}

