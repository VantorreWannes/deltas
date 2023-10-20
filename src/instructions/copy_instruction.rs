use std::{iter::Peekable, slice::Iter};

use super::{
    InstructionBytes, InstructionContent, InstructionError, InstructionInfo, InstructionItem,
    InstructionLength, Result, COPY_INSTRUCTION_SIGN, NON_ZERO_MAX_COUNT_PERCENT,
};

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

    fn non_default_item_count(&self) -> Option<InstructionLength> {
        Some(
            self.content
                .iter()
                .filter(|item| **item != InstructionItem::default())
                .count() as InstructionLength,
        )
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

    fn fill(
        &mut self,
        lcs: &mut super::InstructionItemIter,
        source: &mut super::InstructionItemIter,
        target: &mut super::InstructionItemIter,
    ) {
        while ((lcs.peek().is_some()
            && (source.peek() == lcs.peek() && lcs.peek() == target.peek()))
            || (self.non_default_item_count().unwrap() <= self.threshold()))
            && !self.is_full()
            && (source.peek().is_some() && target.peek().is_some())
        {
            self.push(target.next().unwrap().wrapping_sub(*source.next().unwrap()))
                .unwrap();
            lcs.next();
        }
    }
}

impl InstructionBytes for CopyInstruction {
    fn byte_sign(&self) -> u8 {
        COPY_INSTRUCTION_SIGN
    }

    fn byte_length(&self) -> usize {
        usize::try_from(self.len()).unwrap() + std::mem::size_of::<InstructionLength>() + 1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.byte_length());
        bytes.push(COPY_INSTRUCTION_SIGN);
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
        let length = InstructionLength::from_be_bytes(
            length_bytes
                .as_slice()
                .try_into()
                .map_err(|_| InstructionError::InvalidLength)?,
        );

        let content_bytes: Vec<u8> = bytes.take(length.try_into().unwrap()).copied().collect();

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
        Self {
            content: vec![InstructionItem::default(); InstructionLength::MIN.try_into().unwrap()],
        }
    }
}

impl From<&CopyInstruction> for Vec<u8> {
    fn from(value: &CopyInstruction) -> Self {
        value.to_bytes()
    }
}

impl From<CopyInstruction> for Vec<u8> {
    fn from(value: CopyInstruction) -> Self {
        value.to_bytes()
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for CopyInstruction {
    type Error = InstructionError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        CopyInstruction::try_from_bytes(value)
    }
}

impl TryFrom<Peekable<Iter<'_, u8>>> for CopyInstruction {
    type Error = InstructionError;

    fn try_from(mut value: Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        CopyInstruction::try_from_bytes(&mut value)
    }
}

impl TryFrom<Vec<u8>> for CopyInstruction {
    type Error = InstructionError;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        CopyInstruction::try_from_bytes(&mut value.iter().peekable())
    }
}

impl TryFrom<&[u8]> for CopyInstruction {
    type Error = InstructionError;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        CopyInstruction::try_from_bytes(&mut value.iter().peekable())
    }
}

#[cfg(test)]
mod copy_instruction_tests {
    use crate::lcs::Lcs;

    use super::*;

    #[test]
    fn instruction_info() {
        let mut instruction = CopyInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
        assert_eq!(instruction.len(), InstructionLength::MAX);
        assert!(instruction.is_full());

        instruction = CopyInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MIN.try_into().unwrap()
        ]);
        assert_eq!(instruction.len(), InstructionLength::MIN);
        assert!(instruction.is_empty());

        let default_instruction = CopyInstruction::default();
        assert_eq!(default_instruction, instruction);
    }

    #[test]
    fn non_default_item_count() {
        let mut instruction = CopyInstruction::default();
        for _ in 0..(InstructionLength::MAX / 2) {
            instruction.push(InstructionItem::default()).unwrap();
            assert_eq!(instruction.non_default_item_count().unwrap(), 0);
        }
        for i in 0..(InstructionLength::MAX / 2) {
            instruction.push(InstructionItem::default() + 1).unwrap();
            assert_eq!(instruction.non_default_item_count().unwrap(), i + 1);
        }
    }

    #[test]
    fn instruction_content_push() {
        let mut instruction =
            CopyInstruction::new(vec![
                InstructionItem::default();
                (InstructionLength::MAX - 1).try_into().unwrap()
            ]);
        assert!(instruction.push(InstructionItem::default()).is_ok());
        assert!(instruction
            .push(InstructionItem::default())
            .is_err_and(|err| err == InstructionError::ContentOverflow));
    }

    fn fill_wrapper(source: &[u8], target: &[u8]) -> CopyInstruction {
        let mut instruction = CopyInstruction::default();
        let lcs = Lcs::new(source, target).subsequence();
        let mut lcs_iter = lcs.iter().peekable();
        let mut source_iter = source.iter().peekable();
        let mut target_iter = target.iter().peekable();
        instruction.fill(&mut lcs_iter, &mut source_iter, &mut target_iter);
        instruction
    }

    #[test]
    fn instruction_content_fill() {
        assert_eq!(fill_wrapper(b"ABC", b"AYZ").len(), 2);
        assert_eq!(fill_wrapper(b"", b"AYZ").len(), 0);
        assert_eq!(fill_wrapper(b"ABC", b"").len(), 0);
        assert_eq!(fill_wrapper(b"AABB", b"AACC").len(), 4);
    }

    #[test]
    fn instruction_bytes_to_bytes() {
        let mut instruction = CopyInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
        let mut bytes = vec![COPY_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        bytes.extend(instruction.content.iter());
        assert_eq!(instruction.to_bytes(), bytes);

        instruction = CopyInstruction::default();
        bytes = vec![COPY_INSTRUCTION_SIGN];
        bytes.extend(instruction.len().to_be_bytes());
        assert_eq!(instruction.to_bytes(), bytes);
    }

    #[test]
    fn instruction_bytes_try_from_bytes_ok() {
        let mut instruction = CopyInstruction::new(vec![
            InstructionItem::default();
            InstructionLength::MAX.try_into().unwrap()
        ]);
        assert_eq!(
            CopyInstruction::try_from_bytes(&mut instruction.to_bytes().iter().peekable()).unwrap(),
            instruction
        );

        instruction = CopyInstruction::default();
        assert_eq!(
            CopyInstruction::try_from_bytes(&mut instruction.to_bytes().iter().peekable()).unwrap(),
            instruction
        );
    }

    #[test]
    fn instruction_bytes_try_from_bytes_err() {
        let mut bytes = vec![];

        assert_eq!(
            CopyInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::MissignSign
        );

        bytes = vec![0];
        assert_eq!(
            CopyInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::InvalidSign
        );

        bytes = vec![COPY_INSTRUCTION_SIGN];
        bytes.extend(InstructionLength::MAX.to_be_bytes());
        bytes.append(&mut vec![
            InstructionItem::default();
            InstructionLength::MAX as usize - 1
        ]);
        assert_eq!(
            CopyInstruction::try_from_bytes(&mut bytes.iter().peekable()).unwrap_err(),
            InstructionError::MissingContent
        );
    }
}
