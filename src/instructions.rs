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
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == u8::MAX
    }
}
