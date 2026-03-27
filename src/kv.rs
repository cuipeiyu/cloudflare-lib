use serde::{Deserialize, Serialize};

use chrono::{Duration, DateTime, Utc};

pub type Metadata = ::std::collections::HashMap<String, String>;

/// Whether to parse JSON values in the response.
#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NamespaceKeyType {
    /// value is "text"
    #[serde(rename = "text")]
    Text,
    /// value is "json"
    #[serde(rename = "json")]
    Json,
}

#[cfg(any(feature = "full", feature = "with-kv"))]
pub type NamespaceKeyBulkGetParamsType = NamespaceKeyType;

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Namespace {
    /// Namespace identifier tag.
    pub id: String,

    /// A human-readable string name for a Namespace.
    pub title: String,

    /// True if keys written on the URL will be URL-decoded before storing. For example,
    /// if set to "true", a key written on the URL as "%3F" will be stored as "?".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_url_encoding: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceBulkDeleteResponse {
    /// Number of keys successfully updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_key_count: Option<f64>,

    /// Name of the keys that failed to be fully updated. They should be retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful_keys: Option<Vec<String>>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceBulkGetResponse<T> {
    /// This field can have the runtime type of
    /// - HashMap<String, UnionString>
    /// - HashMap<String, UnionFloat>
    /// - HashMap<String, UnionBool>
    /// - HashMap<String, NamespaceBulkGetResponseWorkersKVBulkGetResultValuesMap>
    /// - HashMap<String, NamespaceBulkGetResponseWorkersKVBulkGetResultWithMetadataValue>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<::std::collections::HashMap<String, T>>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceBulkGetResponseWorkersKVBulkGetResultWithMetadataValue<M, V> {
    /// The metadata associated with the key.
    pub metadata: M,

    /// The value associated with the key.
    pub value: V,

    /// Expires the key at a certain time, measured in number of seconds since the UNIX
    /// epoch.
    pub expiration: Option<f64>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceBulkUpdateResponse {
    /// Number of keys successfully updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_key_count: Option<f64>,

    /// Name of the keys that failed to be fully updated. They should be retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful_keys: Option<Vec<String>>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceNewBody {
    /// A human-readable string name for a Namespace.
    pub title: String,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceUpdateBody {
    /// A human-readable string name for a Namespace.
    pub title: String,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceListQuery {
    /// Direction to order namespaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<NamespaceListQueryDirection>,

    /// Field to order results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<NamespaceListQueryOrder>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
pub type NamespaceListQueryDirection = crate::shared::SortDirection;

/// Field to order results by.
#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NamespaceListQueryOrder {
    /// value is "id"
    #[serde(rename = "id")]
    Id,
    /// value is "title"
    #[serde(rename = "title")]
    Title,
}

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceDeleteParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

// }

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceBulkDeleteParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

//     pub body: Vec<String>,

// }

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceBulkGetBody {
    /// Array of keys to retrieve (maximum of 100).
    pub keys: Vec<String>,

    /// Whether to parse JSON values in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<NamespaceBulkGetBodyType>,

    /// Whether to include metadata in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "withMetadata")]
    pub with_metadata: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
pub type NamespaceBulkGetBodyType = NamespaceKeyType;

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceBulkUpdateParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

//     pub body: Vec<NamespaceBulkUpdateParamsBody>,

// }

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceGetParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

// }

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    /// A key's name. The name may be at most 512 bytes. All printable, non-whitespace
    /// characters are valid. Use percent-encoding to define key names as part of a URL.
    pub name: String,

    /// The time, measured in number of seconds since the UNIX epoch, at which the key
    /// will expire. This property is omitted for keys that will not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,

    /// Arbitrary JSON that is associated with a key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

}

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkDeleteResponse {
//     /// Number of keys successfully updated.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub successful_key_count: Option<f64>,

//     /// Name of the keys that failed to be fully updated. They should be retried.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub unsuccessful_keys: Option<Vec<String>>,

// }

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkGetResponse<T> {
//     /// This field can have the runtime type of
//     /// - HashMap<String, UnionString>
//     /// - HashMap<String, UnionFloat>
//     /// - HashMap<String, UnionBool>
//     /// - HashMap<String, NamespaceKeyBulkGetResponseWorkersKVBulkGetResultValuesMap>
//     /// - HashMap<String, NamespaceKeyBulkGetResponseWorkersKVBulkGetResultWithMetadataValue>
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub values: Option<::std::collections::HashMap<String, T>>,

// }

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkGetResponseWorkersKVBulkGetResultWithMetadataValue<M, V> {
//     /// The metadata associated with the key.
//     pub metadata: M,

//     /// The value associated with the key.
//     pub value: V,

//     /// Expires the key at a certain time, measured in number of seconds since the UNIX
//     /// epoch.
//     pub expiration: Option<f64>,
// }

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkUpdateResponse {
//     /// Number of keys successfully updated.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub successful_key_count: Option<f64>,

//     /// Name of the keys that failed to be fully updated. They should be retried.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub unsuccessful_keys: Option<Vec<String>>,

// }

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceKeyListCursorQuery {
    /// Opaque token indicating the position from which to continue when requesting the
    /// next set of records if the amount of list results was limited by the limit
    /// parameter. A valid value for the cursor can be obtained from the `cursors`
    /// object in the `result_info` structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Limits the number of keys returned in the response. The cursor attribute may be
    /// used to iterate over the next batch of keys if there are more than the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Filters returned keys by a name prefix. Exact matches and any key names that
    /// begin with the prefix will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

}

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkDeleteParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

//     pub body: Vec<String>,

// }

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkGetParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

//     /// Array of keys to retrieve (maximum of 100).
//     pub keys: Vec<String>,

//     /// Whether to parse JSON values in the response.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub r#type: Option<NamespaceKeyBulkGetParamsType>,

//     /// Whether to include metadata in the response.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(rename = "withMetadata")]
//     pub with_metadata: Option<bool>,

// }

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkUpdateParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

//     pub body: Vec<NamespaceKeyBulkUpdateParamsBody>,

// }

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceMetadataGetParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

// }

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceValueUpdateQuery {
    /// Expires the key at a certain time, measured in number of seconds since the UNIX
    /// epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,

    /// Expires the key after a number of seconds. Must be at least 60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ttl: Option<i64>,

}

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceValueUpdateBody {
    /// A byte sequence to be stored, up to 25 MiB in length.
    pub value: crate::shared::UnionString,

    /// Associates arbitrary JSON data with a key/value pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

}

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceValueDeleteParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

// }

// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceValueGetParams {
//     /// [path] Identifier.
//     #[serde(skip)]
//     pub account_id: String,

// }

#[cfg(any(feature = "full", feature = "with-kv"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamespaceBulkUpdateParamsBody {
    /// A key's name. The name may be at most 512 bytes. All printable, non-whitespace
    /// characters are valid.
    pub key: String,

    /// A UTF-8 encoded string to be stored, up to 25 MiB in length.
    pub value: String,

    /// Indicates whether or not the server should base64 decode the value before
    /// storing it. Useful for writing values that wouldn't otherwise be valid JSON
    /// strings, such as images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<bool>,

    /// Expires the key at a certain time, measured in number of seconds since the UNIX
    /// epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,

    /// Expires the key after a number of seconds. Must be at least 60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ttl: Option<i64>,

    /// Arbitrary JSON that is associated with a key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

}

// #[deprecated]
// #[cfg(any(feature = "full", feature = "with-kv"))]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NamespaceKeyBulkUpdateParamsBody {
//     /// A key's name. The name may be at most 512 bytes. All printable, non-whitespace
//     /// characters are valid.
//     pub key: String,

//     /// A UTF-8 encoded string to be stored, up to 25 MiB in length.
//     pub value: String,

//     /// Indicates whether or not the server should base64 decode the value before
//     /// storing it. Useful for writing values that wouldn't otherwise be valid JSON
//     /// strings, such as images.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub base64: Option<bool>,

//     /// Expires the key at a certain time, measured in number of seconds since the UNIX
//     /// epoch.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub expiration: Option<i64>,

//     /// Expires the key after a number of seconds. Must be at least 60.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub expiration_ttl: Option<i64>,

//     /// Arbitrary JSON that is associated with a key.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub metadata: Option<Metadata>,

// }
