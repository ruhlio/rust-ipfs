extern crate crypto;

use std::result;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2;
use crypto::sha3::{Sha3, Sha3Mode};
use crypto::blake2b::Blake2b;
use crypto::blake2s::Blake2s;

#[derive(Debug, PartialEq, Eq)]
pub enum MultihashError {
    Invalid,
    InvalidLength { expected: usize, actual: usize },
    UnsupportedType(usize),
}

pub type Result<T> = result::Result<T, MultihashError>;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum Algorithm {
    Sha1,
    Sha2_256,
    Sha2_512,
    Sha3_512,
    Sha3_384,
    Sha3_256,
    Sha3_224,
    Shake128,
    Shake256,
    Blake2b,
    Blake2s,
}

impl Algorithm {
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        match self {
            &Algorithm::Sha1 => digest(0x11, Sha1::new(), data, None),
            &Algorithm::Sha2_256 => digest(0x12, sha2::Sha256::new(), data, None),
            &Algorithm::Sha2_512 => digest(0x13, sha2::Sha512::new(), data, None),
            &Algorithm::Sha3_512 => digest(0x14, Sha3::new(Sha3Mode::Sha3_512), data, None),
            &Algorithm::Sha3_384 => digest(0x15, Sha3::new(Sha3Mode::Sha3_384), data, None),
            &Algorithm::Sha3_256 => digest(0x16, Sha3::new(Sha3Mode::Sha3_256), data, None),
            &Algorithm::Sha3_224 => digest(0x17, Sha3::new(Sha3Mode::Sha3_224), data, None),
            &Algorithm::Shake128 => digest(0x18, Sha3::new(Sha3Mode::Shake128), data, Some(64)),
            &Algorithm::Shake256 => digest(0x19, Sha3::new(Sha3Mode::Shake256), data, Some(64)),
            &Algorithm::Blake2b => digest(0x40, Blake2b::new(64), data, None),
            &Algorithm::Blake2s => digest(0x41, Blake2s::new(64), data, None),
        }
    }

    pub fn from_code(code: usize) -> Result<Algorithm> {
        match code {
            0x11 => Ok(Algorithm::Sha1),
            0x12 => Ok(Algorithm::Sha2_256),
            0x13 => Ok(Algorithm::Sha2_512),
            0x14 => Ok(Algorithm::Sha3_512),
            0x15 => Ok(Algorithm::Sha3_384),
            0x16 => Ok(Algorithm::Sha3_256),
            0x17 => Ok(Algorithm::Sha3_224),
            0x18 => Ok(Algorithm::Shake128),
            0x19 => Ok(Algorithm::Shake256),
            0x40 => Ok(Algorithm::Blake2b),
            0x41 => Ok(Algorithm::Blake2s),
            _ => Err(MultihashError::UnsupportedType(code))
        }
    }
}

fn digest<D: Digest>(code: usize, mut digest: D, data: &[u8], size: Option<usize>) -> Vec<u8> {
    digest.input(data);
    let size = size.unwrap_or_else(|| digest.output_bytes());
    //TODO: varint
    let mut buffer = vec![0; size + 2];
    buffer[0] = code as u8;
    buffer[1] = size as u8;
    digest.result(&mut buffer[2..]);

    buffer
}

#[derive(Debug, PartialEq, Eq)]
pub struct Multihash {
    pub algorithm: Algorithm,
    pub hash: Vec<u8>,
}

impl Multihash {
    pub fn decode(raw: &[u8]) -> Result<Multihash> {
        if 2 > raw.len() {
            return Err(MultihashError::Invalid);
        }

        let code = raw[0] as usize;
        let length = raw[1] as usize;
        let algorithm = try!(Algorithm::from_code(code));
        let expected_length = 2 + length;
        if expected_length != raw.len() {
            return Err(MultihashError::InvalidLength{ expected: expected_length, actual: length });
        }

        Ok(Multihash {
            algorithm: algorithm,
            hash: raw[2..].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use self::rustc_serialize::hex::FromHex;
    use super::*;

    struct TestCase {
        algorithm: Algorithm,
        data: &'static [u8],
        expected_code: u8,
        expected_length: u8,
        expected_hash: &'static str,
    }

    const TEST_CASES: &'static [TestCase] = &[
        TestCase {
            algorithm: Algorithm::Sha1,
            data: b"nope",
            expected_code: 0x11,
            expected_length: 20,
            expected_hash: "76272dc4faf660733711f58c736830d27159fb55",
        },
        TestCase {
            algorithm: Algorithm::Sha2_256,
            data: b"nopers",
            expected_code: 0x12,
            expected_length: 32,
            expected_hash: "6ca691a413f334a6619f06104d929483de9c178c8e33122ade7952e4f93054ea",
        },
        TestCase {
            algorithm: Algorithm::Sha2_512,
            data: b"nopers",
            expected_code: 0x13,
            expected_length: 64,
            expected_hash: "ef68c95a6b163629e748a5b9344ea893d4135ca10405da3a337971458c5f02d5df666daa49f62bd366dc72fa20cf059b0914c25e10fcd82a2d2fc4ea471fea00",
        },
        TestCase {
            algorithm: Algorithm::Sha3_512,
            data: b"ARGH",
            expected_code: 0x14,
            expected_length: 64,
            expected_hash: "d72ee8b6ee8bc05e9b99d82cd561c3fafbfd580e33b7fb6c69d662b270c2447a233633fc8a3248d1a68666b5acdae11400b80692c1b8300827d6933e3b0bb1da",
        },
        TestCase {
            algorithm: Algorithm::Sha3_256,
            data: b"bowser browser",
            expected_code: 0x16,
            expected_length: 32,
            expected_hash: "7ae24d5decfb7d8ab3abd54c5625c7d0c082710d34a2369509d6c440426c9043",
        },
        TestCase {
            algorithm: Algorithm::Shake256,
            data: b"G.I. Joe",
            expected_code: 0x19,
            expected_length: 64,
            expected_hash: "6ef3ef5814e49ff4f253399b0bdc6a5f41139e961e5b957dfa8c4c267f844e6152363f4274c400cc4f6a6b961a66ebf208e73b2d5dd9a52fdc3b3214e696cabc",
        },
    ];

    #[test]
    fn encode() {
        for case in TEST_CASES.iter() {
            println!("Testing {:?}", case.algorithm);
            let result = case.algorithm.encode(case.data);
            assert_eq!(result[0], case.expected_code);
            assert_eq!(result[1], case.expected_length);
            let expected_hash = case.expected_hash.from_hex().unwrap();
            assert_eq!(&result[2..], &expected_hash[..]);
        }
    }

    #[test]
    fn decode() {
        for case in TEST_CASES.iter() {
            let raw = case.algorithm.encode(case.data);
            let decoded = Multihash::decode(&raw).unwrap();
            assert_eq!(decoded.algorithm, case.algorithm);
            let expected_hash = case.expected_hash.from_hex().unwrap();
            assert_eq!(decoded.hash, expected_hash);
        }
    }

    #[test]
    fn decode_bad_boys() {
        assert_eq!(
            Multihash::decode(&vec![34]),
            Err(MultihashError::Invalid));
        assert_eq!(
            Multihash::decode(&vec![0xff, 2, 1, 2]),
            Err(MultihashError::UnsupportedType(0xff)));
        assert_eq!(
            Multihash::decode(&vec![0x14, 5,
                                   1, 2, 3]),
            Err(MultihashError::InvalidLength{ expected: 7, actual: 5 }));
    }
}
