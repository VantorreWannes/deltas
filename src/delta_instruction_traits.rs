use std::slice::Iter;
use crate::delta_instruction_error::InstructionConvertBetweenBytesError;



pub trait ConvertBetweenBytes{

     fn to_bytes(&self) -> Vec<u8>;
     fn try_from_bytes(bytes: &mut Iter<u8>) -> Result<Self, InstructionConvertBetweenBytesError> where Self: Sized;
}