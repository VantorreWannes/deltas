use self::error::InstructionError;

pub mod instruction;
pub mod error;
pub mod traits;
mod add_instruction;
mod copy_instruction;
mod remove_instruction;

type InstructionItem = u8;
type InstructionLength = u8;

type Result<T> = std::result::Result<T, InstructionError>;

const MAX_INSTRUCTION_LENGTH: u8 = InstructionLength::MAX;
const MIN_INSTRUCTION_LENGTH: u8 = InstructionLength::MIN;

const REMOVE_INSTRUCTION_SIGN: u8 = b'-';
const ADD_INSTRUCTION_SIGN: u8 = b'+';
const COPY_INSTRUCTION_SIGN: u8 = b'|';