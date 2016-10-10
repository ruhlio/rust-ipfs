extern crate num;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod base;
mod base16;
mod base58;
mod base64;

pub use base::{Base, Error, Result};
pub use base16::Base16;
pub use base58::{BitcoinBase58, FlickrBase58};
pub use base64::{Base64, UrlSafeBase64};

pub enum Encoding {
    Base1,
    Base2,
    Base8,
    Base10,
    Base16,
    Base58Flickr,
    Base58Bitcoin,
    Base64,
    Base64Url,
}

impl Encoding {
    pub fn encode(&self, data: &[u8]) -> Result<String> {
        match *self {
            Encoding::Base16 =>
                Ok(format!("f{}", Base16::encode(data))),
            Encoding::Base58Flickr =>
                Ok(format!("Z{}", FlickrBase58::encode(data))),
            Encoding::Base58Bitcoin =>
                Ok(format!("z{}", BitcoinBase58::encode(data))),
            Encoding::Base64 =>
                Ok(format!("y{}", Base64::encode(data))),
            Encoding::Base64Url =>
                Ok(format!("Y{}", UrlSafeBase64::encode(data))),
            _ =>
                Err(Error::UnsupportedEncoding)
        }
    }
}

pub fn decode(encoded: &str) -> Result<Vec<u8>> {
    match encoded.chars().next() {
        Some('f') => Base16::decode(&encoded[1..]),
        Some('Z') => FlickrBase58::decode(&encoded[1..]),
        Some('z') => BitcoinBase58::decode(&encoded[1..]),
        Some('y') => Base64::decode(&encoded[1..]),
        Some('Y') => UrlSafeBase64::decode(&encoded[1..]),
        _ => Err(Error::UnsupportedEncoding)
    }
}

#[cfg(test)]
mod tests {
    use super::{Encoding, decode};
    use super::base::Error;

    #[test]
    fn dogfooding() {
        for encoding in vec![Encoding::Base16, Encoding::Base58Bitcoin, Encoding::Base58Flickr, Encoding::Base64, Encoding::Base64Url] {
            let data = vec![1, 2, 3, 4, 14];
            let encoded = encoding.encode(&data);
            assert_eq!(decode(&encoded.unwrap()), Ok(data));
        }
    }

    #[test]
    fn unsupported_encoding() {
        assert_eq!(
            Encoding::Base1.encode(&vec![]),
            Err(Error::UnsupportedEncoding));
        assert_eq!(
            decode("1asd"),
            Err(Error::UnsupportedEncoding));
    }
}
