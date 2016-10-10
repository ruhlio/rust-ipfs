use num::{Zero, One};
use num::traits::ToPrimitive;
use num::bigint::{BigUint, ToBigUint};

use base::{Base, Error, Result};

pub struct BitcoinBase58;
pub struct FlickrBase58;

const BITCOIN_ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const FLICKR_ALPHABET: &'static [u8] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";

lazy_static! {
    static ref RADIX: BigUint = 58.to_biguint().unwrap();
}

impl Base for BitcoinBase58 {
    fn encode(data: &[u8]) -> String {
        encode(data, BITCOIN_ALPHABET)
    }

    fn decode(encoded: &str) -> Result<Vec<u8>> {
        decode(encoded, BITCOIN_ALPHABET)
    }
}

impl Base for FlickrBase58 {
    fn encode(data: &[u8]) -> String {
        encode(data, FLICKR_ALPHABET)
    }

    fn decode(encoded: &str) -> Result<Vec<u8>> {
        decode(encoded, FLICKR_ALPHABET)
    }
}

fn encode(data: &[u8], alphabet: &[u8]) -> String {
    let mut next = BigUint::from_bytes_be(data);

    let mut answer = Vec::with_capacity((data.len() * 136) / 100);
    while next > Zero::zero() {
        let index = (&next % &*RADIX).to_usize().unwrap();
        answer.push(alphabet[index]);
        next = &next / &*RADIX;
    }

    for _ in data.iter().take_while(|&elem| *elem == 0) {
        answer.push(alphabet[0]);
    }

    answer.reverse();
    let encoded = unsafe { String::from_utf8_unchecked(answer) };
    encoded
}

fn decode(encoded: &str, alphabet: &[u8]) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Ok(vec![]);
    }

    let mut answer: BigUint = Zero::zero();
    let mut position: BigUint = One::one();
    let encoded_bytes = encoded.as_bytes();

    for &ch in encoded_bytes.iter().rev() {
        match index_of(alphabet, ch) {
            Some(index) => {
                answer = answer + (index.to_biguint().unwrap() * &position);
            },
            None => return Err(Error::InvalidEncoding),
        }

        position = &position * &*RADIX;
    }

    let mut bytes_answer = Vec::with_capacity(encoded.len());
    for _ in encoded_bytes.iter().take_while(|&elem| *elem == alphabet[0]) {
        bytes_answer.push(0);
    }
    bytes_answer.append(&mut answer.to_bytes_be());

    Ok(bytes_answer)
}

fn index_of(haystack: &[u8], needle: u8) -> Option<usize> {
    for index in 0..haystack.len() {
        if haystack[index] == needle {
            return Some(index);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use rustc_serialize::hex::FromHex;
    use super::super::base::{Base, Error};
    use super::{FlickrBase58, BitcoinBase58};

    #[test]
    fn encode() {
        assert_eq!(
            BitcoinBase58::encode(&vec![]),
            "");
        assert_eq!(
            BitcoinBase58::encode(&"001234".from_hex().unwrap()),
            "12PM");
        assert_eq!(
            BitcoinBase58::encode(&"01230e001b10120c0a".from_hex().unwrap()),
            "qgbWq96H3ww");
        assert_eq!(
            FlickrBase58::encode(b"CREAM OF THE CROP"),
            "CGamgK11pQ2U4ouidC2Q3kJ");
    }

    #[test]
    fn decode() {
        assert_eq!(
            BitcoinBase58::decode(""),
            Ok(vec![]));
        assert_eq!(
            BitcoinBase58::decode("12PM"),
            Ok("001234".from_hex().unwrap()));
        assert_eq!(
            BitcoinBase58::decode("qgbWq96H3ww"),
            Ok("01230e001b10120c0a".from_hex().unwrap()));
        assert_eq!(
            FlickrBase58::decode("CGamgK11pQ2U4ouidC2Q3kJ"),
            Ok(b"CREAM OF THE CROP".to_vec()));

        assert_eq!(
            BitcoinBase58::decode("OOOOOOOH YEAH!"),
            Err(Error::InvalidEncoding));
    }
}
