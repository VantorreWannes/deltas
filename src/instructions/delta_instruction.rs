use std::{iter::Peekable, slice::Iter};

use super::{
    add_instruction::AddInstruction, copy_instruction::CopyInstruction,
    remove_instruction::RemoveInstruction, InstructionBytes, InstructionContent, InstructionInfo,
    Result, ADD_INSTRUCTION_SIGN, COPY_INSTRUCTION_SIGN, REMOVE_INSTRUCTION_SIGN, InstructionError,
};

#[derive(Debug, PartialEq, Clone)]
pub enum DeltaInstruction {
    Remove(RemoveInstruction),
    Add(AddInstruction),
    Copy(CopyInstruction),
}

impl InstructionInfo for DeltaInstruction {
    fn len(&self) -> super::InstructionLength {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.len(),
            DeltaInstruction::Add(instruction) => instruction.len(),
            DeltaInstruction::Copy(instruction) => instruction.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.is_empty(),
            DeltaInstruction::Add(instruction) => instruction.is_empty(),
            DeltaInstruction::Copy(instruction) => instruction.is_empty(),
        }
    }

    fn is_full(&self) -> bool {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.is_full(),
            DeltaInstruction::Add(instruction) => instruction.is_full(),
            DeltaInstruction::Copy(instruction) => instruction.is_full(),
        }
    }

    fn non_default_item_count(&self) -> Option<super::InstructionLength> {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.non_default_item_count(),
            DeltaInstruction::Add(instruction) => instruction.non_default_item_count(),
            DeltaInstruction::Copy(instruction) => instruction.non_default_item_count(),
        }
    }
}

impl InstructionContent for DeltaInstruction {
    fn push(&mut self, content: super::InstructionItem) -> Result<()> {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.push(content),
            DeltaInstruction::Add(instruction) => instruction.push(content),
            DeltaInstruction::Copy(instruction) => instruction.push(content),
        }
    }
}

impl InstructionBytes for DeltaInstruction {
    fn byte_sign(&self) -> u8 {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.byte_sign(),
            DeltaInstruction::Add(instruction) => instruction.byte_sign(),
            DeltaInstruction::Copy(instruction) => instruction.byte_sign(),
        }
    }

    fn byte_length(&self) -> usize {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.byte_length(),
            DeltaInstruction::Add(instruction) => instruction.byte_length(),
            DeltaInstruction::Copy(instruction) => instruction.byte_length(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            DeltaInstruction::Remove(instruction) => instruction.to_bytes(),
            DeltaInstruction::Add(instruction) => instruction.to_bytes(),
            DeltaInstruction::Copy(instruction) => instruction.to_bytes(),
        }
    }

    fn try_from_bytes(bytes: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>) -> Result<Self>
    where
        Self: Sized,
    {
        match bytes.peek() {
            Some(&&ADD_INSTRUCTION_SIGN) => Ok(DeltaInstruction::Add(
                AddInstruction::try_from_bytes(bytes)?,
            )),
            Some(&&REMOVE_INSTRUCTION_SIGN) => Ok(DeltaInstruction::Remove(
                RemoveInstruction::try_from_bytes(bytes)?,
            )),
            Some(&&COPY_INSTRUCTION_SIGN) => Ok(DeltaInstruction::Copy(
                CopyInstruction::try_from_bytes(bytes)?,
            )),
            None => Err(super::InstructionError::MissignSign),
            _ => Err(super::InstructionError::InvalidSign),
        }
    }
}

impl From<RemoveInstruction> for DeltaInstruction {
    fn from(instruction: RemoveInstruction) -> Self {
        DeltaInstruction::Remove(instruction)
    }
}

impl From<AddInstruction> for DeltaInstruction {
    fn from(instruction: AddInstruction) -> Self {
        DeltaInstruction::Add(instruction)
    }
}

impl From<CopyInstruction> for DeltaInstruction {
    fn from(instruction: CopyInstruction) -> Self {
        DeltaInstruction::Copy(instruction)
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

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for DeltaInstruction {
    type Error = InstructionError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(value)
    }
}

impl TryFrom<Peekable<Iter<'_, u8>>> for DeltaInstruction {
    type Error = InstructionError;

    fn try_from(mut value: Peekable<Iter<'_, u8>>) -> std::result::Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value)
    }
}

impl TryFrom<Vec<u8>> for DeltaInstruction {
    type Error = InstructionError;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        DeltaInstruction::try_from_bytes(&mut value.iter().peekable())
    }
}

#[cfg(test)]
mod delta_instruction_tests {
    use crate::instructions::InstructionItem;

    use super::*;

    #[test]
    pub fn instruction_info() {
        let remove_instruction = RemoveInstruction::default();
        let add_instruction = AddInstruction::default();
        let copy_instruction = CopyInstruction::default();

        let wrapped_remove_instruction: DeltaInstruction = RemoveInstruction::default().into();
        let wrapped_add_instruction: DeltaInstruction = AddInstruction::default().into();
        let wrapped_copy_instruction: DeltaInstruction = CopyInstruction::default().into();

        assert_eq!(wrapped_remove_instruction.len(), remove_instruction.len());
        assert_eq!(wrapped_add_instruction.len(), add_instruction.len());
        assert_eq!(wrapped_copy_instruction.len(), copy_instruction.len());

        assert_eq!(wrapped_remove_instruction.is_empty(), remove_instruction.is_empty());
        assert_eq!(wrapped_add_instruction.is_empty(), add_instruction.is_empty());
        assert_eq!(wrapped_copy_instruction.is_empty(), copy_instruction.is_empty());

        assert_eq!(wrapped_remove_instruction.is_full(), remove_instruction.is_full());
        assert_eq!(wrapped_add_instruction.is_full(), add_instruction.is_full());
        assert_eq!(wrapped_copy_instruction.is_full(), copy_instruction.is_full());

        assert_eq!(wrapped_remove_instruction.non_default_item_count(), remove_instruction.non_default_item_count());
        assert_eq!(wrapped_add_instruction.non_default_item_count(), add_instruction.non_default_item_count());
        assert_eq!(wrapped_copy_instruction.non_default_item_count(), copy_instruction.non_default_item_count());
    }

    #[test]
    fn instruction_content() {
        let mut remove_instruction = RemoveInstruction::default();
        let mut add_instruction = AddInstruction::default();
        let mut copy_instruction = CopyInstruction::default();

        let mut wrapped_remove_instruction: DeltaInstruction = RemoveInstruction::default().into();
        let mut wrapped_add_instruction: DeltaInstruction = AddInstruction::default().into();
        let mut wrapped_copy_instruction: DeltaInstruction = CopyInstruction::default().into();

        let instruction_item = InstructionItem::default();
        assert_eq!(wrapped_remove_instruction.push(instruction_item), remove_instruction.push(instruction_item));
        assert_eq!(wrapped_add_instruction.push(instruction_item), add_instruction.push(instruction_item));
        assert_eq!(wrapped_copy_instruction.push(instruction_item), copy_instruction.push(instruction_item));
    }

    #[test]
    fn instruction_bytes() {
        let remove_instruction = RemoveInstruction::default();
        let add_instruction = AddInstruction::default();
        let copy_instruction = CopyInstruction::default();

        let wrapped_remove_instruction: DeltaInstruction = RemoveInstruction::default().into();
        let wrapped_add_instruction: DeltaInstruction = AddInstruction::default().into();
        let wrapped_copy_instruction: DeltaInstruction = CopyInstruction::default().into();

        assert_eq!(remove_instruction.byte_sign(), wrapped_remove_instruction.byte_sign());
        assert_eq!(add_instruction.byte_sign(), wrapped_add_instruction.byte_sign());
        assert_eq!(copy_instruction.byte_sign(), wrapped_copy_instruction.byte_sign());

        assert_eq!(wrapped_remove_instruction.byte_length(), remove_instruction.byte_length());
        assert_eq!(wrapped_add_instruction.byte_length(), add_instruction.byte_length());
        assert_eq!(wrapped_copy_instruction.byte_length(), copy_instruction.byte_length());

        assert_eq!(wrapped_remove_instruction.to_bytes(), remove_instruction.to_bytes());
        assert_eq!(wrapped_add_instruction.to_bytes(), add_instruction.to_bytes());
        assert_eq!(wrapped_copy_instruction.to_bytes(), copy_instruction.to_bytes());

        let remove_instruction_result: Result<RemoveInstruction> = remove_instruction.to_bytes().try_into();
        let add_instruction_result: Result<AddInstruction> = add_instruction.to_bytes().try_into();
        let copy_instruction_result: Result<CopyInstruction> = copy_instruction.to_bytes().try_into();

        let wrapped_remove_instruction_result: Result<DeltaInstruction> = wrapped_remove_instruction.to_bytes().try_into();
        let wrapped_add_instruction_result: Result<DeltaInstruction> = wrapped_add_instruction.to_bytes().try_into();
        let wrapped_copy_instruction_result: Result<DeltaInstruction> = wrapped_copy_instruction.to_bytes().try_into();

        assert!(remove_instruction_result.is_ok());
        assert!(add_instruction_result.is_ok());
        assert!(copy_instruction_result.is_ok());

        assert!(wrapped_remove_instruction_result.is_ok());
        assert!(wrapped_add_instruction_result.is_ok());
        assert!(wrapped_copy_instruction_result.is_ok());

        assert_eq!(DeltaInstruction::from(remove_instruction_result.unwrap()), wrapped_remove_instruction_result.unwrap());
        assert_eq!(DeltaInstruction::from(add_instruction_result.unwrap()), wrapped_add_instruction_result.unwrap());
        assert_eq!(DeltaInstruction::from(copy_instruction_result.unwrap()), wrapped_copy_instruction_result.unwrap());
    }

    #[test]
    fn into() {
        let remove_instruction = RemoveInstruction::default();
        let add_instruction = AddInstruction::default();
        let copy_instruction = CopyInstruction::default();
        assert_eq!(DeltaInstruction::from(remove_instruction.clone()), DeltaInstruction::Remove(remove_instruction));
        assert_eq!(DeltaInstruction::from(add_instruction.clone()), DeltaInstruction::Add(add_instruction));
        assert_eq!(DeltaInstruction::from(copy_instruction.clone()), DeltaInstruction::Copy(copy_instruction));
    }
}
