use crate::core::reader::{Reader, ReaderError};
use crate::core::transaction::Transaction;
use crate::core::serializer::Serializer;
use crate::core::block::CompleteBlock;
use super::handshake::Handshake;

const HANDSHAKE_ID: u8 = 0;
const TX_ID: u8 = 1;
const BLOCK_ID: u8 = 2;
const REQUEST_BLOCK_ID: u8 = 3;

pub enum PacketOut<'a> { // Outgoing Packet
    Handshake(&'a Handshake),
    Transaction(&'a Transaction),
    Block(&'a CompleteBlock),
    RequestBlock(u64)
}

impl<'a> Serializer for PacketOut<'a> {
    fn from_bytes(_: &mut Reader) -> Result<PacketOut<'a>, ReaderError> {
        Err(ReaderError::InvalidValue)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let (id, packet) = match self {
            PacketOut::Handshake(handshake) => (HANDSHAKE_ID, handshake.to_bytes()),
            PacketOut::Transaction(tx) => (TX_ID, tx.to_bytes()),
            PacketOut::Block(block) => (BLOCK_ID, block.to_bytes()),
            PacketOut::RequestBlock(height) => (REQUEST_BLOCK_ID, height.to_be_bytes().to_vec())
        };

        let packet_len: u32 = packet.len() as u32 + 1;
        bytes.extend(packet_len.to_be_bytes());
        bytes.push(id);
        bytes.extend(packet);

        bytes
    }
}

pub enum PacketIn { // Incoming Packet
    Handshake(Handshake),
    Transaction(Transaction),
    Block(CompleteBlock),
    RequestBlock(u64)
}

impl Serializer for PacketIn {
    fn from_bytes(reader: &mut Reader) -> Result<PacketIn, ReaderError> {
        let res = match reader.read_u8()? {
            HANDSHAKE_ID => PacketIn::Handshake(Handshake::from_bytes(reader)?),
            TX_ID => PacketIn::Transaction(Transaction::from_bytes(reader)?),
            BLOCK_ID => PacketIn::Block(CompleteBlock::from_bytes(reader)?),
            REQUEST_BLOCK_ID => PacketIn::RequestBlock(reader.read_u64()?),
            _ => return Err(ReaderError::InvalidValue)
        };
        Ok(res)
    }

    fn to_bytes(&self) -> Vec<u8> {
        panic!("Packet Incoming can't be serialized.")
    }
}