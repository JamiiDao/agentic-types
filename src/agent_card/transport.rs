
use serde::{
    Deserialize, Deserializer, Serializer,
};

/// Provides a declaration of a combination of the target URL and the supported transport to interact with the agent.
/// This enables agents to expose the same functionality through multiple transport protocols.
/// Supported A2A transport protocols.
/// Additional transport values MAY be used for future extensions,
/// but such extensions MUST not conflict with core A2A protocol functionality.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
#[non_exhaustive]
pub enum TransportProtocol {
    ///  "JSONRPC", JSON-RPC 2.0 over HTTP
    #[default]
    JsonRpc,
    /// gRPC over HTTP/2
    Grpc,
    /// "HTTP+JSON" REST-style HTTP with JSON
    HttpJson,
}

impl TransportProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::JsonRpc => "JSONRPC",
            Self::Grpc => "GRPC",
            Self::HttpJson => "HTTP+JSON",
        }
    }
}

impl From<&str> for TransportProtocol {
    fn from(value: &str) -> Self {
        match value {
            val if val == TransportProtocol::Grpc.as_str() => TransportProtocol::Grpc,
            val if val == TransportProtocol::HttpJson.as_str() => TransportProtocol::HttpJson,
            val if val == TransportProtocol::JsonRpc.as_str() => TransportProtocol::JsonRpc,
            _ => TransportProtocol::JsonRpc,
        }
    }
}

pub fn serialize_transport<S>(
    transport: &TransportProtocol,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(transport.as_str())
}

pub fn deserialize_transport<'de, D>(deserializer: D) -> Result<TransportProtocol, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(<&'de str>::deserialize(deserializer)?.into())
}
