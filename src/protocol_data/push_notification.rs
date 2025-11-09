use serde::{Deserialize, Serialize};

/// Used as the params object for the tasks/pushNotificationConfig/set method
/// and as the result object for the tasks/pushNotificationConfig/get method.
/// A container associating a push notification configuration with a specific task.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct TaskPushNotificationConfig<'a> {
    /// The unique identifier (e.g. UUID) of the task.
    task_id: &'a str,
    /** The push notification configuration for this task. */
    push_notification_config: PushNotificationConfig<'a>,
}

/// Configuration provided by the client to the server for sending asynchronous push notifications about task updates.
/// Defines the configuration for setting up push notifications for task updates.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct PushNotificationConfig<'a> {
    /// A unique identifier (e.g. UUID) for the push notification configuration, set by the client
    /// to support multiple notification callbacks.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    /// The callback URL where the agent should send push notifications.
    url: &'a str,
    ///  A unique token for this task or session to validate incoming push notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,
    /// Optional authentication details for the agent to use when calling the notification URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<PushNotificationAuthenticationInfo<'a>>,
}

/// A generic structure for specifying authentication requirements,
/// typically used within PushNotificationConfig to describe how the A2A Server should authenticate to the client's webhook.
/// Defines authentication details for a push notification endpoint.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct PushNotificationAuthenticationInfo<'a> {
    /// A list of supported authentication schemes (e.g., 'Basic', 'Bearer').
    schemes: Vec<&'a str>,
    /// Optional credentials required by the push notification endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<&'a str>,
}
