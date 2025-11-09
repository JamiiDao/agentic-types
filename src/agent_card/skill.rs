use std::{borrow::Cow, collections::BTreeMap};

use serde::{Deserialize, Serialize};

/// Describes a specific capability, function, or area of expertise the agent can perform or address.
/// Represents a distinct capability or function that an agent can perform.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSkill<'a> {
    /// A unique identifier for the agent's skill.
    id: &'a str,
    /// A human-readable name for the skill.
    name: &'a str,
    /// A detailed description of the skill, intended to help clients or users understand its purpose and functionality.
    description: &'a str,
    /// A set of keywords describing the skill's capabilities.
    /// examples:  vec!["cooking", "customer support", "billing"]
    tags: Vec<&'a str>,
    /// Example prompts or scenarios that this skill can handle. Provides a hint to the client on how to use the skill.
    /// vec!["I need a recipe for bread"]
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<Cow<'a, str>>>,
    /// The set of supported input MIME types for this skill, overriding the agent's defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    input_modes: Option<Vec<&'a str>>,
    /// The set of supported output MIME types for this skill, overriding the agent's defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    output_modes: Option<Vec<&'a str>>,
    /// Security schemes necessary for the agent to leverage this skill.
    /// As in the overall AgentCard.security, this list represents a logical OR of security
    /// requirement objects. Each object is a set of security schemes that must be used together (a logical AND).
    /// examples [[{"google": ["oidc"]}]]
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Vec<BTreeMap<&'a str, Vec<&'a str>>>>,
}
