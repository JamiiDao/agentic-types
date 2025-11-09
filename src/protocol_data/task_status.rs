use serde::{Deserialize, Serialize};

use crate::{Message, TaskState};

/// Represents the current state and associated context (e.g., a message from the agent) of a Task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct TaskStatus<'a> {
    /// The current state of the task's lifecycle.
    state: TaskState,
    ///  An optional, human-readable message providing more details about the current status.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Message<'a>>,
    /// An ISO 8601 datetime string indicating when this status was recorded.
    /// examples ["2023-10-27T10:00:00Z"]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<&'a str>,
}
