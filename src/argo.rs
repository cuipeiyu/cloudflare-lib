#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingEditResponse {
    /// Specifies the identifier of the Argo Smart Routing setting.
    pub id: String,

    /// Specifies if the setting is editable.
    pub editable: bool,

    /// Specifies the enablement value of Argo Smart Routing.
    pub value: SmartRoutingEditResponseValue,

    /// Specifies the time when the setting was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingGetResponse {
    /// Specifies the identifier of the Argo Smart Routing setting.
    pub id: String,

    /// Specifies if the setting is editable.
    pub editable: bool,

    /// Specifies the enablement value of Argo Smart Routing.
    pub value: SmartRoutingGetResponseValue,

    /// Specifies the time when the setting was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingEditParams {
    /// Specifies the zone associated with the API call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Specifies the enablement value of Argo Smart Routing.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoutingGetParams {
    /// Specifies the zone associated with the API call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredCachingEditResponse {
    /// The identifier of the caching setting.
    pub id: TieredCachingEditResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Tiered Cache zone setting.
    pub value: TieredCachingEditResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredCachingGetResponse {
    /// The identifier of the caching setting.
    pub id: TieredCachingGetResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Tiered Cache zone setting.
    pub value: TieredCachingGetResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredCachingEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Enables Tiered Caching.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredCachingGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartRoutingEditResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartRoutingGetResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TieredCachingEditResponseID {
    #[serde(rename = "tiered_caching")]
    TieredCaching,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TieredCachingEditResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TieredCachingGetResponseID {
    #[serde(rename = "tiered_caching")]
    TieredCaching,
}

#[cfg(any(feature = "full", feature = "with-argo"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TieredCachingGetResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
