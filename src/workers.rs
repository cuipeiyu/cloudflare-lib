#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_usage_model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_compute: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_usage_model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_compute: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_usage_model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_compute: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadNewResponse {
    /// A "completion" JWT which can be redeemed when creating a Worker version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Whether the file contents are base64-encoded. Must be `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,

    pub body: ::std::collections::HashMap<String, String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    /// Immutable ID of the Worker.
    pub id: String,

    /// When the Worker was created.
    pub created_on: DateTime<Utc>,

    /// Whether logpush is enabled for the Worker.
    pub logpush: bool,

    /// Name of the Worker.
    pub name: String,

    /// Observability settings for the Worker.
    pub observability: WorkerObservability,

    /// Other resources that reference the Worker and depend on it existing.
    pub references: WorkerReferences,

    /// Subdomain settings for the Worker.
    pub subdomain: WorkerSubdomain,

    /// Tags associated with the Worker.
    pub tags: Vec<String>,

    /// Other Workers that should consume logs from the Worker.
    pub tail_consumers: Vec<WorkerTailConsumer>,

    /// When the Worker was most recently updated.
    pub updated_on: DateTime<Utc>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerParam {
    /// Whether logpush is enabled for the Worker.
    pub logpush: String,

    /// Name of the Worker.
    pub name: String,

    /// Observability settings for the Worker.
    pub observability: String,

    /// Subdomain settings for the Worker.
    pub subdomain: String,

    /// Tags associated with the Worker.
    pub tags: String,

    /// Other Workers that should consume logs from the Worker.
    pub tail_consumers: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteResponse {
    pub errors: Vec<BetaWorkerDeleteResponseError>,

    pub messages: Vec<BetaWorkerDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: BetaWorkerDeleteResponseSuccess,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub worker: WorkerParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub worker: WorkerParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Items per-page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub worker: WorkerParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    /// Version identifier.
    pub id: String,

    /// When the version was created.
    pub created_on: DateTime<Utc>,

    /// The integer version number, starting from one.
    pub number: i64,

    /// Metadata about the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<VersionAnnotations>,

    /// Configuration for assets within a Worker.
    /// [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers)
    /// and
    /// [`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/)
    /// files should be included as modules named `_headers` and `_redirects` with
    /// content type `text/plain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<VersionAssets>,

    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<VersionBinding>>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// Resource limits enforced at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<VersionLimits>,

    /// The name of the main module in the `modules` array (e.g. the name of the module
    /// that exports a `fetch` handler).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_module: Option<String>,

    /// Migrations for Durable Objects associated with the version. Migrations are
    /// applied when the version is deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<VersionMigrations>,

    /// Code, sourcemaps, and other content used at runtime.
    /// This includes
    /// [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers)
    /// and
    /// [`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/)
    /// files used to configure
    /// [Static Assets](https://developers.cloudflare.com/workers/static-assets/).
    /// `_headers` and `_redirects` files should be included as modules named `_headers`
    /// and `_redirects` with content type `text/plain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<VersionModule>>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<VersionPlacement>,

    /// The client used to create the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Time in milliseconds spent on
    /// [Worker startup](https://developers.cloudflare.com/workers/platform/limits/#worker-startup-time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_time_ms: Option<i64>,

    /// Usage model for the version.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<VersionUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionParam {
    /// Metadata about the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,

    /// Configuration for assets within a Worker.
    /// [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers)
    /// and
    /// [`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/)
    /// files should be included as modules named `_headers` and `_redirects` with
    /// content type `text/plain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<String>,

    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<String>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<String>,

    /// Resource limits enforced at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<String>,

    /// The name of the main module in the `modules` array (e.g. the name of the module
    /// that exports a `fetch` handler).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_module: Option<String>,

    /// Migrations for Durable Objects associated with the version. Migrations are
    /// applied when the version is deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<String>,

    /// Code, sourcemaps, and other content used at runtime.
    /// This includes
    /// [`_headers`](https://developers.cloudflare.com/workers/static-assets/headers/#custom-headers)
    /// and
    /// [`_redirects`](https://developers.cloudflare.com/workers/static-assets/redirects/)
    /// files used to configure
    /// [Static Assets](https://developers.cloudflare.com/workers/static-assets/).
    /// `_headers` and `_redirects` files should be included as modules named `_headers`
    /// and `_redirects` with content type `text/plain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<String>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,

    /// Usage model for the version.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteResponse {
    pub errors: Vec<BetaWorkerVersionDeleteResponseError>,

    pub messages: Vec<BetaWorkerVersionDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: BetaWorkerVersionDeleteResponseSuccess,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub version: VersionParam,

    /// If true, a deployment will be created that sends 100% of traffic to the new
    /// version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Items per-page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Whether to include the `modules` property of the version in the response, which
    /// contains code and sourcemap content and may add several megabytes to the
    /// response size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Identifer of the Worker Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Worker environment associated with the zone and hostname.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Hostname of the Worker Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Worker service associated with the zone and hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Identifier of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Name of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainUpdateParams {
    /// Identifer of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Hostname of the Worker Domain.
    pub hostname: String,

    /// Worker service associated with the zone and hostname.
    pub service: String,

    /// Identifier of the zone.
    pub zone_id: String,

    /// Worker environment associated with the zone and hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainListParams {
    /// Identifer of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Worker environment associated with the zone and hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Hostname of the Worker Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Worker service associated with the zone and hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Identifier of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Name of the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainDeleteParams {
    /// Identifer of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGetParams {
    /// Identifer of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryKeysResponse {
    pub key: String,

    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: f64,

    pub r#type: ObservabilityTelemetryKeysResponseType,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponse {
    /// A Workers Observability Query Object
    pub run: ObservabilityTelemetryQueryResponseRun,

    /// The statistics object contains information about query performance from the
    /// database, it does not include any network latency
    pub statistics: ObservabilityTelemetryQueryResponseStatistics,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculations: Option<Vec<ObservabilityTelemetryQueryResponseCalculation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare: Option<Vec<ObservabilityTelemetryQueryResponseCompare>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<ObservabilityTelemetryQueryResponseEvents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocations: Option<::std::collections::HashMap<String, Vec<ObservabilityTelemetryQueryResponseInvocation>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<ObservabilityTelemetryQueryResponsePattern>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<ObservabilityTelemetryQueryResponseTrace>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryValuesResponse {
    pub dataset: String,

    pub key: String,

    pub r#type: ObservabilityTelemetryValuesResponseType,

    pub value: ObservabilityTelemetryValuesResponseValueUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryKeysParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Search for a specific substring in the keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyNeedle")]
    pub key_needle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Search for a specific substring in any of the events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Unique identifier for the query to execute
    #[serde(rename = "queryId")]
    pub query_id: String,

    /// Time range for the query execution
    pub timeframe: String,

    /// Whether to include timeseties data in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<String>,

    /// Whether to include comparison data with previous time periods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare: Option<String>,

    /// Whether to perform a dry run without saving the results of the query. Useful for
    /// validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry: Option<String>,

    /// Time granularity for aggregating results (in milliseconds). Controls the
    /// bucketing of time-series data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,

    /// Whether to ignore time-series data in the results and return only aggregated
    /// values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreSeries")]
    pub ignore_series: Option<String>,

    /// Maximum number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Cursor for pagination to retrieve the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// Number of events to skip for pagination. Used in conjunction with offset
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "offsetBy")]
    pub offset_by: Option<String>,

    /// Direction for offset-based pagination (e.g., 'next', 'prev')
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "offsetDirection")]
    pub offset_direction: Option<String>,

    /// Optional parameters to pass to the query execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,

    /// Type of pattern to search for when using pattern-based views
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "patternType")]
    pub pattern_type: Option<String>,

    /// View type for presenting the query results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryValuesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub datasets: String,

    pub key: String,

    pub timeframe: String,

    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Search for a specific substring in the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needle: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteNewResponse {
    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteUpdateResponse {
    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteListResponse {
    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDeleteResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteGetResponse {
    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// The name used to identify the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// When the script was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Hashed script content, can be used in a If-None-Match header when updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The names of handlers exported as part of the default export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// Whether a Worker contains assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_assets: Option<bool>,

    /// Whether a Worker contains modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_modules: Option<bool>,

    /// The client most recently used to deploy this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployed_from: Option<String>,

    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// The tag of the Durable Object migration that was most recently applied for this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_tag: Option<String>,

    /// When the script was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Named exports, such as Durable Object class implementations and named
    /// entrypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_handlers: Option<Vec<ScriptNamedHandler>>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptObservability>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ScriptPlacement>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_mode: Option<ScriptPlacementMode>,

    /// Status of
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_status: Option<ScriptPlacementStatus>,

    /// The immutable ID of the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSetting {
    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptSettingObservability>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSettingParam {
    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<String>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<String>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateResponse {
    pub startup_time_ms: i64,

    /// The name used to identify the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// When the script was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The entry point for the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,

    /// Hashed script content, can be used in a If-None-Match header when updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The names of handlers exported as part of the default export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// Whether a Worker contains assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_assets: Option<bool>,

    /// Whether a Worker contains modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_modules: Option<bool>,

    /// The client most recently used to deploy this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployed_from: Option<String>,

    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// The tag of the Durable Object migration that was most recently applied for this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_tag: Option<String>,

    /// When the script was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Named exports, such as Durable Object class implementations and named
    /// entrypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_handlers: Option<Vec<ScriptUpdateResponseNamedHandler>>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptUpdateResponseObservability>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ScriptUpdateResponsePlacement>,

    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_mode: Option<ScriptUpdateResponsePlacementMode>,

    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_status: Option<ScriptUpdateResponsePlacementStatus>,

    /// The immutable ID of the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptUpdateResponseUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponse {
    /// The name used to identify the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// When the script was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Hashed script content, can be used in a If-None-Match header when updating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The names of handlers exported as part of the default export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// Whether a Worker contains assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_assets: Option<bool>,

    /// Whether a Worker contains modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_modules: Option<bool>,

    /// The client most recently used to deploy this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployed_from: Option<String>,

    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// The tag of the Durable Object migration that was most recently applied for this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_tag: Option<String>,

    /// When the script was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Named exports, such as Durable Object class implementations and named
    /// entrypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_handlers: Option<Vec<ScriptListResponseNamedHandler>>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptListResponseObservability>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ScriptListResponsePlacement>,

    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_mode: Option<ScriptListResponsePlacementMode>,

    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_status: Option<ScriptListResponsePlacementStatus>,

    /// Routes associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<ScriptListResponseRoute>>,

    /// The immutable ID of the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptListResponseUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSearchResponse {
    /// Identifier.
    pub id: String,

    /// When the script was created.
    pub created_on: DateTime<Utc>,

    /// When the script was last modified.
    pub modified_on: DateTime<Utc>,

    /// Name of the script, used in URLs and route configuration.
    pub script_name: String,

    /// Whether the environment is the default environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_is_default: Option<bool>,

    /// Name of the environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,

    /// Name of the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// JSON-encoded metadata about the uploaded parts and Worker configuration.
    pub metadata: String,

    /// When set to "strict", the upload will fail if any `inherit` type bindings cannot
    /// be resolved against the previous version of the Worker. Without this,
    /// unresolvable inherit bindings are silently dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings_inherit: Option<String>,

    /// An array of modules (often JavaScript files) comprising a Worker script. At
    /// least one module must be present and referenced in the metadata as `main_module`
    /// or `body_part` by filename.<br/>Possible Content-Type(s) are:
    /// `application/javascript+module`, `text/javascript+module`,
    /// `application/javascript`, `text/javascript`, `text/x-python`,
    /// `text/x-python-requirement`, `application/wasm`, `text/plain`,
    /// `application/octet-stream`, `application/source-map`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Filter scripts by tags. Format: comma-separated list of tag:allowed pairs where
    /// allowed is 'yes' or 'no'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// If set to true, delete will not be stopped by associated service binding,
    /// durable object, or other binding. Any of these associated bindings/durable
    /// objects will be deleted along with the script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSearchParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Worker ID (also called tag) to search for. Only exact matches are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Worker name to search for. Both exact and partial matches are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Property to sort results by. Results are sorted in ascending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAssetUploadNewResponse {
    /// The requests to make to upload assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<Vec<String>>>,

    /// A JWT to use as authentication for uploading assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAssetUploadNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A manifest ([path]: {hash, size}) map of files to upload. As an example,
    /// `/blog/hello-world.html` would be a valid path key.
    pub manifest: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptContentUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// JSON-encoded metadata about the uploaded parts and Worker configuration.
    pub metadata: String,

    /// An array of modules (often JavaScript files) comprising a Worker script. At
    /// least one module must be present and referenced in the metadata as `main_module`
    /// or `body_part` by filename.<br/>Possible Content-Type(s) are:
    /// `application/javascript+module`, `text/javascript+module`,
    /// `application/javascript`, `text/javascript`, `text/x-python`,
    /// `text/x-python-requirement`, `application/wasm`, `text/plain`,
    /// `application/octet-stream`, `application/source-map`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf_worker_body_part: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf_worker_main_module_part: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptContentGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,

    pub created_on: DateTime<Utc>,

    pub source: String,

    pub strategy: DeploymentStrategy,

    pub versions: Vec<DeploymentVersion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<DeploymentAnnotations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentParam {
    pub strategy: String,

    pub versions: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentListResponse {
    pub deployments: Vec<Deployment>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteResponse {
    pub errors: Vec<ScriptDeploymentDeleteResponseError>,

    pub messages: Vec<ScriptDeploymentDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: ScriptDeploymentDeleteResponseSuccess,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub deployment: DeploymentParam,

    /// If set to true, the deployment will be created even if normally blocked by
    /// something such rolling back to an older version when a secret has changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleUpdateResponse {
    pub schedules: Vec<ScriptScheduleUpdateResponseSchedule>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleGetResponse {
    pub schedules: Vec<ScriptScheduleGetResponseSchedule>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: Vec<ScriptScheduleUpdateParamsBody>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponse {
    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ScriptScriptAndVersionSettingEditResponseBinding>>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// Limits to apply for this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ScriptScriptAndVersionSettingEditResponseLimits>,

    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptScriptAndVersionSettingEditResponseObservability>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ScriptScriptAndVersionSettingEditResponsePlacement>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptScriptAndVersionSettingEditResponseUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponse {
    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ScriptScriptAndVersionSettingGetResponseBinding>>,

    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime. Used to
    /// enable upcoming features or opt in or out of specific changes not included in a
    /// `compatibility_date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// Limits to apply for this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ScriptScriptAndVersionSettingGetResponseLimits>,

    /// Whether Logpush is turned on for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpush: Option<bool>,

    /// Observability settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability: Option<ScriptScriptAndVersionSettingGetResponseObservability>,

    /// Configuration for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    /// Specify mode='smart' for Smart Placement, or one of region/hostname/host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ScriptScriptAndVersionSettingGetResponsePlacement>,

    /// Tags associated with the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// List of Workers that will consume logs from the attached Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_consumers: Option<Vec<ConsumerScript>>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptScriptAndVersionSettingGetResponseUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretUpdateResponse {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptSecretUpdateResponseType,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptSecretUpdateResponseFormat>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptSecretUpdateResponseWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretListResponse {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptSecretListResponseType,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptSecretListResponseFormat>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptSecretListResponseWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretGetResponse {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptSecretGetResponseType,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptSecretGetResponseFormat>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptSecretGetResponseWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A secret value accessible through a binding.
    pub body: ScriptSecretUpdateParamsBodyUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Flag that indicates whether the secret name is URL encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encoded: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSecretGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Flag that indicates whether the secret name is URL encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encoded: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSettingEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub script_setting: ScriptSettingParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSettingGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainNewResponse {
    /// Whether the Worker is available on the workers.dev subdomain.
    pub enabled: bool,

    /// Whether the Worker's Preview URLs are available on the workers.dev subdomain.
    pub previews_enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainDeleteResponse {
    /// Whether the Worker is available on the workers.dev subdomain.
    pub enabled: bool,

    /// Whether the Worker's Preview URLs are available on the workers.dev subdomain.
    pub previews_enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainGetResponse {
    /// Whether the Worker is available on the workers.dev subdomain.
    pub enabled: bool,

    /// Whether the Worker's Preview URLs are available on the workers.dev subdomain.
    pub previews_enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Whether the Worker should be available on the workers.dev subdomain.
    pub enabled: String,

    /// Whether the Worker's Preview URLs should be available on the workers.dev
    /// subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews_enabled: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSubdomainGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerScript {
    /// Name of Worker that is to be the consumer.
    pub service: String,

    /// Optional environment if the Worker utilizes one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Optional dispatch namespace the script belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerScriptParam {
    /// Name of Worker that is to be the consumer.
    pub service: String,

    /// Optional environment if the Worker utilizes one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Optional dispatch namespace the script belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailNewResponse {
    /// Identifier.
    pub id: String,

    pub expires_at: String,

    pub url: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteResponse {
    pub errors: Vec<ScriptTailDeleteResponseError>,

    pub messages: Vec<ScriptTailDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: ScriptTailDeleteResponseSuccess,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailGetResponse {
    /// Identifier.
    pub id: String,

    pub expires_at: String,

    pub url: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponse {
    pub resources: ScriptVersionNewResponseResources,

    /// Unique identifier for the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ScriptVersionNewResponseMetadata>,

    /// Sequential version number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,

    /// Time in milliseconds spent on
    /// [Worker startup](https://developers.cloudflare.com/workers/platform/limits/#worker-startup-time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_time_ms: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionListResponse {
    /// Unique identifier for the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ScriptVersionListResponseMetadata>,

    /// Sequential version number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponse {
    pub resources: ScriptVersionGetResponseResources,

    /// Unique identifier for the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ScriptVersionGetResponseMetadata>,

    /// Sequential version number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// JSON-encoded metadata about the uploaded parts and Worker configuration.
    pub metadata: String,

    /// When set to "strict", the upload will fail if any `inherit` type bindings cannot
    /// be resolved against the previous version of the Worker. Without this,
    /// unresolvable inherit bindings are silently dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings_inherit: Option<String>,

    /// An array of modules (often JavaScript files) comprising a Worker script. At
    /// least one module must be present and referenced in the metadata as `main_module`
    /// or `body_part` by filename.<br/>Possible Content-Type(s) are:
    /// `application/javascript+module`, `text/javascript+module`,
    /// `application/javascript`, `text/javascript`, `text/x-python`,
    /// `text/x-python-requirement`, `application/wasm`, `text/plain`,
    /// `application/octet-stream`, `application/source-map`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Only return versions that can be used in a deployment. Ignores pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployable: Option<String>,

    /// Current page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Items per-page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainUpdateResponse {
    pub subdomain: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainGetResponse {
    pub subdomain: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub subdomain: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStepParam {
    /// A list of classes to delete Durable Object namespaces from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_classes: Option<String>,

    /// A list of classes to create Durable Object namespaces from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_classes: Option<String>,

    /// A list of classes to create Durable Object namespaces with SQLite from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sqlite_classes: Option<String>,

    /// A list of classes with Durable Object namespaces that were renamed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_classes: Option<String>,

    /// A list of transfers for Durable Object namespaces from a different Worker and
    /// class to a class defined in this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_classes: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleStepMigration {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleStepMigrationParam {
    /// A list of classes to delete Durable Object namespaces from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_classes: Option<String>,

    /// A list of classes to create Durable Object namespaces from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_classes: Option<String>,

    /// A list of classes to create Durable Object namespaces with SQLite from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sqlite_classes: Option<String>,

    /// Tag to set as the latest migration tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tag: Option<String>,

    /// Tag used to verify against the latest migration tag for this Worker. If they
    /// don't match, the upload is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_tag: Option<String>,

    /// A list of classes with Durable Object namespaces that were renamed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_classes: Option<String>,

    /// A list of transfers for Durable Object namespaces from a different Worker and
    /// class to a class defined in this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_classes: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetadataParam {
    /// Name of the part in the multipart request that contains the script (e.g. the
    /// file adding a listener to the `fetch` event). Indicates a
    /// `service worker syntax` Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_part: Option<String>,

    /// Name of the part in the multipart request that contains the main module (e.g.
    /// the file exporting a `fetch` handler). Indicates a `module syntax` Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_module: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerObservability {
    /// Whether observability is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The sampling rate for observability. From 0 to 1 (1 = 100%, 0.1 = 10%).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<WorkerObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferences {
    /// Other Workers that reference the Worker as an outbound for a dispatch namespace.
    pub dispatch_namespace_outbounds: Vec<WorkerReferencesDispatchNamespaceOutbound>,

    /// Custom domains connected to the Worker.
    pub domains: Vec<WorkerReferencesDomain>,

    /// Other Workers that reference Durable Object classes implemented by the Worker.
    pub durable_objects: Vec<WorkerReferencesDurableObject>,

    /// Queues that send messages to the Worker.
    pub queues: Vec<WorkerReferencesQueue>,

    /// Other Workers that reference the Worker using
    /// [service bindings](https://developers.cloudflare.com/workers/runtime-apis/bindings/service-bindings/).
    pub workers: Vec<WorkerReferencesWorker>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerSubdomain {
    /// Whether the \*.workers.dev subdomain is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Whether
    /// [preview URLs](https://developers.cloudflare.com/workers/configuration/previews/)
    /// are enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews_enabled: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerTailConsumer {
    /// Name of the consumer Worker.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<BetaWorkerDeleteResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<BetaWorkerDeleteResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAnnotations {
    /// Human-readable message about the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workers/message")]
    pub workers_message: Option<String>,

    /// User-provided identifier for the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workers/tag")]
    pub workers_tag: Option<String>,

    /// Operation that triggered the creation of the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workers/triggered_by")]
    pub workers_triggered_by: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssets {
    /// Configuration for assets within a Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<VersionAssetsConfig>,

    /// Token provided upon successful upload of all files from a registered manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionBinding {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: VersionBindingsType,

    /// Identifier of the D1 database to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_destination_addresses: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sender_addresses: Option<String>,

    /// R2 bucket to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    /// Identifier of the certificate to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,

    /// The exported class name of the Durable Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The name of the dataset to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Destination address for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// The environment of the script_name to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VersionBindingsFormat>,

    /// Name of the Vectorize index to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,

    /// The
    /// [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions)
    /// of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<VersionBindingsJurisdiction>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// The name of the dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Namespace identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// The old name of the inherited binding. If set, the binding will be renamed from
    /// `old_name` to `name` in the new version. If not set, the binding will keep the
    /// same name between versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,

    /// This field can have the runtime type of
    /// [VersionBindingsWorkersBindingKindDispatchNamespaceOutbound].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,

    /// The name of the file containing the data content. Only accepted for
    /// `service worker syntax` Workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,

    /// Name of the Pipeline to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    /// Name of the Queue to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// The script where the Durable Object is defined, if it is external to this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// Name of the secret in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// Name of Worker to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// This field can have the runtime type of
    /// [VersionBindingsWorkersBindingKindRatelimitSimple].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<String>,

    /// ID of the store containing the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,

    /// The text value to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// This field can have the runtime type of
    /// [[]VersionBindingsWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

    /// Identifier for the version to inherit the binding from, which can be the version
    /// ID or the literal "latest" to inherit from the latest version. Defaults to
    /// inheriting the binding from the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Name of the Workflow to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionLimits {
    /// CPU time limit in milliseconds.
    pub cpu_ms: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMigrations {
    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_classes: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_classes: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sqlite_classes: Option<String>,

    /// This field can have the runtime type of [[]SingleStepMigrationRenamedClass].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_classes: Option<String>,

    /// This field can have the runtime type of [[]MigrationStep].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,

    /// This field can have the runtime type of [[]SingleStepMigrationTransferredClass].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_classes: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionModule {
    /// The base64-encoded module content.
    pub content_base64: String,

    /// The content type of the module.
    pub content_type: String,

    /// The name of the module.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionPlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<VersionPlacementModeMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// This field can have the runtime type of [[]VersionPlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<BetaWorkerVersionDeleteResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<BetaWorkerVersionDeleteResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRun {
    pub id: String,

    #[serde(rename = "accountId")]
    pub account_id: String,

    pub dry: bool,

    /// Deprecated: deprecated
    #[serde(rename = "environmentId")]
    pub environment_id: String,

    pub granularity: f64,

    pub query: ObservabilityTelemetryQueryResponseRunQuery,

    pub status: ObservabilityTelemetryQueryResponseRunStatus,

    /// Time range for the query execution
    pub timeframe: ObservabilityTelemetryQueryResponseRunTimeframe,

    #[serde(rename = "userId")]
    pub user_id: String,

    /// Deprecated: deprecated
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<ObservabilityTelemetryQueryResponseRunStatistics>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseStatistics {
    /// Number of uncompressed bytes read from the table.
    pub bytes_read: f64,

    /// Time in seconds for the query to run.
    pub elapsed: f64,

    /// Number of rows scanned from the table.
    pub rows_read: f64,

    /// The level of Adaptive Bit Rate (ABR) sampling used for the query. If empty the
    /// ABR level is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abr_level: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculation {
    pub aggregates: Vec<ObservabilityTelemetryQueryResponseCalculationsAggregate>,

    pub calculation: String,

    pub series: Vec<ObservabilityTelemetryQueryResponseCalculationsSeries>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompare {
    pub aggregates: Vec<ObservabilityTelemetryQueryResponseCompareAggregate>,

    pub calculation: String,

    pub series: Vec<ObservabilityTelemetryQueryResponseCompareSeries>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEvents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ObservabilityTelemetryQueryResponseEventsEvent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ObservabilityTelemetryQueryResponseEventsField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<ObservabilityTelemetryQueryResponseEventsSeries>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseInvocation {
    #[serde(rename = "$metadata")]
    pub metadata: ObservabilityTelemetryQueryResponseInvocationsMetadata,

    pub dataset: String,

    pub source: String,

    pub timestamp: i64,

    /// Cloudflare Containers event information enriches your logs so you can easily
    /// identify and debug issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$containers")]
    pub containers: Option<String>,

    /// Cloudflare Workers event information enriches your logs so you can easily
    /// identify and debug issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$workers")]
    pub workers: Option<ObservabilityTelemetryQueryResponseInvocationsWorkers>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponsePattern {
    pub count: f64,

    pub pattern: String,

    pub series: Vec<ObservabilityTelemetryQueryResponsePatternsSeries>,

    pub service: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseTrace {
    #[serde(rename = "rootSpanName")]
    pub root_span_name: String,

    #[serde(rename = "rootTransactionName")]
    pub root_transaction_name: String,

    pub service: Vec<String>,

    pub spans: f64,

    #[serde(rename = "traceDurationMs")]
    pub trace_duration_ms: f64,

    #[serde(rename = "traceEndMs")]
    pub trace_end_ms: f64,

    #[serde(rename = "traceId")]
    pub trace_id: String,

    #[serde(rename = "traceStartMs")]
    pub trace_start_ms: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptNamedHandler {
    /// The names of handlers exported as part of the named export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The name of the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// The last time the script was analyzed for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_at: Option<DateTime<Utc>>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ScriptPlacementMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Status of
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScriptPlacementStatus>,

    /// This field can have the runtime type of [[]ScriptPlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSettingObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptSettingObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateResponseNamedHandler {
    /// The names of handlers exported as part of the named export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The name of the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateResponseObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptUpdateResponseObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateResponsePlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// The last time the script was analyzed for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_at: Option<DateTime<Utc>>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ScriptUpdateResponsePlacementMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Status of
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScriptUpdateResponsePlacementStatus>,

    /// This field can have the runtime type of
    /// [[]ScriptUpdateResponsePlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponseNamedHandler {
    /// The names of handlers exported as part of the named export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The name of the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponseObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptListResponseObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponsePlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// The last time the script was analyzed for
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_analyzed_at: Option<DateTime<Utc>>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ScriptListResponsePlacementMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Status of
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScriptListResponsePlacementStatus>,

    /// This field can have the runtime type of
    /// [[]ScriptListResponsePlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponseRoute {
    /// Identifier.
    pub id: String,

    /// Pattern to match incoming requests against.
    /// [Learn more](https://developers.cloudflare.com/workers/configuration/routing/routes/#matching-behavior).
    pub pattern: String,

    /// Name of the script to run if the route matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentVersion {
    pub percentage: f64,

    pub version_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentAnnotations {
    /// Human-readable message about the deployment. Truncated to 100 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workers/message")]
    pub workers_message: Option<String>,

    /// Operation that triggered the creation of the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workers/triggered_by")]
    pub workers_triggered_by: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptDeploymentDeleteResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptDeploymentDeleteResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleUpdateResponseSchedule {
    pub cron: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleGetResponseSchedule {
    pub cron: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScheduleUpdateParamsBody {
    pub cron: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponseBinding {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptScriptAndVersionSettingEditResponseBindingsType,

    /// Identifier of the D1 database to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_destination_addresses: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sender_addresses: Option<String>,

    /// R2 bucket to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    /// Identifier of the certificate to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,

    /// The exported class name of the Durable Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The name of the dataset to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Destination address for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// The environment of the script_name to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptScriptAndVersionSettingEditResponseBindingsFormat>,

    /// Name of the Vectorize index to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,

    /// The
    /// [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions)
    /// of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<ScriptScriptAndVersionSettingEditResponseBindingsJurisdiction>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// The name of the dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Namespace identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// The old name of the inherited binding. If set, the binding will be renamed from
    /// `old_name` to `name` in the new version. If not set, the binding will keep the
    /// same name between versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptScriptAndVersionSettingEditResponseBindingsWorkersBindingKindDispatchNamespaceOutbound].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,

    /// The name of the file containing the data content. Only accepted for
    /// `service worker syntax` Workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,

    /// Name of the Pipeline to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    /// Name of the Queue to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// The script where the Durable Object is defined, if it is external to this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// Name of the secret in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// Name of Worker to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptScriptAndVersionSettingEditResponseBindingsWorkersBindingKindRatelimitSimple].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<String>,

    /// ID of the store containing the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,

    /// The text value to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptScriptAndVersionSettingEditResponseBindingsWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

    /// Identifier for the version to inherit the binding from, which can be the version
    /// ID or the literal "latest" to inherit from the latest version. Defaults to
    /// inheriting the binding from the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Name of the Workflow to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponseLimits {
    /// The amount of CPU time this Worker can use in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_ms: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponseObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptScriptAndVersionSettingEditResponseObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponsePlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ScriptScriptAndVersionSettingEditResponsePlacementModeMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptScriptAndVersionSettingEditResponsePlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponseBinding {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptScriptAndVersionSettingGetResponseBindingsType,

    /// Identifier of the D1 database to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_destination_addresses: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sender_addresses: Option<String>,

    /// R2 bucket to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    /// Identifier of the certificate to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,

    /// The exported class name of the Durable Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The name of the dataset to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Destination address for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// The environment of the script_name to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptScriptAndVersionSettingGetResponseBindingsFormat>,

    /// Name of the Vectorize index to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,

    /// The
    /// [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions)
    /// of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<ScriptScriptAndVersionSettingGetResponseBindingsJurisdiction>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// The name of the dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Namespace identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// The old name of the inherited binding. If set, the binding will be renamed from
    /// `old_name` to `name` in the new version. If not set, the binding will keep the
    /// same name between versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptScriptAndVersionSettingGetResponseBindingsWorkersBindingKindDispatchNamespaceOutbound].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,

    /// The name of the file containing the data content. Only accepted for
    /// `service worker syntax` Workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,

    /// Name of the Pipeline to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    /// Name of the Queue to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// The script where the Durable Object is defined, if it is external to this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// Name of the secret in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// Name of Worker to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptScriptAndVersionSettingGetResponseBindingsWorkersBindingKindRatelimitSimple].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<String>,

    /// ID of the store containing the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,

    /// The text value to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptScriptAndVersionSettingGetResponseBindingsWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

    /// Identifier for the version to inherit the binding from, which can be the version
    /// ID or the literal "latest" to inherit from the latest version. Defaults to
    /// inheriting the binding from the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Name of the Workflow to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponseLimits {
    /// The amount of CPU time this Worker can use in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_ms: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponseObservability {
    /// Whether observability is enabled for the Worker.
    pub enabled: bool,

    /// The sampling rate for incoming requests. From 0 to 1 (1 = 100%, 0.1 = 10%).
    /// Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Log settings for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<ScriptScriptAndVersionSettingGetResponseObservabilityLogs>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponsePlacement {
    /// TCP host and port for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// HTTP hostname for targeted placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Enables
    /// [Smart Placement](https://developers.cloudflare.com/workers/configuration/smart-placement).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ScriptScriptAndVersionSettingGetResponsePlacementModeMode>,

    /// Cloud region for targeted placement in format 'provider:region'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptScriptAndVersionSettingGetResponsePlacementObjectTarget].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptTailDeleteResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptTailDeleteResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResources {
    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ScriptVersionNewResponseResourcesBinding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptVersionNewResponseResourcesScript>,

    /// Runtime configuration for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_runtime: Option<ScriptVersionNewResponseResourcesScriptRuntime>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseMetadata {
    /// Email of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

    /// Identifier of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,

    /// When the version was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    /// Whether the version can be previewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasPreview")]
    pub has_preview: Option<bool>,

    /// When the version was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

    /// The source of the version upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptVersionNewResponseMetadataSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionListResponseMetadata {
    /// Email of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

    /// Identifier of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,

    /// When the version was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    /// Whether the version can be previewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasPreview")]
    pub has_preview: Option<bool>,

    /// When the version was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

    /// The source of the version upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptVersionListResponseMetadataSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResources {
    /// List of bindings attached to a Worker. You can find more about bindings on our
    /// docs:
    /// https://developers.cloudflare.com/workers/configuration/multipart-upload-metadata/#bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ScriptVersionGetResponseResourcesBinding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptVersionGetResponseResourcesScript>,

    /// Runtime configuration for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_runtime: Option<ScriptVersionGetResponseResourcesScriptRuntime>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseMetadata {
    /// Email of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

    /// Identifier of the user who created the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,

    /// When the version was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    /// Whether the version can be previewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasPreview")]
    pub has_preview: Option<bool>,

    /// When the version was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,

    /// The source of the version upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ScriptVersionGetResponseMetadataSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_logs: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferencesDispatchNamespaceOutbound {
    /// ID of the dispatch namespace.
    pub namespace_id: String,

    /// Name of the dispatch namespace.
    pub namespace_name: String,

    /// ID of the Worker using the dispatch namespace.
    pub worker_id: String,

    /// Name of the Worker using the dispatch namespace.
    pub worker_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferencesDomain {
    /// ID of the custom domain.
    pub id: String,

    /// ID of the TLS certificate issued for the custom domain.
    pub certificate_id: String,

    /// Full hostname of the custom domain, including the zone name.
    pub hostname: String,

    /// ID of the zone.
    pub zone_id: String,

    /// Name of the zone.
    pub zone_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferencesDurableObject {
    /// ID of the Durable Object namespace being used.
    pub namespace_id: String,

    /// Name of the Durable Object namespace being used.
    pub namespace_name: String,

    /// ID of the Worker using the Durable Object implementation.
    pub worker_id: String,

    /// Name of the Worker using the Durable Object implementation.
    pub worker_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferencesQueue {
    /// ID of the queue consumer configuration.
    pub queue_consumer_id: String,

    /// ID of the queue.
    pub queue_id: String,

    /// Name of the queue.
    pub queue_name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReferencesWorker {
    /// ID of the referencing Worker.
    pub id: String,

    /// Name of the referencing Worker.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssetsConfig {
    /// Determines the redirects and rewrites of requests for HTML content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_handling: Option<VersionAssetsConfigHTMLHandling>,

    /// Determines the response when a request does not match a static asset, and there
    /// is no Worker script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_found_handling: Option<VersionAssetsConfigNotFoundHandling>,

    /// Contains a list path rules to control routing to either the Worker or assets.
    /// Glob (\*) and negative (!) rules are supported. Rules must start with either '/'
    /// or '!/'. At least one non-negative rule must be provided, and negative rules
    /// have higher precedence than non-negative rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_worker_first: Option<VersionAssetsConfigRunWorkerFirstUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaWorkerVersionDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQuery {
    /// ID of the query
    pub id: String,

    pub created: String,

    pub description: String,

    /// ID of your environment
    #[serde(rename = "environmentId")]
    pub environment_id: String,

    /// Flag for alerts automatically created
    pub generated: bool,

    /// Query name
    pub name: String,

    pub parameters: ObservabilityTelemetryQueryResponseRunQueryParameters,

    pub updated: String,

    #[serde(rename = "userId")]
    pub user_id: String,

    /// ID of your workspace
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunTimeframe {
    /// Start timestamp for the query timeframe (Unix timestamp in milliseconds)
    pub from: f64,

    /// End timestamp for the query timeframe (Unix timestamp in milliseconds)
    pub to: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunStatistics {
    /// Number of uncompressed bytes read from the table.
    pub bytes_read: f64,

    /// Time in seconds for the query to run.
    pub elapsed: f64,

    /// Number of rows scanned from the table.
    pub rows_read: f64,

    /// The level of Adaptive Bit Rate (ABR) sampling used for the query. If empty the
    /// ABR level is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abr_level: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculationsAggregate {
    pub count: f64,

    pub interval: f64,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ObservabilityTelemetryQueryResponseCalculationsAggregatesGroup>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculationsSeries {
    pub data: Vec<ObservabilityTelemetryQueryResponseCalculationsSeriesData>,

    pub time: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompareAggregate {
    pub count: f64,

    pub interval: f64,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ObservabilityTelemetryQueryResponseCompareAggregatesGroup>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompareSeries {
    pub data: Vec<ObservabilityTelemetryQueryResponseCompareSeriesData>,

    pub time: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsEvent {
    #[serde(rename = "$metadata")]
    pub metadata: ObservabilityTelemetryQueryResponseEventsEventsMetadata,

    pub dataset: String,

    pub source: String,

    pub timestamp: i64,

    /// Cloudflare Containers event information enriches your logs so you can easily
    /// identify and debug issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$containers")]
    pub containers: Option<String>,

    /// Cloudflare Workers event information enriches your logs so you can easily
    /// identify and debug issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$workers")]
    pub workers: Option<ObservabilityTelemetryQueryResponseEventsEventsWorkers>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsField {
    pub key: String,

    pub r#type: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsSeries {
    pub data: Vec<ObservabilityTelemetryQueryResponseEventsSeriesData>,

    pub time: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseInvocationsMetadata {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudService")]
    pub cloud_service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coldStart")]
    pub cold_start: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorTemplate")]
    pub error_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageTemplate")]
    pub message_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metricName")]
    pub metric_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentSpanId")]
    pub parent_span_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spanId")]
    pub span_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spanName")]
    pub span_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackId")]
    pub stack_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statusCode")]
    pub status_code: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceDuration")]
    pub trace_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transactionName")]
    pub transaction_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseInvocationsWorkers {
    #[serde(rename = "eventType")]
    pub event_type: ObservabilityTelemetryQueryResponseInvocationsWorkersEventType,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "scriptName")]
    pub script_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cpuTimeMs")]
    pub cpu_time_ms: Option<f64>,

    /// This field can have the runtime type of
    /// [[]ObservabilityTelemetryQueryResponseInvocationsWorkersObjectDiagnosticsChannelEvent].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diagnosticsChannelEvents")]
    pub diagnostics_channel_events: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dispatchNamespace")]
    pub dispatch_namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durableObjectId")]
    pub durable_object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,

    /// This field can have the runtime type of
    /// [map[string]ObservabilityTelemetryQueryResponseInvocationsWorkersObjectEventUnion].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionModel")]
    pub execution_model: Option<ObservabilityTelemetryQueryResponseInvocationsWorkersExecutionModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,

    /// This field can have the runtime type of
    /// [ObservabilityTelemetryQueryResponseInvocationsWorkersObjectScriptVersion].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptVersion")]
    pub script_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wallTimeMs")]
    pub wall_time_ms: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponsePatternsSeries {
    pub data: ObservabilityTelemetryQueryResponsePatternsSeriesData,

    pub time: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSettingObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptUpdateResponseObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListResponseObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDeploymentDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingEditResponseObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScriptAndVersionSettingGetResponseObservabilityLogs {
    /// Whether logs are enabled for the Worker.
    pub enabled: bool,

    /// Whether
    /// [invocation logs](https://developers.cloudflare.com/workers/observability/logs/workers-logs/#invocation-logs)
    /// are enabled for the Worker.
    pub invocation_logs: bool,

    /// A list of destinations where logs will be exported to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// The sampling rate for logs. From 0 to 1 (1 = 100%, 0.1 = 10%). Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sampling_rate: Option<f64>,

    /// Whether log persistence is enabled for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTailDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResourcesBinding {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptVersionNewResponseResourcesBindingsType,

    /// Identifier of the D1 database to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_destination_addresses: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sender_addresses: Option<String>,

    /// R2 bucket to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    /// Identifier of the certificate to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,

    /// The exported class name of the Durable Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The name of the dataset to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Destination address for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// The environment of the script_name to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptVersionNewResponseResourcesBindingsFormat>,

    /// Name of the Vectorize index to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,

    /// The
    /// [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions)
    /// of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<ScriptVersionNewResponseResourcesBindingsJurisdiction>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// The name of the dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Namespace identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// The old name of the inherited binding. If set, the binding will be renamed from
    /// `old_name` to `name` in the new version. If not set, the binding will keep the
    /// same name between versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptVersionNewResponseResourcesBindingsWorkersBindingKindDispatchNamespaceOutbound].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,

    /// The name of the file containing the data content. Only accepted for
    /// `service worker syntax` Workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,

    /// Name of the Pipeline to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    /// Name of the Queue to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// The script where the Durable Object is defined, if it is external to this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// Name of the secret in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// Name of Worker to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptVersionNewResponseResourcesBindingsWorkersBindingKindRatelimitSimple].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<String>,

    /// ID of the store containing the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,

    /// The text value to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptVersionNewResponseResourcesBindingsWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

    /// Identifier for the version to inherit the binding from, which can be the version
    /// ID or the literal "latest" to inherit from the latest version. Defaults to
    /// inheriting the binding from the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Name of the Workflow to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResourcesScript {
    /// Hashed script content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The names of handlers exported as part of the default export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The client most recently used to deploy this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployed_from: Option<String>,

    /// Named exports, such as Durable Object class implementations and named
    /// entrypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_handlers: Option<Vec<ScriptVersionNewResponseResourcesScriptNamedHandler>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResourcesScriptRuntime {
    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// Resource limits for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ScriptVersionNewResponseResourcesScriptRuntimeLimits>,

    /// The tag of the Durable Object migration that was most recently applied for this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_tag: Option<String>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptVersionNewResponseResourcesScriptRuntimeUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResourcesBinding {
    /// A JavaScript variable name for the binding.
    pub name: String,

    /// The kind of resource that the binding provides.
    pub r#type: ScriptVersionGetResponseResourcesBindingsType,

    /// Identifier of the D1 database to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_destination_addresses: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_sender_addresses: Option<String>,

    /// R2 bucket to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,

    /// Identifier of the certificate to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,

    /// The exported class name of the Durable Object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The name of the dataset to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Destination address for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,

    /// The environment of the script_name to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Data format of the key.
    /// [Learn more](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScriptVersionGetResponseResourcesBindingsFormat>,

    /// Name of the Vectorize index to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,

    /// The
    /// [jurisdiction](https://developers.cloudflare.com/r2/reference/data-location/#jurisdictional-restrictions)
    /// of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<ScriptVersionGetResponseResourcesBindingsJurisdiction>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_jwk: Option<String>,

    /// The name of the dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Namespace identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// The old name of the inherited binding. If set, the binding will be renamed from
    /// `old_name` to `name` in the new version. If not set, the binding will keep the
    /// same name between versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptVersionGetResponseResourcesBindingsWorkersBindingKindDispatchNamespaceOutbound].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,

    /// The name of the file containing the data content. Only accepted for
    /// `service worker syntax` Workers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<String>,

    /// Name of the Pipeline to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    /// Name of the Queue to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,

    /// The script where the Durable Object is defined, if it is external to this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    /// Name of the secret in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,

    /// Name of Worker to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// This field can have the runtime type of
    /// [ScriptVersionGetResponseResourcesBindingsWorkersBindingKindRatelimitSimple].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<String>,

    /// ID of the store containing the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,

    /// The text value to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// This field can have the runtime type of
    /// [[]ScriptVersionGetResponseResourcesBindingsWorkersBindingKindSecretKeyUsage].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<String>,

    /// Identifier for the version to inherit the binding from, which can be the version
    /// ID or the literal "latest" to inherit from the latest version. Defaults to
    /// inheriting the binding from the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Name of the Workflow to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResourcesScript {
    /// Hashed script content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The names of handlers exported as part of the default export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The client most recently used to deploy this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployed_from: Option<String>,

    /// Named exports, such as Durable Object class implementations and named
    /// entrypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_handlers: Option<Vec<ScriptVersionGetResponseResourcesScriptNamedHandler>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResourcesScriptRuntime {
    /// Date indicating targeted support in the Workers runtime. Backwards incompatible
    /// fixes to the runtime following this date will not affect this Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_date: Option<String>,

    /// Flags that enable or disable certain features in the Workers runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_flags: Option<Vec<String>>,

    /// Resource limits for the Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ScriptVersionGetResponseResourcesScriptRuntimeLimits>,

    /// The tag of the Durable Object migration that was most recently applied for this
    /// Worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_tag: Option<String>,

    /// Usage model for the Worker invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<ScriptVersionGetResponseResourcesScriptRuntimeUsageModel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParameters {
    /// Create Calculations to compute as part of the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculations: Option<Vec<ObservabilityTelemetryQueryResponseRunQueryParametersCalculation>>,

    /// Set the Datasets to query. Leave it empty to query all the datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<String>>,

    /// Set a Flag to describe how to combine the filters on the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filterCombination")]
    pub filter_combination: Option<ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombination>,

    /// Configure the Filters to apply to the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ObservabilityTelemetryQueryResponseRunQueryParametersFilter>>,

    /// Define how to group the results of the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupBys")]
    pub group_bys: Option<Vec<ObservabilityTelemetryQueryResponseRunQueryParametersGroupBy>>,

    /// Configure the Having clauses that filter on calculations in the query result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub havings: Option<Vec<ObservabilityTelemetryQueryResponseRunQueryParametersHaving>>,

    /// Set a limit on the number of results / records returned by the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Define an expression to search using full-text search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needle: Option<ObservabilityTelemetryQueryResponseRunQueryParametersNeedle>,

    /// Configure the order of the results returned by the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "orderBy")]
    pub order_by: Option<ObservabilityTelemetryQueryResponseRunQueryParametersOrderBy>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculationsAggregatesGroup {
    pub key: String,

    pub value: ObservabilityTelemetryQueryResponseCalculationsAggregatesGroupsValueUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculationsSeriesData {
    pub count: f64,

    #[serde(rename = "firstSeen")]
    pub first_seen: String,

    pub interval: f64,

    #[serde(rename = "lastSeen")]
    pub last_seen: String,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ObservabilityTelemetryQueryResponseCalculationsSeriesDataGroup>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompareAggregatesGroup {
    pub key: String,

    pub value: ObservabilityTelemetryQueryResponseCompareAggregatesGroupsValueUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompareSeriesData {
    pub count: f64,

    #[serde(rename = "firstSeen")]
    pub first_seen: String,

    pub interval: f64,

    #[serde(rename = "lastSeen")]
    pub last_seen: String,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ObservabilityTelemetryQueryResponseCompareSeriesDataGroup>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsEventsMetadata {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudService")]
    pub cloud_service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coldStart")]
    pub cold_start: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorTemplate")]
    pub error_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageTemplate")]
    pub message_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metricName")]
    pub metric_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentSpanId")]
    pub parent_span_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spanId")]
    pub span_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spanName")]
    pub span_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackId")]
    pub stack_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statusCode")]
    pub status_code: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceDuration")]
    pub trace_duration: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transactionName")]
    pub transaction_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsEventsWorkers {
    #[serde(rename = "eventType")]
    pub event_type: ObservabilityTelemetryQueryResponseEventsEventsWorkersEventType,

    #[serde(rename = "requestId")]
    pub request_id: String,

    #[serde(rename = "scriptName")]
    pub script_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cpuTimeMs")]
    pub cpu_time_ms: Option<f64>,

    /// This field can have the runtime type of
    /// [[]ObservabilityTelemetryQueryResponseEventsEventsWorkersObjectDiagnosticsChannelEvent].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "diagnosticsChannelEvents")]
    pub diagnostics_channel_events: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dispatchNamespace")]
    pub dispatch_namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "durableObjectId")]
    pub durable_object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,

    /// This field can have the runtime type of
    /// [map[string]ObservabilityTelemetryQueryResponseEventsEventsWorkersObjectEventUnion].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionModel")]
    pub execution_model: Option<ObservabilityTelemetryQueryResponseEventsEventsWorkersExecutionModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,

    /// This field can have the runtime type of
    /// [ObservabilityTelemetryQueryResponseEventsEventsWorkersObjectScriptVersion].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptVersion")]
    pub script_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wallTimeMs")]
    pub wall_time_ms: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsSeriesData {
    pub aggregates: ObservabilityTelemetryQueryResponseEventsSeriesDataAggregates,

    pub count: f64,

    pub interval: f64,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<f64>,

    /// Groups in the query results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<::std::collections::HashMap<String, ObservabilityTelemetryQueryResponseEventsSeriesDataGroupsUnion>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponsePatternsSeriesData {
    pub count: f64,

    pub interval: f64,

    #[serde(rename = "sampleInterval")]
    pub sample_interval: f64,

    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ObservabilityTelemetryQueryResponsePatternsSeriesDataGroup>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResourcesScriptNamedHandler {
    /// The names of handlers exported as part of the named export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The name of the exported class or entrypoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionNewResponseResourcesScriptRuntimeLimits {
    /// The amount of CPU time this Worker can use in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_ms: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResourcesScriptNamedHandler {
    /// The names of handlers exported as part of the named export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<String>>,

    /// The name of the exported class or entrypoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersionGetResponseResourcesScriptRuntimeLimits {
    /// The amount of CPU time this Worker can use in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_ms: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersCalculation {
    pub operator: ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperator,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyType")]
    pub key_type: Option<ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsKeyType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersFilter {
    pub key: String,

    pub operation: ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperation,

    pub r#type: ObservabilityTelemetryQueryResponseRunQueryParametersFiltersType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ObservabilityTelemetryQueryResponseRunQueryParametersFiltersValueUnion>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersGroupBy {
    pub r#type: ObservabilityTelemetryQueryResponseRunQueryParametersGroupBysType,

    pub value: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersHaving {
    pub key: String,

    pub operation: ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperation,

    pub value: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersNeedle {
    pub value: ObservabilityTelemetryQueryResponseRunQueryParametersNeedleValueUnion,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isRegex")]
    pub is_regex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchCase")]
    pub match_case: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseRunQueryParametersOrderBy {
    /// Configure which Calculation to order the results by.
    pub value: String,

    /// Set the order of the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ObservabilityTelemetryQueryResponseRunQueryParametersOrderByOrder>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCalculationsSeriesDataGroup {
    pub key: String,

    pub value: ObservabilityTelemetryQueryResponseCalculationsSeriesDataGroupsValueUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseCompareSeriesDataGroup {
    pub key: String,

    pub value: ObservabilityTelemetryQueryResponseCompareSeriesDataGroupsValueUnion,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponseEventsSeriesDataAggregates {
    /// Deprecated: deprecated
    #[serde(rename = "_count")]
    pub count: i64,

    /// Deprecated: deprecated
    #[serde(rename = "_firstSeen")]
    pub first_seen: String,

    /// Deprecated: deprecated
    #[serde(rename = "_interval")]
    pub interval: i64,

    /// Deprecated: deprecated
    #[serde(rename = "_lastSeen")]
    pub last_seen: String,

    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityTelemetryQueryResponsePatternsSeriesDataGroup {
    pub key: String,

    pub value: ObservabilityTelemetryQueryResponsePatternsSeriesDataGroupsValueUnion,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionUsageModel {
    #[serde(rename = "standard")]
    VersionUsageModelStandard,
    #[serde(rename = "bundled")]
    VersionUsageModelBundled,
    #[serde(rename = "unbound")]
    VersionUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryKeysResponseType {
    #[serde(rename = "string")]
    ObservabilityTelemetryKeysResponseTypeString,
    #[serde(rename = "boolean")]
    ObservabilityTelemetryKeysResponseTypeBoolean,
    #[serde(rename = "number")]
    ObservabilityTelemetryKeysResponseTypeNumber,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryValuesResponseType {
    #[serde(rename = "string")]
    ObservabilityTelemetryValuesResponseTypeString,
    #[serde(rename = "boolean")]
    ObservabilityTelemetryValuesResponseTypeBoolean,
    #[serde(rename = "number")]
    ObservabilityTelemetryValuesResponseTypeNumber,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptPlacementMode {
    #[serde(rename = "smart")]
    ScriptPlacementModeSmart,
    #[serde(rename = "targeted")]
    ScriptPlacementModeTargeted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptPlacementStatus {
    #[serde(rename = "SUCCESS")]
    ScriptPlacementStatusSuccess,
    #[serde(rename = "UNSUPPORTED_APPLICATION")]
    ScriptPlacementStatusUnsupportedApplication,
    #[serde(rename = "INSUFFICIENT_INVOCATIONS")]
    ScriptPlacementStatusInsufficientInvocations,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptUsageModel {
    #[serde(rename = "standard")]
    ScriptUsageModelStandard,
    #[serde(rename = "bundled")]
    ScriptUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptUpdateResponsePlacementMode {
    #[serde(rename = "smart")]
    ScriptUpdateResponsePlacementModeSmart,
    #[serde(rename = "targeted")]
    ScriptUpdateResponsePlacementModeTargeted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptUpdateResponsePlacementStatus {
    #[serde(rename = "SUCCESS")]
    ScriptUpdateResponsePlacementStatusSuccess,
    #[serde(rename = "UNSUPPORTED_APPLICATION")]
    ScriptUpdateResponsePlacementStatusUnsupportedApplication,
    #[serde(rename = "INSUFFICIENT_INVOCATIONS")]
    ScriptUpdateResponsePlacementStatusInsufficientInvocations,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptUpdateResponseUsageModel {
    #[serde(rename = "standard")]
    ScriptUpdateResponseUsageModelStandard,
    #[serde(rename = "bundled")]
    ScriptUpdateResponseUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptUpdateResponseUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptListResponsePlacementMode {
    #[serde(rename = "smart")]
    ScriptListResponsePlacementModeSmart,
    #[serde(rename = "targeted")]
    ScriptListResponsePlacementModeTargeted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptListResponsePlacementStatus {
    #[serde(rename = "SUCCESS")]
    ScriptListResponsePlacementStatusSuccess,
    #[serde(rename = "UNSUPPORTED_APPLICATION")]
    ScriptListResponsePlacementStatusUnsupportedApplication,
    #[serde(rename = "INSUFFICIENT_INVOCATIONS")]
    ScriptListResponsePlacementStatusInsufficientInvocations,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptListResponseUsageModel {
    #[serde(rename = "standard")]
    ScriptListResponseUsageModelStandard,
    #[serde(rename = "bundled")]
    ScriptListResponseUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptListResponseUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStrategy {
    #[serde(rename = "percentage")]
    DeploymentStrategyPercentage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingEditResponseUsageModel {
    #[serde(rename = "standard")]
    ScriptScriptAndVersionSettingEditResponseUsageModelStandard,
    #[serde(rename = "bundled")]
    ScriptScriptAndVersionSettingEditResponseUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptScriptAndVersionSettingEditResponseUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingGetResponseUsageModel {
    #[serde(rename = "standard")]
    ScriptScriptAndVersionSettingGetResponseUsageModelStandard,
    #[serde(rename = "bundled")]
    ScriptScriptAndVersionSettingGetResponseUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptScriptAndVersionSettingGetResponseUsageModelUnbound,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretUpdateResponseType {
    #[serde(rename = "secret_text")]
    ScriptSecretUpdateResponseTypeSecretText,
    #[serde(rename = "secret_key")]
    ScriptSecretUpdateResponseTypeSecretKey,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretUpdateResponseFormat {
    #[serde(rename = "raw")]
    ScriptSecretUpdateResponseFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptSecretUpdateResponseFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptSecretUpdateResponseFormatSpki,
    #[serde(rename = "jwk")]
    ScriptSecretUpdateResponseFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretListResponseType {
    #[serde(rename = "secret_text")]
    ScriptSecretListResponseTypeSecretText,
    #[serde(rename = "secret_key")]
    ScriptSecretListResponseTypeSecretKey,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretListResponseFormat {
    #[serde(rename = "raw")]
    ScriptSecretListResponseFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptSecretListResponseFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptSecretListResponseFormatSpki,
    #[serde(rename = "jwk")]
    ScriptSecretListResponseFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretGetResponseType {
    #[serde(rename = "secret_text")]
    ScriptSecretGetResponseTypeSecretText,
    #[serde(rename = "secret_key")]
    ScriptSecretGetResponseTypeSecretKey,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptSecretGetResponseFormat {
    #[serde(rename = "raw")]
    ScriptSecretGetResponseFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptSecretGetResponseFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptSecretGetResponseFormatSpki,
    #[serde(rename = "jwk")]
    ScriptSecretGetResponseFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionBindingsType {
    #[serde(rename = "ai")]
    VersionBindingsTypeAI,
    #[serde(rename = "analytics_engine")]
    VersionBindingsTypeAnalyticsEngine,
    #[serde(rename = "assets")]
    VersionBindingsTypeAssets,
    #[serde(rename = "browser")]
    VersionBindingsTypeBrowser,
    #[serde(rename = "d1")]
    VersionBindingsTypeD1,
    #[serde(rename = "data_blob")]
    VersionBindingsTypeDataBlob,
    #[serde(rename = "dispatch_namespace")]
    VersionBindingsTypeDispatchNamespace,
    #[serde(rename = "durable_object_namespace")]
    VersionBindingsTypeDurableObjectNamespace,
    #[serde(rename = "hyperdrive")]
    VersionBindingsTypeHyperdrive,
    #[serde(rename = "inherit")]
    VersionBindingsTypeInherit,
    #[serde(rename = "images")]
    VersionBindingsTypeImages,
    #[serde(rename = "json")]
    VersionBindingsTypeJson,
    #[serde(rename = "kv_namespace")]
    VersionBindingsTypeKVNamespace,
    #[serde(rename = "mtls_certificate")]
    VersionBindingsTypeMTLSCertificate,
    #[serde(rename = "plain_text")]
    VersionBindingsTypePlainText,
    #[serde(rename = "pipelines")]
    VersionBindingsTypePipelines,
    #[serde(rename = "queue")]
    VersionBindingsTypeQueue,
    #[serde(rename = "ratelimit")]
    VersionBindingsTypeRatelimit,
    #[serde(rename = "r2_bucket")]
    VersionBindingsTypeR2Bucket,
    #[serde(rename = "secret_text")]
    VersionBindingsTypeSecretText,
    #[serde(rename = "send_email")]
    VersionBindingsTypeSendEmail,
    #[serde(rename = "service")]
    VersionBindingsTypeService,
    #[serde(rename = "text_blob")]
    VersionBindingsTypeTextBlob,
    #[serde(rename = "vectorize")]
    VersionBindingsTypeVectorize,
    #[serde(rename = "version_metadata")]
    VersionBindingsTypeVersionMetadata,
    #[serde(rename = "secrets_store_secret")]
    VersionBindingsTypeSecretsStoreSecret,
    #[serde(rename = "secret_key")]
    VersionBindingsTypeSecretKey,
    #[serde(rename = "workflow")]
    VersionBindingsTypeWorkflow,
    #[serde(rename = "wasm_module")]
    VersionBindingsTypeWasmModule,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionBindingsFormat {
    #[serde(rename = "raw")]
    VersionBindingsFormatRaw,
    #[serde(rename = "pkcs8")]
    VersionBindingsFormatPkcs8,
    #[serde(rename = "spki")]
    VersionBindingsFormatSpki,
    #[serde(rename = "jwk")]
    VersionBindingsFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionBindingsJurisdiction {
    #[serde(rename = "eu")]
    VersionBindingsJurisdictionEu,
    #[serde(rename = "fedramp")]
    VersionBindingsJurisdictionFedramp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionPlacementModeMode {
    #[serde(rename = "smart")]
    VersionPlacementModeModeSmart,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunStatus {
    #[serde(rename = "STARTED")]
    ObservabilityTelemetryQueryResponseRunStatusStarted,
    #[serde(rename = "COMPLETED")]
    ObservabilityTelemetryQueryResponseRunStatusCompleted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingEditResponseBindingsType {
    #[serde(rename = "ai")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeAI,
    #[serde(rename = "analytics_engine")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeAnalyticsEngine,
    #[serde(rename = "assets")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeAssets,
    #[serde(rename = "browser")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeBrowser,
    #[serde(rename = "d1")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeD1,
    #[serde(rename = "data_blob")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeDataBlob,
    #[serde(rename = "dispatch_namespace")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeDispatchNamespace,
    #[serde(rename = "durable_object_namespace")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeDurableObjectNamespace,
    #[serde(rename = "hyperdrive")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeHyperdrive,
    #[serde(rename = "inherit")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeInherit,
    #[serde(rename = "images")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeImages,
    #[serde(rename = "json")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeJson,
    #[serde(rename = "kv_namespace")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeKVNamespace,
    #[serde(rename = "mtls_certificate")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeMTLSCertificate,
    #[serde(rename = "plain_text")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypePlainText,
    #[serde(rename = "pipelines")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypePipelines,
    #[serde(rename = "queue")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeQueue,
    #[serde(rename = "ratelimit")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeRatelimit,
    #[serde(rename = "r2_bucket")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeR2Bucket,
    #[serde(rename = "secret_text")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeSecretText,
    #[serde(rename = "send_email")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeSendEmail,
    #[serde(rename = "service")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeService,
    #[serde(rename = "text_blob")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeTextBlob,
    #[serde(rename = "vectorize")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeVectorize,
    #[serde(rename = "version_metadata")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeVersionMetadata,
    #[serde(rename = "secrets_store_secret")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeSecretsStoreSecret,
    #[serde(rename = "secret_key")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeSecretKey,
    #[serde(rename = "workflow")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeWorkflow,
    #[serde(rename = "wasm_module")]
    ScriptScriptAndVersionSettingEditResponseBindingsTypeWasmModule,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingEditResponseBindingsFormat {
    #[serde(rename = "raw")]
    ScriptScriptAndVersionSettingEditResponseBindingsFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptScriptAndVersionSettingEditResponseBindingsFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptScriptAndVersionSettingEditResponseBindingsFormatSpki,
    #[serde(rename = "jwk")]
    ScriptScriptAndVersionSettingEditResponseBindingsFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingEditResponseBindingsJurisdiction {
    #[serde(rename = "eu")]
    ScriptScriptAndVersionSettingEditResponseBindingsJurisdictionEu,
    #[serde(rename = "fedramp")]
    ScriptScriptAndVersionSettingEditResponseBindingsJurisdictionFedramp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingEditResponsePlacementModeMode {
    #[serde(rename = "smart")]
    ScriptScriptAndVersionSettingEditResponsePlacementModeModeSmart,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingGetResponseBindingsType {
    #[serde(rename = "ai")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeAI,
    #[serde(rename = "analytics_engine")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeAnalyticsEngine,
    #[serde(rename = "assets")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeAssets,
    #[serde(rename = "browser")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeBrowser,
    #[serde(rename = "d1")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeD1,
    #[serde(rename = "data_blob")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeDataBlob,
    #[serde(rename = "dispatch_namespace")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeDispatchNamespace,
    #[serde(rename = "durable_object_namespace")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeDurableObjectNamespace,
    #[serde(rename = "hyperdrive")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeHyperdrive,
    #[serde(rename = "inherit")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeInherit,
    #[serde(rename = "images")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeImages,
    #[serde(rename = "json")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeJson,
    #[serde(rename = "kv_namespace")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeKVNamespace,
    #[serde(rename = "mtls_certificate")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeMTLSCertificate,
    #[serde(rename = "plain_text")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypePlainText,
    #[serde(rename = "pipelines")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypePipelines,
    #[serde(rename = "queue")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeQueue,
    #[serde(rename = "ratelimit")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeRatelimit,
    #[serde(rename = "r2_bucket")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeR2Bucket,
    #[serde(rename = "secret_text")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeSecretText,
    #[serde(rename = "send_email")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeSendEmail,
    #[serde(rename = "service")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeService,
    #[serde(rename = "text_blob")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeTextBlob,
    #[serde(rename = "vectorize")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeVectorize,
    #[serde(rename = "version_metadata")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeVersionMetadata,
    #[serde(rename = "secrets_store_secret")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeSecretsStoreSecret,
    #[serde(rename = "secret_key")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeSecretKey,
    #[serde(rename = "workflow")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeWorkflow,
    #[serde(rename = "wasm_module")]
    ScriptScriptAndVersionSettingGetResponseBindingsTypeWasmModule,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingGetResponseBindingsFormat {
    #[serde(rename = "raw")]
    ScriptScriptAndVersionSettingGetResponseBindingsFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptScriptAndVersionSettingGetResponseBindingsFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptScriptAndVersionSettingGetResponseBindingsFormatSpki,
    #[serde(rename = "jwk")]
    ScriptScriptAndVersionSettingGetResponseBindingsFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingGetResponseBindingsJurisdiction {
    #[serde(rename = "eu")]
    ScriptScriptAndVersionSettingGetResponseBindingsJurisdictionEu,
    #[serde(rename = "fedramp")]
    ScriptScriptAndVersionSettingGetResponseBindingsJurisdictionFedramp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptScriptAndVersionSettingGetResponsePlacementModeMode {
    #[serde(rename = "smart")]
    ScriptScriptAndVersionSettingGetResponsePlacementModeModeSmart,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionNewResponseMetadataSource {
    #[serde(rename = "unknown")]
    ScriptVersionNewResponseMetadataSourceUnknown,
    #[serde(rename = "api")]
    ScriptVersionNewResponseMetadataSourceAPI,
    #[serde(rename = "wrangler")]
    ScriptVersionNewResponseMetadataSourceWrangler,
    #[serde(rename = "terraform")]
    ScriptVersionNewResponseMetadataSourceTerraform,
    #[serde(rename = "dash")]
    ScriptVersionNewResponseMetadataSourceDash,
    #[serde(rename = "dash_template")]
    ScriptVersionNewResponseMetadataSourceDashTemplate,
    #[serde(rename = "integration")]
    ScriptVersionNewResponseMetadataSourceIntegration,
    #[serde(rename = "quick_editor")]
    ScriptVersionNewResponseMetadataSourceQuickEditor,
    #[serde(rename = "playground")]
    ScriptVersionNewResponseMetadataSourcePlayground,
    #[serde(rename = "workersci")]
    ScriptVersionNewResponseMetadataSourceWorkersci,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionListResponseMetadataSource {
    #[serde(rename = "unknown")]
    ScriptVersionListResponseMetadataSourceUnknown,
    #[serde(rename = "api")]
    ScriptVersionListResponseMetadataSourceAPI,
    #[serde(rename = "wrangler")]
    ScriptVersionListResponseMetadataSourceWrangler,
    #[serde(rename = "terraform")]
    ScriptVersionListResponseMetadataSourceTerraform,
    #[serde(rename = "dash")]
    ScriptVersionListResponseMetadataSourceDash,
    #[serde(rename = "dash_template")]
    ScriptVersionListResponseMetadataSourceDashTemplate,
    #[serde(rename = "integration")]
    ScriptVersionListResponseMetadataSourceIntegration,
    #[serde(rename = "quick_editor")]
    ScriptVersionListResponseMetadataSourceQuickEditor,
    #[serde(rename = "playground")]
    ScriptVersionListResponseMetadataSourcePlayground,
    #[serde(rename = "workersci")]
    ScriptVersionListResponseMetadataSourceWorkersci,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionGetResponseMetadataSource {
    #[serde(rename = "unknown")]
    ScriptVersionGetResponseMetadataSourceUnknown,
    #[serde(rename = "api")]
    ScriptVersionGetResponseMetadataSourceAPI,
    #[serde(rename = "wrangler")]
    ScriptVersionGetResponseMetadataSourceWrangler,
    #[serde(rename = "terraform")]
    ScriptVersionGetResponseMetadataSourceTerraform,
    #[serde(rename = "dash")]
    ScriptVersionGetResponseMetadataSourceDash,
    #[serde(rename = "dash_template")]
    ScriptVersionGetResponseMetadataSourceDashTemplate,
    #[serde(rename = "integration")]
    ScriptVersionGetResponseMetadataSourceIntegration,
    #[serde(rename = "quick_editor")]
    ScriptVersionGetResponseMetadataSourceQuickEditor,
    #[serde(rename = "playground")]
    ScriptVersionGetResponseMetadataSourcePlayground,
    #[serde(rename = "workersci")]
    ScriptVersionGetResponseMetadataSourceWorkersci,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionAssetsConfigHTMLHandling {
    #[serde(rename = "auto-trailing-slash")]
    VersionAssetsConfigHTMLHandlingAutoTrailingSlash,
    #[serde(rename = "force-trailing-slash")]
    VersionAssetsConfigHTMLHandlingForceTrailingSlash,
    #[serde(rename = "drop-trailing-slash")]
    VersionAssetsConfigHTMLHandlingDropTrailingSlash,
    #[serde(rename = "none")]
    VersionAssetsConfigHTMLHandlingNone,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionAssetsConfigNotFoundHandling {
    #[serde(rename = "none")]
    VersionAssetsConfigNotFoundHandlingNone,
    #[serde(rename = "404-page")]
    VersionAssetsConfigNotFoundHandling404Page,
    #[serde(rename = "single-page-application")]
    VersionAssetsConfigNotFoundHandlingSinglePageApplication,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseInvocationsWorkersEventType {
    #[serde(rename = "fetch")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeFetch,
    #[serde(rename = "scheduled")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeScheduled,
    #[serde(rename = "alarm")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeAlarm,
    #[serde(rename = "cron")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeCron,
    #[serde(rename = "queue")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeQueue,
    #[serde(rename = "email")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeEmail,
    #[serde(rename = "tail")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeTail,
    #[serde(rename = "rpc")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeRpc,
    #[serde(rename = "websocket")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeWebsocket,
    #[serde(rename = "unknown")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersEventTypeUnknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseInvocationsWorkersExecutionModel {
    #[serde(rename = "durableObject")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersExecutionModelDurableObject,
    #[serde(rename = "stateless")]
    ObservabilityTelemetryQueryResponseInvocationsWorkersExecutionModelStateless,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionNewResponseResourcesBindingsType {
    #[serde(rename = "ai")]
    ScriptVersionNewResponseResourcesBindingsTypeAI,
    #[serde(rename = "analytics_engine")]
    ScriptVersionNewResponseResourcesBindingsTypeAnalyticsEngine,
    #[serde(rename = "assets")]
    ScriptVersionNewResponseResourcesBindingsTypeAssets,
    #[serde(rename = "browser")]
    ScriptVersionNewResponseResourcesBindingsTypeBrowser,
    #[serde(rename = "d1")]
    ScriptVersionNewResponseResourcesBindingsTypeD1,
    #[serde(rename = "data_blob")]
    ScriptVersionNewResponseResourcesBindingsTypeDataBlob,
    #[serde(rename = "dispatch_namespace")]
    ScriptVersionNewResponseResourcesBindingsTypeDispatchNamespace,
    #[serde(rename = "durable_object_namespace")]
    ScriptVersionNewResponseResourcesBindingsTypeDurableObjectNamespace,
    #[serde(rename = "hyperdrive")]
    ScriptVersionNewResponseResourcesBindingsTypeHyperdrive,
    #[serde(rename = "inherit")]
    ScriptVersionNewResponseResourcesBindingsTypeInherit,
    #[serde(rename = "images")]
    ScriptVersionNewResponseResourcesBindingsTypeImages,
    #[serde(rename = "json")]
    ScriptVersionNewResponseResourcesBindingsTypeJson,
    #[serde(rename = "kv_namespace")]
    ScriptVersionNewResponseResourcesBindingsTypeKVNamespace,
    #[serde(rename = "mtls_certificate")]
    ScriptVersionNewResponseResourcesBindingsTypeMTLSCertificate,
    #[serde(rename = "plain_text")]
    ScriptVersionNewResponseResourcesBindingsTypePlainText,
    #[serde(rename = "pipelines")]
    ScriptVersionNewResponseResourcesBindingsTypePipelines,
    #[serde(rename = "queue")]
    ScriptVersionNewResponseResourcesBindingsTypeQueue,
    #[serde(rename = "ratelimit")]
    ScriptVersionNewResponseResourcesBindingsTypeRatelimit,
    #[serde(rename = "r2_bucket")]
    ScriptVersionNewResponseResourcesBindingsTypeR2Bucket,
    #[serde(rename = "secret_text")]
    ScriptVersionNewResponseResourcesBindingsTypeSecretText,
    #[serde(rename = "send_email")]
    ScriptVersionNewResponseResourcesBindingsTypeSendEmail,
    #[serde(rename = "service")]
    ScriptVersionNewResponseResourcesBindingsTypeService,
    #[serde(rename = "text_blob")]
    ScriptVersionNewResponseResourcesBindingsTypeTextBlob,
    #[serde(rename = "vectorize")]
    ScriptVersionNewResponseResourcesBindingsTypeVectorize,
    #[serde(rename = "version_metadata")]
    ScriptVersionNewResponseResourcesBindingsTypeVersionMetadata,
    #[serde(rename = "secrets_store_secret")]
    ScriptVersionNewResponseResourcesBindingsTypeSecretsStoreSecret,
    #[serde(rename = "secret_key")]
    ScriptVersionNewResponseResourcesBindingsTypeSecretKey,
    #[serde(rename = "workflow")]
    ScriptVersionNewResponseResourcesBindingsTypeWorkflow,
    #[serde(rename = "wasm_module")]
    ScriptVersionNewResponseResourcesBindingsTypeWasmModule,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionNewResponseResourcesBindingsFormat {
    #[serde(rename = "raw")]
    ScriptVersionNewResponseResourcesBindingsFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptVersionNewResponseResourcesBindingsFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptVersionNewResponseResourcesBindingsFormatSpki,
    #[serde(rename = "jwk")]
    ScriptVersionNewResponseResourcesBindingsFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionNewResponseResourcesBindingsJurisdiction {
    #[serde(rename = "eu")]
    ScriptVersionNewResponseResourcesBindingsJurisdictionEu,
    #[serde(rename = "fedramp")]
    ScriptVersionNewResponseResourcesBindingsJurisdictionFedramp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionNewResponseResourcesScriptRuntimeUsageModel {
    #[serde(rename = "bundled")]
    ScriptVersionNewResponseResourcesScriptRuntimeUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptVersionNewResponseResourcesScriptRuntimeUsageModelUnbound,
    #[serde(rename = "standard")]
    ScriptVersionNewResponseResourcesScriptRuntimeUsageModelStandard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionGetResponseResourcesBindingsType {
    #[serde(rename = "ai")]
    ScriptVersionGetResponseResourcesBindingsTypeAI,
    #[serde(rename = "analytics_engine")]
    ScriptVersionGetResponseResourcesBindingsTypeAnalyticsEngine,
    #[serde(rename = "assets")]
    ScriptVersionGetResponseResourcesBindingsTypeAssets,
    #[serde(rename = "browser")]
    ScriptVersionGetResponseResourcesBindingsTypeBrowser,
    #[serde(rename = "d1")]
    ScriptVersionGetResponseResourcesBindingsTypeD1,
    #[serde(rename = "data_blob")]
    ScriptVersionGetResponseResourcesBindingsTypeDataBlob,
    #[serde(rename = "dispatch_namespace")]
    ScriptVersionGetResponseResourcesBindingsTypeDispatchNamespace,
    #[serde(rename = "durable_object_namespace")]
    ScriptVersionGetResponseResourcesBindingsTypeDurableObjectNamespace,
    #[serde(rename = "hyperdrive")]
    ScriptVersionGetResponseResourcesBindingsTypeHyperdrive,
    #[serde(rename = "inherit")]
    ScriptVersionGetResponseResourcesBindingsTypeInherit,
    #[serde(rename = "images")]
    ScriptVersionGetResponseResourcesBindingsTypeImages,
    #[serde(rename = "json")]
    ScriptVersionGetResponseResourcesBindingsTypeJson,
    #[serde(rename = "kv_namespace")]
    ScriptVersionGetResponseResourcesBindingsTypeKVNamespace,
    #[serde(rename = "mtls_certificate")]
    ScriptVersionGetResponseResourcesBindingsTypeMTLSCertificate,
    #[serde(rename = "plain_text")]
    ScriptVersionGetResponseResourcesBindingsTypePlainText,
    #[serde(rename = "pipelines")]
    ScriptVersionGetResponseResourcesBindingsTypePipelines,
    #[serde(rename = "queue")]
    ScriptVersionGetResponseResourcesBindingsTypeQueue,
    #[serde(rename = "ratelimit")]
    ScriptVersionGetResponseResourcesBindingsTypeRatelimit,
    #[serde(rename = "r2_bucket")]
    ScriptVersionGetResponseResourcesBindingsTypeR2Bucket,
    #[serde(rename = "secret_text")]
    ScriptVersionGetResponseResourcesBindingsTypeSecretText,
    #[serde(rename = "send_email")]
    ScriptVersionGetResponseResourcesBindingsTypeSendEmail,
    #[serde(rename = "service")]
    ScriptVersionGetResponseResourcesBindingsTypeService,
    #[serde(rename = "text_blob")]
    ScriptVersionGetResponseResourcesBindingsTypeTextBlob,
    #[serde(rename = "vectorize")]
    ScriptVersionGetResponseResourcesBindingsTypeVectorize,
    #[serde(rename = "version_metadata")]
    ScriptVersionGetResponseResourcesBindingsTypeVersionMetadata,
    #[serde(rename = "secrets_store_secret")]
    ScriptVersionGetResponseResourcesBindingsTypeSecretsStoreSecret,
    #[serde(rename = "secret_key")]
    ScriptVersionGetResponseResourcesBindingsTypeSecretKey,
    #[serde(rename = "workflow")]
    ScriptVersionGetResponseResourcesBindingsTypeWorkflow,
    #[serde(rename = "wasm_module")]
    ScriptVersionGetResponseResourcesBindingsTypeWasmModule,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionGetResponseResourcesBindingsFormat {
    #[serde(rename = "raw")]
    ScriptVersionGetResponseResourcesBindingsFormatRaw,
    #[serde(rename = "pkcs8")]
    ScriptVersionGetResponseResourcesBindingsFormatPkcs8,
    #[serde(rename = "spki")]
    ScriptVersionGetResponseResourcesBindingsFormatSpki,
    #[serde(rename = "jwk")]
    ScriptVersionGetResponseResourcesBindingsFormatJwk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionGetResponseResourcesBindingsJurisdiction {
    #[serde(rename = "eu")]
    ScriptVersionGetResponseResourcesBindingsJurisdictionEu,
    #[serde(rename = "fedramp")]
    ScriptVersionGetResponseResourcesBindingsJurisdictionFedramp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptVersionGetResponseResourcesScriptRuntimeUsageModel {
    #[serde(rename = "bundled")]
    ScriptVersionGetResponseResourcesScriptRuntimeUsageModelBundled,
    #[serde(rename = "unbound")]
    ScriptVersionGetResponseResourcesScriptRuntimeUsageModelUnbound,
    #[serde(rename = "standard")]
    ScriptVersionGetResponseResourcesScriptRuntimeUsageModelStandard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombination {
    #[serde(rename = "and")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombinationAnd,
    #[serde(rename = "or")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombinationOr,
    #[serde(rename = "AND")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombinationAndUppercase,
    #[serde(rename = "OR")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFilterCombinationOrUppercase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseEventsEventsWorkersEventType {
    #[serde(rename = "fetch")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeFetch,
    #[serde(rename = "scheduled")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeScheduled,
    #[serde(rename = "alarm")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeAlarm,
    #[serde(rename = "cron")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeCron,
    #[serde(rename = "queue")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeQueue,
    #[serde(rename = "email")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeEmail,
    #[serde(rename = "tail")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeTail,
    #[serde(rename = "rpc")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeRpc,
    #[serde(rename = "websocket")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeWebsocket,
    #[serde(rename = "unknown")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersEventTypeUnknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseEventsEventsWorkersExecutionModel {
    #[serde(rename = "durableObject")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersExecutionModelDurableObject,
    #[serde(rename = "stateless")]
    ObservabilityTelemetryQueryResponseEventsEventsWorkersExecutionModelStateless,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperator {
    #[serde(rename = "uniq")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorUniq,
    #[serde(rename = "count")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorCount,
    #[serde(rename = "max")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMax,
    #[serde(rename = "min")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMin,
    #[serde(rename = "sum")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorSum,
    #[serde(rename = "avg")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorAvg,
    #[serde(rename = "median")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMedian,
    #[serde(rename = "p001")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP001,
    #[serde(rename = "p01")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP01,
    #[serde(rename = "p05")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP05,
    #[serde(rename = "p10")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP10,
    #[serde(rename = "p25")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP25,
    #[serde(rename = "p75")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP75,
    #[serde(rename = "p90")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP90,
    #[serde(rename = "p95")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP95,
    #[serde(rename = "p99")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP99,
    #[serde(rename = "p999")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP999,
    #[serde(rename = "stddev")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorStddev,
    #[serde(rename = "variance")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorVariance,
    #[serde(rename = "COUNT_DISTINCT")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorCountDistinct,
    #[serde(rename = "COUNT")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorCountUppercase,
    #[serde(rename = "MAX")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMaxUppercase,
    #[serde(rename = "MIN")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMinUppercase,
    #[serde(rename = "SUM")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorSumUppercase,
    #[serde(rename = "AVG")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorAvgUppercase,
    #[serde(rename = "MEDIAN")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorMedianUppercase,
    #[serde(rename = "P001")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP001Uppercase,
    #[serde(rename = "P01")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP01Uppercase,
    #[serde(rename = "P05")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP05Uppercase,
    #[serde(rename = "P10")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP10Uppercase,
    #[serde(rename = "P25")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP25Uppercase,
    #[serde(rename = "P75")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP75Uppercase,
    #[serde(rename = "P90")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP90Uppercase,
    #[serde(rename = "P95")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP95Uppercase,
    #[serde(rename = "P99")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP99Uppercase,
    #[serde(rename = "P999")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorP999Uppercase,
    #[serde(rename = "STDDEV")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorStddevUppercase,
    #[serde(rename = "VARIANCE")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsOperatorVarianceUppercase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsKeyType {
    #[serde(rename = "string")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsKeyTypeString,
    #[serde(rename = "number")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsKeyTypeNumber,
    #[serde(rename = "boolean")]
    ObservabilityTelemetryQueryResponseRunQueryParametersCalculationsKeyTypeBoolean,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperation {
    #[serde(rename = "includes")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationIncludes,
    #[serde(rename = "not_includes")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationNotIncludes,
    #[serde(rename = "starts_with")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationStartsWith,
    #[serde(rename = "regex")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationRegex,
    #[serde(rename = "exists")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationExists,
    #[serde(rename = "is_null")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationIsNull,
    #[serde(rename = "in")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationIn,
    #[serde(rename = "not_in")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationNotIn,
    #[serde(rename = "eq")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationEq,
    #[serde(rename = "neq")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationNeq,
    #[serde(rename = "gt")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationGt,
    #[serde(rename = "gte")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationGte,
    #[serde(rename = "lt")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationLt,
    #[serde(rename = "lte")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationLte,
    #[serde(rename = "=")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationEquals,
    #[serde(rename = "!=")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationNotEquals,
    #[serde(rename = ">")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationGreater,
    #[serde(rename = ">=")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationGreaterOrEquals,
    #[serde(rename = "<")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationLess,
    #[serde(rename = "<=")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationLessOrEquals,
    #[serde(rename = "INCLUDES")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationIncludesUppercase,
    #[serde(rename = "DOES_NOT_INCLUDE")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationDoesNotInclude,
    #[serde(rename = "MATCH_REGEX")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationMatchRegex,
    #[serde(rename = "EXISTS")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationExistsUppercase,
    #[serde(rename = "DOES_NOT_EXIST")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationDoesNotExist,
    #[serde(rename = "IN")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationInUppercase,
    #[serde(rename = "NOT_IN")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationNotInUppercase,
    #[serde(rename = "STARTS_WITH")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersOperationStartsWithUppercase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersFiltersType {
    #[serde(rename = "string")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersTypeString,
    #[serde(rename = "number")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersTypeNumber,
    #[serde(rename = "boolean")]
    ObservabilityTelemetryQueryResponseRunQueryParametersFiltersTypeBoolean,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersGroupBysType {
    #[serde(rename = "string")]
    ObservabilityTelemetryQueryResponseRunQueryParametersGroupBysTypeString,
    #[serde(rename = "number")]
    ObservabilityTelemetryQueryResponseRunQueryParametersGroupBysTypeNumber,
    #[serde(rename = "boolean")]
    ObservabilityTelemetryQueryResponseRunQueryParametersGroupBysTypeBoolean,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperation {
    #[serde(rename = "eq")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationEq,
    #[serde(rename = "neq")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationNeq,
    #[serde(rename = "gt")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationGt,
    #[serde(rename = "gte")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationGte,
    #[serde(rename = "lt")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationLt,
    #[serde(rename = "lte")]
    ObservabilityTelemetryQueryResponseRunQueryParametersHavingsOperationLte,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservabilityTelemetryQueryResponseRunQueryParametersOrderByOrder {
    #[serde(rename = "asc")]
    ObservabilityTelemetryQueryResponseRunQueryParametersOrderByOrderAsc,
    #[serde(rename = "desc")]
    ObservabilityTelemetryQueryResponseRunQueryParametersOrderByOrderDesc,
}

