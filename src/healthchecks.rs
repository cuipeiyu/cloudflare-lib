#[cfg(any(feature = "full", feature = "with-healthchecks"))]
mod healthchecks_bindings {

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Healthcheck {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The hostname or IP address of the origin server to run health checks on.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,

        /// A list of regions from which to run health checks. Null means Cloudflare will
        /// pick a default region.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub check_regions: Option<Vec<CheckRegion>>,

        /// The number of consecutive fails required from a health check before changing the
        /// health to unhealthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub consecutive_fails: Option<i64>,

        /// The number of consecutive successes required from a health check before changing
        /// the health to healthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub consecutive_successes: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// A human-readable description of the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The current failure reason if status is unhealthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub failure_reason: Option<String>,

        /// Parameters specific to an HTTP or HTTPS health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub http_config: Option<HTTPConfiguration>,

        /// The interval between each health check. Shorter intervals may give quicker
        /// notifications if the origin status changes, but will increase load on the origin
        /// as we check from multiple locations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        /// A short name to identify the health check. Only alphanumeric characters, hyphens
        /// and underscores are allowed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// The number of retries to attempt in case of a timeout before marking the origin
        /// as unhealthy. Retries are attempted immediately.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retries: Option<i64>,

        /// The current status of the origin server according to the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<HealthcheckStatus>,

        /// If suspended, no health checks are sent to the origin.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub suspended: Option<bool>,

        /// Parameters specific to TCP health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tcp_config: Option<TCPConfiguration>,

        /// The timeout (in seconds) before marking the health check as failed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<i64>,

        /// The protocol to use for the health check. Currently supported protocols are
        /// 'HTTP', 'HTTPS' and 'TCP'.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HTTPConfiguration {
        /// Do not validate the certificate when the health check uses HTTPS.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allow_insecure: Option<bool>,

        /// A case-insensitive sub-string to look for in the response body. If this string
        /// is not found, the origin will be marked as unhealthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expected_body: Option<String>,

        /// The expected HTTP response codes (e.g. "200") or code ranges (e.g. "2xx" for all
        /// codes starting with 2) of the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expected_codes: Option<Vec<String>>,

        /// Follow redirects if the origin returns a 3xx status code.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub follow_redirects: Option<bool>,

        /// The HTTP request headers to send in the health check. It is recommended you set
        /// a Host header by default. The User-Agent header cannot be overridden.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<::std::collections::HashMap<String, Vec<String>>>,

        /// The HTTP method to use for the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<HTTPConfigurationMethod>,

        /// The endpoint path to health check against.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,

        /// Port number to connect to for the health check. Defaults to 80 if type is HTTP
        /// or 443 if type is HTTPS.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HTTPConfigurationParam {
        /// Do not validate the certificate when the health check uses HTTPS.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allow_insecure: Option<String>,

        /// A case-insensitive sub-string to look for in the response body. If this string
        /// is not found, the origin will be marked as unhealthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expected_body: Option<String>,

        /// The expected HTTP response codes (e.g. "200") or code ranges (e.g. "2xx" for all
        /// codes starting with 2) of the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expected_codes: Option<String>,

        /// Follow redirects if the origin returns a 3xx status code.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub follow_redirects: Option<String>,

        /// The HTTP request headers to send in the health check. It is recommended you set
        /// a Host header by default. The User-Agent header cannot be overridden.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<String>,

        /// The HTTP method to use for the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,

        /// The endpoint path to health check against.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,

        /// Port number to connect to for the health check. Defaults to 80 if type is HTTP
        /// or 443 if type is HTTPS.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct QueryHealthcheckParam {
        /// The hostname or IP address of the origin server to run health checks on.
        pub address: String,

        /// A short name to identify the health check. Only alphanumeric characters, hyphens
        /// and underscores are allowed.
        pub name: String,

        /// A list of regions from which to run health checks. Null means Cloudflare will
        /// pick a default region.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub check_regions: Option<String>,

        /// The number of consecutive fails required from a health check before changing the
        /// health to unhealthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub consecutive_fails: Option<String>,

        /// The number of consecutive successes required from a health check before changing
        /// the health to healthy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub consecutive_successes: Option<String>,

        /// A human-readable description of the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// Parameters specific to an HTTP or HTTPS health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub http_config: Option<String>,

        /// The interval between each health check. Shorter intervals may give quicker
        /// notifications if the origin status changes, but will increase load on the origin
        /// as we check from multiple locations.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval: Option<String>,

        /// The number of retries to attempt in case of a timeout before marking the origin
        /// as unhealthy. Retries are attempted immediately.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retries: Option<String>,

        /// If suspended, no health checks are sent to the origin.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub suspended: Option<String>,

        /// Parameters specific to TCP health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tcp_config: Option<String>,

        /// The timeout (in seconds) before marking the health check as failed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<String>,

        /// The protocol to use for the health check. Currently supported protocols are
        /// 'HTTP', 'HTTPS' and 'TCP'.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TCPConfiguration {
        /// The TCP connection method to use for the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<TCPConfigurationMethod>,

        /// Port number to connect to for the health check. Defaults to 80.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TCPConfigurationParam {
        /// The TCP connection method to use for the health check.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,

        /// Port number to connect to for the health check. Defaults to 80.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckDeleteResponse {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckNewParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub query_healthcheck: QueryHealthcheckParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckUpdateParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub query_healthcheck: QueryHealthcheckParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckListParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Maximum number of results per page. Must be a multiple of 5.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckDeleteParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckEditParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub query_healthcheck: QueryHealthcheckParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthcheckGetParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PreviewDeleteResponse {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PreviewNewParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub query_healthcheck: QueryHealthcheckParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PreviewDeleteParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PreviewGetParams {
        /// Identifier
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum CheckRegion {
        #[serde(rename = "WNAM")]
        CheckRegionWnam,
        #[serde(rename = "ENAM")]
        CheckRegionEnam,
        #[serde(rename = "WEU")]
        CheckRegionWeu,
        #[serde(rename = "EEU")]
        CheckRegionEeu,
        #[serde(rename = "NSAM")]
        CheckRegionNsam,
        #[serde(rename = "SSAM")]
        CheckRegionSsam,
        #[serde(rename = "OC")]
        CheckRegionOc,
        #[serde(rename = "ME")]
        CheckRegionMe,
        #[serde(rename = "NAF")]
        CheckRegionNaf,
        #[serde(rename = "SAF")]
        CheckRegionSaf,
        #[serde(rename = "IN")]
        CheckRegionIn,
        #[serde(rename = "SEAS")]
        CheckRegionSeas,
        #[serde(rename = "NEAS")]
        CheckRegionNeas,
        #[serde(rename = "ALL_REGIONS")]
        CheckRegionAllRegions,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HealthcheckStatus {
        #[serde(rename = "unknown")]
        HealthcheckStatusUnknown,
        #[serde(rename = "healthy")]
        HealthcheckStatusHealthy,
        #[serde(rename = "unhealthy")]
        HealthcheckStatusUnhealthy,
        #[serde(rename = "suspended")]
        HealthcheckStatusSuspended,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HTTPConfigurationMethod {
        #[serde(rename = "GET")]
        HTTPConfigurationMethodGet,
        #[serde(rename = "HEAD")]
        HTTPConfigurationMethodHead,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum TCPConfigurationMethod {
        #[serde(rename = "connection_established")]
        TCPConfigurationMethodConnectionEstablished,
    }
}

#[cfg(any(feature = "full", feature = "with-healthchecks"))]
pub use healthchecks_bindings::*;

#[cfg(any(feature = "full", feature = "with-healthchecks"))]
impl Client {}
