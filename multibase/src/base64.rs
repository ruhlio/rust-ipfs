use rustc_serialize::base64::{ToBase64, FromBase64,
                              Config, Newline, CharacterSet};
use base::{Base, Error, Result};

const CONFIG: Config = Config {
    char_set: CharacterSet::Standard,
    newline: Newline::LF,
    pad: true,
    line_length: None,
};

const URL_SAFE_CONFIG: Config = Config {
    char_set: CharacterSet::UrlSafe,
    newline: Newline::LF,
    pad: true,
    line_length: None,
};

pub struct Base64;
impl Base for Base64 {
    fn encode(data: &[u8]) -> String {
        data.to_base64(CONFIG)
    }

    fn decode(encoded: &str) -> Result<Vec<u8>> {
        match encoded.from_base64() {
            Ok(raw) => Ok(raw),
            Err(_) => Err(Error::InvalidEncoding)
        }
    }
}

pub struct UrlSafeBase64;
impl Base for UrlSafeBase64 {
    fn encode(data: &[u8]) -> String {
        data.to_base64(URL_SAFE_CONFIG)
    }

    fn decode(encoded: &str) -> Result<Vec<u8>> {
        match encoded.from_base64() {
            Ok(raw) => Ok(raw),
            Err(_) => Err(Error::InvalidEncoding)
        }
    }
}
