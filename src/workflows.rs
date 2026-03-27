#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceNewResponse {
    pub id: String,

    pub status: InstanceNewResponseStatus,

    pub version_id: String,

    pub workflow_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceListResponse {
    pub id: String,

    pub created_on: DateTime<Utc>,

    pub ended_on: DateTime<Utc>,

    pub modified_on: DateTime<Utc>,

    pub started_on: DateTime<Utc>,

    pub status: InstanceListResponseStatus,

    pub version_id: String,

    pub workflow_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceBulkResponse {
    pub id: String,

    pub status: InstanceBulkResponseStatus,

    pub version_id: String,

    pub workflow_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGetResponse {
    pub end: DateTime<Utc>,

    pub error: InstanceGetResponseError,

    pub output: InstanceGetResponseOutputUnion,

    pub params: String,

    pub queued: DateTime<Utc>,

    pub start: DateTime<Utc>,

    pub status: InstanceGetResponseStatus,

    pub steps: Vec<InstanceGetResponseStep>,

    pub success: bool,

    pub trigger: InstanceGetResponseTrigger,

    #[serde(rename = "versionId")]
    pub version_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_retention: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// `page` and `cursor` are mutually exclusive, use one or the other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Accepts ISO 8601 with no timezone offsets and in UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_end: Option<String>,

    /// Accepts ISO 8601 with no timezone offsets and in UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_start: Option<String>,

    /// should only be used when `cursor` is used, defines a new direction for the
    /// cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// `page` and `cursor` are mutually exclusive, use one or the other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceBulkParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<InstanceBulkParamsBody>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEventNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatusEditResponse {
    pub status: InstanceStatusEditResponseStatus,

    /// Accepts ISO 8601 with no timezone offsets and in UTC.
    pub timestamp: DateTime<Utc>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatusEditParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Apply action to instance.
    pub status: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListResponse {
    pub id: String,

    pub class_name: String,

    pub created_on: DateTime<Utc>,

    pub has_dag: bool,

    pub modified_on: DateTime<Utc>,

    pub workflow_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGetResponse {
    pub id: String,

    pub class_name: String,

    pub created_on: DateTime<Utc>,

    pub has_dag: bool,

    pub modified_on: DateTime<Utc>,

    pub workflow_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowUpdateResponse {
    pub id: String,

    pub class_name: String,

    pub created_on: DateTime<Utc>,

    pub is_deleted: f64,

    pub modified_on: DateTime<Utc>,

    pub name: String,

    pub script_name: String,

    pub terminator_running: f64,

    pub triggered_on: DateTime<Utc>,

    pub version_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListResponse {
    pub id: String,

    pub class_name: String,

    pub created_on: DateTime<Utc>,

    pub instances: WorkflowListResponseInstances,

    pub modified_on: DateTime<Utc>,

    pub name: String,

    pub script_name: String,

    pub triggered_on: DateTime<Utc>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDeleteResponse {
    pub status: WorkflowDeleteResponseStatus,

    pub success: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGetResponse {
    pub id: String,

    pub class_name: String,

    pub created_on: DateTime<Utc>,

    pub instances: WorkflowGetResponseInstances,

    pub modified_on: DateTime<Utc>,

    pub name: String,

    pub script_name: String,

    pub triggered_on: DateTime<Utc>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub class_name: String,

    pub script_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Allows filtering workflows` name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGetResponseError {
    pub message: String,

    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGetResponseStep {
    pub r#type: InstanceGetResponseStepsType,

    /// This field can have the runtime type of
    /// [[]InstanceGetResponseStepsObjectAttempt].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<String>,

    /// This field can have the runtime type of [InstanceGetResponseStepsObjectConfig].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,

    /// This field can have the runtime type of [InstanceGetResponseStepsObjectError].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// This field can have the runtime type of [InstanceGetResponseStepsObjectTrigger].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGetResponseTrigger {
    pub source: InstanceGetResponseTriggerSource,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceBulkParamsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_retention: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListResponseInstances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waitingForPause")]
    pub waiting_for_pause: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGetResponseInstances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waitingForPause")]
    pub waiting_for_pause: Option<f64>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceNewResponseStatus {
    #[serde(rename = "queued")]
    InstanceNewResponseStatusQueued,
    #[serde(rename = "running")]
    InstanceNewResponseStatusRunning,
    #[serde(rename = "paused")]
    InstanceNewResponseStatusPaused,
    #[serde(rename = "errored")]
    InstanceNewResponseStatusErrored,
    #[serde(rename = "terminated")]
    InstanceNewResponseStatusTerminated,
    #[serde(rename = "complete")]
    InstanceNewResponseStatusComplete,
    #[serde(rename = "waitingForPause")]
    InstanceNewResponseStatusWaitingForPause,
    #[serde(rename = "waiting")]
    InstanceNewResponseStatusWaiting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceListResponseStatus {
    #[serde(rename = "queued")]
    InstanceListResponseStatusQueued,
    #[serde(rename = "running")]
    InstanceListResponseStatusRunning,
    #[serde(rename = "paused")]
    InstanceListResponseStatusPaused,
    #[serde(rename = "errored")]
    InstanceListResponseStatusErrored,
    #[serde(rename = "terminated")]
    InstanceListResponseStatusTerminated,
    #[serde(rename = "complete")]
    InstanceListResponseStatusComplete,
    #[serde(rename = "waitingForPause")]
    InstanceListResponseStatusWaitingForPause,
    #[serde(rename = "waiting")]
    InstanceListResponseStatusWaiting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceBulkResponseStatus {
    #[serde(rename = "queued")]
    InstanceBulkResponseStatusQueued,
    #[serde(rename = "running")]
    InstanceBulkResponseStatusRunning,
    #[serde(rename = "paused")]
    InstanceBulkResponseStatusPaused,
    #[serde(rename = "errored")]
    InstanceBulkResponseStatusErrored,
    #[serde(rename = "terminated")]
    InstanceBulkResponseStatusTerminated,
    #[serde(rename = "complete")]
    InstanceBulkResponseStatusComplete,
    #[serde(rename = "waitingForPause")]
    InstanceBulkResponseStatusWaitingForPause,
    #[serde(rename = "waiting")]
    InstanceBulkResponseStatusWaiting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceGetResponseStatus {
    #[serde(rename = "queued")]
    InstanceGetResponseStatusQueued,
    #[serde(rename = "running")]
    InstanceGetResponseStatusRunning,
    #[serde(rename = "paused")]
    InstanceGetResponseStatusPaused,
    #[serde(rename = "errored")]
    InstanceGetResponseStatusErrored,
    #[serde(rename = "terminated")]
    InstanceGetResponseStatusTerminated,
    #[serde(rename = "complete")]
    InstanceGetResponseStatusComplete,
    #[serde(rename = "waitingForPause")]
    InstanceGetResponseStatusWaitingForPause,
    #[serde(rename = "waiting")]
    InstanceGetResponseStatusWaiting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceStatusEditResponseStatus {
    #[serde(rename = "queued")]
    InstanceStatusEditResponseStatusQueued,
    #[serde(rename = "running")]
    InstanceStatusEditResponseStatusRunning,
    #[serde(rename = "paused")]
    InstanceStatusEditResponseStatusPaused,
    #[serde(rename = "errored")]
    InstanceStatusEditResponseStatusErrored,
    #[serde(rename = "terminated")]
    InstanceStatusEditResponseStatusTerminated,
    #[serde(rename = "complete")]
    InstanceStatusEditResponseStatusComplete,
    #[serde(rename = "waitingForPause")]
    InstanceStatusEditResponseStatusWaitingForPause,
    #[serde(rename = "waiting")]
    InstanceStatusEditResponseStatusWaiting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowDeleteResponseStatus {
    #[serde(rename = "ok")]
    WorkflowDeleteResponseStatusOk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceGetResponseStepsType {
    #[serde(rename = "step")]
    InstanceGetResponseStepsTypeStep,
    #[serde(rename = "sleep")]
    InstanceGetResponseStepsTypeSleep,
    #[serde(rename = "termination")]
    InstanceGetResponseStepsTypeTermination,
    #[serde(rename = "waitForEvent")]
    InstanceGetResponseStepsTypeWaitForEvent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstanceGetResponseTriggerSource {
    #[serde(rename = "unknown")]
    InstanceGetResponseTriggerSourceUnknown,
    #[serde(rename = "api")]
    InstanceGetResponseTriggerSourceAPI,
    #[serde(rename = "binding")]
    InstanceGetResponseTriggerSourceBinding,
    #[serde(rename = "event")]
    InstanceGetResponseTriggerSourceEvent,
    #[serde(rename = "cron")]
    InstanceGetResponseTriggerSourceCron,
}

