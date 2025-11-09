use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::OpenIdExtension;

/// Describes the authentication requirements for accessing the agent's url endpoint. Refer Sample Agent Card for an example.
/// Defines a security scheme that can be used to secure an agent's endpoints.
/// This is a discriminated union type based on the OpenAPI 3.0 Security Scheme Object.
/// https://swagger.io/specification/#security-scheme-object}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SecurityScheme<'a> {
    #[serde(rename_all = "camelCase")]
    ApiKey {
        /// REQUIRED. The name of the header, query, or cookie parameter to be used.
        name: String,
        /// REQUIRED. The location of the API key. ("query", "header", or "cookie")
        #[serde(rename = "in")]
        location: String,
        /// Optional description in CommonMark syntax.
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Specification extensions (x-*)
        #[serde(flatten)]
        #[serde(borrow)]
        extensions: OpenIdExtension<'a>,
    },

    #[serde(rename_all = "camelCase")]
    Http {
        /// REQUIRED. The HTTP authentication scheme (e.g., "basic", "bearer").
        scheme: String,
        /// Optional hint describing the format of the bearer token.
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
        /// Optional description.
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: OpenIdExtension<'a>,
    },

    #[serde(rename_all = "camelCase")]
    MutualTLS {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: OpenIdExtension<'a>,
    },

    #[serde(rename_all = "camelCase")]
    OAuth2 {
        /// REQUIRED. Object containing OAuth2 flow configurations.
        flows: Box<OAuthFlows>,
        /// Optional metadata URL (RFC 8414).
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2_metadata_url: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: OpenIdExtension<'a>,
    },

    #[serde(rename_all = "camelCase")]
    OpenIdConnect {
        /// REQUIRED. Well-known OpenID Connect discovery URL.
        open_id_connect_url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(flatten)]
        extensions: OpenIdExtension<'a>,
    },
}

impl<'a> Default for SecurityScheme<'a> {
    fn default() -> Self {
        Self::Http {
            scheme: String::default(),
            bearer_format: Option::None,
            description: Option::default(),
            extensions: BTreeMap::default(),
        }
    }
}

/// Represents OAuth2 Flows Object (simplified)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<OAuthFlow>,
}

/// Represents a single OAuth2 Flow
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<BTreeMap<String, String>>,
}
