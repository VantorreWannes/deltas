pub mod instruction;
pub mod error;
pub mod traits;
mod add_instruction;
mod copy_instruction;
mod remove_instruction;

type InstructionLength = u8;
const MAX_INSTRUCTION_LENGTH: u8 = InstructionLength::MAX;
const MIN_INSTRUCTION_LENGTH: u8 = InstructionLength::MIN;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';