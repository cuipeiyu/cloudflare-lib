#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRunParams<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,


    // Satisfied by [ai.AIRunParamsBodyTextClassification],
    // [ai.AIRunParamsBodyTextToImage], [ai.AIRunParamsBodyTextToSpeech],
    // [ai.AIRunParamsBodyTextEmbeddings],
    // [ai.AIRunParamsBodyAutomaticSpeechRecognition],
    // [ai.AIRunParamsBodyImageClassification], [ai.AIRunParamsBodyObjectDetection],
    // [ai.AIRunParamsBodyPrompt], [ai.AIRunParamsBodyTextGeneration],
    // [ai.AIRunParamsBodyTranslation], [ai.AIRunParamsBodySummarization],
    // [ai.AIRunParamsBodyImageToText], [ai.AIRunParamsBodyObject],
    // [ai.AIRunParamsBodyImageTextToText], [ai.AIRunParamsBodyMultimodalEmbeddings],
    // [AIRunParamsBody].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneNewResponse {
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub model: String,

    pub modified_at: DateTime<Utc>,

    pub name: String,

    pub public: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneListResponse {
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub model: String,

    pub modified_at: DateTime<Utc>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub model: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneAssetNewResponse {
    pub success: bool,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetuneAssetNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetunePublicListResponse {
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub model: String,

    pub modified_at: DateTime<Utc>,

    pub name: String,

    pub public: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetunePublicListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Pagination Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Pagination Offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// Order By Column Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Filter by Author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Filter to hide experimental models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_experimental: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Filter by Source Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Filter by Task Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSchemaGetResponse {
    pub input: ModelSchemaGetResponseInput,

    pub output: ModelSchemaGetResponseOutput,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSchemaGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Model Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMarkdownSupportedResponse {
    pub extension: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMarkdownTransformResponse {
    pub data: String,

    pub format: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    pub name: String,

    pub tokens: String,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMarkdownSupportedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMarkdownTransformParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub file: ToMarkdownTransformParamsFile,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSchemaGetResponseInput {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,

    pub description: String,

    pub r#type: String,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSchemaGetResponseOutput {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,

    pub description: String,

    pub r#type: String,
}
#[cfg(any(feature = "full", feature = "with-ai"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToMarkdownTransformParamsFile {
    pub files: String,
}
