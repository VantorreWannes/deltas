use crate::instructions::Instruction;


#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    content: Vec<Instruction>,
}

impl Patch {

    pub fn new(source: &[u8], target: &[u8]) -> Self {
        todo!();
    }
}