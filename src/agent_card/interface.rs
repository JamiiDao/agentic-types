
use serde::{
    Deserialize, Serialize,
};


use crate::{deserialize_transport, serialize_transport, TransportProtocol};

/// Declares a combination of a target URL and a transport protocol for interacting with the agent.
/// This allows agents to expose the same functionality over multiple transport mechanisms.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
pub struct AgentInterface<'a> {
    /// The URL where this interface is available. Must be a valid absolute HTTPS URL in production.
    /// examples vec!["https://api.example.com/a2a/v1", "https://grpc.example.com/a2a", "https://rest.example.com/v1"]
    url: &'a str,
    /// The transport protocol supported at this URL.
    /// examples vec!["JSONRPC", "GRPC", "HTTP+JSON"]    
    #[serde(serialize_with = "serialize_transport")]
    #[serde(deserialize_with = "deserialize_transport")]
    transport: TransportProtocol,
}
