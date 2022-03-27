pub mod handshake;
pub mod request_chain;
pub mod ping;

use crate::core::reader::{Reader, ReaderError};
use crate::core::transaction::Transaction;
use crate::core::serializer::Serializer;
use crate::core::block::CompleteBlock;
use super::packet::handshake::Handshake;
use super::packet::request_chain::RequestChain;
use super::packet::ping::Ping;

const HANDSHAKE_ID: u8 = 0;
const TX_ID: u8 = 1;
const BLOCK_ID: u8 = 2;
const REQUEST_CHAIN_ID: u8 = 3;
const PING_ID: u8 = 4;

// TODO Rework this

pub enum PacketOut<'a> { // Outgoing Packet
    Handshake(&'a Handshake),
    Transaction(&'a Transaction),
    Block(&'a CompleteBlock),
    RequestChain(&'a RequestChain),
    Ping(&'a Ping)
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
            PacketOut::RequestChain(request) => (REQUEST_CHAIN_ID, request.to_bytes()),
            PacketOut::Ping(ping) => (PING_ID, ping.to_bytes())
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
    RequestChain(RequestChain),
    Ping(Ping)
}

impl Serializer for PacketIn {
    fn from_bytes(reader: &mut Reader) -> Result<PacketIn, ReaderError> {
        let res = match reader.read_u8()? {
            HANDSHAKE_ID => PacketIn::Handshake(Handshake::from_bytes(reader)?),
            TX_ID => PacketIn::Transaction(Transaction::from_bytes(reader)?),
            BLOCK_ID => PacketIn::Block(CompleteBlock::from_bytes(reader)?),
            REQUEST_CHAIN_ID => PacketIn::RequestChain(RequestChain::from_bytes(reader)?),
            PING_ID => PacketIn::Ping(Ping::from_bytes(reader)?),
            _ => return Err(ReaderError::InvalidValue)
        };
        Ok(res)
    }

    fn to_bytes(&self) -> Vec<u8> {
        panic!("Packet Incoming can't be serialized.")
    }
}