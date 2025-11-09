use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::JsonStr;

/// Represents a JSON Web Signature (JWS) used to verify the integrity of the AgentCard.
/// AgentCardSignature represents a JWS signature of an AgentCard.
/// This follows the JSON format of an RFC 7515 JSON Web Signature (JWS).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
pub struct AgentCardSignature<'a> {
    /// The protected JWS header for the signature.
    /// This is a Base64url-encoded JSON object, as per RFC 7515.
    protected: &'a str,
    /// The computed signature, Base64url-encoded.
    #[serde(borrow)]
    signature: &'a str,
    /// The unprotected JWS header values.
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<BTreeMap<&'a str, JsonStr<'a>>>,
}
