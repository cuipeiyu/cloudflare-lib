#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationCheckTriggerResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationCheckTriggerParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponse {
    pub errors: Vec<CustomNameserverGetResponseError>,

    pub messages: Vec<CustomNameserverGetResponseMessage>,

    /// Whether the API call was successful.
    pub success: CustomNameserverGetResponseSuccess,

    /// Whether zone uses account-level custom nameservers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The number of the name server set to assign to the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_set: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<CustomNameserverGetResponseResultInfo>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Whether zone uses account-level custom nameservers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// The number of the name server set to assign to the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_set: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneHold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If provided, the zone hold will extend to block any subdomain of the given zone,
    /// as well as SSL4SaaS Custom Hostnames. For example, a zone hold on a zone with
    /// the hostname 'example.com' and include_subdomains=true will block 'example.com',
    /// 'staging.example.com', 'api.staging.example.com', etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If `hold_after` is provided, the hold will be temporarily disabled, then
    /// automatically re-enabled by the system at the time specified in this
    /// RFC3339-formatted timestamp. Otherwise, the hold will be disabled indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_after: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If `hold_after` is provided and future-dated, the hold will be temporarily
    /// disabled, then automatically re-enabled by the system at the time specified in
    /// this RFC3339-formatted timestamp. A past-dated `hold_after` value will have no
    /// effect on an existing, enabled hold. Providing an empty string will set its
    /// value to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_after: Option<String>,

    /// If `true`, the zone hold will extend to block any subdomain of the given zone,
    /// as well as SSL4SaaS Custom Hostnames. For example, a zone hold on a zone with
    /// the hostname 'example.com' and include_subdomains=true will block 'example.com',
    /// 'staging.example.com', 'api.staging.example.com', etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableRatePlan {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Indicates whether you can subscribe to this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_subscribe: Option<bool>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Indicates whether this plan is managed externally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,

    /// The frequency at which you will be billed for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<AvailableRatePlanFrequency>,

    /// Indicates whether you are currently subscribed to this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,

    /// Indicates whether this plan has a legacy discount applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_discount: Option<bool>,

    /// The legacy identifier for this rate plan, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,

    /// The plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The amount you will be billed for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanGetResponse {
    /// Plan identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Array of available components values for the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<RatePlanGetResponseComponent>>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The duration of the plan subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// The frequency at which you will be billed for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RatePlanGetResponseFrequency>,

    /// The plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedDDoS {
    /// ID of the zone setting.
    pub id: AdvancedDDoSID,

    /// Current value of the zone setting.
    pub value: AdvancedDDoSValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<AdvancedDDoSEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlwaysOnline {
    /// ID of the zone setting.
    pub id: AlwaysOnlineID,

    /// Current value of the zone setting.
    pub value: AlwaysOnlineValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<AlwaysOnlineEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlwaysUseHTTPS {
    /// If enabled, any ` http://“ URL is converted to  `https://` through a 301
    /// redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AlwaysUseHTTPSID>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlwaysUseHTTPSParam {
    /// If enabled, any ` http://“ URL is converted to  `https://` through a 301
    /// redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticHTTPSRewrites {
    /// Turn on or off Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<AutomaticHTTPSRewritesID>,

    /// The status of Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AutomaticHTTPSRewritesValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticHTTPSRewritesParam {
    /// Turn on or off Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticPlatformOptimization {
    /// Indicates whether or not
    /// [cache by device type](https://developers.cloudflare.com/automatic-platform-optimization/reference/cache-device-type/)
    /// is enabled.
    pub cache_by_device_type: bool,

    /// Indicates whether or not Cloudflare proxy is enabled.
    pub cf: bool,

    /// Indicates whether or not Automatic Platform Optimization is enabled.
    pub enabled: bool,

    /// An array of hostnames where Automatic Platform Optimization for WordPress is
    /// activated.
    pub hostnames: Vec<String>,

    /// Indicates whether or not site is powered by WordPress.
    pub wordpress: bool,

    /// Indicates whether or not
    /// [Cloudflare for WordPress plugin](https://wordpress.org/plugins/cloudflare/) is
    /// installed.
    pub wp_plugin: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticPlatformOptimizationParam {
    /// Indicates whether or not
    /// [cache by device type](https://developers.cloudflare.com/automatic-platform-optimization/reference/cache-device-type/)
    /// is enabled.
    pub cache_by_device_type: String,

    /// Indicates whether or not Cloudflare proxy is enabled.
    pub cf: String,

    /// Indicates whether or not Automatic Platform Optimization is enabled.
    pub enabled: String,

    /// An array of hostnames where Automatic Platform Optimization for WordPress is
    /// activated.
    pub hostnames: String,

    /// Indicates whether or not site is powered by WordPress.
    pub wordpress: String,

    /// Indicates whether or not
    /// [Cloudflare for WordPress plugin](https://wordpress.org/plugins/cloudflare/) is
    /// installed.
    pub wp_plugin: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brotli {
    /// ID of the zone setting.
    pub id: BrotliID,

    /// Current value of the zone setting.
    pub value: BrotliValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<BrotliEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCacheTTL {
    /// Control how long resources cached by client browsers remain valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BrowserCacheTTLID>,

    /// The number of seconds to cache resources for. Setting this to 0 enables "Respect
    /// Existing Headers".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCacheTTLParam {
    /// Control how long resources cached by client browsers remain valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The number of seconds to cache resources for. Setting this to 0 enables "Respect
    /// Existing Headers".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCheck {
    /// Inspect the visitor's browser for headers commonly associated with spammers and
    /// certain bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BrowserCheckID>,

    /// The status of Browser Integrity Check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<BrowserCheckValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCheckParam {
    /// Inspect the visitor's browser for headers commonly associated with spammers and
    /// certain bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Browser Integrity Check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLevel {
    /// Apply custom caching based on the option selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CacheLevelID>,

    /// - `bypass`: Cloudflare does not cache.
    /// - `basic`: Delivers resources from cache when there is no query string.
    /// - `simplified`: Delivers the same resource to everyone independent of the query
    /// string.
    /// - `aggressive`: Caches all static content that has a query string.
    /// - `cache_everything`: Treats all content as static and caches all file types
    /// beyond the
    /// [Cloudflare default cached content](https://developers.cloudflare.com/cache/concepts/default-cache-behavior/#default-cached-file-extensions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<CacheLevelValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLevelParam {
    /// Apply custom caching based on the option selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// - `bypass`: Cloudflare does not cache.
    /// - `basic`: Delivers resources from cache when there is no query string.
    /// - `simplified`: Delivers the same resource to everyone independent of the query
    /// string.
    /// - `aggressive`: Caches all static content that has a query string.
    /// - `cache_everything`: Treats all content as static and caches all file types
    /// beyond the
    /// [Cloudflare default cached content](https://developers.cloudflare.com/cache/concepts/default-cache-behavior/#default-cached-file-extensions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeTTL {
    /// ID of the zone setting.
    pub id: ChallengeTTLID,

    /// Current value of the zone setting.
    pub value: ChallengeTTLValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<ChallengeTTLEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ciphers {
    /// ID of the zone setting.
    pub id: CiphersID,

    /// Current value of the zone setting.
    pub value: Vec<String>,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<CiphersEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentMode {
    /// ID of the zone setting.
    pub id: DevelopmentModeID,

    /// Current value of the zone setting.
    pub value: DevelopmentModeValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<DevelopmentModeEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Value of the zone setting. Notes: The interval (in seconds) from when
    /// development mode expires (positive integer) or last expired (negative integer)
    /// for the domain. If development mode has never been enabled, this value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remaining: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyHints {
    /// ID of the zone setting.
    pub id: EarlyHintsID,

    /// Current value of the zone setting.
    pub value: EarlyHintsValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<EarlyHintsEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailObfuscation {
    /// Turn on or off **Email Obfuscation**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EmailObfuscationID>,

    /// The status of Email Obfuscation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EmailObfuscationValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailObfuscationParam {
    /// Turn on or off **Email Obfuscation**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Email Obfuscation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotlinkProtection {
    /// ID of the zone setting.
    pub id: HotlinkProtectionID,

    /// Current value of the zone setting.
    pub value: HotlinkProtectionValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<HotlinkProtectionEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResizing {
    /// ID of the zone setting.
    pub id: ImageResizingID,

    /// Current value of the zone setting.
    pub value: ImageResizingValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<ImageResizingEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGeolocation {
    /// Cloudflare adds a CF-IPCountry HTTP header containing the country code that
    /// corresponds to the visitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IPGeolocationID>,

    /// The status of adding the IP Geolocation Header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<IPGeolocationValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGeolocationParam {
    /// Cloudflare adds a CF-IPCountry HTTP header containing the country code that
    /// corresponds to the visitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of adding the IP Geolocation Header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinTLSVersion {
    /// ID of the zone setting.
    pub id: MinTLSVersionID,

    /// Current value of the zone setting.
    pub value: MinTLSVersionValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<MinTLSVersionEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mirage {
    /// Cloudflare Mirage reduces bandwidth used by images in mobile browsers. It can
    /// accelerate loading of image-heavy websites on very slow mobile connections and
    /// HTTP/1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<MirageID>,

    /// The status of Mirage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MirageValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirageParam {
    /// Cloudflare Mirage reduces bandwidth used by images in mobile browsers. It can
    /// accelerate loading of image-heavy websites on very slow mobile connections and
    /// HTTP/1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Mirage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NEL {
    /// Zone setting identifier.
    pub id: NELID,

    /// Current value of the zone setting.
    pub value: NELValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<NELEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunisticEncryption {
    /// Opportunistic Encryption allows browsers to access HTTP URIs over an encrypted
    /// TLS channel. It's not a substitute for HTTPS, but provides additional security
    /// for otherwise vulnerable requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<OpportunisticEncryptionID>,

    /// The status of Opportunistic Encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<OpportunisticEncryptionValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunisticEncryptionParam {
    /// Opportunistic Encryption allows browsers to access HTTP URIs over an encrypted
    /// TLS channel. It's not a substitute for HTTPS, but provides additional security
    /// for otherwise vulnerable requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Opportunistic Encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunisticOnion {
    /// ID of the zone setting.
    pub id: OpportunisticOnionID,

    /// Current value of the zone setting.
    pub value: OpportunisticOnionValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<OpportunisticOnionEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrangeToOrange {
    /// ID of the zone setting.
    pub id: OrangeToOrangeID,

    /// Current value of the zone setting.
    pub value: OrangeToOrangeValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<OrangeToOrangeEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginErrorPagePassThru {
    /// Turn on or off Cloudflare error pages generated from issues sent from the origin
    /// server. If enabled, this setting triggers error pages issued by the origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<OriginErrorPagePassThruID>,

    /// The status of Origin Error Page Passthru.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<OriginErrorPagePassThruValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginErrorPagePassThruParam {
    /// Turn on or off Cloudflare error pages generated from issues sent from the origin
    /// server. If enabled, this setting triggers error pages issued by the origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Origin Error Page Passthru.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polish {
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PolishID>,

    /// The level of Polish you want applied to your origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<PolishValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolishParam {
    /// Apply options from the Polish feature of the Cloudflare Speed app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The level of Polish you want applied to your origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefetchPreload {
    /// ID of the zone setting.
    pub id: PrefetchPreloadID,

    /// Current value of the zone setting.
    pub value: PrefetchPreloadValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<PrefetchPreloadEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyReadTimeout {
    /// ID of the zone setting.
    pub id: ProxyReadTimeoutID,

    /// Current value of the zone setting.
    pub value: f64,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<ProxyReadTimeoutEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBuffering {
    /// Turn on or off whether Cloudflare should wait for an entire file from the origin
    /// server before forwarding it to the site visitor. By default, Cloudflare sends
    /// packets to the client as they arrive from the origin server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ResponseBufferingID>,

    /// The status of Response Buffering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ResponseBufferingValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBufferingParam {
    /// Turn on or off whether Cloudflare should wait for an entire file from the origin
    /// server before forwarding it to the site visitor. By default, Cloudflare sends
    /// packets to the client as they arrive from the origin server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Response Buffering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RocketLoader {
    /// Turn on or off Rocket Loader in the Cloudflare Speed app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RocketLoaderID>,

    /// The status of Rocket Loader
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RocketLoaderValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RocketLoaderParam {
    /// Turn on or off Rocket Loader in the Cloudflare Speed app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Rocket Loader
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeaders {
    /// ID of the zone's security header.
    pub id: SecurityHeadersID,

    /// Current value of the zone setting.
    pub value: SecurityHeadersValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<SecurityHeadersEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLevel {
    /// Control options for the **Security Level** feature from the **Security** app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SecurityLevelID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SecurityLevelValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLevelParam {
    /// Control options for the **Security Level** feature from the **Security** app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSideExcludes {
    /// ID of the zone setting.
    pub id: ServerSideExcludesID,

    /// Current value of the zone setting.
    pub value: ServerSideExcludesValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<ServerSideExcludesEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortQueryStringForCache {
    /// Turn on or off the reordering of query strings. When query strings have the same
    /// structure, caching improves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SortQueryStringForCacheID>,

    /// The status of Query String Sort
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SortQueryStringForCacheValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortQueryStringForCacheParam {
    /// Turn on or off the reordering of query strings. When query strings have the same
    /// structure, caching improves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of Query String Sort
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSL {
    /// Control options for the SSL feature of the Edge Certificates tab in the
    /// Cloudflare SSL/TLS app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SSLID>,

    /// The encryption mode that Cloudflare uses to connect to your origin server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SSLValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLParam {
    /// Control options for the SSL feature of the Edge Certificates tab in the
    /// Cloudflare SSL/TLS app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The encryption mode that Cloudflare uses to connect to your origin server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLRecommender {
    /// Enrollment value for SSL/TLS Recommender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SSLRecommenderID>,

    /// ssl-recommender enrollment setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSClientAuth {
    /// ID of the zone setting.
    pub id: TLSClientAuthID,

    /// Current value of the zone setting.
    pub value: TLSClientAuthValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<TLSClientAuthEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrueClientIPHeader {
    /// Turn on or off the True-Client-IP Header feature of the Cloudflare Network app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TrueClientIPHeaderID>,

    /// The status of True Client IP Header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TrueClientIPHeaderValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrueClientIPHeaderParam {
    /// Turn on or off the True-Client-IP Header feature of the Cloudflare Network app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of True Client IP Header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WAF {
    /// Turn on or off
    /// [WAF managed rules (previous version, deprecated)](https://developers.cloudflare.com/waf/reference/legacy/old-waf-managed-rules/).
    /// You cannot enable or disable individual WAF managed rules via Page Rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<WAFID>,

    /// The status of WAF managed rules (previous version).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<WAFValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WAFParam {
    /// Turn on or off
    /// [WAF managed rules (previous version, deprecated)](https://developers.cloudflare.com/waf/reference/legacy/old-waf-managed-rules/).
    /// You cannot enable or disable individual WAF managed rules via Page Rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The status of WAF managed rules (previous version).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebP {
    /// ID of the zone setting.
    pub id: WebPID,

    /// Current value of the zone setting.
    pub value: WebPValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<WebPEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Websocket {
    /// ID of the zone setting.
    pub id: WebsocketID,

    /// Current value of the zone setting.
    pub value: WebsocketValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<WebsocketEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroRTT {
    /// ID of the zone setting.
    pub id: ZeroRTTID,

    /// Current value of the zone setting.
    pub value: ZeroRTTValue,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<ZeroRTTEditable>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingEditResponse {
    /// ID of the zone setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SettingEditResponseID>,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<SettingEditResponseEditable>,

    /// ssl-recommender enrollment setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Value of the zone setting. Notes: The interval (in seconds) from when
    /// development mode expires (positive integer) or last expired (negative integer)
    /// for the domain. If development mode has never been enabled, this value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remaining: Option<f64>,

    /// This field can have the runtime type of [ZeroRTTValue], [AdvancedDDoSValue],
    /// [SettingEditResponseZonesCacheRulesAegisValue], [AlwaysOnlineValue],
    /// [SettingEditResponseZonesSchemasAlwaysUseHTTPSValue],
    /// [SettingEditResponseZonesSchemasAutomaticHTTPSRewritesValue], [BrotliValue],
    /// [int64], [SettingEditResponseZonesSchemasBrowserCheckValue],
    /// [SettingEditResponseZonesSchemasCacheLevelValue], [ChallengeTTLValue],
    /// [SettingEditResponseZonesChinaNetworkEnabledValue],
    /// [SettingEditResponseZonesContentConverterValue], [[]string],
    /// [SettingEditResponseZonesCNAMEFlatteningValue], [DevelopmentModeValue],
    /// [EarlyHintsValue], [SettingEditResponseZonesSchemasEdgeCacheTTLValue],
    /// [SettingEditResponseZonesSchemasEmailObfuscationValue], [H2PrioritizationValue],
    /// [HotlinkProtectionValue], [HTTP2Value], [HTTP3Value], [ImageResizingValue],
    /// [SettingEditResponseZonesSchemasIPGeolocationValue], [IPV6Value],
    /// [SettingEditResponseZonesMaxUploadValue], [MinTLSVersionValue],
    /// [SettingEditResponseZonesSchemasMirageValue], [NELValue],
    /// [SettingEditResponseZonesSchemasOpportunisticEncryptionValue],
    /// [OpportunisticOnionValue], [OrangeToOrangeValue],
    /// [SettingEditResponseZonesSchemasOriginErrorPagePassThruValue],
    /// [SettingEditResponseZonesCacheRulesOriginMaxHTTPVersionValue],
    /// [SettingEditResponseZonesSchemasPolishValue], [PrefetchPreloadValue],
    /// [SettingEditResponseZonesPrivacyPassValue], [float64], [PseudoIPV4Value],
    /// [SettingEditResponseZonesReplaceInsecureJSValue],
    /// [SettingEditResponseZonesSchemasResponseBufferingValue],
    /// [SettingEditResponseZonesSchemasRocketLoaderValue],
    /// [AutomaticPlatformOptimization], [SecurityHeadersValue],
    /// [SettingEditResponseZonesSchemasSecurityLevelValue], [ServerSideExcludesValue],
    /// [SettingEditResponseZonesSha1SupportValue],
    /// [SettingEditResponseZonesSchemasSortQueryStringForCacheValue],
    /// [SettingEditResponseZonesSchemasSSLValue],
    /// [SettingEditResponseZonesTLS1_2OnlyValue], [TLS1_3Value], [TLSClientAuthValue],
    /// [SettingEditResponseZonesTransformationsValue], [string],
    /// [SettingEditResponseZonesSchemasTrueClientIPHeaderValue],
    /// [SettingEditResponseZonesSchemasWAFValue], [WebPValue], [WebsocketValue].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingGetResponse {
    /// ID of the zone setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SettingGetResponseID>,

    /// Whether or not this setting can be modified for this zone (based on your
    /// Cloudflare plan level).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<SettingGetResponseEditable>,

    /// ssl-recommender enrollment setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// last time this setting was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Value of the zone setting. Notes: The interval (in seconds) from when
    /// development mode expires (positive integer) or last expired (negative integer)
    /// for the domain. If development mode has never been enabled, this value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remaining: Option<f64>,

    /// This field can have the runtime type of [ZeroRTTValue], [AdvancedDDoSValue],
    /// [SettingGetResponseZonesCacheRulesAegisValue], [AlwaysOnlineValue],
    /// [SettingGetResponseZonesSchemasAlwaysUseHTTPSValue],
    /// [SettingGetResponseZonesSchemasAutomaticHTTPSRewritesValue], [BrotliValue],
    /// [int64], [SettingGetResponseZonesSchemasBrowserCheckValue],
    /// [SettingGetResponseZonesSchemasCacheLevelValue], [ChallengeTTLValue],
    /// [SettingGetResponseZonesChinaNetworkEnabledValue],
    /// [SettingGetResponseZonesContentConverterValue], [[]string],
    /// [SettingGetResponseZonesCNAMEFlatteningValue], [DevelopmentModeValue],
    /// [EarlyHintsValue], [SettingGetResponseZonesSchemasEdgeCacheTTLValue],
    /// [SettingGetResponseZonesSchemasEmailObfuscationValue], [H2PrioritizationValue],
    /// [HotlinkProtectionValue], [HTTP2Value], [HTTP3Value], [ImageResizingValue],
    /// [SettingGetResponseZonesSchemasIPGeolocationValue], [IPV6Value],
    /// [SettingGetResponseZonesMaxUploadValue], [MinTLSVersionValue],
    /// [SettingGetResponseZonesSchemasMirageValue], [NELValue],
    /// [SettingGetResponseZonesSchemasOpportunisticEncryptionValue],
    /// [OpportunisticOnionValue], [OrangeToOrangeValue],
    /// [SettingGetResponseZonesSchemasOriginErrorPagePassThruValue],
    /// [SettingGetResponseZonesCacheRulesOriginMaxHTTPVersionValue],
    /// [SettingGetResponseZonesSchemasPolishValue], [PrefetchPreloadValue],
    /// [SettingGetResponseZonesPrivacyPassValue], [float64], [PseudoIPV4Value],
    /// [SettingGetResponseZonesReplaceInsecureJSValue],
    /// [SettingGetResponseZonesSchemasResponseBufferingValue],
    /// [SettingGetResponseZonesSchemasRocketLoaderValue],
    /// [AutomaticPlatformOptimization], [SecurityHeadersValue],
    /// [SettingGetResponseZonesSchemasSecurityLevelValue], [ServerSideExcludesValue],
    /// [SettingGetResponseZonesSha1SupportValue],
    /// [SettingGetResponseZonesSchemasSortQueryStringForCacheValue],
    /// [SettingGetResponseZonesSchemasSSLValue],
    /// [SettingGetResponseZonesTLS1_2OnlyValue], [TLS1_3Value], [TLSClientAuthValue],
    /// [SettingGetResponseZonesTransformationsValue], [string],
    /// [SettingGetResponseZonesSchemasTrueClientIPHeaderValue],
    /// [SettingGetResponseZonesSchemasWAFValue], [WebPValue], [WebsocketValue].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingEditParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: SettingEditParamsBodyUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewResponse {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The end of the current period and also when the next billing is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<DateTime<Utc>>,

    /// When the current billing period started. May match initial_period_start if this
    /// is the first period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<DateTime<Utc>>,

    /// How often the subscription is renewed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SubscriptionNewResponseFrequency>,

    /// The price of the subscription that will be billed, in US dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// The rate plan applied to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_plan: Option<crate::shared::RatePlan>,

    /// The state that the subscription is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SubscriptionNewResponseState>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateResponse {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The end of the current period and also when the next billing is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<DateTime<Utc>>,

    /// When the current billing period started. May match initial_period_start if this
    /// is the first period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<DateTime<Utc>>,

    /// How often the subscription is renewed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SubscriptionUpdateResponseFrequency>,

    /// The price of the subscription that will be billed, in US dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// The rate plan applied to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_plan: Option<crate::shared::RatePlan>,

    /// The state that the subscription is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SubscriptionUpdateResponseState>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetResponse {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The end of the current period and also when the next billing is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<DateTime<Utc>>,

    /// When the current billing period started. May match initial_period_start if this
    /// is the first period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<DateTime<Utc>>,

    /// How often the subscription is renewed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SubscriptionGetResponseFrequency>,

    /// The price of the subscription that will be billed, in US dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// The rate plan applied to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_plan: Option<crate::shared::RatePlan>,

    /// The state that the subscription is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SubscriptionGetResponseState>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub subscription: crate::shared::SubscriptionParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub subscription: crate::shared::SubscriptionParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Identifier
    pub id: String,

    /// The account the zone belongs to.
    pub account: ZoneAccount,

    /// The last time proof of ownership was detected and the zone was made active.
    pub activated_on: DateTime<Utc>,

    /// When the zone was created.
    pub created_on: DateTime<Utc>,

    /// The interval (in seconds) from when development mode expires (positive integer)
    /// or last expired (negative integer) for the domain. If development mode has never
    /// been enabled, this value is 0.
    pub development_mode: f64,

    /// Metadata about the zone.
    pub meta: ZoneMeta,

    /// When the zone was last modified.
    pub modified_on: DateTime<Utc>,

    /// The domain name. Per
    /// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the
    /// overall zone name can be up to 253 characters, with each segment ("label") not
    /// exceeding 63 characters.
    pub name: String,

    /// The name servers Cloudflare assigns to a zone.
    pub name_servers: Vec<String>,

    /// DNS host at the time of switching to Cloudflare.
    pub original_dnshost: String,

    /// Original name servers before moving to Cloudflare.
    pub original_name_servers: Vec<String>,

    /// Registrar for the domain at the time of switching to Cloudflare.
    pub original_registrar: String,

    /// The owner of the zone.
    pub owner: ZoneOwner,

    /// A Zones subscription information.
    /// Deprecated: Please use the `/zones/{zone_id}/subscription` API to update a
    /// zone's plan. Changing this value will create/cancel associated subscriptions. To
    /// view available plans for this zone, see
    /// [Zone Plans](https://developers.cloudflare.com/api/resources/zones/subresources/plans/).
    pub plan: ZonePlan,

    /// Allows the customer to use a custom apex. _Tenants Only Configuration_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_suffix: Option<String>,

    /// Indicates whether the zone is only using Cloudflare DNS services. A true value
    /// means the zone will not receive security or performance benefits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,

    /// Legacy permissions based on legacy user membership information.
    /// Deprecated: This has been replaced by Account memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    /// The zone status on Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ZoneStatus>,

    /// The root organizational unit that this zone belongs to (such as a tenant or
    /// organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<ZoneTenant>,

    /// The immediate parent organizational unit that this zone belongs to (such as
    /// under a tenant or sub-organization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_unit: Option<ZoneTenantUnit>,

    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is
    /// typically a partner-hosted zone or a CNAME setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,

    /// An array of domains used for custom name servers. This is only available for
    /// Business and Enterprise plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_name_servers: Option<Vec<String>>,

    /// Verification key for partial zone setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_key: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDeleteResponse {
    /// Identifier
    pub id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneNewParams {
    pub account: String,

    /// The domain name. Per
    /// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the
    /// overall zone name can be up to 253 characters, with each segment ("label") not
    /// exceeding 63 characters.
    pub name: String,

    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is
    /// typically a partner-hosted zone or a CNAME setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Direction to order zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Whether to match all search requirements or at least one (any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    /// A domain name. Optional filter operators can be provided to extend refine the
    /// search:
    /// - `equal` (default)
    /// - `not_equal`
    /// - `starts_with`
    /// - `ends_with`
    /// - `contains`
    /// - `starts_with_case_sensitive`
    /// - `ends_with_case_sensitive`
    /// - `contains_case_sensitive`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to order zones by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of zones per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Specify a zone status to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneEditParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Indicates whether the zone is only using Cloudflare DNS services. A true value
    /// means the zone will not receive security or performance benefits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<String>,

    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is
    /// typically a partner-hosted zone or a CNAME setup. This parameter is only
    /// available to Enterprise customers or if it has been explicitly enabled on a
    /// zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// An array of domains used for custom name servers. This is only available for
    /// Business and Enterprise plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_name_servers: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CustomNameserverGetResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CustomNameserverGetResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponseResultInfo {
    /// Total number of results for the requested service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

    /// Current page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,

    /// Number of results per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<f64>,

    /// Total results available without any search parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanGetResponseComponent {
    /// The default amount allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f64>,

    /// The unique component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<RatePlanGetResponseComponentsName>,

    /// The unit price of the addon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NELValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersValue {
    /// Strict Transport Security.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_transport_security: Option<SecurityHeadersValueStrictTransportSecurity>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneAccount {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneMeta {
    /// The zone is only configured for CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_only: Option<bool>,

    /// Number of Custom Certificates the zone can have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_certificate_quota: Option<i64>,

    /// The zone is only configured for DNS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_only: Option<bool>,

    /// The zone is setup with Foundation DNS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_dns: Option<bool>,

    /// Number of Page Rules a zone can have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_rule_quota: Option<i64>,

    /// The zone has been flagged for phishing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phishing_detected: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneOwner {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZonePlan {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// States if the subscription can be activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_subscribe: Option<bool>,

    /// The denomination of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// If this Zone is managed by another company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,

    /// How often the customer is billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,

    /// States if the subscription active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,

    /// If the legacy discount applies to this Zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_discount: Option<bool>,

    /// The legacy name of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,

    /// Name of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// How much the customer is paying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTenant {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the Tenant account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTenantUnit {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomNameserverGetResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersValueStrictTransportSecurity {
    /// Whether or not strict transport security is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Include all subdomains for strict transport security.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,

    /// Max age in seconds of the strict transport security.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<f64>,

    /// Whether or not to include 'X-Content-Type-Options: nosniff' header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosniff: Option<bool>,

    /// Enable automatic preload of the HSTS configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<bool>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    #[serde(rename = "full")]
    TypeFull,
    #[serde(rename = "partial")]
    TypePartial,
    #[serde(rename = "secondary")]
    TypeSecondary,
    #[serde(rename = "internal")]
    TypeInternal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AvailableRatePlanFrequency {
    #[serde(rename = "weekly")]
    AvailableRatePlanFrequencyWeekly,
    #[serde(rename = "monthly")]
    AvailableRatePlanFrequencyMonthly,
    #[serde(rename = "quarterly")]
    AvailableRatePlanFrequencyQuarterly,
    #[serde(rename = "yearly")]
    AvailableRatePlanFrequencyYearly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RatePlanGetResponseFrequency {
    #[serde(rename = "weekly")]
    RatePlanGetResponseFrequencyWeekly,
    #[serde(rename = "monthly")]
    RatePlanGetResponseFrequencyMonthly,
    #[serde(rename = "quarterly")]
    RatePlanGetResponseFrequencyQuarterly,
    #[serde(rename = "yearly")]
    RatePlanGetResponseFrequencyYearly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvancedDDoSID {
    #[serde(rename = "advanced_ddos")]
    AdvancedDDoSIDAdvancedDDoS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvancedDDoSValue {
    #[serde(rename = "on")]
    AdvancedDDoSValueOn,
    #[serde(rename = "off")]
    AdvancedDDoSValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlwaysOnlineID {
    #[serde(rename = "always_online")]
    AlwaysOnlineIDAlwaysOnline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlwaysOnlineValue {
    #[serde(rename = "on")]
    AlwaysOnlineValueOn,
    #[serde(rename = "off")]
    AlwaysOnlineValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlwaysUseHTTPSID {
    #[serde(rename = "always_use_https")]
    AlwaysUseHTTPSIDAlwaysUseHTTPS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutomaticHTTPSRewritesID {
    #[serde(rename = "automatic_https_rewrites")]
    AutomaticHTTPSRewritesIDAutomaticHTTPSRewrites,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutomaticHTTPSRewritesValue {
    #[serde(rename = "on")]
    AutomaticHTTPSRewritesValueOn,
    #[serde(rename = "off")]
    AutomaticHTTPSRewritesValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrotliID {
    #[serde(rename = "brotli")]
    BrotliIDBrotli,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrotliValue {
    #[serde(rename = "off")]
    BrotliValueOff,
    #[serde(rename = "on")]
    BrotliValueOn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrowserCacheTTLID {
    #[serde(rename = "browser_cache_ttl")]
    BrowserCacheTTLIDBrowserCacheTTL,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrowserCheckID {
    #[serde(rename = "browser_check")]
    BrowserCheckIDBrowserCheck,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrowserCheckValue {
    #[serde(rename = "on")]
    BrowserCheckValueOn,
    #[serde(rename = "off")]
    BrowserCheckValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheLevelID {
    #[serde(rename = "cache_level")]
    CacheLevelIDCacheLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheLevelValue {
    #[serde(rename = "bypass")]
    CacheLevelValueBypass,
    #[serde(rename = "basic")]
    CacheLevelValueBasic,
    #[serde(rename = "simplified")]
    CacheLevelValueSimplified,
    #[serde(rename = "aggressive")]
    CacheLevelValueAggressive,
    #[serde(rename = "cache_everything")]
    CacheLevelValueCacheEverything,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChallengeTTLID {
    #[serde(rename = "challenge_ttl")]
    ChallengeTTLIDChallengeTTL,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CiphersID {
    #[serde(rename = "ciphers")]
    CiphersIDCiphers,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DevelopmentModeID {
    #[serde(rename = "development_mode")]
    DevelopmentModeIDDevelopmentMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DevelopmentModeValue {
    #[serde(rename = "on")]
    DevelopmentModeValueOn,
    #[serde(rename = "off")]
    DevelopmentModeValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EarlyHintsID {
    #[serde(rename = "early_hints")]
    EarlyHintsIDEarlyHints,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EarlyHintsValue {
    #[serde(rename = "on")]
    EarlyHintsValueOn,
    #[serde(rename = "off")]
    EarlyHintsValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailObfuscationID {
    #[serde(rename = "email_obfuscation")]
    EmailObfuscationIDEmailObfuscation,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailObfuscationValue {
    #[serde(rename = "on")]
    EmailObfuscationValueOn,
    #[serde(rename = "off")]
    EmailObfuscationValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HotlinkProtectionID {
    #[serde(rename = "hotlink_protection")]
    HotlinkProtectionIDHotlinkProtection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HotlinkProtectionValue {
    #[serde(rename = "on")]
    HotlinkProtectionValueOn,
    #[serde(rename = "off")]
    HotlinkProtectionValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImageResizingID {
    #[serde(rename = "image_resizing")]
    ImageResizingIDImageResizing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImageResizingValue {
    #[serde(rename = "on")]
    ImageResizingValueOn,
    #[serde(rename = "off")]
    ImageResizingValueOff,
    #[serde(rename = "open")]
    ImageResizingValueOpen,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IPGeolocationID {
    #[serde(rename = "ip_geolocation")]
    IPGeolocationIDIPGeolocation,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IPGeolocationValue {
    #[serde(rename = "on")]
    IPGeolocationValueOn,
    #[serde(rename = "off")]
    IPGeolocationValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MinTLSVersionID {
    #[serde(rename = "min_tls_version")]
    MinTLSVersionIDMinTLSVersion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MinTLSVersionValue {
    #[serde(rename = "1.0")]
    MinTLSVersionValue1_0,
    #[serde(rename = "1.1")]
    MinTLSVersionValue1_1,
    #[serde(rename = "1.2")]
    MinTLSVersionValue1_2,
    #[serde(rename = "1.3")]
    MinTLSVersionValue1_3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MirageID {
    #[serde(rename = "mirage")]
    MirageIDMirage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MirageValue {
    #[serde(rename = "on")]
    MirageValueOn,
    #[serde(rename = "off")]
    MirageValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NELID {
    #[serde(rename = "nel")]
    NELIDNEL,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpportunisticEncryptionID {
    #[serde(rename = "opportunistic_encryption")]
    OpportunisticEncryptionIDOpportunisticEncryption,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpportunisticEncryptionValue {
    #[serde(rename = "on")]
    OpportunisticEncryptionValueOn,
    #[serde(rename = "off")]
    OpportunisticEncryptionValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpportunisticOnionID {
    #[serde(rename = "opportunistic_onion")]
    OpportunisticOnionIDOpportunisticOnion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpportunisticOnionValue {
    #[serde(rename = "on")]
    OpportunisticOnionValueOn,
    #[serde(rename = "off")]
    OpportunisticOnionValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrangeToOrangeID {
    #[serde(rename = "orange_to_orange")]
    OrangeToOrangeIDOrangeToOrange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrangeToOrangeValue {
    #[serde(rename = "on")]
    OrangeToOrangeValueOn,
    #[serde(rename = "off")]
    OrangeToOrangeValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OriginErrorPagePassThruID {
    #[serde(rename = "origin_error_page_pass_thru")]
    OriginErrorPagePassThruIDOriginErrorPagePassThru,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OriginErrorPagePassThruValue {
    #[serde(rename = "on")]
    OriginErrorPagePassThruValueOn,
    #[serde(rename = "off")]
    OriginErrorPagePassThruValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolishID {
    #[serde(rename = "polish")]
    PolishIDPolish,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolishValue {
    #[serde(rename = "off")]
    PolishValueOff,
    #[serde(rename = "lossless")]
    PolishValueLossless,
    #[serde(rename = "lossy")]
    PolishValueLossy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrefetchPreloadID {
    #[serde(rename = "prefetch_preload")]
    PrefetchPreloadIDPrefetchPreload,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrefetchPreloadValue {
    #[serde(rename = "on")]
    PrefetchPreloadValueOn,
    #[serde(rename = "off")]
    PrefetchPreloadValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProxyReadTimeoutID {
    #[serde(rename = "proxy_read_timeout")]
    ProxyReadTimeoutIDProxyReadTimeout,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseBufferingID {
    #[serde(rename = "response_buffering")]
    ResponseBufferingIDResponseBuffering,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseBufferingValue {
    #[serde(rename = "on")]
    ResponseBufferingValueOn,
    #[serde(rename = "off")]
    ResponseBufferingValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RocketLoaderID {
    #[serde(rename = "rocket_loader")]
    RocketLoaderIDRocketLoader,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RocketLoaderValue {
    #[serde(rename = "on")]
    RocketLoaderValueOn,
    #[serde(rename = "off")]
    RocketLoaderValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityHeadersID {
    #[serde(rename = "security_header")]
    SecurityHeadersIDSecurityHeader,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevelID {
    #[serde(rename = "security_level")]
    SecurityLevelIDSecurityLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevelValue {
    #[serde(rename = "off")]
    SecurityLevelValueOff,
    #[serde(rename = "essentially_off")]
    SecurityLevelValueEssentiallyOff,
    #[serde(rename = "low")]
    SecurityLevelValueLow,
    #[serde(rename = "medium")]
    SecurityLevelValueMedium,
    #[serde(rename = "high")]
    SecurityLevelValueHigh,
    #[serde(rename = "under_attack")]
    SecurityLevelValueUnderAttack,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerSideExcludesID {
    #[serde(rename = "server_side_exclude")]
    ServerSideExcludesIDServerSideExclude,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerSideExcludesValue {
    #[serde(rename = "on")]
    ServerSideExcludesValueOn,
    #[serde(rename = "off")]
    ServerSideExcludesValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortQueryStringForCacheID {
    #[serde(rename = "sort_query_string_for_cache")]
    SortQueryStringForCacheIDSortQueryStringForCache,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortQueryStringForCacheValue {
    #[serde(rename = "on")]
    SortQueryStringForCacheValueOn,
    #[serde(rename = "off")]
    SortQueryStringForCacheValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SSLID {
    #[serde(rename = "ssl")]
    SSLIDSSL,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SSLValue {
    #[serde(rename = "off")]
    SSLValueOff,
    #[serde(rename = "flexible")]
    SSLValueFlexible,
    #[serde(rename = "full")]
    SSLValueFull,
    #[serde(rename = "strict")]
    SSLValueStrict,
    #[serde(rename = "origin_pull")]
    SSLValueOriginPull,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SSLRecommenderID {
    #[serde(rename = "ssl_recommender")]
    SSLRecommenderIDSSLRecommender,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TLSClientAuthID {
    #[serde(rename = "tls_client_auth")]
    TLSClientAuthIDTLSClientAuth,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TLSClientAuthValue {
    #[serde(rename = "on")]
    TLSClientAuthValueOn,
    #[serde(rename = "off")]
    TLSClientAuthValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrueClientIPHeaderID {
    #[serde(rename = "true_client_ip_header")]
    TrueClientIPHeaderIDTrueClientIPHeader,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrueClientIPHeaderValue {
    #[serde(rename = "on")]
    TrueClientIPHeaderValueOn,
    #[serde(rename = "off")]
    TrueClientIPHeaderValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WAFID {
    #[serde(rename = "waf")]
    WAFIDWAF,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WAFValue {
    #[serde(rename = "on")]
    WAFValueOn,
    #[serde(rename = "off")]
    WAFValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebPID {
    #[serde(rename = "webp")]
    WebPIDWebP,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebPValue {
    #[serde(rename = "off")]
    WebPValueOff,
    #[serde(rename = "on")]
    WebPValueOn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebsocketID {
    #[serde(rename = "websockets")]
    WebsocketIDWebsockets,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebsocketValue {
    #[serde(rename = "off")]
    WebsocketValueOff,
    #[serde(rename = "on")]
    WebsocketValueOn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZeroRTTID {
    #[serde(rename = "0rtt")]
    ZeroRTTID0rtt,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZeroRTTValue {
    #[serde(rename = "on")]
    ZeroRTTValueOn,
    #[serde(rename = "off")]
    ZeroRTTValueOff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingEditResponseID {
    #[serde(rename = "0rtt")]
    SettingEditResponseID0rtt,
    #[serde(rename = "advanced_ddos")]
    SettingEditResponseIDAdvancedDDoS,
    #[serde(rename = "aegis")]
    SettingEditResponseIDAegis,
    #[serde(rename = "always_online")]
    SettingEditResponseIDAlwaysOnline,
    #[serde(rename = "always_use_https")]
    SettingEditResponseIDAlwaysUseHTTPS,
    #[serde(rename = "automatic_https_rewrites")]
    SettingEditResponseIDAutomaticHTTPSRewrites,
    #[serde(rename = "brotli")]
    SettingEditResponseIDBrotli,
    #[serde(rename = "browser_cache_ttl")]
    SettingEditResponseIDBrowserCacheTTL,
    #[serde(rename = "browser_check")]
    SettingEditResponseIDBrowserCheck,
    #[serde(rename = "cache_level")]
    SettingEditResponseIDCacheLevel,
    #[serde(rename = "challenge_ttl")]
    SettingEditResponseIDChallengeTTL,
    #[serde(rename = "china_network_enabled")]
    SettingEditResponseIDChinaNetworkEnabled,
    #[serde(rename = "content_converter")]
    SettingEditResponseIDContentConverter,
    #[serde(rename = "ciphers")]
    SettingEditResponseIDCiphers,
    #[serde(rename = "cname_flattening")]
    SettingEditResponseIDCNAMEFlattening,
    #[serde(rename = "development_mode")]
    SettingEditResponseIDDevelopmentMode,
    #[serde(rename = "early_hints")]
    SettingEditResponseIDEarlyHints,
    #[serde(rename = "edge_cache_ttl")]
    SettingEditResponseIDEdgeCacheTTL,
    #[serde(rename = "email_obfuscation")]
    SettingEditResponseIDEmailObfuscation,
    #[serde(rename = "h2_prioritization")]
    SettingEditResponseIDH2Prioritization,
    #[serde(rename = "hotlink_protection")]
    SettingEditResponseIDHotlinkProtection,
    #[serde(rename = "http2")]
    SettingEditResponseIDHTTP2,
    #[serde(rename = "http3")]
    SettingEditResponseIDHTTP3,
    #[serde(rename = "image_resizing")]
    SettingEditResponseIDImageResizing,
    #[serde(rename = "ip_geolocation")]
    SettingEditResponseIDIPGeolocation,
    #[serde(rename = "ipv6")]
    SettingEditResponseIDIPV6,
    #[serde(rename = "max_upload")]
    SettingEditResponseIDMaxUpload,
    #[serde(rename = "min_tls_version")]
    SettingEditResponseIDMinTLSVersion,
    #[serde(rename = "mirage")]
    SettingEditResponseIDMirage,
    #[serde(rename = "nel")]
    SettingEditResponseIDNEL,
    #[serde(rename = "opportunistic_encryption")]
    SettingEditResponseIDOpportunisticEncryption,
    #[serde(rename = "opportunistic_onion")]
    SettingEditResponseIDOpportunisticOnion,
    #[serde(rename = "orange_to_orange")]
    SettingEditResponseIDOrangeToOrange,
    #[serde(rename = "origin_error_page_pass_thru")]
    SettingEditResponseIDOriginErrorPagePassThru,
    #[serde(rename = "origin_h2_max_streams")]
    SettingEditResponseIDOriginH2MaxStreams,
    #[serde(rename = "origin_max_http_version")]
    SettingEditResponseIDOriginMaxHTTPVersion,
    #[serde(rename = "polish")]
    SettingEditResponseIDPolish,
    #[serde(rename = "prefetch_preload")]
    SettingEditResponseIDPrefetchPreload,
    #[serde(rename = "privacy_pass")]
    SettingEditResponseIDPrivacyPass,
    #[serde(rename = "proxy_read_timeout")]
    SettingEditResponseIDProxyReadTimeout,
    #[serde(rename = "pseudo_ipv4")]
    SettingEditResponseIDPseudoIPV4,
    #[serde(rename = "replace_insecure_js")]
    SettingEditResponseIDReplaceInsecureJS,
    #[serde(rename = "response_buffering")]
    SettingEditResponseIDResponseBuffering,
    #[serde(rename = "rocket_loader")]
    SettingEditResponseIDRocketLoader,
    #[serde(rename = "automatic_platform_optimization")]
    SettingEditResponseIDAutomaticPlatformOptimization,
    #[serde(rename = "security_header")]
    SettingEditResponseIDSecurityHeader,
    #[serde(rename = "security_level")]
    SettingEditResponseIDSecurityLevel,
    #[serde(rename = "server_side_exclude")]
    SettingEditResponseIDServerSideExclude,
    #[serde(rename = "sha1_support")]
    SettingEditResponseIDSha1Support,
    #[serde(rename = "sort_query_string_for_cache")]
    SettingEditResponseIDSortQueryStringForCache,
    #[serde(rename = "ssl")]
    SettingEditResponseIDSSL,
    #[serde(rename = "ssl_recommender")]
    SettingEditResponseIDSSLRecommender,
    #[serde(rename = "tls_1_2_only")]
    SettingEditResponseIDTLS1_2Only,
    #[serde(rename = "tls_1_3")]
    SettingEditResponseIDTLS1_3,
    #[serde(rename = "tls_client_auth")]
    SettingEditResponseIDTLSClientAuth,
    #[serde(rename = "transformations")]
    SettingEditResponseIDTransformations,
    #[serde(rename = "transformations_allowed_origins")]
    SettingEditResponseIDTransformationsAllowedOrigins,
    #[serde(rename = "true_client_ip_header")]
    SettingEditResponseIDTrueClientIPHeader,
    #[serde(rename = "waf")]
    SettingEditResponseIDWAF,
    #[serde(rename = "webp")]
    SettingEditResponseIDWebP,
    #[serde(rename = "websockets")]
    SettingEditResponseIDWebsockets,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingGetResponseID {
    #[serde(rename = "0rtt")]
    SettingGetResponseID0rtt,
    #[serde(rename = "advanced_ddos")]
    SettingGetResponseIDAdvancedDDoS,
    #[serde(rename = "aegis")]
    SettingGetResponseIDAegis,
    #[serde(rename = "always_online")]
    SettingGetResponseIDAlwaysOnline,
    #[serde(rename = "always_use_https")]
    SettingGetResponseIDAlwaysUseHTTPS,
    #[serde(rename = "automatic_https_rewrites")]
    SettingGetResponseIDAutomaticHTTPSRewrites,
    #[serde(rename = "brotli")]
    SettingGetResponseIDBrotli,
    #[serde(rename = "browser_cache_ttl")]
    SettingGetResponseIDBrowserCacheTTL,
    #[serde(rename = "browser_check")]
    SettingGetResponseIDBrowserCheck,
    #[serde(rename = "cache_level")]
    SettingGetResponseIDCacheLevel,
    #[serde(rename = "challenge_ttl")]
    SettingGetResponseIDChallengeTTL,
    #[serde(rename = "china_network_enabled")]
    SettingGetResponseIDChinaNetworkEnabled,
    #[serde(rename = "content_converter")]
    SettingGetResponseIDContentConverter,
    #[serde(rename = "ciphers")]
    SettingGetResponseIDCiphers,
    #[serde(rename = "cname_flattening")]
    SettingGetResponseIDCNAMEFlattening,
    #[serde(rename = "development_mode")]
    SettingGetResponseIDDevelopmentMode,
    #[serde(rename = "early_hints")]
    SettingGetResponseIDEarlyHints,
    #[serde(rename = "edge_cache_ttl")]
    SettingGetResponseIDEdgeCacheTTL,
    #[serde(rename = "email_obfuscation")]
    SettingGetResponseIDEmailObfuscation,
    #[serde(rename = "h2_prioritization")]
    SettingGetResponseIDH2Prioritization,
    #[serde(rename = "hotlink_protection")]
    SettingGetResponseIDHotlinkProtection,
    #[serde(rename = "http2")]
    SettingGetResponseIDHTTP2,
    #[serde(rename = "http3")]
    SettingGetResponseIDHTTP3,
    #[serde(rename = "image_resizing")]
    SettingGetResponseIDImageResizing,
    #[serde(rename = "ip_geolocation")]
    SettingGetResponseIDIPGeolocation,
    #[serde(rename = "ipv6")]
    SettingGetResponseIDIPV6,
    #[serde(rename = "max_upload")]
    SettingGetResponseIDMaxUpload,
    #[serde(rename = "min_tls_version")]
    SettingGetResponseIDMinTLSVersion,
    #[serde(rename = "mirage")]
    SettingGetResponseIDMirage,
    #[serde(rename = "nel")]
    SettingGetResponseIDNEL,
    #[serde(rename = "opportunistic_encryption")]
    SettingGetResponseIDOpportunisticEncryption,
    #[serde(rename = "opportunistic_onion")]
    SettingGetResponseIDOpportunisticOnion,
    #[serde(rename = "orange_to_orange")]
    SettingGetResponseIDOrangeToOrange,
    #[serde(rename = "origin_error_page_pass_thru")]
    SettingGetResponseIDOriginErrorPagePassThru,
    #[serde(rename = "origin_h2_max_streams")]
    SettingGetResponseIDOriginH2MaxStreams,
    #[serde(rename = "origin_max_http_version")]
    SettingGetResponseIDOriginMaxHTTPVersion,
    #[serde(rename = "polish")]
    SettingGetResponseIDPolish,
    #[serde(rename = "prefetch_preload")]
    SettingGetResponseIDPrefetchPreload,
    #[serde(rename = "privacy_pass")]
    SettingGetResponseIDPrivacyPass,
    #[serde(rename = "proxy_read_timeout")]
    SettingGetResponseIDProxyReadTimeout,
    #[serde(rename = "pseudo_ipv4")]
    SettingGetResponseIDPseudoIPV4,
    #[serde(rename = "replace_insecure_js")]
    SettingGetResponseIDReplaceInsecureJS,
    #[serde(rename = "response_buffering")]
    SettingGetResponseIDResponseBuffering,
    #[serde(rename = "rocket_loader")]
    SettingGetResponseIDRocketLoader,
    #[serde(rename = "automatic_platform_optimization")]
    SettingGetResponseIDAutomaticPlatformOptimization,
    #[serde(rename = "security_header")]
    SettingGetResponseIDSecurityHeader,
    #[serde(rename = "security_level")]
    SettingGetResponseIDSecurityLevel,
    #[serde(rename = "server_side_exclude")]
    SettingGetResponseIDServerSideExclude,
    #[serde(rename = "sha1_support")]
    SettingGetResponseIDSha1Support,
    #[serde(rename = "sort_query_string_for_cache")]
    SettingGetResponseIDSortQueryStringForCache,
    #[serde(rename = "ssl")]
    SettingGetResponseIDSSL,
    #[serde(rename = "ssl_recommender")]
    SettingGetResponseIDSSLRecommender,
    #[serde(rename = "tls_1_2_only")]
    SettingGetResponseIDTLS1_2Only,
    #[serde(rename = "tls_1_3")]
    SettingGetResponseIDTLS1_3,
    #[serde(rename = "tls_client_auth")]
    SettingGetResponseIDTLSClientAuth,
    #[serde(rename = "transformations")]
    SettingGetResponseIDTransformations,
    #[serde(rename = "transformations_allowed_origins")]
    SettingGetResponseIDTransformationsAllowedOrigins,
    #[serde(rename = "true_client_ip_header")]
    SettingGetResponseIDTrueClientIPHeader,
    #[serde(rename = "waf")]
    SettingGetResponseIDWAF,
    #[serde(rename = "webp")]
    SettingGetResponseIDWebP,
    #[serde(rename = "websockets")]
    SettingGetResponseIDWebsockets,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionNewResponseFrequency {
    #[serde(rename = "weekly")]
    SubscriptionNewResponseFrequencyWeekly,
    #[serde(rename = "monthly")]
    SubscriptionNewResponseFrequencyMonthly,
    #[serde(rename = "quarterly")]
    SubscriptionNewResponseFrequencyQuarterly,
    #[serde(rename = "yearly")]
    SubscriptionNewResponseFrequencyYearly,
    #[serde(rename = "not-applicable")]
    SubscriptionNewResponseFrequencyNotApplicable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionNewResponseState {
    #[serde(rename = "Trial")]
    SubscriptionNewResponseStateTrial,
    #[serde(rename = "Provisioned")]
    SubscriptionNewResponseStateProvisioned,
    #[serde(rename = "Paid")]
    SubscriptionNewResponseStatePaid,
    #[serde(rename = "AwaitingPayment")]
    SubscriptionNewResponseStateAwaitingPayment,
    #[serde(rename = "Cancelled")]
    SubscriptionNewResponseStateCancelled,
    #[serde(rename = "Failed")]
    SubscriptionNewResponseStateFailed,
    #[serde(rename = "Expired")]
    SubscriptionNewResponseStateExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionUpdateResponseFrequency {
    #[serde(rename = "weekly")]
    SubscriptionUpdateResponseFrequencyWeekly,
    #[serde(rename = "monthly")]
    SubscriptionUpdateResponseFrequencyMonthly,
    #[serde(rename = "quarterly")]
    SubscriptionUpdateResponseFrequencyQuarterly,
    #[serde(rename = "yearly")]
    SubscriptionUpdateResponseFrequencyYearly,
    #[serde(rename = "not-applicable")]
    SubscriptionUpdateResponseFrequencyNotApplicable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionUpdateResponseState {
    #[serde(rename = "Trial")]
    SubscriptionUpdateResponseStateTrial,
    #[serde(rename = "Provisioned")]
    SubscriptionUpdateResponseStateProvisioned,
    #[serde(rename = "Paid")]
    SubscriptionUpdateResponseStatePaid,
    #[serde(rename = "AwaitingPayment")]
    SubscriptionUpdateResponseStateAwaitingPayment,
    #[serde(rename = "Cancelled")]
    SubscriptionUpdateResponseStateCancelled,
    #[serde(rename = "Failed")]
    SubscriptionUpdateResponseStateFailed,
    #[serde(rename = "Expired")]
    SubscriptionUpdateResponseStateExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionGetResponseFrequency {
    #[serde(rename = "weekly")]
    SubscriptionGetResponseFrequencyWeekly,
    #[serde(rename = "monthly")]
    SubscriptionGetResponseFrequencyMonthly,
    #[serde(rename = "quarterly")]
    SubscriptionGetResponseFrequencyQuarterly,
    #[serde(rename = "yearly")]
    SubscriptionGetResponseFrequencyYearly,
    #[serde(rename = "not-applicable")]
    SubscriptionGetResponseFrequencyNotApplicable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionGetResponseState {
    #[serde(rename = "Trial")]
    SubscriptionGetResponseStateTrial,
    #[serde(rename = "Provisioned")]
    SubscriptionGetResponseStateProvisioned,
    #[serde(rename = "Paid")]
    SubscriptionGetResponseStatePaid,
    #[serde(rename = "AwaitingPayment")]
    SubscriptionGetResponseStateAwaitingPayment,
    #[serde(rename = "Cancelled")]
    SubscriptionGetResponseStateCancelled,
    #[serde(rename = "Failed")]
    SubscriptionGetResponseStateFailed,
    #[serde(rename = "Expired")]
    SubscriptionGetResponseStateExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZoneStatus {
    #[serde(rename = "initializing")]
    ZoneStatusInitializing,
    #[serde(rename = "pending")]
    ZoneStatusPending,
    #[serde(rename = "active")]
    ZoneStatusActive,
    #[serde(rename = "moved")]
    ZoneStatusMoved,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RatePlanGetResponseComponentsName {
    #[serde(rename = "zones")]
    RatePlanGetResponseComponentsNameZones,
    #[serde(rename = "page_rules")]
    RatePlanGetResponseComponentsNamePageRules,
    #[serde(rename = "dedicated_certificates")]
    RatePlanGetResponseComponentsNameDedicatedCertificates,
    #[serde(rename = "dedicated_certificates_custom")]
    RatePlanGetResponseComponentsNameDedicatedCertificatesCustom,
}

