use protobuf::Message;
use multihash;

use error::Result;
use pb_merkledag::{PBNode, PBLink};

#[derive(Debug, Clone)]
pub struct Node {
    pb_node: PBNode,
    encoded: Vec<u8>,
    hash: Vec<u8>,
}

impl Node {
    pub fn new(data: Vec<u8>) -> Result<Node> {
        let mut pb_node = PBNode::new();
        pb_node.set_Data(data);
        let encoded = try!(pb_node.write_to_bytes());

        Ok(Node {
            pb_node: pb_node,
            hash: hash(&encoded),
            encoded: encoded,
        })
    }

    pub fn add_node(&mut self, node: &Node) -> Result<()> {
        let mut link = PBLink::new();
        link.set_Tsize(node.pb_node.compute_size() as u64);
        link.set_Hash(node.hash.clone());
        self.pb_node.mut_Links().push(link);
        try!(self.reencode());

        Ok(())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn reencode(&mut self) -> Result<()> {
        self.encoded = try!(self.pb_node.write_to_bytes());
        self.hash = hash(&self.encoded);

        Ok(())
    }
}

#[inline]
fn hash(data: &[u8]) -> Vec<u8> {
    // unwrap() will only fail if HashType is invalid
    multihash::encode(multihash::HashTypes::SHA2256, data).unwrap()
}
