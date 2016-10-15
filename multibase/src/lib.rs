extern crate num;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod base;
mod base16;
mod base58;
mod base64;

pub use base::{Base, MultibaseError, Result};
pub use base16::Base16;
pub use base58::{BitcoinBase58, FlickrBase58};
pub use base64::{Base64, UrlSafeBase64};

pub enum Multibase {
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

impl Multibase {
    pub fn encode(&self, data: &[u8]) -> Result<String> {
        match *self {
            Multibase::Base16 =>
                Ok(format!("f{}", Base16::encode(data))),
            Multibase::Base58Flickr =>
                Ok(format!("Z{}", FlickrBase58::encode(data))),
            Multibase::Base58Bitcoin =>
                Ok(format!("z{}", BitcoinBase58::encode(data))),
            Multibase::Base64 =>
                Ok(format!("y{}", Base64::encode(data))),
            Multibase::Base64Url =>
                Ok(format!("Y{}", UrlSafeBase64::encode(data))),
            _ =>
                Err(MultibaseError::UnsupportedEncoding)
        }
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>> {
        match encoded.chars().next() {
            Some('f') => Base16::decode(&encoded[1..]),
            Some('Z') => FlickrBase58::decode(&encoded[1..]),
            Some('z') => BitcoinBase58::decode(&encoded[1..]),
            Some('y') => Base64::decode(&encoded[1..]),
            Some('Y') => UrlSafeBase64::decode(&encoded[1..]),
            _ => Err(MultibaseError::UnsupportedEncoding)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dogfooding() {
        for encoding in vec![Multibase::Base16, Multibase::Base58Bitcoin, Multibase::Base58Flickr, Multibase::Base64, Multibase::Base64Url] {
            let data = vec![1, 2, 3, 4, 14];
            let encoded = encoding.encode(&data);
            assert_eq!(Multibase::decode(&encoded.unwrap()), Ok(data));
        }
    }

    #[test]
    fn unsupported_encoding() {
        assert_eq!(
            Multibase::Base1.encode(&vec![]),
            Err(MultibaseError::UnsupportedEncoding));
        assert_eq!(
            Multibase::decode("1asd"),
            Err(MultibaseError::UnsupportedEncoding));
    }
}
