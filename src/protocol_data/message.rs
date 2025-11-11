use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{JsonStr, JsonStrMemKV};

/// Represents a single communication turn or a piece of contextual information between a client and an agent.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message<'a> {
    pub role: MessageRole,
    #[serde(borrow)]
    pub parts: Vec<Part<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub metadata: Option<JsonStrMemKV<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub extensions: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub reference_task_ids: Option<Vec<&'a str>>,
    #[serde(borrow)]
    pub message_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub task_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub context_id: Option<&'a str>,
    #[serde(borrow)]
    pub kind: &'a str,
}

impl<'a> Message<'a> {
    pub fn new() -> Self {
        Self {
            kind: "message",
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "agent")]
    Agent,
}

/// A discriminated union representing a part of a message or artifact
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Part<'a> {
    #[serde(rename = "text")]
    #[serde(borrow)]
    Text(TextPart<'a>),
    #[serde(borrow)]
    #[serde(rename = "file")]
    File(FilePart<'a>),
    #[serde(borrow)]
    #[serde(rename = "data")]
    Data(DataPart<'a>),
}

impl<'a> Default for Part<'a> {
    fn default() -> Self {
        Self::Text(TextPart::default())
    }
}

/// Base properties common to all parts
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct PartBase<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    metadata: Option<JsonStrMemKV<'a>>,
}

/// Represents a text segment within a message or artifact
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct TextPart<'a> {
    #[serde(flatten)]
    base: PartBase<'a>,
    text: &'a str,
}

/// Represents a file segment within a message or artifact
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct FilePart<'a> {
    #[serde(flatten)]
    #[serde(borrow)]
    base: PartBase<'a>,
    #[serde(borrow)]
    file: FileWith<'a>,
}

/// Represents structured data within a message or artifact
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct DataPart<'a> {
    #[serde(flatten)]
    #[serde(borrow)]
    base: PartBase<'a>,
    #[serde(borrow)]
    data: BTreeMap<&'a str, JsonStr<'a>>,
}

/// File content as either bytes or URI
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileWith<'a> {
    #[serde(borrow)]
    Bytes(FileWithBytes<'a>),
    #[serde(borrow)]
    Uri(FileWithUri<'a>),
}

impl<'a> Default for FileWith<'a> {
    fn default() -> Self {
        Self::Uri(FileWithUri::default())
    }
}

/// File with base64-encoded content
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct FileWithBytes<'a> {
    #[serde(flatten)]
    base: FileBase<'a>,
    bytes: &'a str,
    // uri is absent (handled by never type in TS)
}

/// File with URI reference
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct FileWithUri<'a> {
    #[serde(flatten)]
    base: FileBase<'a>,
    uri: &'a str,
    // bytes is absent (handled by never type in TS)
}

/// Base properties for a file
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct FileBase<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<&'a str>,
}
