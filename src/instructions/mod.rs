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

const REMOVE_INSTRUCTION_SIGN: InstructionItem = b'-';
const ADD_INSTRUCTION_SIGN: InstructionItem = b'+';
const COPY_INSTRUCTION_SIGN: InstructionItem = b'|';