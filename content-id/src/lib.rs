extern crate multihash;

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
            Protobuf => 0x70,
            CBOR => 0x71,
            Raw => 0x72,
            JSON => 0x73,
        }
    }

    pub fn from_raw(raw: u64) -> Option<Codec> {
        match raw {
            0x70 => Some(Codec::Protobuf),
            0x71 => Some(Codec::CBOR),
            0x72 => Some(Codec::Raw),
            0x73 => Some(Codec::JSON),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cid {
    version: u64,
    codec: Codec,
    hash: Vec<u8>,
}

impl Cid {
    pub fn new(code: Codec, hash: Vec<u8>) -> Cid {
        Cid {
            version: 1,
            codec: codec,
            hash: hash
        }
    }

    pub fn decode(value: &str) -> Option<Cid> {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
