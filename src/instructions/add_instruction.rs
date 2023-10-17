
#[derive(Debug, PartialEq, Clone, Default)]
pub struct AddInstruction {
    content: Vec<u8>,
}

impl AddInstruction {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}
