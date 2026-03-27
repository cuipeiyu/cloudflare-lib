#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<IndexDimensionConfiguration>,

    /// Specifies the timestamp the resource was created as an ISO8601 string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Specifies the description of the index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specifies the timestamp the resource was modified as an ISO8601 string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDimensionConfiguration {
    /// Specifies the number of dimensions for the index
    pub dimensions: i64,

    /// Specifies the type of metric to use calculating distance.
    pub metric: IndexDimensionConfigurationMetric,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDimensionConfigurationParam {
    /// Specifies the number of dimensions for the index
    pub dimensions: String,

    /// Specifies the type of metric to use calculating distance.
    pub metric: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDeleteByIDsResponse {
    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mutationId")]
    pub mutation_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfoResponse {
    /// Specifies the number of dimensions for the index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i64>,

    /// Specifies the timestamp the last mutation batch was processed as an ISO8601
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processedUpToDatetime")]
    pub processed_up_to_datetime: Option<DateTime<Utc>>,

    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processedUpToMutation")]
    pub processed_up_to_mutation: Option<String>,

    /// Specifies the number of vectors present in the index
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vectorCount")]
    pub vector_count: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInsertResponse {
    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mutationId")]
    pub mutation_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexListVectorsResponse {
    /// Number of vectors returned in this response
    pub count: i64,

    /// Whether there are more vectors available beyond this response
    #[serde(rename = "isTruncated")]
    pub is_truncated: bool,

    /// Total number of vectors in the index
    #[serde(rename = "totalCount")]
    pub total_count: i64,

    /// Array of vector items
    pub vectors: Vec<IndexListVectorsResponseVector>,

    /// When the cursor expires as an ISO8601 string
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursorExpirationTimestamp")]
    pub cursor_expiration_timestamp: Option<DateTime<Utc>>,

    /// Cursor for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexQueryResponse {
    /// Specifies the count of vectors returned by the search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    /// Array of vectors matched by the search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<IndexQueryResponseMatch>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexUpsertResponse {
    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mutationId")]
    pub mutation_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies the type of configuration to use for the index.
    pub config: String,

    pub name: String,

    /// Specifies the description of the index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDeleteByIDsParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A list of vector identifiers to delete from the index indicated by the path.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ids")]
    pub i_ds: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexGetByIDsParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A list of vector identifiers to retrieve from the index indicated by the path.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ids")]
    pub i_ds: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfoParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInsertParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// ndjson file containing vectors to insert.
    pub body: Vec<u8>,

    /// Behavior for ndjson parse failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unparsable_behavior: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexListVectorsParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Maximum number of vectors to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,

    /// Cursor for pagination to get the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexQueryParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The search vector that will be used to find the nearest neighbors.
    pub vector: String,

    /// A metadata filter expression used to limit nearest neighbor results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Whether to return no metadata, indexed metadata or all metadata associated with
    /// the closest vectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnMetadata")]
    pub return_metadata: Option<String>,

    /// Whether to return the values associated with the closest vectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnValues")]
    pub return_values: Option<String>,

    /// The number of nearest neighbors to find.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topK")]
    pub top_k: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexUpsertParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// ndjson file containing vectors to upsert.
    pub body: Vec<u8>,

    /// Behavior for ndjson parse failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unparsable_behavior: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexNewResponse {
    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mutationId")]
    pub mutation_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexListResponse {
    /// Array of indexed metadata properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadataIndexes")]
    pub metadata_indexes: Option<Vec<IndexMetadataIndexListResponseMetadataIndex>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexDeleteResponse {
    /// The unique identifier for the async mutation operation containing the changeset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mutationId")]
    pub mutation_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies the type of metadata property to index.
    #[serde(rename = "indexType")]
    pub index_type: String,

    /// Specifies the metadata property to index.
    #[serde(rename = "propertyName")]
    pub property_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Specifies the metadata property for which the index must be deleted.
    #[serde(rename = "propertyName")]
    pub property_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexListVectorsResponseVector {
    /// Identifier for a Vector
    pub id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexQueryResponseMatch {
    /// Identifier for a Vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The score of the vector according to the index's distance metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadataIndexListResponseMetadataIndex {
    /// Specifies the type of indexed metadata property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "indexType")]
    pub index_type: Option<IndexMetadataIndexListResponseMetadataIndexesIndexType>,

    /// Specifies the indexed metadata property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propertyName")]
    pub property_name: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndexDimensionConfigurationMetric {
    #[serde(rename = "cosine")]
    IndexDimensionConfigurationMetricCosine,
    #[serde(rename = "euclidean")]
    IndexDimensionConfigurationMetricEuclidean,
    #[serde(rename = "dot-product")]
    IndexDimensionConfigurationMetricDOTProduct,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndexMetadataIndexListResponseMetadataIndexesIndexType {
    #[serde(rename = "string")]
    IndexMetadataIndexListResponseMetadataIndexesIndexTypeString,
    #[serde(rename = "number")]
    IndexMetadataIndexListResponseMetadataIndexesIndexTypeNumber,
    #[serde(rename = "boolean")]
    IndexMetadataIndexListResponseMetadataIndexesIndexTypeBoolean,
}

