
#[derive(Debug, PartialEq, Clone, Default)]
pub struct CopyInstruction {
    content: Vec<u8>,
}

impl CopyInstruction {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }

}
