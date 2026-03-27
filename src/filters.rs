#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallFilter {
    /// The unique identifier of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An informative summary of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The filter expression. For more information, refer to
    /// [Expressions](https://developers.cloudflare.com/ruleset-engine/rules-language/expressions/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// When true, indicates that the filter is currently paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,

    /// A short reference tag. Allows you to select related filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallFilterParam {
    /// An informative summary of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The filter expression. For more information, refer to
    /// [Expressions](https://developers.cloudflare.com/ruleset-engine/rules-language/expressions/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// When true, indicates that the filter is currently paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<String>,

    /// A short reference tag. Allows you to select related filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDeleteResponse {
    /// The unique identifier of the filter.
    pub id: String,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBulkDeleteResponse {
    /// The unique identifier of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterNewParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: Vec<FirewallFilterParam>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterUpdateParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub firewall_filter: FirewallFilterParam,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterListParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The unique identifier of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A case-insensitive string to find in the description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A case-insensitive string to find in the expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// When true, indicates that the filter is currently paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<String>,

    /// Number of filters per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// The filter ref (a short reference tag) to search for. Must be an exact match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDeleteParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBulkDeleteParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBulkUpdateParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: Vec<FilterBulkUpdateParamsBody>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterGetParams {
    /// Defines an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-filters"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBulkUpdateParamsBody {
    /// An informative summary of the filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The filter expression. For more information, refer to
    /// [Expressions](https://developers.cloudflare.com/ruleset-engine/rules-language/expressions/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// When true, indicates that the filter is currently paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<String>,

    /// A short reference tag. Allows you to select related filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}
