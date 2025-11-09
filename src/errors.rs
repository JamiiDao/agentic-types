/// -32000 to -32099 error codes are reserved for implementation-defined server-errors.
/// A2A-specific errors use this range.
#[derive(Debug, PartialEq, Eq)]
pub enum JsonRpcError {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    TaskNotFoundError,
    TaskNotCancelableError,
    PushNotificationNotSupportedError,
    UnsupportedOperationError,
    ContentTypeNotSupportedError,
    InvalidAgentResponseError,
    AuthenticatedExtendedCardNotConfiguredError,
    UnknownErrorEncountered,
}

impl JsonRpcError {
    pub fn error_code(&self) -> i64 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::TaskNotFoundError => -32001,
            Self::TaskNotCancelableError => -32002,
            Self::PushNotificationNotSupportedError => -32003,
            Self::UnsupportedOperationError => -32004,
            Self::ContentTypeNotSupportedError => -32005,
            Self::InvalidAgentResponseError => -32006,
            Self::AuthenticatedExtendedCardNotConfiguredError => -32007,
            Self::UnknownErrorEncountered => -32000,
        }
    }

    pub fn foo(value: i64) -> Self {
        (value).into()
    }

    pub fn description(&self) -> &str {
        match self {
            Self::ParseError => "Server received JSON that was not well-formed.",
            Self::InvalidRequest => {
                "The JSON payload was valid JSON, but not a valid JSON-RPC Request object."
            }
            Self::MethodNotFound => "The requested A2A RPC method (e.g., `tasks/foo`) does not exist or is not supported.",
            Self::InvalidParams => "The params provided for the method are invalid (e.g., wrong type, missing required field).",
            Self::InternalError => "An unexpected error occurred on the server during processing.",
            Self::TaskNotFoundError => "The specified task id does not correspond to an existing or active task. It might be invalid, expired, or already completed and purged.",
            Self::TaskNotCancelableError => "An attempt was made to cancel a task that is not in a cancelable state (e.g., it has already reached a terminal state like completed, failed, or canceled).",
            Self::PushNotificationNotSupportedError => "Client attempted to use push notification features (e.g., tasks/pushNotificationConfig/set) but the server agent does not support them (i.e., AgentCard.capabilities.pushNotifications is false).",
            Self::UnsupportedOperationError => "The requested operation or a specific aspect of it (perhaps implied by parameters) is not supported by this server agent implementation. Broader than just method not found.",
            Self::ContentTypeNotSupportedError => "A Media Type provided in the request's message.parts (or implied for an artifact) is not supported by the agent or the specific skill being invoked.",
            Self::InvalidAgentResponseError =>  "Agent generated an invalid response for the requested method",
            Self::AuthenticatedExtendedCardNotConfiguredError => "The agent does not have an Authenticated Extended Card configured.",
            Self::UnknownErrorEncountered => "An unknown error was parsed. If this error is valid then open an issue on the repository"
        }
    }
}

impl From<i64> for JsonRpcError {
    fn from(value: i64) -> Self {
        match value {
            val if val == Self::ParseError.error_code() => Self::ParseError,
            val if val == Self::InvalidRequest.error_code() => Self::InvalidRequest,
            val if val == Self::MethodNotFound.error_code() => Self::MethodNotFound,
            val if val == Self::InvalidParams.error_code() => Self::InvalidParams,
            val if val == Self::InternalError.error_code() => Self::InternalError,
            val if val == Self::TaskNotFoundError.error_code() => Self::TaskNotFoundError,
            val if val == Self::TaskNotCancelableError.error_code() => Self::TaskNotCancelableError,
            val if val == Self::PushNotificationNotSupportedError.error_code() => {
                Self::PushNotificationNotSupportedError
            }
            val if val == Self::UnsupportedOperationError.error_code() => {
                Self::UnsupportedOperationError
            }
            val if val == Self::ContentTypeNotSupportedError.error_code() => {
                Self::ContentTypeNotSupportedError
            }
            val if val == Self::InvalidAgentResponseError.error_code() => {
                Self::InvalidAgentResponseError
            }
            val if val == Self::AuthenticatedExtendedCardNotConfiguredError.error_code() => {
                Self::AuthenticatedExtendedCardNotConfiguredError
            }
            _ => Self::UnknownErrorEncountered,
        }
    }
}
