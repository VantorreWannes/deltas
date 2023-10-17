pub mod instruction;
pub mod error;
pub mod traits;
mod add_instruction;
mod copy_instruction;
mod remove_instruction;

const MAX_INSTRUCTION_LENGTH: u8 = u8::MAX;
const MIN_INSTRUCTION_LENGTH: u8 = u8::MIN;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';