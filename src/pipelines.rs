#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponse {
    /// Specifies the pipeline identifier.
    pub id: String,

    pub destination: PipelineNewResponseDestination,

    /// Indicates the endpoint URL to send traffic.
    pub endpoint: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: Vec<PipelineNewResponseSource>,

    /// Indicates the version number of last saved configuration.
    pub version: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponse {
    /// Specifies the pipeline identifier.
    pub id: String,

    pub destination: PipelineUpdateResponseDestination,

    /// Indicates the endpoint URL to send traffic.
    pub endpoint: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: Vec<PipelineUpdateResponseSource>,

    /// Indicates the version number of last saved configuration.
    pub version: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponse {
    pub result_info: PipelineListResponseResultInfo,

    pub results: Vec<PipelineListResponseResult>,

    /// Indicates whether the API call was successful.
    pub success: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponse {
    /// Specifies the pipeline identifier.
    pub id: String,

    pub destination: PipelineGetResponseDestination,

    /// Indicates the endpoint URL to send traffic.
    pub endpoint: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: Vec<PipelineGetResponseSource>,

    /// Indicates the version number of last saved configuration.
    pub version: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlResponse {
    /// Indicates tables involved in the processing.
    pub tables: ::std::collections::HashMap<String, PipelineValidateSqlResponseTable>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<PipelineValidateSqlResponseGraph>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub destination: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub destination: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies which page to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Specifies the number of pipelines per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Specifies the prefix of pipeline name to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDeleteParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies SQL to validate.
    pub sql: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponse {
    /// Indicates a unique identifier for this sink.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub modified_at: DateTime<Utc>,

    /// Defines the name of the Sink.
    pub name: String,

    /// Specifies the type of sink.
    pub r#type: SinkNewResponseType,

    /// R2 Data Catalog Sink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SinkNewResponseConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkNewResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SinkNewResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponse {
    /// Indicates a unique identifier for this sink.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub modified_at: DateTime<Utc>,

    /// Defines the name of the Sink.
    pub name: String,

    /// Specifies the type of sink.
    pub r#type: SinkListResponseType,

    /// Defines the configuration of the R2 Sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SinkListResponseConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkListResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SinkListResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponse {
    /// Indicates a unique identifier for this sink.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub modified_at: DateTime<Utc>,

    /// Defines the name of the Sink.
    pub name: String,

    /// Specifies the type of sink.
    pub r#type: SinkGetResponseType,

    /// Defines the configuration of the R2 Sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SinkGetResponseConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkGetResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SinkGetResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Defines the name of the Sink.
    pub name: String,

    /// Specifies the type of sink.
    pub r#type: String,

    /// Defines the configuration of the R2 Sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkDeleteParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Delete sink forcefully, including deleting any dependent pipelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponse {
    /// Indicates a unique identifier for this stream.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub http: StreamNewResponseHTTP,

    pub modified_at: DateTime<Utc>,

    /// Indicates the name of the Stream.
    pub name: String,

    /// Indicates the current version of this stream.
    pub version: i64,

    pub worker_binding: StreamNewResponseWorkerBinding,

    /// Indicates the endpoint URL of this stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamNewResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<StreamNewResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateResponse {
    /// Indicates a unique identifier for this stream.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub http: StreamUpdateResponseHTTP,

    pub modified_at: DateTime<Utc>,

    /// Indicates the name of the Stream.
    pub name: String,

    /// Indicates the current version of this stream.
    pub version: i64,

    pub worker_binding: StreamUpdateResponseWorkerBinding,

    /// Indicates the endpoint URL of this stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamUpdateResponseFormat>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponse {
    /// Indicates a unique identifier for this stream.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub http: StreamListResponseHTTP,

    pub modified_at: DateTime<Utc>,

    /// Indicates the name of the Stream.
    pub name: String,

    /// Indicates the current version of this stream.
    pub version: i64,

    pub worker_binding: StreamListResponseWorkerBinding,

    /// Indicates the endpoint URL of this stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamListResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<StreamListResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponse {
    /// Indicates a unique identifier for this stream.
    pub id: String,

    pub created_at: DateTime<Utc>,

    pub http: StreamGetResponseHTTP,

    pub modified_at: DateTime<Utc>,

    /// Indicates the name of the Stream.
    pub name: String,

    /// Indicates the current version of this stream.
    pub version: i64,

    pub worker_binding: StreamGetResponseWorkerBinding,

    /// Indicates the endpoint URL of this stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamGetResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<StreamGetResponseSchema>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies the name of the Stream.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_binding: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_binding: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Specifies the public ID of the pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDeleteParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Delete stream forcefully, including deleting any dependent pipelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetParams {
    /// Specifies the public ID of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponseDestination {
    pub batch: PipelineNewResponseDestinationBatch,

    pub compression: PipelineNewResponseDestinationCompression,

    /// Specifies the format of data to deliver.
    pub format: PipelineNewResponseDestinationFormat,

    pub path: PipelineNewResponseDestinationPath,

    /// Specifies the type of destination.
    pub r#type: PipelineNewResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponseSource {
    /// Specifies the format of source data.
    pub format: PipelineNewResponseSourceFormat,

    pub r#type: String,

    /// Specifies whether authentication is required to send to this pipeline via HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<bool>,

    /// This field can have the runtime type of
    /// [PipelineNewResponseSourceCloudflarePipelinesWorkersPipelinesHTTPSourceCORS].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponseDestination {
    pub batch: PipelineUpdateResponseDestinationBatch,

    pub compression: PipelineUpdateResponseDestinationCompression,

    /// Specifies the format of data to deliver.
    pub format: PipelineUpdateResponseDestinationFormat,

    pub path: PipelineUpdateResponseDestinationPath,

    /// Specifies the type of destination.
    pub r#type: PipelineUpdateResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponseSource {
    /// Specifies the format of source data.
    pub format: PipelineUpdateResponseSourceFormat,

    pub r#type: String,

    /// Specifies whether authentication is required to send to this pipeline via HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<bool>,

    /// This field can have the runtime type of
    /// [PipelineUpdateResponseSourceCloudflarePipelinesWorkersPipelinesHTTPSourceCORS].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultInfo {
    /// Indicates the number of items on current page.
    pub count: f64,

    /// Indicates the current page number.
    pub page: f64,

    /// Indicates the number of items per page.
    pub per_page: f64,

    /// Indicates the total number of items.
    pub total_count: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResult {
    /// Specifies the pipeline identifier.
    pub id: String,

    pub destination: PipelineListResponseResultsDestination,

    /// Indicates the endpoint URL to send traffic.
    pub endpoint: String,

    /// Defines the name of the pipeline.
    pub name: String,

    pub source: Vec<PipelineListResponseResultsSource>,

    /// Indicates the version number of last saved configuration.
    pub version: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponseDestination {
    pub batch: PipelineGetResponseDestinationBatch,

    pub compression: PipelineGetResponseDestinationCompression,

    /// Specifies the format of data to deliver.
    pub format: PipelineGetResponseDestinationFormat,

    pub path: PipelineGetResponseDestinationPath,

    /// Specifies the type of destination.
    pub r#type: PipelineGetResponseDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponseSource {
    /// Specifies the format of source data.
    pub format: PipelineGetResponseSourceFormat,

    pub r#type: String,

    /// Specifies whether authentication is required to send to this pipeline via HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<bool>,

    /// This field can have the runtime type of
    /// [PipelineGetResponseSourceCloudflarePipelinesWorkersPipelinesHTTPSourceCORS].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlResponseTable {
    pub id: String,

    pub name: String,

    pub r#type: String,

    pub version: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlResponseGraph {
    pub edges: Vec<PipelineValidateSqlResponseGraphEdge>,

    pub nodes: Vec<PipelineValidateSqlResponseGraphNode>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponseConfig {
    /// Cloudflare Account ID for the bucket
    pub account_id: String,

    /// R2 Bucket to write to
    pub bucket: String,

    /// Authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// This field can have the runtime type of
    /// [SinkNewResponseConfigCloudflarePipelinesR2TableCredentials].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,

    /// This field can have the runtime type of
    /// [SinkNewResponseConfigCloudflarePipelinesR2TableFileNaming].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_naming: Option<String>,

    /// Jurisdiction this bucket is hosted in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Table namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// This field can have the runtime type of
    /// [SinkNewResponseConfigCloudflarePipelinesR2TablePartitioning].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioning: Option<String>,

    /// Subpath within the bucket to write to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// This field can have the runtime type of
    /// [SinkNewResponseConfigCloudflarePipelinesR2TableRollingPolicy],
    /// [SinkNewResponseConfigCloudflarePipelinesR2DataCatalogTableRollingPolicy].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<String>,

    /// Table name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponseFormat {
    pub r#type: SinkNewResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkNewResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkNewResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkNewResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<SinkNewResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkNewResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponseConfig {
    /// Cloudflare Account ID for the bucket
    pub account_id: String,

    /// R2 Bucket to write to
    pub bucket: String,

    /// Authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// This field can have the runtime type of
    /// [SinkListResponseConfigCloudflarePipelinesR2TableCredentials].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,

    /// This field can have the runtime type of
    /// [SinkListResponseConfigCloudflarePipelinesR2TableFileNaming].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_naming: Option<String>,

    /// Jurisdiction this bucket is hosted in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Table namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// This field can have the runtime type of
    /// [SinkListResponseConfigCloudflarePipelinesR2TablePartitioning].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioning: Option<String>,

    /// Subpath within the bucket to write to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// This field can have the runtime type of
    /// [SinkListResponseConfigCloudflarePipelinesR2TableRollingPolicy],
    /// [SinkListResponseConfigCloudflarePipelinesR2DataCatalogTableRollingPolicy].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<String>,

    /// Table name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponseFormat {
    pub r#type: SinkListResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkListResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkListResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkListResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<SinkListResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkListResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponseConfig {
    /// Cloudflare Account ID for the bucket
    pub account_id: String,

    /// R2 Bucket to write to
    pub bucket: String,

    /// Authentication token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// This field can have the runtime type of
    /// [SinkGetResponseConfigCloudflarePipelinesR2TableCredentials].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,

    /// This field can have the runtime type of
    /// [SinkGetResponseConfigCloudflarePipelinesR2TableFileNaming].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_naming: Option<String>,

    /// Jurisdiction this bucket is hosted in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Table namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// This field can have the runtime type of
    /// [SinkGetResponseConfigCloudflarePipelinesR2TablePartitioning].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioning: Option<String>,

    /// Subpath within the bucket to write to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// This field can have the runtime type of
    /// [SinkGetResponseConfigCloudflarePipelinesR2TableRollingPolicy],
    /// [SinkGetResponseConfigCloudflarePipelinesR2DataCatalogTableRollingPolicy].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<String>,

    /// Table name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponseFormat {
    pub r#type: SinkGetResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkGetResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkGetResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkGetResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<SinkGetResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SinkGetResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseHTTP {
    /// Indicates that authentication is required for the HTTP endpoint.
    pub authentication: bool,

    /// Indicates that the HTTP endpoint is enabled.
    pub enabled: bool,

    /// Specifies the CORS options for the HTTP endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<StreamNewResponseHTTPCORS>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseWorkerBinding {
    /// Indicates that the worker binding is enabled.
    pub enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseFormat {
    pub r#type: StreamNewResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamNewResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamNewResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamNewResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<StreamNewResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamNewResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateResponseHTTP {
    /// Indicates that authentication is required for the HTTP endpoint.
    pub authentication: bool,

    /// Indicates that the HTTP endpoint is enabled.
    pub enabled: bool,

    /// Specifies the CORS options for the HTTP endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<StreamUpdateResponseHTTPCORS>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateResponseWorkerBinding {
    /// Indicates that the worker binding is enabled.
    pub enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateResponseFormat {
    pub r#type: StreamUpdateResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamUpdateResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamUpdateResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamUpdateResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseHTTP {
    /// Indicates that authentication is required for the HTTP endpoint.
    pub authentication: bool,

    /// Indicates that the HTTP endpoint is enabled.
    pub enabled: bool,

    /// Specifies the CORS options for the HTTP endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<StreamListResponseHTTPCORS>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseWorkerBinding {
    /// Indicates that the worker binding is enabled.
    pub enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseFormat {
    pub r#type: StreamListResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamListResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamListResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamListResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<StreamListResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamListResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseHTTP {
    /// Indicates that authentication is required for the HTTP endpoint.
    pub authentication: bool,

    /// Indicates that the HTTP endpoint is enabled.
    pub enabled: bool,

    /// Specifies the CORS options for the HTTP endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<StreamGetResponseHTTPCORS>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseWorkerBinding {
    /// Indicates that the worker binding is enabled.
    pub enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseFormat {
    pub r#type: StreamGetResponseFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamGetResponseFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamGetResponseFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamGetResponseFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<StreamGetResponseSchemaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StreamGetResponseSchemaFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponseDestinationBatch {
    /// Specifies rough maximum size of files.
    pub max_bytes: i64,

    /// Specifies duration to wait to aggregate batches files.
    pub max_duration_s: f64,

    /// Specifies rough maximum number of rows per file.
    pub max_rows: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponseDestinationCompression {
    /// Specifies the desired compression algorithm and format.
    pub r#type: PipelineNewResponseDestinationCompressionType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNewResponseDestinationPath {
    /// Specifies the R2 Bucket to store files.
    pub bucket: String,

    /// Specifies the name pattern to for individual data files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Specifies the name pattern for directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,

    /// Specifies the base directory within the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponseDestinationBatch {
    /// Specifies rough maximum size of files.
    pub max_bytes: i64,

    /// Specifies duration to wait to aggregate batches files.
    pub max_duration_s: f64,

    /// Specifies rough maximum number of rows per file.
    pub max_rows: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponseDestinationCompression {
    /// Specifies the desired compression algorithm and format.
    pub r#type: PipelineUpdateResponseDestinationCompressionType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdateResponseDestinationPath {
    /// Specifies the R2 Bucket to store files.
    pub bucket: String,

    /// Specifies the name pattern to for individual data files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Specifies the name pattern for directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,

    /// Specifies the base directory within the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultsDestination {
    pub batch: PipelineListResponseResultsDestinationBatch,

    pub compression: PipelineListResponseResultsDestinationCompression,

    /// Specifies the format of data to deliver.
    pub format: PipelineListResponseResultsDestinationFormat,

    pub path: PipelineListResponseResultsDestinationPath,

    /// Specifies the type of destination.
    pub r#type: PipelineListResponseResultsDestinationType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultsSource {
    /// Specifies the format of source data.
    pub format: PipelineListResponseResultsSourceFormat,

    pub r#type: String,

    /// Specifies whether authentication is required to send to this pipeline via HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<bool>,

    /// This field can have the runtime type of
    /// [PipelineListResponseResultsSourceCloudflarePipelinesWorkersPipelinesHTTPSourceCORS].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponseDestinationBatch {
    /// Specifies rough maximum size of files.
    pub max_bytes: i64,

    /// Specifies duration to wait to aggregate batches files.
    pub max_duration_s: f64,

    /// Specifies rough maximum number of rows per file.
    pub max_rows: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponseDestinationCompression {
    /// Specifies the desired compression algorithm and format.
    pub r#type: PipelineGetResponseDestinationCompressionType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineGetResponseDestinationPath {
    /// Specifies the R2 Bucket to store files.
    pub bucket: String,

    /// Specifies the name pattern to for individual data files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Specifies the name pattern for directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,

    /// Specifies the base directory within the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlResponseGraphEdge {
    pub dest_id: i64,

    pub edge_type: String,

    pub key_type: String,

    pub src_id: i64,

    pub value_type: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineValidateSqlResponseGraphNode {
    pub description: String,

    pub node_id: i64,

    pub operator: String,

    pub parallelism: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponseSchemaField {
    pub r#type: SinkNewResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<SinkNewResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkNewResponseSchemaFormat {
    pub r#type: SinkNewResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkNewResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkNewResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkNewResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponseSchemaField {
    pub r#type: SinkListResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<SinkListResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponseSchemaFormat {
    pub r#type: SinkListResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkListResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkListResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkListResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponseSchemaField {
    pub r#type: SinkGetResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<SinkGetResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkGetResponseSchemaFormat {
    pub r#type: SinkGetResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<SinkGetResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<SinkGetResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<SinkGetResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseHTTPCORS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseSchemaField {
    pub r#type: StreamNewResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StreamNewResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewResponseSchemaFormat {
    pub r#type: StreamNewResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamNewResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamNewResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamNewResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUpdateResponseHTTPCORS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseHTTPCORS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseSchemaField {
    pub r#type: StreamListResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StreamListResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListResponseSchemaFormat {
    pub r#type: StreamListResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamListResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamListResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamListResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseHTTPCORS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseSchemaField {
    pub r#type: StreamGetResponseSchemaFieldsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<StreamGetResponseSchemaFieldsUnit>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetResponseSchemaFormat {
    pub r#type: StreamGetResponseSchemaFormatType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<StreamGetResponseSchemaFormatCompression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_encoding: Option<StreamGetResponseSchemaFormatDecimalEncoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_bytes: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<StreamGetResponseSchemaFormatTimestampFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstructured: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultsDestinationBatch {
    /// Specifies rough maximum size of files.
    pub max_bytes: i64,

    /// Specifies duration to wait to aggregate batches files.
    pub max_duration_s: f64,

    /// Specifies rough maximum number of rows per file.
    pub max_rows: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultsDestinationCompression {
    /// Specifies the desired compression algorithm and format.
    pub r#type: PipelineListResponseResultsDestinationCompressionType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineListResponseResultsDestinationPath {
    /// Specifies the R2 Bucket to store files.
    pub bucket: String,

    /// Specifies the name pattern to for individual data files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Specifies the name pattern for directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,

    /// Specifies the base directory within the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseType {
    #[serde(rename = "r2")]
    SinkNewResponseTypeR2,
    #[serde(rename = "r2_data_catalog")]
    SinkNewResponseTypeR2DataCatalog,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseType {
    #[serde(rename = "r2")]
    SinkListResponseTypeR2,
    #[serde(rename = "r2_data_catalog")]
    SinkListResponseTypeR2DataCatalog,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseType {
    #[serde(rename = "r2")]
    SinkGetResponseTypeR2,
    #[serde(rename = "r2_data_catalog")]
    SinkGetResponseTypeR2DataCatalog,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineNewResponseDestinationFormat {
    #[serde(rename = "json")]
    PipelineNewResponseDestinationFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineNewResponseDestinationType {
    #[serde(rename = "r2")]
    PipelineNewResponseDestinationTypeR2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineNewResponseSourceFormat {
    #[serde(rename = "json")]
    PipelineNewResponseSourceFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineUpdateResponseDestinationFormat {
    #[serde(rename = "json")]
    PipelineUpdateResponseDestinationFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineUpdateResponseDestinationType {
    #[serde(rename = "r2")]
    PipelineUpdateResponseDestinationTypeR2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineUpdateResponseSourceFormat {
    #[serde(rename = "json")]
    PipelineUpdateResponseSourceFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineGetResponseDestinationFormat {
    #[serde(rename = "json")]
    PipelineGetResponseDestinationFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineGetResponseDestinationType {
    #[serde(rename = "r2")]
    PipelineGetResponseDestinationTypeR2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineGetResponseSourceFormat {
    #[serde(rename = "json")]
    PipelineGetResponseSourceFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseFormatType {
    #[serde(rename = "json")]
    SinkNewResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkNewResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkNewResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkNewResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkNewResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkNewResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkNewResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkNewResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkNewResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkNewResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkNewResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkNewResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseFormatType {
    #[serde(rename = "json")]
    SinkListResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkListResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkListResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkListResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkListResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkListResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkListResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkListResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkListResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkListResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkListResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkListResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseFormatType {
    #[serde(rename = "json")]
    SinkGetResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkGetResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkGetResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkGetResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkGetResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkGetResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkGetResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkGetResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkGetResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkGetResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkGetResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkGetResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseFormatType {
    #[serde(rename = "json")]
    StreamNewResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamNewResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamNewResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamNewResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamNewResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamNewResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamNewResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamNewResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamNewResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamNewResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamNewResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamNewResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamUpdateResponseFormatType {
    #[serde(rename = "json")]
    StreamUpdateResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamUpdateResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamUpdateResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamUpdateResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamUpdateResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamUpdateResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamUpdateResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamUpdateResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamUpdateResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamUpdateResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamUpdateResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamUpdateResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamUpdateResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamUpdateResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamUpdateResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseFormatType {
    #[serde(rename = "json")]
    StreamListResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamListResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamListResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamListResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamListResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamListResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamListResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamListResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamListResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamListResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamListResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamListResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseFormatType {
    #[serde(rename = "json")]
    StreamGetResponseFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamGetResponseFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamGetResponseFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamGetResponseFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamGetResponseFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamGetResponseFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamGetResponseFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamGetResponseFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamGetResponseFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamGetResponseFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamGetResponseFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamGetResponseFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineNewResponseDestinationCompressionType {
    #[serde(rename = "none")]
    PipelineNewResponseDestinationCompressionTypeNone,
    #[serde(rename = "gzip")]
    PipelineNewResponseDestinationCompressionTypeGzip,
    #[serde(rename = "deflate")]
    PipelineNewResponseDestinationCompressionTypeDeflate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineUpdateResponseDestinationCompressionType {
    #[serde(rename = "none")]
    PipelineUpdateResponseDestinationCompressionTypeNone,
    #[serde(rename = "gzip")]
    PipelineUpdateResponseDestinationCompressionTypeGzip,
    #[serde(rename = "deflate")]
    PipelineUpdateResponseDestinationCompressionTypeDeflate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineListResponseResultsDestinationFormat {
    #[serde(rename = "json")]
    PipelineListResponseResultsDestinationFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineListResponseResultsDestinationType {
    #[serde(rename = "r2")]
    PipelineListResponseResultsDestinationTypeR2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineListResponseResultsSourceFormat {
    #[serde(rename = "json")]
    PipelineListResponseResultsSourceFormatJson,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineGetResponseDestinationCompressionType {
    #[serde(rename = "none")]
    PipelineGetResponseDestinationCompressionTypeNone,
    #[serde(rename = "gzip")]
    PipelineGetResponseDestinationCompressionTypeGzip,
    #[serde(rename = "deflate")]
    PipelineGetResponseDestinationCompressionTypeDeflate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    SinkNewResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    SinkNewResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    SinkNewResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    SinkNewResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    SinkNewResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    SinkNewResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    SinkNewResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    SinkNewResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    SinkNewResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    SinkNewResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    SinkNewResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    SinkNewResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    SinkNewResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    SinkNewResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    SinkNewResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFormatType {
    #[serde(rename = "json")]
    SinkNewResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkNewResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkNewResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkNewResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkNewResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkNewResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkNewResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkNewResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkNewResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkNewResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkNewResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkNewResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkNewResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    SinkListResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    SinkListResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    SinkListResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    SinkListResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    SinkListResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    SinkListResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    SinkListResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    SinkListResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    SinkListResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    SinkListResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    SinkListResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    SinkListResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    SinkListResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    SinkListResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    SinkListResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFormatType {
    #[serde(rename = "json")]
    SinkListResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkListResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkListResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkListResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkListResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkListResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkListResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkListResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkListResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkListResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkListResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkListResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkListResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    SinkGetResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    SinkGetResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    SinkGetResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    SinkGetResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    SinkGetResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    SinkGetResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    SinkGetResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    SinkGetResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    SinkGetResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    SinkGetResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    SinkGetResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    SinkGetResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    SinkGetResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    SinkGetResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    SinkGetResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFormatType {
    #[serde(rename = "json")]
    SinkGetResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    SinkGetResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    SinkGetResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    SinkGetResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    SinkGetResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    SinkGetResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    SinkGetResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    SinkGetResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    SinkGetResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    SinkGetResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SinkGetResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    SinkGetResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    SinkGetResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    StreamNewResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    StreamNewResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    StreamNewResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    StreamNewResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    StreamNewResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    StreamNewResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    StreamNewResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    StreamNewResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    StreamNewResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    StreamNewResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    StreamNewResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    StreamNewResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    StreamNewResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    StreamNewResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    StreamNewResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFormatType {
    #[serde(rename = "json")]
    StreamNewResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamNewResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamNewResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamNewResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamNewResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamNewResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamNewResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamNewResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamNewResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamNewResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamNewResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamNewResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamNewResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    StreamListResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    StreamListResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    StreamListResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    StreamListResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    StreamListResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    StreamListResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    StreamListResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    StreamListResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    StreamListResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    StreamListResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    StreamListResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    StreamListResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    StreamListResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    StreamListResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    StreamListResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFormatType {
    #[serde(rename = "json")]
    StreamListResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamListResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamListResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamListResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamListResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamListResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamListResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamListResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamListResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamListResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamListResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamListResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamListResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFieldsType {
    #[serde(rename = "int32")]
    StreamGetResponseSchemaFieldsTypeInt32,
    #[serde(rename = "int64")]
    StreamGetResponseSchemaFieldsTypeInt64,
    #[serde(rename = "float32")]
    StreamGetResponseSchemaFieldsTypeFloat32,
    #[serde(rename = "float64")]
    StreamGetResponseSchemaFieldsTypeFloat64,
    #[serde(rename = "bool")]
    StreamGetResponseSchemaFieldsTypeBool,
    #[serde(rename = "string")]
    StreamGetResponseSchemaFieldsTypeString,
    #[serde(rename = "binary")]
    StreamGetResponseSchemaFieldsTypeBinary,
    #[serde(rename = "timestamp")]
    StreamGetResponseSchemaFieldsTypeTimestamp,
    #[serde(rename = "json")]
    StreamGetResponseSchemaFieldsTypeJson,
    #[serde(rename = "struct")]
    StreamGetResponseSchemaFieldsTypeStruct,
    #[serde(rename = "list")]
    StreamGetResponseSchemaFieldsTypeList,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFieldsUnit {
    #[serde(rename = "second")]
    StreamGetResponseSchemaFieldsUnitSecond,
    #[serde(rename = "millisecond")]
    StreamGetResponseSchemaFieldsUnitMillisecond,
    #[serde(rename = "microsecond")]
    StreamGetResponseSchemaFieldsUnitMicrosecond,
    #[serde(rename = "nanosecond")]
    StreamGetResponseSchemaFieldsUnitNanosecond,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFormatType {
    #[serde(rename = "json")]
    StreamGetResponseSchemaFormatTypeJson,
    #[serde(rename = "parquet")]
    StreamGetResponseSchemaFormatTypeParquet,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFormatCompression {
    #[serde(rename = "uncompressed")]
    StreamGetResponseSchemaFormatCompressionUncompressed,
    #[serde(rename = "snappy")]
    StreamGetResponseSchemaFormatCompressionSnappy,
    #[serde(rename = "gzip")]
    StreamGetResponseSchemaFormatCompressionGzip,
    #[serde(rename = "zstd")]
    StreamGetResponseSchemaFormatCompressionZstd,
    #[serde(rename = "lz4")]
    StreamGetResponseSchemaFormatCompressionLz4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFormatDecimalEncoding {
    #[serde(rename = "number")]
    StreamGetResponseSchemaFormatDecimalEncodingNumber,
    #[serde(rename = "string")]
    StreamGetResponseSchemaFormatDecimalEncodingString,
    #[serde(rename = "bytes")]
    StreamGetResponseSchemaFormatDecimalEncodingBytes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StreamGetResponseSchemaFormatTimestampFormat {
    #[serde(rename = "rfc3339")]
    StreamGetResponseSchemaFormatTimestampFormatRfc3339,
    #[serde(rename = "unix_millis")]
    StreamGetResponseSchemaFormatTimestampFormatUnixMillis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineListResponseResultsDestinationCompressionType {
    #[serde(rename = "none")]
    PipelineListResponseResultsDestinationCompressionTypeNone,
    #[serde(rename = "gzip")]
    PipelineListResponseResultsDestinationCompressionTypeGzip,
    #[serde(rename = "deflate")]
    PipelineListResponseResultsDestinationCompressionTypeDeflate,
}

