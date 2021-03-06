//! MQTT 3.1.1 Client/Server framework

pub mod client;
pub mod codec;
mod connect;
pub mod control;
mod default;
mod dispatcher;
pub mod error;
mod publish;
mod router;
mod server;
mod sink;

pub type Session<St> = crate::Session<MqttSink, St>;

pub use self::client::Client;
pub use self::connect::{Connect, ConnectAck};
pub use self::control::{ControlMessage, ControlResult};
pub use self::publish::Publish;
pub use self::router::Router;
pub use self::server::MqttServer;
pub use self::sink::{MqttSink, PublishBuilder};

pub use crate::error::MqttError;
pub use crate::topic::Topic;
pub use crate::types::QoS;

#[doc(hidden)]
pub type ControlPacket = ControlMessage;
