use rustc_serialize::hex::{ToHex, FromHex};
use base::{Base, Result, MultibaseError};

pub struct Base16;
impl Base for Base16 {
    fn encode(data: &[u8]) -> String {
        data.to_hex()
    }

    fn decode(encoded: &str) -> Result<Vec<u8>> {
        match encoded.from_hex() {
            Ok(raw) => Ok(raw),
            Err(_) => Err(MultibaseError::InvalidEncoding)
        }
    }
}

