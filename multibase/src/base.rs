use std::result;

#[derive(Debug, PartialEq, Eq)]
pub enum MultibaseError {
    InvalidEncoding,
    UnsupportedEncoding,
}

pub type Result<T> = result::Result<T, MultibaseError>;

pub trait Base {
    fn encode(data: &[u8]) -> String;
    fn decode(encoded: &str) -> Result<Vec<u8>>;
}
