extern crate multibase;
extern crate multihash;
extern crate protobuf;

use std::result;
use std::convert::From;
use multibase::{Multibase, Base, BitcoinBase58, MultibaseError};
use multihash::{Multihash, MultihashError};
use protobuf::{CodedInputStream, ProtobufError};

#[derive(Debug, PartialEq, Eq)]
pub enum CidError {
    UnsupportedVersion(u64),
    UnsupportedCodec(u64),
    Encoding,
    BaseEncoding(MultibaseError),
    Hash(MultihashError),
}

impl From<MultibaseError> for CidError {
    fn from(error: MultibaseError) -> Self {
        CidError::BaseEncoding(error)
    }
}

impl From<MultihashError> for CidError {
    fn from(error: MultihashError) -> Self {
        CidError::Hash(error)
    }
}

impl From<ProtobufError> for CidError {
    fn from(_: ProtobufError) -> Self {
        CidError::Encoding
    }
}

pub type Result<T> = result::Result<T, CidError>;

#[derive(Debug, PartialEq, Eq)]
pub enum Codec {
    Protobuf,
    CBOR,
    Raw,
    JSON,
}

impl Codec {
    pub fn to_raw(&self) -> u64 {
        match self {
            &Codec::Protobuf => 0x70,
            &Codec::CBOR => 0x71,
            &Codec::Raw => 0x72,
            &Codec::JSON => 0x73,
        }
    }

    pub fn from_raw(raw: u64) -> Result<Codec> {
        match raw {
            0x70 => Ok(Codec::Protobuf),
            0x71 => Ok(Codec::CBOR),
            0x72 => Ok(Codec::Raw),
            0x73 => Ok(Codec::JSON),
            _ => Err(CidError::UnsupportedCodec(raw))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cid {
    version: u64,
    codec: Codec,
    hash: Multihash,
}

impl Cid {
    // pub fn new(codec: Codec, hash: Multihash) -> Cid {
    //     Cid {
    //         version: 1,
    //         codec: codec,
    //         hash: hash,
    //     }
    // }

    pub fn decode(value: &str) -> Result<Cid> {
        if 46 == value.len() && "Qm" == &value[..2] {
            let raw_hash = try!(BitcoinBase58::decode(value));
            let hash = try!(Multihash::decode(&raw_hash));
            Ok(Cid {
                version: 0,
                codec: Codec::Protobuf,
                hash: hash,
            })
        }
        else {
            let raw = try!(Multibase::decode(value));
            Self::from_raw(&raw)
        }
    }

    pub fn from_raw(raw: &[u8]) -> Result<Cid> {
        if 34 == raw.len() && 18 == raw[0] && 32 == raw[1] {
            let hash = try!(Multihash::decode(&raw));
            Ok(Cid {
                version: 0,
                codec: Codec::Protobuf,
                hash: hash,
            })
        }
        else {
            let mut coded_input = CodedInputStream::from_bytes(&raw);
            let version = try!(coded_input.read_raw_varint64());
            if version > 1 {
                return Err(CidError::UnsupportedVersion(version));
            }
            let raw_codec = try!(coded_input.read_raw_varint64());
            let codec = try!(Codec::from_raw(raw_codec));
            let raw_hash = &raw[(coded_input.pos() as usize)..];
            let hash = try!(Multihash::decode(&raw_hash));

            Ok(Cid {
                version: version,
                codec: codec,
                hash: hash,
            })
        }
    }

}

#[cfg(test)]
mod tests {
    use multihash::Multihash;
    use super::*;

    #[test]
    fn decode_v0() {
        let raw = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";
        let hash = Multihash::decode(&vec![0x12, 32,
                                           227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85]).unwrap();

        let cid = Cid::decode(raw).unwrap();
        assert_eq!(cid, Cid {
            version: 0,
            codec: Codec::Protobuf,
            hash: hash,
        });
    }

    #[test]
    fn decode_bad_v0() {
        let raw = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zIII";

        assert!(Cid::decode(raw).is_err());
    }
}
