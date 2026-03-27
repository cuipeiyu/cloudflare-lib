#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hostname {
    pub url_hostname: String,

    /// Only applies to wildcard hostnames (e.g., \*.example.com). When true (default),
    /// only subdomains are blocked. When false, both the root domain and subdomains are
    /// blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_exact_hostname: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostnameParam {
    pub url_hostname: String,

    /// Only applies to wildcard hostnames (e.g., \*.example.com). When true (default),
    /// only subdomains are blocked. When false, both the root domain and subdomains are
    /// blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_exact_hostname: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsList {
    /// The unique ID of the list.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The type of the list. Each type supports specific list items (IP addresses,
    /// ASNs, hostnames or redirects).
    pub kind: ListsListKind,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// An informative name for the list. Use this name in filter and rule expressions.
    pub name: String,

    /// The number of items in the list.
    pub num_items: f64,

    /// The number of [filters](/api/resources/filters/) referencing the list.
    pub num_referencing_filters: f64,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redirect {
    pub source_url: String,

    pub target_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_path_suffix: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_query_string: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RedirectStatusCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath_matching: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectParam {
    pub source_url: String,

    pub target_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_path_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_query_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath_matching: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNewResponse {
    /// The unique ID of the list.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The type of the list. Each type supports specific list items (IP addresses,
    /// ASNs, hostnames or redirects).
    pub kind: ListNewResponseKind,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// An informative name for the list. Use this name in filter and rule expressions.
    pub name: String,

    /// The number of items in the list.
    pub num_items: f64,

    /// The number of [filters](/api/resources/filters/) referencing the list.
    pub num_referencing_filters: f64,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUpdateResponse {
    /// The unique ID of the list.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The type of the list. Each type supports specific list items (IP addresses,
    /// ASNs, hostnames or redirects).
    pub kind: ListUpdateResponseKind,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// An informative name for the list. Use this name in filter and rule expressions.
    pub name: String,

    /// The number of items in the list.
    pub num_items: f64,

    /// The number of [filters](/api/resources/filters/) referencing the list.
    pub num_referencing_filters: f64,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeleteResponse {
    /// The unique ID of the list.
    pub id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGetResponse {
    /// The unique ID of the list.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The type of the list. Each type supports specific list items (IP addresses,
    /// ASNs, hostnames or redirects).
    pub kind: ListGetResponseKind,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// An informative name for the list. Use this name in filter and rule expressions.
    pub name: String,

    /// The number of items in the list.
    pub num_items: f64,

    /// The number of [filters](/api/resources/filters/) referencing the list.
    pub num_referencing_filters: f64,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNewParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The type of the list. Each type supports specific list items (IP addresses,
    /// ASNs, hostnames or redirects).
    pub kind: String,

    /// An informative name for the list. Use this name in filter and rule expressions.
    pub name: String,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUpdateParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// An informative summary of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListListParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeleteParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGetParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBulkOperationGetResponse {
    /// The unique operation ID of the asynchronous action.
    pub id: String,

    /// The current status of the asynchronous operation.
    pub status: ListBulkOperationGetResponseStatus,

    /// The RFC 3339 timestamp of when the operation was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,

    /// A message describing the error when the status is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBulkOperationGetParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCursor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemNewResponse {
    /// The unique operation ID of the asynchronous action.
    pub operation_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemUpdateResponse {
    /// The unique operation ID of the asynchronous action.
    pub operation_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemListResponse {
    /// Defines the unique ID of the item in the List.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// Defines a non-negative 32 bit integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// Defines an informative summary of the list item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Valid characters for hostnames are ASCII(7) letters from a to z, the digits from
    /// 0 to 9, wildcards (\*), and the hyphen (-).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,

    /// An IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The definition of the redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Redirect>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemDeleteResponse {
    /// The unique operation ID of the asynchronous action.
    pub operation_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemGetResponse {
    /// Defines the unique ID of the item in the List.
    pub id: String,

    /// The RFC 3339 timestamp of when the list was created.
    pub created_on: String,

    /// The RFC 3339 timestamp of when the list was last modified.
    pub modified_on: String,

    /// Defines a non-negative 32 bit integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// Defines an informative summary of the list item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Valid characters for hostnames are ASCII(7) letters from a to z, the digits from
    /// 0 to 9, wildcards (\*), and the hyphen (-).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,

    /// An IPv4 address, an IPv4 CIDR, an IPv6 address, or an IPv6 CIDR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The definition of the redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Redirect>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemNewParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: Vec<ListItemNewParamsBodyUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemUpdateParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: Vec<ListItemUpdateParamsBodyUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemListParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The pagination cursor. An opaque string token indicating the position from which
    /// to continue when requesting the next/previous set of records. Cursor values are
    /// provided under `result_info.cursors` in the response. You should make no
    /// assumptions about a cursor's content or length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Amount of results to include in each paginated response. A non-negative 32 bit
    /// integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// A search query to filter returned items. Its meaning depends on the list type:
    /// IP addresses must start with the provided string, hostnames and bulk redirects
    /// must contain the string, and ASNs must match the string exactly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemDeleteParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemGetParams {
    /// The Account ID for this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListsListKind {
    #[serde(rename = "ip")]
    ListsListKindIP,
    #[serde(rename = "redirect")]
    ListsListKindRedirect,
    #[serde(rename = "hostname")]
    ListsListKindHostname,
    #[serde(rename = "asn")]
    ListsListKindASN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RedirectStatusCode {
    RedirectStatusCode301 = 301,
    RedirectStatusCode302 = 302,
    RedirectStatusCode307 = 307,
    RedirectStatusCode308 = 308,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListNewResponseKind {
    #[serde(rename = "ip")]
    ListNewResponseKindIP,
    #[serde(rename = "redirect")]
    ListNewResponseKindRedirect,
    #[serde(rename = "hostname")]
    ListNewResponseKindHostname,
    #[serde(rename = "asn")]
    ListNewResponseKindASN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListUpdateResponseKind {
    #[serde(rename = "ip")]
    ListUpdateResponseKindIP,
    #[serde(rename = "redirect")]
    ListUpdateResponseKindRedirect,
    #[serde(rename = "hostname")]
    ListUpdateResponseKindHostname,
    #[serde(rename = "asn")]
    ListUpdateResponseKindASN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListGetResponseKind {
    #[serde(rename = "ip")]
    ListGetResponseKindIP,
    #[serde(rename = "redirect")]
    ListGetResponseKindRedirect,
    #[serde(rename = "hostname")]
    ListGetResponseKindHostname,
    #[serde(rename = "asn")]
    ListGetResponseKindASN,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListBulkOperationGetResponseStatus {
    #[serde(rename = "pending")]
    ListBulkOperationGetResponseStatusPending,
    #[serde(rename = "running")]
    ListBulkOperationGetResponseStatusRunning,
    #[serde(rename = "completed")]
    ListBulkOperationGetResponseStatusCompleted,
    #[serde(rename = "failed")]
    ListBulkOperationGetResponseStatusFailed,
}

