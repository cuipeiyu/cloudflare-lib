#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumer {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Name of the dead letter queue, or empty string if not configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_queue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// Name of a Worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// This field can have the runtime type of
    /// [ConsumerMqWorkerConsumerResponseSettings],
    /// [ConsumerMqHTTPConsumerResponseSettings].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConsumerType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::shared::ResponseInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    /// Indicates if the API call was successful or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<ConsumerDeleteResponseSuccess>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerNewParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Request body for creating or updating a consumer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ConsumerNewParamsBodyUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerUpdateParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Request body for creating or updating a consumer
    pub body: ConsumerUpdateParamsBodyUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerListParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerDeleteParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerGetParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAckResponse {
    /// The number of messages that were succesfully acknowledged.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ackCount")]
    pub ack_count: Option<f64>,

    /// The number of messages that were succesfully retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retryCount")]
    pub retry_count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBulkPushResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::shared::ResponseInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    /// Indicates if the API call was successful or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<MessageBulkPushResponseSuccess>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePullResponse {
    /// The number of unacknowledged messages in the queue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_backlog_count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessagePullResponseMessage>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePushResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::shared::ResponseInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    /// Indicates if the API call was successful or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<MessagePushResponseSuccess>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAckParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acks: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBulkPushParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The number of seconds to wait for attempting to deliver this batch to consumers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePullParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The maximum number of messages to include in a batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<String>,

    /// The number of milliseconds that a message is exclusively leased. After the
    /// timeout, the message becomes available for another attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout_ms: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePushParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<MessagePushParamsBodyUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeStatusResponse {
    /// Indicates if the last purge operation completed successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,

    /// Timestamp when the last purge operation started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeStartParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Confimation that all messages will be deleted permanently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_messages_permanently: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeStatusParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<Consumer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers_total_count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers: Option<Vec<QueueProducer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers_total_count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<QueueSettings>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::shared::ResponseInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,

    /// Indicates if the API call was successful or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<QueueDeleteResponseSuccess>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueNewParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub queue_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueUpdateParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueParam>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueListParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueDeleteParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEditParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueParam>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueGetParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewResponse {
    /// Unique identifier for the subscription
    pub id: String,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// Destination configuration for the subscription
    pub destination: SubscriptionNewResponseDestination,

    /// Whether the subscription is active
    pub enabled: bool,

    /// List of event types this subscription handles
    pub events: Vec<String>,

    /// When the subscription was last modified
    pub modified_at: DateTime<Utc>,

    /// Name of the subscription
    pub name: String,

    /// Source configuration for the subscription
    pub source: SubscriptionNewResponseSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateResponse {
    /// Unique identifier for the subscription
    pub id: String,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// Destination configuration for the subscription
    pub destination: SubscriptionUpdateResponseDestination,

    /// Whether the subscription is active
    pub enabled: bool,

    /// List of event types this subscription handles
    pub events: Vec<String>,

    /// When the subscription was last modified
    pub modified_at: DateTime<Utc>,

    /// Name of the subscription
    pub name: String,

    /// Source configuration for the subscription
    pub source: SubscriptionUpdateResponseSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionListResponse {
    /// Unique identifier for the subscription
    pub id: String,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// Destination configuration for the subscription
    pub destination: SubscriptionListResponseDestination,

    /// Whether the subscription is active
    pub enabled: bool,

    /// List of event types this subscription handles
    pub events: Vec<String>,

    /// When the subscription was last modified
    pub modified_at: DateTime<Utc>,

    /// Name of the subscription
    pub name: String,

    /// Source configuration for the subscription
    pub source: SubscriptionListResponseSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteResponse {
    /// Unique identifier for the subscription
    pub id: String,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// Destination configuration for the subscription
    pub destination: SubscriptionDeleteResponseDestination,

    /// Whether the subscription is active
    pub enabled: bool,

    /// List of event types this subscription handles
    pub events: Vec<String>,

    /// When the subscription was last modified
    pub modified_at: DateTime<Utc>,

    /// Name of the subscription
    pub name: String,

    /// Source configuration for the subscription
    pub source: SubscriptionDeleteResponseSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetResponse {
    /// Unique identifier for the subscription
    pub id: String,

    /// When the subscription was created
    pub created_at: DateTime<Utc>,

    /// Destination configuration for the subscription
    pub destination: SubscriptionGetResponseDestination,

    /// Whether the subscription is active
    pub enabled: bool,

    /// List of event types this subscription handles
    pub events: Vec<String>,

    /// When the subscription was last modified
    pub modified_at: DateTime<Utc>,

    /// Name of the subscription
    pub name: String,

    /// Source configuration for the subscription
    pub source: SubscriptionGetResponseSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Destination configuration for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    /// Whether the subscription is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// List of event types this subscription handles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<String>,

    /// Name of the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Source configuration for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Destination configuration for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    /// Whether the subscription is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// List of event types this subscription handles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<String>,

    /// Name of the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionListParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Field to sort by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetParams {
    /// A Resource identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePullResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// An ID that represents an "in-flight" message that has been pulled from a Queue.
    /// You must hold on to this ID and use it to acknowledge this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_ms: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueProducer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<QueueProducersType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueSettings {
    /// Number of seconds to delay delivery of all messages to consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_delay: Option<f64>,

    /// Indicates if message delivery to consumers is currently paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_paused: Option<bool>,

    /// Number of seconds after which an unconsumed message will be delayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_period: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewResponseDestination {
    /// ID of the target queue
    pub queue_id: String,

    /// Type of destination
    pub r#type: SubscriptionNewResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewResponseSource {
    /// Name of the Workers AI model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Type of source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionNewResponseSourceType>,

    /// Name of the worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// Name of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateResponseDestination {
    /// ID of the target queue
    pub queue_id: String,

    /// Type of destination
    pub r#type: SubscriptionUpdateResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateResponseSource {
    /// Name of the Workers AI model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Type of source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionUpdateResponseSourceType>,

    /// Name of the worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// Name of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionListResponseDestination {
    /// ID of the target queue
    pub queue_id: String,

    /// Type of destination
    pub r#type: SubscriptionListResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionListResponseSource {
    /// Name of the Workers AI model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Type of source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionListResponseSourceType>,

    /// Name of the worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// Name of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteResponseDestination {
    /// ID of the target queue
    pub queue_id: String,

    /// Type of destination
    pub r#type: SubscriptionDeleteResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteResponseSource {
    /// Name of the Workers AI model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Type of source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionDeleteResponseSourceType>,

    /// Name of the worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// Name of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetResponseDestination {
    /// ID of the target queue
    pub queue_id: String,

    /// Type of destination
    pub r#type: SubscriptionGetResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetResponseSource {
    /// Name of the Workers AI model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    /// Type of source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionGetResponseSourceType>,

    /// Name of the worker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// Name of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsumerType {
    #[serde(rename = "worker")]
    ConsumerTypeWorker,
    #[serde(rename = "http_pull")]
    ConsumerTypeHTTPPull,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueueProducersType {
    #[serde(rename = "worker")]
    QueueProducersTypeWorker,
    #[serde(rename = "r2_bucket")]
    QueueProducersTypeR2Bucket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionNewResponseDestinationType {
    #[serde(rename = "queues.queue")]
    SubscriptionNewResponseDestinationTypeQueuesQueue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionNewResponseSourceType {
    #[serde(rename = "images")]
    SubscriptionNewResponseSourceTypeImages,
    #[serde(rename = "kv")]
    SubscriptionNewResponseSourceTypeKV,
    #[serde(rename = "r2")]
    SubscriptionNewResponseSourceTypeR2,
    #[serde(rename = "superSlurper")]
    SubscriptionNewResponseSourceTypeSuperSlurper,
    #[serde(rename = "vectorize")]
    SubscriptionNewResponseSourceTypeVectorize,
    #[serde(rename = "workersAi.model")]
    SubscriptionNewResponseSourceTypeWorkersAIModel,
    #[serde(rename = "workersBuilds.worker")]
    SubscriptionNewResponseSourceTypeWorkersBuildsWorker,
    #[serde(rename = "workflows.workflow")]
    SubscriptionNewResponseSourceTypeWorkflowsWorkflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionUpdateResponseDestinationType {
    #[serde(rename = "queues.queue")]
    SubscriptionUpdateResponseDestinationTypeQueuesQueue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionUpdateResponseSourceType {
    #[serde(rename = "images")]
    SubscriptionUpdateResponseSourceTypeImages,
    #[serde(rename = "kv")]
    SubscriptionUpdateResponseSourceTypeKV,
    #[serde(rename = "r2")]
    SubscriptionUpdateResponseSourceTypeR2,
    #[serde(rename = "superSlurper")]
    SubscriptionUpdateResponseSourceTypeSuperSlurper,
    #[serde(rename = "vectorize")]
    SubscriptionUpdateResponseSourceTypeVectorize,
    #[serde(rename = "workersAi.model")]
    SubscriptionUpdateResponseSourceTypeWorkersAIModel,
    #[serde(rename = "workersBuilds.worker")]
    SubscriptionUpdateResponseSourceTypeWorkersBuildsWorker,
    #[serde(rename = "workflows.workflow")]
    SubscriptionUpdateResponseSourceTypeWorkflowsWorkflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionListResponseDestinationType {
    #[serde(rename = "queues.queue")]
    SubscriptionListResponseDestinationTypeQueuesQueue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionListResponseSourceType {
    #[serde(rename = "images")]
    SubscriptionListResponseSourceTypeImages,
    #[serde(rename = "kv")]
    SubscriptionListResponseSourceTypeKV,
    #[serde(rename = "r2")]
    SubscriptionListResponseSourceTypeR2,
    #[serde(rename = "superSlurper")]
    SubscriptionListResponseSourceTypeSuperSlurper,
    #[serde(rename = "vectorize")]
    SubscriptionListResponseSourceTypeVectorize,
    #[serde(rename = "workersAi.model")]
    SubscriptionListResponseSourceTypeWorkersAIModel,
    #[serde(rename = "workersBuilds.worker")]
    SubscriptionListResponseSourceTypeWorkersBuildsWorker,
    #[serde(rename = "workflows.workflow")]
    SubscriptionListResponseSourceTypeWorkflowsWorkflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionDeleteResponseDestinationType {
    #[serde(rename = "queues.queue")]
    SubscriptionDeleteResponseDestinationTypeQueuesQueue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionDeleteResponseSourceType {
    #[serde(rename = "images")]
    SubscriptionDeleteResponseSourceTypeImages,
    #[serde(rename = "kv")]
    SubscriptionDeleteResponseSourceTypeKV,
    #[serde(rename = "r2")]
    SubscriptionDeleteResponseSourceTypeR2,
    #[serde(rename = "superSlurper")]
    SubscriptionDeleteResponseSourceTypeSuperSlurper,
    #[serde(rename = "vectorize")]
    SubscriptionDeleteResponseSourceTypeVectorize,
    #[serde(rename = "workersAi.model")]
    SubscriptionDeleteResponseSourceTypeWorkersAIModel,
    #[serde(rename = "workersBuilds.worker")]
    SubscriptionDeleteResponseSourceTypeWorkersBuildsWorker,
    #[serde(rename = "workflows.workflow")]
    SubscriptionDeleteResponseSourceTypeWorkflowsWorkflow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionGetResponseDestinationType {
    #[serde(rename = "queues.queue")]
    SubscriptionGetResponseDestinationTypeQueuesQueue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionGetResponseSourceType {
    #[serde(rename = "images")]
    SubscriptionGetResponseSourceTypeImages,
    #[serde(rename = "kv")]
    SubscriptionGetResponseSourceTypeKV,
    #[serde(rename = "r2")]
    SubscriptionGetResponseSourceTypeR2,
    #[serde(rename = "superSlurper")]
    SubscriptionGetResponseSourceTypeSuperSlurper,
    #[serde(rename = "vectorize")]
    SubscriptionGetResponseSourceTypeVectorize,
    #[serde(rename = "workersAi.model")]
    SubscriptionGetResponseSourceTypeWorkersAIModel,
    #[serde(rename = "workersBuilds.worker")]
    SubscriptionGetResponseSourceTypeWorkersBuildsWorker,
    #[serde(rename = "workflows.workflow")]
    SubscriptionGetResponseSourceTypeWorkflowsWorkflow,
}

