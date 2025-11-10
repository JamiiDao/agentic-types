use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    AgentCardSignature, AgentInterface, AgentSkill, JsonStr, SecurityScheme, TransportProtocol,
};

/// Agent Cards themselves might contain information that is considered sensitive.
///
/// - If an Agent Card contains sensitive information, the endpoint serving the card MUST be
///   protected by appropriate access controls (e.g., mTLS, network restrictions, authentication required to fetch the card).
/// - It is generally NOT RECOMMENDED to include plaintext secrets (like static API keys) directly in an Agent Card.
///   Prefer authentication schemes where clients obtain dynamic credentials out-of-band.
///
/// The AgentCard is a self-describing manifest for an agent. It provides essential
/// metadata including the agent's identity, capabilities, skills, supported
/// communication methods, and security requirements.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCard<'a> {
    /// The version of the A2A protocol this agent supports. defaults to "0.3.0"
    protocol_version: &'a str,
    ///  A human-readable name for the agent (UTF-8). Example ["Recipe Agent"]
    name: &'a str,
    /// A human-readable description of the agent, assisting users and other agents
    /// in understanding its purpose. Examples ["Agent that helps users with recipes and cooking."]
    description: &'a str,
    /// The preferred endpoint URL for interacting with the agent.
    /// This URL MUST support the transport specified by 'preferredTransport'.
    /// examples ["https://api.example.com/a2a/v1"]
    url: &'a str,
    /// The transport protocol for the preferred endpoint (the main 'url' field).
    /// If not specified, defaults to 'JSONRPC'.
    ///
    /// IMPORTANT: The transport specified here MUST be available at the main 'url'.
    /// This creates a binding between the main URL and its supported transport protocol.
    /// Clients should prefer this transport and URL combination when both are supported.
    /// default "JSONRPC", examples ["JSONRPC", "GRPC", "HTTP+JSON"]   
    #[serde(serialize_with = "serialize_transport_optional")]
    #[serde(deserialize_with = "deserialize_transport_optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_transport: Option<TransportProtocol>,
    /// A list of additional supported interfaces (transport and URL combinations).
    /// This allows agents to expose multiple transports, potentially at different URLs.
    ///
    /// Best practices:
    /// - SHOULD include all supported transports for completeness
    /// - SHOULD include an entry matching the main 'url' and 'preferredTransport'
    /// - MAY reuse URLs if multiple transports are available at the same endpoint
    /// - MUST accurately declare the transport available at each URL
    ///
    /// Clients can select any interface from this list based on their transport capabilities
    /// and preferences. This enables transport negotiation and fallback scenarios.
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_interfaces: Option<Vec<AgentInterface<'a>>>,
    /// An optional URL to an icon for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<&'a str>,
    /// Information about the agent's service provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<AgentProvider<'a>>,
    /// The agent's own version number. The format is defined by the provider. examples ["1.0.0"]
    version: &'a str,
    /// An optional URL to the agent's documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation_url: Option<&'a str>,
    /// A declaration of optional capabilities supported by the agent.
    capabilities: AgentCapabilities<'a>,
    /// A declaration of the security schemes available to authorize requests.
    /// The key is the scheme name. Follows the OpenAPI 3.0 Security Scheme Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    security_schemes: Option<BTreeMap<&'a str, SecurityScheme<'a>>>,
    /// A list of security requirement objects that apply to all agent interactions. Each object
    /// lists security schemes that can be used. Follows the OpenAPI 3.0 Security Requirement Object.
    /// This list can be seen as an OR of ANDs. Each object in the list describes one possible
    /// set of security requirements that must be present on a request. This allows specifying,
    /// for example, "callers must either use OAuth OR an API Key AND mTLS."
    /// examples [[{"oauth": ["read"]}, {"api-key": [], "mtls": []}]]
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Vec<BTreeMap<&'a str, Vec<&'a str>>>>,
    ///  Default set of supported input MIME types for all skills, which can be
    ///  overridden on a per-skill basis.
    default_input_modes: Vec<&'a str>,
    /// Default set of supported output MIME types for all skills, which can be overridden on a per-skill basis.
    default_output_modes: Vec<&'a str>,
    /** The set of skills, or distinct capabilities, that the agent can perform. */
    skills: Vec<AgentSkill<'a>>,
    /// If true, the agent can provide an extended agent card with additional details to authenticated users. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_authenticated_extended_card: Option<bool>,
    /// JSON Web Signatures computed for this AgentCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    signatures: Option<Vec<AgentCardSignature<'a>>>,
}

impl<'a> AgentCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn protocol_version(&self) -> &str {
        self.protocol_version
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn preferred_transport(&self) -> Option<&TransportProtocol> {
        self.preferred_transport.as_ref()
    }

    pub fn additional_interfaces(&self) -> Option<&Vec<AgentInterface<'_>>> {
        self.additional_interfaces.as_ref()
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url
    }

    pub fn provider(&self) -> Option<&AgentProvider<'_>> {
        self.provider.as_ref()
    }

    pub fn documentation_url(&self) -> Option<&str> {
        self.documentation_url
    }

    pub fn security_schemes(&self) -> Option<&BTreeMap<&'a str, SecurityScheme<'a>>> {
        self.security_schemes.as_ref()
    }

    pub fn security(&self) -> Option<&Vec<BTreeMap<&'a str, Vec<&'a str>>>> {
        self.security.as_ref()
    }

    pub fn default_input_modes(&self) -> &[&str] {
        self.default_input_modes.as_slice()
    }

    pub fn default_output_modes(&self) -> &[&str] {
        self.default_output_modes.as_slice()
    }

    pub fn supports_authenticated_extended_card(&self) -> Option<bool> {
        self.supports_authenticated_extended_card
    }

    pub fn signatures(&self) -> Option<&Vec<AgentCardSignature<'a>>> {
        self.signatures.as_ref()
    }
}

/// Information about the organization or entity providing the agent.
/// Represents the service provider of an agent.
/// examples [{ "organization": "Google", "url": "https://ai.google.dev" }]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentProvider<'a> {
    /// The name of the agent provider's organization.
    pub organization: &'a str,
    /// A URL for the agent provider's website or relevant documentation.
    pub url: &'a str,
}

/// Specifies optional A2A protocol features supported by the agent.
/// Defines optional capabilities supported by an agent.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilities<'a> {
    /// Indicates if the agent supports Server-Sent Events (SSE) for streaming responses.
    streaming: bool,
    /// Indicates if the agent supports sending push notifications for asynchronous task updates.
    push_notifications: bool,
    /// Indicates if the agent provides a history of state transitions for a task.
    #[serde(skip_serializing_if = "Option::is_none")]
    state_transition_history: Option<bool>,
    /// A list of protocol extensions supported by the agent.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<Vec<AgentExtension<'a>>>,
}

/// Specifies an extension to the A2A protocol supported by the agent.
/// A declaration of a protocol extension supported by an Agent.
/// examples [{"uri": "https://developers.google.com/identity/protocols/oauth2", "description": "Google OAuth 2.0 authentication", "required": false}]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentExtension<'a> {
    /// The unique URI identifying the extension.
    uri: String,
    /// A human-readable description of how this agent uses the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// If true, the client must understand and comply with the extension's requirements to interact with the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// Optional, extension-specific configuration parameters.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<BTreeMap<String, JsonStr<'a>>>,
}

pub fn serialize_transport_optional<S>(
    transport: &Option<TransportProtocol>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match transport {
        Some(s) => serializer.serialize_some(s.as_str()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_transport_optional<'de, D>(
    deserializer: D,
) -> Result<Option<TransportProtocol>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<&'de str>::deserialize(deserializer)?;
    Ok(opt.map(|value| value.into()))
}

pub fn serialize_additional_interfaces<S>(
    additional_interfaces: &Vec<AgentInterface>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if additional_interfaces.is_empty() {
        serializer.serialize_none()
    } else {
        serializer.serialize_some(additional_interfaces)
    }
}
