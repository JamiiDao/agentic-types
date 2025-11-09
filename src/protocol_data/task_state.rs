use serde::{Deserialize, Serialize};

/// Defines the lifecycle states of a Task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    /// The task has been submitted and is awaiting execution.
    Submitted,
    /// The agent is actively working on the task.
    Working,
    /// The task is paused and waiting for input from the user.
    InputRequired,
    /// The task has been successfully completed.
    Completed,
    /// The task has been canceled by the user.
    Canceled,
    /// The task failed due to an error during execution.
    Failed,
    /// The task was rejected by the agent and was not started.
    Rejected,
    /// The task requires authentication to proceed.
    AuthRequired,
    /// The task is in an unknown or indeterminate state.
    #[default]
    Unknown,
}
