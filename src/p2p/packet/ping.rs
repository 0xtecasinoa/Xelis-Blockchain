use crate::core::reader::{Reader, ReaderError};
use crate::core::serializer::Serializer;
use crate::p2p::connection::Connection;
use crate::crypto::hash::Hash;
use std::sync::Arc;

pub struct Ping {
    block_top_hash: Hash,
    block_height: u64
}

impl Ping {
    pub fn new(block_top_hash: Hash, block_height: u64) -> Self {
        Self {
            block_top_hash,
            block_height
        }
    }

    pub fn update_connection(self, connection: &Arc<Connection>) {
        let _ = connection.set_block_top_hash(self.block_top_hash);
        connection.set_block_height(self.block_height);
    }
}

impl Serializer for Ping {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.block_top_hash.as_bytes());
        bytes.extend(self.block_height.to_be_bytes());
        bytes
    }

    fn from_bytes(reader: &mut Reader) -> Result<Self, ReaderError> {
        let block_top_hash = reader.read_hash()?;
        let block_height = reader.read_u64()?;

        Ok(Self { block_top_hash, block_height })
    }
}