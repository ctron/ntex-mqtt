use derive_more::{Display, From};
use either::Either;
use std::io;

pub use crate::error::*;
pub use crate::v5::codec;

/// Errors which can occur when attempting to handle mqtt client connection.
#[derive(Debug, Display, From)]
pub enum ClientError {
    /// Connect negotiation failed
    #[display(fmt = "Connect ack failed: {:?}", _0)]
    Ack(codec::ConnectAck),
    /// Protocol error
    #[display(fmt = "Protocol error: {:?}", _0)]
    Protocol(ProtocolError),
    /// Handshake timeout
    #[display(fmt = "Handshake timeout")]
    HandshakeTimeout,
    /// Peer disconnected
    #[display(fmt = "Peer disconnected")]
    Disconnected,
    /// Connect error
    #[display(fmt = "Connect error: {}", _0)]
    Connect(ntex::connect::ConnectError),
}

impl std::error::Error for ClientError {}

impl From<Either<EncodeError, io::Error>> for ClientError {
    fn from(err: Either<EncodeError, io::Error>) -> Self {
        match err {
            Either::Left(err) => ClientError::Protocol(ProtocolError::Encode(err)),
            Either::Right(err) => ClientError::Protocol(ProtocolError::Io(err)),
        }
    }
}

#[derive(Debug, Display)]
pub enum PublishQos0Error {
    /// Encoder error
    Encode(EncodeError),
    /// Can not allocate next packet id
    #[display(fmt = "Can not allocate next packet id")]
    PacketIdNotAvailable,
    /// Peer disconnected
    #[display(fmt = "Peer disconnected")]
    Disconnected,
}

#[derive(Debug, Display)]
pub enum PublishQos1Error {
    /// Negative ack from peer
    #[display(fmt = "Negative ack: {:?}", _0)]
    Fail(codec::PublishAck),
    /// Encoder error
    Encode(EncodeError),
    /// Can not allocate next packet id
    #[display(fmt = "Can not allocate next packet id")]
    PacketIdNotAvailable,
    /// Peer disconnected
    #[display(fmt = "Peer disconnected")]
    Disconnected,
}

#[derive(Debug, Display)]
pub enum SubscribeError {
    /// Encoder error
    Encode(EncodeError),
    /// Can not allocate next packet id
    #[display(fmt = "Can not allocate next packet id")]
    PacketIdNotAvailable,
    /// Peer disconnected
    #[display(fmt = "Peer disconnected")]
    Disconnected,
}

#[derive(Debug, Display)]
pub enum UnsubscribeError {
    /// Encoder error
    Encode(EncodeError),
    /// Can not allocate next packet id
    #[display(fmt = "Can not allocate next packet id")]
    PacketIdNotAvailable,
    /// Peer disconnected
    #[display(fmt = "Peer disconnected")]
    Disconnected,
}