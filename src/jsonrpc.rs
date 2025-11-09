use serde::{Deserialize, Serialize};

use crate::{
    AgentCard, Artifact, JsonStr, JsonStrMemKV, Message, PushNotificationConfig, Task, TaskState,
    TaskStatus,
};

/// A2A adheres to the standard JSON-RPC 2.0 structures for requests and responses.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct JsonRpcRequest<'a> {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    jsonrpc: &'a str,
    /// A String containing the name of the method to be invoked (e.g., "message/send", "tasks/get").
    method: &'a str,
    /// A Structured value that holds the parameter values to be used during the invocation of the method. This member MAY be omitted if the method expects no parameters. A2A methods typically use an object for params.
    #[serde(borrow)]
    params: Option<JsonStr<'a>>,
    /// An identifier established by the Client that MUST contain a String, Number, or NULL value if included.
    /// If it is not included it is assumed to be a notification. The value SHOULD NOT be NULL for requests expecting a response,
    /// and Numbers SHOULD NOT contain fractional parts. The Server MUST reply with the same value in the Response object
    /// if included. This member is used to correlate the context between the two objects.
    /// A2A methods typically expect a response or stream, so id will usually be present and non-null.
    id: Option<JsonRpcId>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(untagged)]
pub enum JsonRpcId {
    String(String),
    Number(i64),
    Null,
}

/// The A2A Server's HTTP response body MUST be a JSONRPCResponse object
///  (or, for streaming methods, an SSE stream where each event's data is a JSONRPCResponse).
/// The Content-Type for JSON-RPC responses is `application/json`. For SSE streams, it is `text/event-stream`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct JsonRpcResponse<'a, T> {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    #[serde(borrow)]
    jsonrpc: &'a str,
    /// This member is REQUIRED. It MUST be the same as the value of the id member in the Request Object.
    /// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<JsonRpcId>,
    /// [JsonRpcPayload] representing a success or error
    #[serde(flatten)]
    #[serde(borrow)]
    payload: JsonRpcPayload<'a, T>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(untagged)]
pub enum JsonRpcPayload<'a, T> {
    Success {
        result: T,
    },
    Error {
        #[serde(borrow)]
        error: JsonRpcError<'a>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct JsonRpcError<'a> {
    code: i32,
    #[serde(borrow)]
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    data: Option<JsonStr<'a>>,
}

/// Represents a JSON-RPC 2.0 Error object, included in an error response.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct JSONRPCError<'a> {
    /// A number that indicates the error type that occurred.
    code: i64,
    /// A string providing a short description of the error.
    message: &'a str,
    /// A primitive or structured value containing additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonStr<'a>>,
}

pub enum JsonRpcMethods {
    MessageSend,
    MessageStream,
    TasksGet,
    TasksStream,
    TasksList,
    TasksCancel,
    TasksPushNotificationConfigSet,
    TasksPushNotificationConfigGet,
    TasksPushNotificationConfigList,
    TasksPushNotificationConfigDelete,
    TasksResubscribe,
    AgentGetAuthenticatedExtendedCard,
}

/// Defines the parameters for a request to send a message to an agent. This can be used
/// to create a new task, continue an existing one, or restart a task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct MessageSendParams<'a> {
    /// The message object being sent to the agent.
    #[serde(borrow)]
    message: Message<'a>,
    /// Optional configuration for the send request.
    configuration: Option<MessageSendConfiguration<'a>>,
    /// Optional metadata for extensions.
    metadata: Option<JsonStrMemKV<'a>>,
}

/// Defines configuration options for a `message/send` or `message/stream` request.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct MessageSendConfiguration<'a> {
    ///  A list of output MIME types the client is prepared to accept in the response.
    accepted_output_modes: Option<&'a str>,
    ///  The number of most recent messages from the task's history to retrieve in the response.
    history_length: Option<i16>,
    ///  Configuration for the agent to send push notifications for updates after the initial response.
    push_notification_config: Option<PushNotificationConfig<'a>>,
    ///  If true, the client will wait for the task to complete. The server may reject this if the task is long-running.
    blocking: Option<bool>,
}

/// Represents a successful JSON-RPC response for the `message/stream` method.
/// The server may send multiple response objects for a single request.
/// The result, which can be a Message, Task, or a streaming update event.
pub enum SendStreamingMessageSuccessResponse<'a> {
    Message(Message<'a>),
    Task(Task<'a>),
    TaskStatusUpdateEvent(TaskStatusUpdateEvent<'a>),
    TaskArtifactUpdateEvent(TaskArtifactUpdateEvent<'a>),
}

/// Carries information about a change in the task's status during streaming.
/// This is one of the possible result types in a SendStreamingMessageSuccessResponse.
/// An event sent by the agent to notify the client of a change in a task's status.
/// This is typically used in streaming or subscription models.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatusUpdateEvent<'a> {
    /// The ID of the task that was updated.
    task_id: &'a str,
    /// The context ID associated with the task.
    context_id: &'a str,
    /// The type of this event, used as a discriminator. Always 'status-update'.
    kind: &'a str,
    /// The new status of the task.
    status: TaskStatus<'a>,
    /// If true, this is the final event in the stream for this interaction.
    r#final: bool,
    /// Optional metadata for extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<JsonStrMemKV<'a>>,
}

impl<'a> TaskStatusUpdateEvent<'a> {
    pub fn new() -> Self {
        Self {
            kind: "status-update",
            ..Default::default()
        }
    }
}

/// Carries a new or updated artifact (or a chunk of an artifact) generated by the task during streaming.
/// This is one of the possible result types in a SendTaskStreamingResponse.
/// An event sent by the agent to notify the client that an artifact has been
/// generated or updated. This is typically used in streaming models.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskArtifactUpdateEvent<'a> {
    ///  The ID of the task this artifact belongs to.
    task_id: &'a str,
    /// The context ID associated with the task.
    context_id: &'a str,
    /// The type of this event, used as a discriminator. Always 'artifact-update'.
    kind: &'a str,
    /// The artifact that was generated or updated.
    artifact: Artifact<'a>,
    /// If true, the content of this artifact should be appended to a previously sent artifact with the same ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    append: Option<bool>,
    /// If true, this is the final chunk of the artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    last_chunk: Option<bool>,
    /// Optional metadata for extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<JsonStrMemKV<'a>>,
}

impl<'a> TaskArtifactUpdateEvent<'a> {
    pub fn new() -> Self {
        Self {
            kind: "artifact-update",
            ..Default::default()
        }
    }
}

/// Retrieves the current state (including status, artifacts, and optionally history) of a previously initiated task.
/// This is typically used for polling the status of a task initiated with message/send,
/// or for fetching the final state of a task after being notified via a push notification or after an SSE stream has ended.
/// Defines parameters for querying a task, with an option to limit history length.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskQueryParams {
    /// The number of most recent messages from the task's history to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    history_length: Option<i64>,
}

/// Parameters for filtering and paginating task results.
/// Parameters for listing tasks with optional filtering criteria. @since 0.4.0 */
/// Note on includeArtifacts parameter: When includeArtifacts is false (the default),
/// the artifacts field MUST be omitted entirely from each Task object in the response.
/// The field should not be present as an empty array or null value. When includeArtifacts is true,
/// the artifacts field should be included with its actual content (which may be an empty array if the task has no artifacts).
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTasksParams<'a> {
    /// Filter tasks by context ID to get tasks from a specific conversation or session.
    #[serde(skip_serializing_if = "Option::is_none")]
    context_id: Option<&'a str>,
    ///Filter tasks by their current status state.
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<TaskState>,
    ///Maximum number of tasks to return. Must be between 1 and 100. Defaults to 50 if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i64>,
    ///Token for pagination. Use the nextPageToken from a previous ListTasksResult response.
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<&'a str>,
    ///Number of recent messages to include in each task's history. Must be non-negative. Defaults to 0 if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    history_length: Option<i64>,
    ///Filter tasks updated after this timestamp (milliseconds since epoch). Only tasks with a last updated time greater than or equal to this value will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    last_updated_after: Option<i64>,
    ///Whether to include artifacts in the returned tasks. Defaults to false to reduce payload size.
    #[serde(skip_serializing_if = "Option::is_none")]
    include_artifacts: Option<bool>,
    ///Request-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<JsonStrMemKV<'a>>,
}

/// Result object containing the filtered tasks and pagination information.
/// Note on nextPageToken: The nextPageToken field MUST always be present in the response.
/// When there are no more results to retrieve (i.e., this is the final page), the field MUST be set to an empty string ("").
/// Clients should check for an empty string to determine if more pages are available.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTasksResult<'a> {
    /// Array of tasks matching the specified criteria.
    #[serde(borrow)]
    pub tasks: Vec<Task<'a>>,
    /// Total number of tasks available (before pagination).
    pub total_size: i64,
    /// Maximum number of tasks returned in this response.
    pub page_size: i64,
    /// Token for retrieving the next page. Empty string if no more results.
    #[serde(borrow)]
    pub next_page_token: &'a str,
}

/// Defines parameters for fetching a specific push notification configuration for a task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskPushNotificationConfigParams<'a> {
    /// The ID of the push notification configuration to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notification_config_id: Option<&'a str>,
}

/// Defines parameters for listing all push notification configurations associated with a task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct ListTaskPushNotificationConfigParams {
    // id: TaskIdParams, //TODO Extends
}

/// Defines parameters for deleting a specific push notification configuration for a task.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskPushNotificationConfigParams<'a> {
    /// The ID of the push notification configuration to delete.
    pub push_notification_config_id: &'a str,
    // id: TaskIdParams, //TODO Extends
}

/// Represents a successful JSON-RPC response for the `agent/getAuthenticatedExtendedCard` method.
pub struct GetAuthenticatedExtendedCardSuccessResponse<'a> {
    /// The result is an Agent Card object.
    pub result: AgentCard<'a>,
}
