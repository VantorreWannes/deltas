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

    fn apply_to(&self, source: &mut Iter<u8>) -> Result<Vec<u8>, Self::Error>;
}
