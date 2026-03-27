#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableAlertListResponseItem {
	/// Describes the alert type.
	pub description: Option<String>,
	/// Alert type name.
	pub display_name: Option<String>,
	/// Format of additional configuration options (filters) for the alert type. Data
	/// type of filters during policy creation: Array of strings.
	pub filter_options: Option<Vec<String>>,
	/// Use this value when creating and updating a notification policy.
	pub r#type: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
pub type AvailableAlertListResponse = ::std::collections::HashMap<String, Vec<AvailableAlertListResponseItem>>;

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationEligibleListResponseItem {
	/// Determines whether or not the account is eligible for the delivery mechanism.
	pub eligible: Option<bool>,
	/// Beta flag. Users can create a policy with a mechanism that is not ready, but we
	/// cannot guarantee successful delivery of notifications.
	pub ready: Option<bool>,
	/// Determines type of delivery mechanism.
	pub r#type: Option<DestinationEligibleListResponseItemType>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
pub type DestinationEligibleListResponse = ::std::collections::HashMap<String, Vec<DestinationEligibleListResponseItem>>;

/// Determines type of delivery mechanism.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DestinationEligibleListResponseItemType {
    /// value is "email"
	#[serde(rename = "email")]
	Email,
    /// value is "pagerduty"
	#[serde(rename = "pagerduty")]
	Pagerduty,
    /// value is "webhook"
	#[serde(rename = "webhook")]
	Webhook,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableAlertListParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationEligibleGetParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagerduty {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the pagerduty service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyNewResponse {
    /// token in form of UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyDeleteResponse {
    pub errors: Vec<DestinationPagerdutyDeleteResponseError>,

    pub messages: Vec<DestinationPagerdutyDeleteResponseMessage>,

    /// Whether the API call was successful
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyLinkResponse {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyNewParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyDeleteParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyGetParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyLinkParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhooks {
    /// The unique identifier of a webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Timestamp of when the webhook destination was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Timestamp of the last time an attempt to dispatch a notification to this webhook
    /// failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure: Option<DateTime<Utc>>,

    /// Timestamp of the last time Cloudflare was able to successfully dispatch a
    /// notification using this webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_success: Option<DateTime<Utc>>,

    /// The name of the webhook destination. This will be included in the request body
    /// when you receive a webhook notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Type of webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WebhooksType>,

    /// The POST endpoint to call when dispatching a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookNewResponse {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookUpdateResponse {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookDeleteResponse {
    pub errors: Vec<DestinationWebhookDeleteResponseError>,

    pub messages: Vec<DestinationWebhookDeleteResponseMessage>,

    /// Whether the API call was successful
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookNewBody {
    /// The name of the webhook destination. This will be included in the request body
    /// when you receive a webhook notification.
    pub name: String,

    /// The POST endpoint to call when dispatching a notification.
    pub url: String,

    /// Optional secret that will be passed in the `cf-webhook-auth` header when
    /// dispatching generic webhook notifications or formatted for supported
    /// destinations. Secrets are not returned in any API response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookUpdateBody {
    /// The name of the webhook destination. This will be included in the request body
    /// when you receive a webhook notification.
    pub name: String,

    /// The POST endpoint to call when dispatching a notification.
    pub url: String,

    /// Optional secret that will be passed in the `cf-webhook-auth` header when
    /// dispatching generic webhook notifications or formatted for supported
    /// destinations. Secrets are not returned in any API response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookListParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookDeleteParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookGetParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Message body included in the notification sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_body: Option<String>,

    /// Type of notification that has been dispatched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,

    /// Description of the notification policy (if present).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The mechanism to which the notification has been dispatched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<String>,

    /// The type of mechanism to which the notification has been dispatched. This can be
    /// email/pagerduty/webhook based on the mechanism configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanism_type: Option<HistoryMechanismType>,

    /// Name of the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// Timestamp of when the notification was dispatched in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListQuery {
    /// Limit the returned results to history records older than the specified date.
    /// This must be a timestamp that conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<Utc>>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,

    /// Limit the returned results to history records newer than the specified date.
    /// This must be a timestamp that conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mechanism {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<MechanismEmail>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagerduty: Option<Vec<MechanismPagerduty>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<MechanismWebhook>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanismParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagerduty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional specification of how often to re-alert from the same incident, not
    /// support on all alert types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_interval: Option<String>,

    /// Refers to which event will trigger a Notification dispatch. You can use the
    /// endpoint to get available alert types which then will give you a list of
    /// possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<PolicyAlertType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Optional description for the Notification policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether or not the Notification policy is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Optional filters that allow you to be alerted only on a subset of events for
    /// that alert type based on some criteria. This is only available for select alert
    /// types. See alert type documentation for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PolicyFilter>,

    /// List of IDs that will be used when dispatching a notification. IDs for email
    /// type will be the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanisms: Option<Mechanism>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// Name of the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFilter {
    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,

    /// Used for configuring radar_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "affected_asns")]
    pub affected_as_ns: Option<Vec<String>>,

    /// Used for configuring incident_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_components: Option<Vec<String>>,

    /// Used for configuring radar_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_locations: Option<Vec<String>>,

    /// Used for configuring maintenance_event_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airport_code: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_trigger_preferences: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_trigger_preferences_value: Option<Vec<String>>,

    /// Used for configuring load_balancing_pool_enablement_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Vec<String>>,

    /// Used for configuring load_balancing_health_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,

    /// Used for configuring health_check_status_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<Vec<String>>,

    /// Used for configuring incident_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_impact: Option<Vec<PolicyFilterIncidentImpact>>,

    /// Used for configuring stream_live_notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<Vec<String>>,

    /// Used for configuring security_insights_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_class: Option<Vec<String>>,

    /// Used for configuring billing_usage_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Vec<String>>,

    /// Used for configuring logo_match_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_tag: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megabits_per_second: Option<Vec<String>>,

    /// Used for configuring load_balancing_health_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_health: Option<Vec<String>>,

    /// Used for configuring tunnel_health_event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_per_second: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pop_names: Option<Vec<String>>,

    /// Used for configuring billing_usage_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Vec<String>>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_tag: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<String>>,

    /// Used for configuring clickhouse_alert_fw_ent_anomaly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<Vec<String>>,

    /// Used for configuring health_check_status_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hostname: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ip: Option<Vec<String>>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_name: Option<Vec<String>>,

    /// Used for configuring traffic_anomalies_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_exclusions: Option<Vec<PolicyFilterTrafficExclusion>>,

    /// Used for configuring tunnel_health_event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_name: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Vec<String>>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFilterParam {
    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,

    /// Used for configuring radar_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "affected_asns")]
    pub affected_as_ns: Option<String>,

    /// Used for configuring incident_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_components: Option<String>,

    /// Used for configuring radar_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_locations: Option<String>,

    /// Used for configuring maintenance_event_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub airport_code: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_trigger_preferences: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_trigger_preferences_value: Option<String>,

    /// Used for configuring load_balancing_pool_enablement_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// Used for configuring load_balancing_health_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,

    /// Used for configuring health_check_status_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<String>,

    /// Used for configuring incident_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_impact: Option<String>,

    /// Used for configuring stream_live_notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,

    /// Used for configuring security_insights_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_class: Option<String>,

    /// Used for configuring billing_usage_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Used for configuring logo_match_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_tag: Option<String>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megabits_per_second: Option<String>,

    /// Used for configuring load_balancing_health_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_health: Option<String>,

    /// Used for configuring tunnel_health_event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<String>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_per_second: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pop_names: Option<String>,

    /// Used for configuring billing_usage_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Used for configuring pages_event_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_tag: Option<String>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<String>,

    /// Used for configuring clickhouse_alert_fw_ent_anomaly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<String>,

    /// Used for configuring health_check_status_notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hostname: Option<String>,

    /// Used for configuring advanced_ddos_attack_l4_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ip: Option<String>,

    /// Used for configuring advanced_ddos_attack_l7_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_name: Option<String>,

    /// Used for configuring traffic_anomalies_alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_exclusions: Option<String>,

    /// Used for configuring tunnel_health_event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_name: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<String>,

    /// Usage depends on specific alert type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyNewResponse {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUpdateResponse {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDeleteResponse {
    pub errors: Vec<PolicyDeleteResponseError>,

    pub messages: Vec<PolicyDeleteResponseMessage>,

    /// Whether the API call was successful
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<PolicyDeleteResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyNewBody {
    /// Refers to which event will trigger a Notification dispatch. You can use the
    /// endpoint to get available alert types which then will give you a list of
    /// possible values.
    pub alert_type: PolicyNewParamsAlertType,

    /// Whether or not the Notification policy is enabled.
    pub enabled: bool,

    /// List of IDs that will be used when dispatching a notification. IDs for email
    /// type will be the email address.
    pub mechanisms: MechanismParam,

    /// Name of the policy.
    pub name: String,

    /// Optional specification of how often to re-alert from the same incident, not
    /// support on all alert types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_interval: Option<String>,

    /// Optional description for the Notification policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional filters that allow you to be alerted only on a subset of events for
    /// that alert type based on some criteria. This is only available for select alert
    /// types. See alert type documentation for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PolicyFilterParam>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUpdateBody {
    /// Optional specification of how often to re-alert from the same incident, not
    /// support on all alert types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_interval: Option<String>,

    /// Refers to which event will trigger a Notification dispatch. You can use the
    /// endpoint to get available alert types which then will give you a list of
    /// possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<PolicyUpdateParamsAlertType>,

    /// Optional description for the Notification policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether or not the Notification policy is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Optional filters that allow you to be alerted only on a subset of events for
    /// that alert type based on some criteria. This is only available for select alert
    /// types. See alert type documentation for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PolicyFilterParam>,

    /// List of IDs that will be used when dispatching a notification. IDs for email
    /// type will be the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanisms: Option<MechanismParam>,

    /// Name of the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Refers to which event will trigger a Notification dispatch. You can use the
/// endpoint to get available alert types which then will give you a list of
/// possible values.
#[cfg(any(feature = "full", feature = "with-alerting"))]
pub type PolicyUpdateParamsAlertType = PolicyAlertType;

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyListParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDeleteParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyGetParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceNewResponse {
    pub errors: Vec<SilenceNewResponseError>,

    pub messages: Vec<SilenceNewResponseMessage>,

    /// Whether the API call was successful
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceUpdateResponse {
    /// Silence ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When the silence was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the silence ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// When the silence starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// When the silence was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceListResponse {
    /// Silence ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When the silence was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// When the silence ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// When the silence starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// When the silence was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceDeleteResponse {
    pub errors: Vec<SilenceDeleteResponseError>,

    pub messages: Vec<SilenceDeleteResponseMessage>,

    /// Whether the API call was successful
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceGetResponse {
    /// Silence ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When the silence was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// When the silence ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// When the silence starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// When the silence was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceNewParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: Vec<SilenceNewParamsBody>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceUpdateParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: Vec<SilenceUpdateParamsBody>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceListParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceDeleteParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceGetParams {
    /// The account id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyDeleteResponseError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationPagerdutyDeleteResponseMessage {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookDeleteResponseError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWebhookDeleteResponseMessage {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanismEmail {
    /// The email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanismPagerduty {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanismWebhook {
    /// UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDeleteResponseError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDeleteResponseMessage {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDeleteResponseResultInfo {
    /// Total number of results for the requested service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

    /// Current page within paginated list of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,

    /// Number of results per page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<f64>,

    /// Total results available without any search parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceNewResponseError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceNewResponseMessage {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceDeleteResponseError {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceDeleteResponseMessage {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceNewParamsBody {
    /// When the silence ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    /// The unique identifier of a notification policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    /// When the silence starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceUpdateParamsBody {
    /// Silence ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When the silence ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    /// When the silence starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebhooksType {
    #[serde(rename = "datadog")]
    Datadog,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "feishu")]
    Feishu,
    #[serde(rename = "gchat")]
    Gchat,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "opsgenie")]
    Opsgenie,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "splunk")]
    Splunk,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HistoryMechanismType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "pagerduty")]
    Pagerduty,
    #[serde(rename = "webhook")]
    Webhook,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyAlertType {
    #[serde(rename = "abuse_report_alert")]
    AbuseReportAlert,
    #[serde(rename = "access_custom_certificate_expiration_type")]
    AccessCustomCertificateExpirationType,
    #[serde(rename = "advanced_ddos_attack_l4_alert")]
    AdvancedDDoSAttackL4Alert,
    #[serde(rename = "advanced_ddos_attack_l7_alert")]
    AdvancedDDoSAttackL7Alert,
    #[serde(rename = "advanced_http_alert_error")]
    AdvancedHTTPAlertError,
    #[serde(rename = "bgp_hijack_notification")]
    BGPHijackNotification,
    #[serde(rename = "billing_usage_alert")]
    BillingUsageAlert,
    #[serde(rename = "block_notification_block_removed")]
    BlockNotificationBlockRemoved,
    #[serde(rename = "block_notification_new_block")]
    BlockNotificationNewBlock,
    #[serde(rename = "block_notification_review_rejected")]
    BlockNotificationReviewRejected,
    #[serde(rename = "bot_traffic_basic_alert")]
    BotTrafficBasicAlert,
    #[serde(rename = "brand_protection_alert")]
    BrandProtectionAlert,
    #[serde(rename = "brand_protection_digest")]
    BrandProtectionDigest,
    #[serde(rename = "clickhouse_alert_fw_anomaly")]
    ClickhouseAlertFwAnomaly,
    #[serde(rename = "clickhouse_alert_fw_ent_anomaly")]
    ClickhouseAlertFwEntAnomaly,
    #[serde(rename = "cloudforce_one_request_notification")]
    CloudforceOneRequestNotification,
    #[serde(rename = "custom_analytics")]
    CustomAnalytics,
    #[serde(rename = "custom_bot_detection_alert")]
    CustomBotDetectionAlert,
    #[serde(rename = "custom_ssl_certificate_event_type")]
    CustomSSLCertificateEventType,
    #[serde(rename = "dedicated_ssl_certificate_event_type")]
    DedicatedSSLCertificateEventType,
    #[serde(rename = "device_connectivity_anomaly_alert")]
    DeviceConnectivityAnomalyAlert,
    #[serde(rename = "dos_attack_l4")]
    DosAttackL4,
    #[serde(rename = "dos_attack_l7")]
    DosAttackL7,
    #[serde(rename = "expiring_service_token_alert")]
    ExpiringServiceTokenAlert,
    #[serde(rename = "failing_logpush_job_disabled_alert")]
    FailingLogpushJobDisabledAlert,
    #[serde(rename = "fbm_auto_advertisement")]
    FbmAutoAdvertisement,
    #[serde(rename = "fbm_dosd_attack")]
    FbmDosdAttack,
    #[serde(rename = "fbm_volumetric_attack")]
    FbmVolumetricAttack,
    #[serde(rename = "health_check_status_notification")]
    HealthCheckStatusNotification,
    #[serde(rename = "hostname_aop_custom_certificate_expiration_type")]
    HostnameAopCustomCertificateExpirationType,
    #[serde(rename = "http_alert_edge_error")]
    HTTPAlertEdgeError,
    #[serde(rename = "http_alert_origin_error")]
    HTTPAlertOriginError,
    #[serde(rename = "image_notification")]
    ImageNotification,
    #[serde(rename = "image_resizing_notification")]
    ImageResizingNotification,
    #[serde(rename = "incident_alert")]
    IncidentAlert,
    #[serde(rename = "load_balancing_health_alert")]
    LoadBalancingHealthAlert,
    #[serde(rename = "load_balancing_pool_enablement_alert")]
    LoadBalancingPoolEnablementAlert,
    #[serde(rename = "logo_match_alert")]
    LogoMatchAlert,
    #[serde(rename = "magic_tunnel_health_check_event")]
    MagicTunnelHealthCheckEvent,
    #[serde(rename = "magic_wan_tunnel_health")]
    MagicWANTunnelHealth,
    #[serde(rename = "maintenance_event_notification")]
    MaintenanceEventNotification,
    #[serde(rename = "mtls_certificate_store_certificate_expiration_type")]
    MTLSCertificateStoreCertificateExpirationType,
    #[serde(rename = "pages_event_alert")]
    PagesEventAlert,
    #[serde(rename = "radar_notification")]
    RadarNotification,
    #[serde(rename = "real_origin_monitoring")]
    RealOriginMonitoring,
    #[serde(rename = "scriptmonitor_alert_new_code_change_detections")]
    ScriptmonitorAlertNewCodeChangeDetections,
    #[serde(rename = "scriptmonitor_alert_new_hosts")]
    ScriptmonitorAlertNewHosts,
    #[serde(rename = "scriptmonitor_alert_new_malicious_hosts")]
    ScriptmonitorAlertNewMaliciousHosts,
    #[serde(rename = "scriptmonitor_alert_new_malicious_scripts")]
    ScriptmonitorAlertNewMaliciousScripts,
    #[serde(rename = "scriptmonitor_alert_new_malicious_url")]
    ScriptmonitorAlertNewMaliciousURL,
    #[serde(rename = "scriptmonitor_alert_new_max_length_resource_url")]
    ScriptmonitorAlertNewMaxLengthResourceURL,
    #[serde(rename = "scriptmonitor_alert_new_resources")]
    ScriptmonitorAlertNewResources,
    #[serde(rename = "secondary_dns_all_primaries_failing")]
    SecondaryDNSAllPrimariesFailing,
    #[serde(rename = "secondary_dns_primaries_failing")]
    SecondaryDNSPrimariesFailing,
    #[serde(rename = "secondary_dns_warning")]
    SecondaryDNSWarning,
    #[serde(rename = "secondary_dns_zone_successfully_updated")]
    SecondaryDNSZoneSuccessfullyUpdated,
    #[serde(rename = "secondary_dns_zone_validation_warning")]
    SecondaryDNSZoneValidationWarning,
    #[serde(rename = "security_insights_alert")]
    SecurityInsightsAlert,
    #[serde(rename = "sentinel_alert")]
    SentinelAlert,
    #[serde(rename = "stream_live_notifications")]
    StreamLiveNotifications,
    #[serde(rename = "synthetic_test_latency_alert")]
    SyntheticTestLatencyAlert,
    #[serde(rename = "synthetic_test_low_availability_alert")]
    SyntheticTestLowAvailabilityAlert,
    #[serde(rename = "traffic_anomalies_alert")]
    TrafficAnomaliesAlert,
    #[serde(rename = "tunnel_health_event")]
    TunnelHealthEvent,
    #[serde(rename = "tunnel_update_event")]
    TunnelUpdateEvent,
    #[serde(rename = "universal_ssl_event_type")]
    UniversalSSLEventType,
    #[serde(rename = "web_analytics_metrics_update")]
    WebAnalyticsMetricsUpdate,
    #[serde(rename = "zone_aop_custom_certificate_expiration_type")]
    ZoneAopCustomCertificateExpirationType,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
pub type PolicyNewParamsAlertType = PolicyAlertType;

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyFilterIncidentImpact {
    #[serde(rename = "INCIDENT_IMPACT_NONE")]
    IncidentImpactNone,
    #[serde(rename = "INCIDENT_IMPACT_MINOR")]
    IncidentImpactMinor,
    #[serde(rename = "INCIDENT_IMPACT_MAJOR")]
    IncidentImpactMajor,
    #[serde(rename = "INCIDENT_IMPACT_CRITICAL")]
    IncidentImpactCritical,
}

#[cfg(any(feature = "full", feature = "with-alerting"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyFilterTrafficExclusion {
    #[serde(rename = "security_events")]
    SecurityEvents,
}
