use std::slice::Iter;

pub trait ConvertBetweenBytes {
    type Error;

    fn to_bytes(&self) -> Vec<u8>;
    fn try_from_bytes(bytes: &mut Iter<u8>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ApplyDeltaTo {
    type Error; 

    fn apply_to<'a>(&self, source: &'a mut [u8]) -> Result<&'a [u8], Self::Error>;
}
