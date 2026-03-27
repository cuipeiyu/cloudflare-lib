#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeResponse {
    pub id: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParams<T> {
    pub zone_id: String,

    // Satisfied by [cache::CachePurgeParamsBodyCachePurgeFlexPurgeByTags],
    // [cache::CachePurgeParamsBodyCachePurgeFlexPurgeByHostnames],
    // [cache::CachePurgeParamsBodyCachePurgeFlexPurgeByPrefixes],
    // [cache::CachePurgeParamsBodyCachePurgeEverything],
    // [cache::CachePurgeParamsBodyCachePurgeSingleFile],
    // [cache::CachePurgeParamsBodyCachePurgeSingleFileWithURLAndHeaders],
    // [CachePurgeParamsBody]. // TODO
    pub body: T,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeFlexPurgeByTags {
    /// For more information on cache tags and purging by tags, please refer to
    /// [purge by cache-tags documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge-by-tags/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeFlexPurgeByHostnames {
    /// For more information purging by hostnames, please refer to
    /// [purge by hostname documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge-by-hostname/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeFlexPurgeByPrefixes {
    /// For more information on purging by prefixes, please refer to
    /// [purge by prefix documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge_by_prefix/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixes: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeEverything {
    /// For more information, please refer to
    /// [purge everything documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge-everything/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purge_everything: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeSingleFile {
    /// For more information on purging files, please refer to
    /// [purge by single-file documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge-by-single-file/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeSingleFileWithURLAndHeaders {
    /// For more information on purging files with URL and headers, please refer to
    /// [purge by single-file documentation page](https://developers.cloudflare.com/cache/how-to/purge-cache/purge-by-single-file/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<CachePurgeParamsBodyCachePurgeSingleFileWithURLAndHeadersFile>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBodyCachePurgeSingleFileWithURLAndHeadersFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePurgeParamsBody {
    // TODO
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveClearResponse {
    /// ID of the zone setting.
    pub id: CacheReserveClear,

    /// The time that the latest Cache Reserve Clear operation started.
    pub start_ts: DateTime<Utc>,

    /// The current state of the Cache Reserve Clear operation.
    pub state: State,

    /// The time that the latest Cache Reserve Clear operation completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<DateTime<Utc>>,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveEditResponse {
    /// The identifier of the caching setting.
    pub id: CacheReserve,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Cache Reserve zone setting.
    pub value: CacheReserveEditResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveGetResponse {
    /// The identifier of the caching setting.
    pub id: CacheReserve,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Cache Reserve zone setting.
    pub value: CacheReserveGetResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveStatusResponse {
    /// ID of the zone setting.
    pub id: CacheReserveClear,

    /// The time that the latest Cache Reserve Clear operation started.
    pub start_ts: DateTime<Utc>,

    /// The current state of the Cache Reserve Clear operation.
    pub state: State,

    /// The time that the latest Cache Reserve Clear operation completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<DateTime<Utc>>,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveClearParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Value of the Cache Reserve zone setting.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheReserveStatusParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTieredCacheEditResponse {
    /// The identifier of the caching setting.
    pub id: RegionalTieredCache,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Regional Tiered Cache zone setting.
    pub value: RegionalTieredCacheEditResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTieredCacheGetResponse {
    /// The identifier of the caching setting.
    pub id: RegionalTieredCache,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Regional Tiered Cache zone setting.
    pub value: RegionalTieredCacheGetResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTieredCacheEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Value of the Regional Tiered Cache zone setting.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalTieredCacheGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheDeleteResponse {
    /// The identifier of the caching setting.
    pub id: SmartTieredCacheDeleteResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheEditResponse {
    /// The identifier of the caching setting.
    pub id: SmartTieredCacheEditResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Smart Tiered Cache zone setting.
    pub value: SmartTieredCacheEditResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheGetResponse {
    /// The identifier of the caching setting.
    pub id: SmartTieredCacheGetResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the Smart Tiered Cache zone setting.
    pub value: SmartTieredCacheGetResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Enable or disable the Smart Tiered Cache.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartTieredCacheGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDeleteResponse {
    /// The identifier of the caching setting.
    pub id: VariantDeleteResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantEditResponse {
    /// The identifier of the caching setting.
    pub id: VariantEditResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the zone setting.
    pub value: VariantEditResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGetResponse {
    /// The identifier of the caching setting.
    pub id: VariantGetResponseID,

    /// Whether the setting is editable.
    pub editable: bool,

    /// Value of the zone setting.
    pub value: VariantGetResponseValue,

    /// Last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Value of the zone setting.
    pub value: String,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantEditResponseValue {
    /// List of strings with the MIME types of all the variants that should be served
    /// for avif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for bmp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bmp: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for gif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jp2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp2: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpeg: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpg: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpg2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpg2: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for png.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for tif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for tiff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiff: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for webp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webp: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGetResponseValue {
    /// List of strings with the MIME types of all the variants that should be served
    /// for avif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for bmp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bmp: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for gif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jp2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp2: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpeg: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpg: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for jpg2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpg2: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for png.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for tif.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for tiff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiff: Option<Vec<String>>,

    /// List of strings with the MIME types of all the variants that should be served
    /// for webp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webp: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheReserve {
    #[serde(rename = "cache_reserve")]
    CacheReserve,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheReserveClear {
    #[serde(rename = "cache_reserve_clear")]
    CacheReserveClear,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum State {
    #[serde(rename = "In-progress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionalTieredCache {
    #[serde(rename = "tc_regional")]
    TcRegional,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheReserveEditResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheReserveGetResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionalTieredCacheEditResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionalTieredCacheGetResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartTieredCacheDeleteResponseID {
    #[serde(rename = "tiered_cache_smart_topology_enable")]
    TieredCacheSmartTopologyEnable,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartTieredCacheEditResponseID {
    #[serde(rename = "tiered_cache_smart_topology_enable")]
    TieredCacheSmartTopologyEnable,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartTieredCacheEditResponseValue {
    #[serde(rename = "on")]
    SmartTieredCacheEditResponseValueOn,
    #[serde(rename = "off")]
    SmartTieredCacheEditResponseValueOff,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartTieredCacheGetResponseID {
    #[serde(rename = "tiered_cache_smart_topology_enable")]
    TieredCacheSmartTopologyEnable,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartTieredCacheGetResponseValue {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariantDeleteResponseID {
    #[serde(rename = "variants")]
    Variants,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariantEditResponseID {
    #[serde(rename = "variants")]
    Variants,
}

#[cfg(any(feature = "full", feature = "with-cache"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariantGetResponseID {
    #[serde(rename = "variants")]
    Variants,
}
