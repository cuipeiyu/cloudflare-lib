#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGetParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleUpdateParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Lists snippet rules.
    pub rules: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleListParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetUpdateResponse {
    /// Indicates when the snippet was created.
    pub created_on: DateTime<Utc>,

    /// Identify the snippet.
    pub snippet_name: String,

    /// Indicates when the snippet was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetListResponse {
    /// Indicates when the snippet was created.
    pub created_on: DateTime<Utc>,

    /// Identify the snippet.
    pub snippet_name: String,

    /// Indicates when the snippet was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetGetResponse {
    /// Indicates when the snippet was created.
    pub created_on: DateTime<Utc>,

    /// Identify the snippet.
    pub snippet_name: String,

    /// Indicates when the snippet was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetUpdateParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Provide metadata about the snippet.
    pub metadata: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetListParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Specifies the current page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Specifies how many results to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetDeleteParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetGetParams {
    /// Use this field to specify the unique ID of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

