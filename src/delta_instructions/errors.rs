pub enum AddInstructionError {
    MaxLengthReached,
    InvalidSignByte(u8),
    InvalidLengthBytes(usize),
    MissingByteContent(usize, AddInstructionlength),
}

impl Display for AddInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AddInstructionError::MaxLengthReached => write!(f, "Can't fill this delta instruction type past {} values. Try creating a new instruction instead.", AddInstructionlength::MAX),
            AddInstructionError::InvalidSignByte(found_sign) => write!(f, "Expected this instruction sign: b'+'. But found something this instead: {found_sign}."),
            AddInstructionError::InvalidLengthBytes(found_number_length) => write!(f, "Couldn't construct the instruction content length indicator with {found_number_length} bytes. (Needed {}.)", std::mem::size_of::<AddInstructionlength>()),
            AddInstructionError::MissingByteContent(found_content_length, expected_content_length) => write!(f, "Expected an instruction content length of {expected_content_length} but found it was {found_content_length} long instead."),
        }
    }
}

impl Error for AddInstructionError {}

