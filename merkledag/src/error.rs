use std::result;
use std::convert::From;
use protobuf::ProtobufError;

pub type Result<T> = result::Result<T, MerkleError>;

pub enum MerkleError {
    ProtobufError(ProtobufError)
}

impl From<ProtobufError> for MerkleError {
    fn from(err: ProtobufError) -> Self {
        MerkleError::ProtobufError(err)
    }
}
