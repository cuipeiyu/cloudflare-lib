#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsAggregateCurrentGetResponse {
    /// Application identifier.
    #[serde(rename = "appID")]
    pub app_id: String,

    /// Number of bytes sent
    #[serde(rename = "bytesEgress")]
    pub bytes_egress: f64,

    /// Number of bytes received
    #[serde(rename = "bytesIngress")]
    pub bytes_ingress: f64,

    /// Number of connections
    pub connections: f64,

    /// Average duration of connections
    #[serde(rename = "durationAvg")]
    pub duration_avg: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsAggregateCurrentGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Comma-delimited list of Spectrum Application Id(s). If provided, the response
    /// will be limited to Spectrum Application Id(s) that match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// Co-location identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colo_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventBytimeGetResponse {
    /// List of columns returned by the analytics query.
    pub data: Vec<AnalyticsEventBytimeGetResponseData>,

    /// Number of seconds between current time and last processed event, i.e. how many
    /// seconds of data could be missing.
    pub data_lag: f64,

    /// Maximum result for each selected metrics across all data.
    pub max: ::std::collections::HashMap<String, f64>,

    /// Minimum result for each selected metrics across all data.
    pub min: ::std::collections::HashMap<String, f64>,

    pub query: AnalyticsEventBytimeGetResponseQuery,

    /// Total number of rows in the result.
    pub rows: f64,

    /// Total result for each selected metrics across all data.
    pub totals: ::std::collections::HashMap<String, f64>,

    /// List of time interval buckets: [start, end]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_intervals: Option<Vec<Vec<DateTime<Utc>>>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventBytimeGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Used to select time series resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delta: Option<String>,

    /// Can be used to break down the data by given attributes. Options are:
    /// | Dimension | Name                          | Example                                                    |
    /// | --------- | ----------------------------- | ---------------------------------------------------------- |
    /// | event     | Connection Event              | connect, progress, disconnect, originError, clientFiltered |
    /// | appID     | Application ID                | 40d67c87c6cd4b889a4fd57805225e85                           |
    /// | coloName  | Colo Name                     | SFO                                                        |
    /// | ipVersion | IP version used by the client | 4, 6.                                                      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,

    /// Used to filter rows by one or more dimensions. Filters can be combined using OR
    /// and AND boolean logic. AND takes precedence over OR in all the expressions. The
    /// OR operator is defined using a comma (,) or OR keyword surrounded by whitespace.
    /// The AND operator is defined using a semicolon (;) or AND keyword surrounded by
    /// whitespace. Note that the semicolon is a reserved character in URLs (rfc1738)
    /// and needs to be percent-encoded as %3B. Comparison options are:
    /// | Operator | Name                     | URL Encoded |
    /// | -------- | ------------------------ | ----------- |
    /// | ==       | Equals                   | %3D%3D      |
    /// | !=       | Does not equals          | !%3D        |
    /// | \>       | Greater Than             | %3E         |
    /// | \<       | Less Than                | %3C         |
    /// | \>=      | Greater than or equal to | %3E%3D      |
    /// | \<=      | Less than or equal to    | %3C%3D      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// One or more metrics to compute. Options are:
    /// | Metric         | Name                                | Example | Unit                  |
    /// | -------------- | ----------------------------------- | ------- | --------------------- |
    /// | count          | Count of total events               | 1000    | Count                 |
    /// | bytesIngress   | Sum of ingress bytes                | 1000    | Sum                   |
    /// | bytesEgress    | Sum of egress bytes                 | 1000    | Sum                   |
    /// | durationAvg    | Average connection duration         | 1.0     | Time in milliseconds  |
    /// | durationMedian | Median connection duration          | 1.0     | Time in milliseconds  |
    /// | duration90th   | 90th percentile connection duration | 1.0     | Time in milliseconds  |
    /// | duration99th   | 99th percentile connection duration | 1.0     | Time in milliseconds. |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,

    /// Start of time interval to query, defaults to `until` - 6 hours. Timestamp must
    /// be in RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// The sort order for the result set; sort fields must be included in `metrics` or
    /// `dimensions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// End of time interval to query, defaults to current time. Timestamp must be in
    /// RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventSummaryGetResponse {
    /// List of columns returned by the analytics query.
    pub data: Vec<AnalyticsEventSummaryGetResponseData>,

    /// Number of seconds between current time and last processed event, i.e. how many
    /// seconds of data could be missing.
    pub data_lag: f64,

    /// Maximum result for each selected metrics across all data.
    pub max: ::std::collections::HashMap<String, f64>,

    /// Minimum result for each selected metrics across all data.
    pub min: ::std::collections::HashMap<String, f64>,

    pub query: AnalyticsEventSummaryGetResponseQuery,

    /// Total number of rows in the result.
    pub rows: f64,

    /// Total result for each selected metrics across all data.
    pub totals: ::std::collections::HashMap<String, f64>,

    /// List of time interval buckets: [start, end]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_intervals: Option<Vec<Vec<DateTime<Utc>>>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventSummaryGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Can be used to break down the data by given attributes. Options are:
    /// | Dimension | Name                          | Example                                                    |
    /// | --------- | ----------------------------- | ---------------------------------------------------------- |
    /// | event     | Connection Event              | connect, progress, disconnect, originError, clientFiltered |
    /// | appID     | Application ID                | 40d67c87c6cd4b889a4fd57805225e85                           |
    /// | coloName  | Colo Name                     | SFO                                                        |
    /// | ipVersion | IP version used by the client | 4, 6.                                                      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,

    /// Used to filter rows by one or more dimensions. Filters can be combined using OR
    /// and AND boolean logic. AND takes precedence over OR in all the expressions. The
    /// OR operator is defined using a comma (,) or OR keyword surrounded by whitespace.
    /// The AND operator is defined using a semicolon (;) or AND keyword surrounded by
    /// whitespace. Note that the semicolon is a reserved character in URLs (rfc1738)
    /// and needs to be percent-encoded as %3B. Comparison options are:
    /// | Operator | Name                     | URL Encoded |
    /// | -------- | ------------------------ | ----------- |
    /// | ==       | Equals                   | %3D%3D      |
    /// | !=       | Does not equals          | !%3D        |
    /// | \>       | Greater Than             | %3E         |
    /// | \<       | Less Than                | %3C         |
    /// | \>=      | Greater than or equal to | %3E%3D      |
    /// | \<=      | Less than or equal to    | %3C%3D      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// One or more metrics to compute. Options are:
    /// | Metric         | Name                                | Example | Unit                  |
    /// | -------------- | ----------------------------------- | ------- | --------------------- |
    /// | count          | Count of total events               | 1000    | Count                 |
    /// | bytesIngress   | Sum of ingress bytes                | 1000    | Sum                   |
    /// | bytesEgress    | Sum of egress bytes                 | 1000    | Sum                   |
    /// | durationAvg    | Average connection duration         | 1.0     | Time in milliseconds  |
    /// | durationMedian | Median connection duration          | 1.0     | Time in milliseconds  |
    /// | duration90th   | 90th percentile connection duration | 1.0     | Time in milliseconds  |
    /// | duration99th   | 99th percentile connection duration | 1.0     | Time in milliseconds. |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,

    /// Start of time interval to query, defaults to `until` - 6 hours. Timestamp must
    /// be in RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// The sort order for the result set; sort fields must be included in `metrics` or
    /// `dimensions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// End of time interval to query, defaults to current time. Timestamp must be in
    /// RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNewResponse<T> {
    /// App identifier.
    pub id: String,

    /// When the Application was created.
    pub created_on: DateTime<Utc>,

    /// The name and type of DNS record for the Spectrum application.
    pub dns: DNS,

    /// When the Application was last modified.
    pub modified_on: DateTime<Utc>,

    /// The port configuration at Cloudflare's edge. May specify a single port, for
    /// example `"tcp/1000"`, or a range of ports, for example `"tcp/1000-2000"`.
    pub protocol: String,

    /// Enables Argo Smart Routing for this application. Notes: Only available for TCP
    /// applications with traffic_type set to "direct".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_smart_routing: Option<bool>,

    /// The anycast edge IP configuration for the hostname of this application.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edge_ips")]
    pub edge_i_ps: Option<EdgeIPs>,

    /// Enables IP Access Rules for this application. Notes: Only available for TCP
    /// applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_firewall: Option<bool>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_direct: Option<String>,

    /// The name and type of DNS record for the Spectrum application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_dns: Option<OriginDNS>,

    /// The destination port at the origin. Only specified in conjunction with
    /// origin_dns. May use an integer to specify a single origin port, for example
    /// `1000`, or a string to specify a range of origin ports, for example
    /// `"1000-2000"`. Notes: If specifying a port range, the number of ports in the
    /// range must match the number of ports specified in the "protocol" field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_port: Option<T>,

    /// Enables Proxy Protocol to the origin. Refer to
    /// [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/)
    /// for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple
    /// Proxy Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<AppNewResponseProxyProtocol>,

    /// The type of TLS termination associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<AppNewResponseTLS>,

    /// Determines how data travels from the edge to your origin. When set to "direct",
    /// Spectrum will send traffic directly to your origin, and the application's type
    /// is derived from the `protocol`. When set to "http" or "https", Spectrum will
    /// apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and
    /// the application type matches this property exactly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<AppNewResponseTrafficType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpdateResponse<T> {
    /// App identifier.
    pub id: String,

    /// When the Application was created.
    pub created_on: DateTime<Utc>,

    /// The name and type of DNS record for the Spectrum application.
    pub dns: DNS,

    /// When the Application was last modified.
    pub modified_on: DateTime<Utc>,

    /// The port configuration at Cloudflare's edge. May specify a single port, for
    /// example `"tcp/1000"`, or a range of ports, for example `"tcp/1000-2000"`.
    pub protocol: String,

    /// Enables Argo Smart Routing for this application. Notes: Only available for TCP
    /// applications with traffic_type set to "direct".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_smart_routing: Option<bool>,

    /// The anycast edge IP configuration for the hostname of this application.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edge_ips")]
    pub edge_i_ps: Option<EdgeIPs>,

    /// Enables IP Access Rules for this application. Notes: Only available for TCP
    /// applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_firewall: Option<bool>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_direct: Option<String>,

    /// The name and type of DNS record for the Spectrum application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_dns: Option<OriginDNS>,

    /// The destination port at the origin. Only specified in conjunction with
    /// origin_dns. May use an integer to specify a single origin port, for example
    /// `1000`, or a string to specify a range of origin ports, for example
    /// `"1000-2000"`. Notes: If specifying a port range, the number of ports in the
    /// range must match the number of ports specified in the "protocol" field.
    /// Union satisfied by [shared::UnionInt] or [shared::UnionString].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_port: Option<T>,

    /// Enables Proxy Protocol to the origin. Refer to
    /// [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/)
    /// for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple
    /// Proxy Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<AppUpdateResponseProxyProtocol>,

    /// The type of TLS termination associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<AppUpdateResponseTLS>,

    /// Determines how data travels from the edge to your origin. When set to "direct",
    /// Spectrum will send traffic directly to your origin, and the application's type
    /// is derived from the `protocol`. When set to "http" or "https", Spectrum will
    /// apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and
    /// the application type matches this property exactly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<AppUpdateResponseTrafficType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppListResponse<T> {
    /// App identifier.
    pub id: String,

    /// When the Application was created.
    pub created_on: DateTime<Utc>,

    /// The name and type of DNS record for the Spectrum application.
    pub dns: DNS,

    /// When the Application was last modified.
    pub modified_on: DateTime<Utc>,

    /// The port configuration at Cloudflare's edge. May specify a single port, for
    /// example `"tcp/1000"`, or a range of ports, for example `"tcp/1000-2000"`.
    pub protocol: String,

    /// Enables Argo Smart Routing for this application. Notes: Only available for TCP
    /// applications with traffic_type set to "direct".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_smart_routing: Option<bool>,

    /// The anycast edge IP configuration for the hostname of this application.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edge_ips")]
    pub edge_i_ps: Option<EdgeIPs>,

    /// Enables IP Access Rules for this application. Notes: Only available for TCP
    /// applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_firewall: Option<bool>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_direct: Option<String>,

    /// The name and type of DNS record for the Spectrum application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_dns: Option<OriginDNS>,

    /// The destination port at the origin. Only specified in conjunction with
    /// origin_dns. May use an integer to specify a single origin port, for example
    /// `1000`, or a string to specify a range of origin ports, for example
    /// `"1000-2000"`. Notes: If specifying a port range, the number of ports in the
    /// range must match the number of ports specified in the "protocol" field.
    /// Union satisfied by [shared::UnionInt] or [shared::UnionString].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_port: Option<T>,

    /// Enables Proxy Protocol to the origin. Refer to
    /// [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/)
    /// for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple
    /// Proxy Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<AppListResponseProxyProtocol>,

    /// The type of TLS termination associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<AppListResponseTLS>,

    /// Determines how data travels from the edge to your origin. When set to "direct",
    /// Spectrum will send traffic directly to your origin, and the application's type
    /// is derived from the `protocol`. When set to "http" or "https", Spectrum will
    /// apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and
    /// the application type matches this property exactly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<AppListResponseTrafficType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDeleteResponse {
    /// Identifier.
    pub id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppGetResponse<T> {
    /// App identifier.
    pub id: String,

    /// When the Application was created.
    pub created_on: DateTime<Utc>,

    /// The name and type of DNS record for the Spectrum application.
    pub dns: DNS,

    /// When the Application was last modified.
    pub modified_on: DateTime<Utc>,

    /// The port configuration at Cloudflare's edge. May specify a single port, for
    /// example `"tcp/1000"`, or a range of ports, for example `"tcp/1000-2000"`.
    pub protocol: String,

    /// Enables Argo Smart Routing for this application. Notes: Only available for TCP
    /// applications with traffic_type set to "direct".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argo_smart_routing: Option<bool>,

    /// The anycast edge IP configuration for the hostname of this application.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "edge_ips")]
    pub edge_i_ps: Option<EdgeIPs>,

    /// Enables IP Access Rules for this application. Notes: Only available for TCP
    /// applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_firewall: Option<bool>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_direct: Option<String>,

    /// The name and type of DNS record for the Spectrum application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_dns: Option<OriginDNS>,

    /// The destination port at the origin. Only specified in conjunction with
    /// origin_dns. May use an integer to specify a single origin port, for example
    /// `1000`, or a string to specify a range of origin ports, for example
    /// `"1000-2000"`. Notes: If specifying a port range, the number of ports in the
    /// range must match the number of ports specified in the "protocol" field.
    /// Union satisfied by [shared::UnionInt] or [shared::UnionString].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_port: Option<T>,

    /// Enables Proxy Protocol to the origin. Refer to
    /// [Enable Proxy protocol](https://developers.cloudflare.com/spectrum/getting-started/proxy-protocol/)
    /// for implementation details on PROXY Protocol V1, PROXY Protocol V2, and Simple
    /// Proxy Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<AppGetResponseProxyProtocol>,

    /// The type of TLS termination associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<AppGetResponseTLS>,

    /// Determines how data travels from the edge to your origin. When set to "direct",
    /// Spectrum will send traffic directly to your origin, and the application's type
    /// is derived from the `protocol`. When set to "http" or "https", Spectrum will
    /// apply Cloudflare's HTTP/HTTPS features as it sends traffic to your origin, and
    /// the application type matches this property exactly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<AppGetResponseTrafficType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNewParams<T> {
    /// Zone identifier.
    pub zone_id: String,

    /// Satisfied by [spectrum.AppNewParamsBodySpectrumConfigAppConfig],
    /// [spectrum.AppNewParamsBodySpectrumConfigPaygoAppConfig], [AppNewParamsBody].
    pub body: T,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpdateParams<T> {
    /// Zone identifier.
    pub zone_id: String,

    /// Satisfied by [spectrum.AppUpdateParamsBodySpectrumConfigAppConfig],
    /// [spectrum.AppUpdateParamsBodySpectrumConfigPaygoAppConfig],
    /// [AppUpdateParamsBody].
    pub body: T,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppListParams {
    /// Zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Sets the direction by which results are ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Application field by which results are ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results. This parameter is required in order to use
    /// other pagination parameters. If included in the query, `result_info` will be
    /// present in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Sets the maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDeleteParams {
    /// Zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppGetParams {
    /// Zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNS {
    /// The name of the DNS record associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of DNS record associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DNSType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSParam {
    /// The name of the DNS record associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of DNS record associated with the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeIPs {
    /// The IP versions supported for inbound connections on Spectrum anycast IPs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<EdgeIPsConnectivity>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ips")]
    pub ips: Option<String>,

    /// The type of edge IP configuration specified. Dynamically allocated edge IPs use
    /// Spectrum anycast IPs in accordance with the connectivity you specify. Only valid
    /// with CNAME DNS names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EdgeIPsType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginDNS {
    /// The name of the DNS record associated with the origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The TTL of our resolution of your DNS record in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    /// The type of DNS record associated with the origin. "" is used to specify a
    /// combination of A/AAAA records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OriginDNSType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginDNSParam {
    /// The name of the DNS record associated with the origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The TTL of our resolution of your DNS record in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,

    /// The type of DNS record associated with the origin. "" is used to specify a
    /// combination of A/AAAA records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventBytimeGetResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<f64>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventBytimeGetResponseQuery {
    /// Can be used to break down the data by given attributes. Options are:
    /// | Dimension | Name                          | Example                                                    |
    /// | --------- | ----------------------------- | ---------------------------------------------------------- |
    /// | event     | Connection Event              | connect, progress, disconnect, originError, clientFiltered |
    /// | appID     | Application ID                | 40d67c87c6cd4b889a4fd57805225e85                           |
    /// | coloName  | Colo Name                     | SFO                                                        |
    /// | ipVersion | IP version used by the client | 4, 6.                                                      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,

    /// Used to filter rows by one or more dimensions. Filters can be combined using OR
    /// and AND boolean logic. AND takes precedence over OR in all the expressions. The
    /// OR operator is defined using a comma (,) or OR keyword surrounded by whitespace.
    /// The AND operator is defined using a semicolon (;) or AND keyword surrounded by
    /// whitespace. Note that the semicolon is a reserved character in URLs (rfc1738)
    /// and needs to be percent-encoded as %3B. Comparison options are:
    /// | Operator | Name                     | URL Encoded |
    /// | -------- | ------------------------ | ----------- |
    /// | ==       | Equals                   | %3D%3D      |
    /// | !=       | Does not equals          | !%3D        |
    /// | \>       | Greater Than             | %3E         |
    /// | \<       | Less Than                | %3C         |
    /// | \>=      | Greater than or equal to | %3E%3D      |
    /// | \<=      | Less than or equal to    | %3C%3D      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Limit number of returned metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,

    /// One or more metrics to compute. Options are:
    /// | Metric         | Name                                | Example | Unit                  |
    /// | -------------- | ----------------------------------- | ------- | --------------------- |
    /// | count          | Count of total events               | 1000    | Count                 |
    /// | bytesIngress   | Sum of ingress bytes                | 1000    | Sum                   |
    /// | bytesEgress    | Sum of egress bytes                 | 1000    | Sum                   |
    /// | durationAvg    | Average connection duration         | 1.0     | Time in milliseconds  |
    /// | durationMedian | Median connection duration          | 1.0     | Time in milliseconds  |
    /// | duration90th   | 90th percentile connection duration | 1.0     | Time in milliseconds  |
    /// | duration99th   | 99th percentile connection duration | 1.0     | Time in milliseconds. |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<AnalyticsEventBytimeGetResponseQueryMetric>>,

    /// Start of time interval to query, defaults to `until` - 6 hours. Timestamp must
    /// be in RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,

    /// The sort order for the result set; sort fields must be included in `metrics` or
    /// `dimensions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,

    /// End of time interval to query, defaults to current time. Timestamp must be in
    /// RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventSummaryGetResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<f64>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventSummaryGetResponseQuery {
    /// Can be used to break down the data by given attributes. Options are:
    /// | Dimension | Name                          | Example                                                    |
    /// | --------- | ----------------------------- | ---------------------------------------------------------- |
    /// | event     | Connection Event              | connect, progress, disconnect, originError, clientFiltered |
    /// | appID     | Application ID                | 40d67c87c6cd4b889a4fd57805225e85                           |
    /// | coloName  | Colo Name                     | SFO                                                        |
    /// | ipVersion | IP version used by the client | 4, 6.                                                      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,

    /// Used to filter rows by one or more dimensions. Filters can be combined using OR
    /// and AND boolean logic. AND takes precedence over OR in all the expressions. The
    /// OR operator is defined using a comma (,) or OR keyword surrounded by whitespace.
    /// The AND operator is defined using a semicolon (;) or AND keyword surrounded by
    /// whitespace. Note that the semicolon is a reserved character in URLs (rfc1738)
    /// and needs to be percent-encoded as %3B. Comparison options are:
    /// | Operator | Name                     | URL Encoded |
    /// | -------- | ------------------------ | ----------- |
    /// | ==       | Equals                   | %3D%3D      |
    /// | !=       | Does not equals          | !%3D        |
    /// | \>       | Greater Than             | %3E         |
    /// | \<       | Less Than                | %3C         |
    /// | \>=      | Greater than or equal to | %3E%3D      |
    /// | \<=      | Less than or equal to    | %3C%3D      |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Limit number of returned metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,

    /// One or more metrics to compute. Options are:
    /// | Metric         | Name                                | Example | Unit                  |
    /// | -------------- | ----------------------------------- | ------- | --------------------- |
    /// | count          | Count of total events               | 1000    | Count                 |
    /// | bytesIngress   | Sum of ingress bytes                | 1000    | Sum                   |
    /// | bytesEgress    | Sum of egress bytes                 | 1000    | Sum                   |
    /// | durationAvg    | Average connection duration         | 1.0     | Time in milliseconds  |
    /// | durationMedian | Median connection duration          | 1.0     | Time in milliseconds  |
    /// | duration90th   | 90th percentile connection duration | 1.0     | Time in milliseconds  |
    /// | duration99th   | 99th percentile connection duration | 1.0     | Time in milliseconds. |
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<AnalyticsEventSummaryGetResponseQueryMetric>>,

    /// Start of time interval to query, defaults to `until` - 6 hours. Timestamp must
    /// be in RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,

    /// The sort order for the result set; sort fields must be included in `metrics` or
    /// `dimensions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,

    /// End of time interval to query, defaults to current time. Timestamp must be in
    /// RFC3339 format and uses UTC unless otherwise specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Dimension {
    #[serde(rename = "event")]
    DimensionEvent,
    #[serde(rename = "appID")]
    DimensionAppID,
    #[serde(rename = "coloName")]
    DimensionColoName,
    #[serde(rename = "ipVersion")]
    DimensionIPVersion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppNewResponseProxyProtocol {
    #[serde(rename = "off")]
    AppNewResponseProxyProtocolOff,
    #[serde(rename = "v1")]
    AppNewResponseProxyProtocolV1,
    #[serde(rename = "v2")]
    AppNewResponseProxyProtocolV2,
    #[serde(rename = "simple")]
    AppNewResponseProxyProtocolSimple,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppNewResponseTLS {
    #[serde(rename = "off")]
    AppNewResponseTLSOff,
    #[serde(rename = "flexible")]
    AppNewResponseTLSFlexible,
    #[serde(rename = "full")]
    AppNewResponseTLSFull,
    #[serde(rename = "strict")]
    AppNewResponseTLSStrict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppNewResponseTrafficType {
    #[serde(rename = "direct")]
    AppNewResponseTrafficTypeDirect,
    #[serde(rename = "http")]
    AppNewResponseTrafficTypeHTTP,
    #[serde(rename = "https")]
    AppNewResponseTrafficTypeHTTPS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppUpdateResponseProxyProtocol {
    #[serde(rename = "off")]
    AppUpdateResponseProxyProtocolOff,
    #[serde(rename = "v1")]
    AppUpdateResponseProxyProtocolV1,
    #[serde(rename = "v2")]
    AppUpdateResponseProxyProtocolV2,
    #[serde(rename = "simple")]
    AppUpdateResponseProxyProtocolSimple,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppUpdateResponseTLS {
    #[serde(rename = "off")]
    AppUpdateResponseTLSOff,
    #[serde(rename = "flexible")]
    AppUpdateResponseTLSFlexible,
    #[serde(rename = "full")]
    AppUpdateResponseTLSFull,
    #[serde(rename = "strict")]
    AppUpdateResponseTLSStrict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppUpdateResponseTrafficType {
    #[serde(rename = "direct")]
    AppUpdateResponseTrafficTypeDirect,
    #[serde(rename = "http")]
    AppUpdateResponseTrafficTypeHTTP,
    #[serde(rename = "https")]
    AppUpdateResponseTrafficTypeHTTPS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppListResponseProxyProtocol {
    #[serde(rename = "off")]
    AppListResponseProxyProtocolOff,
    #[serde(rename = "v1")]
    AppListResponseProxyProtocolV1,
    #[serde(rename = "v2")]
    AppListResponseProxyProtocolV2,
    #[serde(rename = "simple")]
    AppListResponseProxyProtocolSimple,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppListResponseTLS {
    #[serde(rename = "off")]
    AppListResponseTLSOff,
    #[serde(rename = "flexible")]
    AppListResponseTLSFlexible,
    #[serde(rename = "full")]
    AppListResponseTLSFull,
    #[serde(rename = "strict")]
    AppListResponseTLSStrict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppListResponseTrafficType {
    #[serde(rename = "direct")]
    AppListResponseTrafficTypeDirect,
    #[serde(rename = "http")]
    AppListResponseTrafficTypeHTTP,
    #[serde(rename = "https")]
    AppListResponseTrafficTypeHTTPS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppGetResponseProxyProtocol {
    #[serde(rename = "off")]
    AppGetResponseProxyProtocolOff,
    #[serde(rename = "v1")]
    AppGetResponseProxyProtocolV1,
    #[serde(rename = "v2")]
    AppGetResponseProxyProtocolV2,
    #[serde(rename = "simple")]
    AppGetResponseProxyProtocolSimple,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppGetResponseTLS {
    #[serde(rename = "off")]
    AppGetResponseTLSOff,
    #[serde(rename = "flexible")]
    AppGetResponseTLSFlexible,
    #[serde(rename = "full")]
    AppGetResponseTLSFull,
    #[serde(rename = "strict")]
    AppGetResponseTLSStrict,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppGetResponseTrafficType {
    #[serde(rename = "direct")]
    AppGetResponseTrafficTypeDirect,
    #[serde(rename = "http")]
    AppGetResponseTrafficTypeHTTP,
    #[serde(rename = "https")]
    AppGetResponseTrafficTypeHTTPS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DNSType {
    #[serde(rename = "CNAME")]
    DNSTypeCNAME,
    #[serde(rename = "ADDRESS")]
    DNSTypeAddress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeIPsConnectivity {
    #[serde(rename = "all")]
    EdgeIPsConnectivityAll,
    #[serde(rename = "ipv4")]
    EdgeIPsConnectivityIPV4,
    #[serde(rename = "ipv6")]
    EdgeIPsConnectivityIPV6,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeIPsType {
    #[serde(rename = "dynamic")]
    EdgeIPsTypeDynamic,
    #[serde(rename = "static")]
    EdgeIPsTypeStatic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OriginDNSType {
    #[serde(rename = "")]
    OriginDNSTypeEmpty,
    #[serde(rename = "A")]
    OriginDNSTypeA,
    #[serde(rename = "AAAA")]
    OriginDNSTypeAAAA,
    #[serde(rename = "SRV")]
    OriginDNSTypeSRV,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalyticsEventBytimeGetResponseQueryMetric {
    #[serde(rename = "count")]
    AnalyticsEventBytimeGetResponseQueryMetricCount,
    #[serde(rename = "bytesIngress")]
    AnalyticsEventBytimeGetResponseQueryMetricBytesIngress,
    #[serde(rename = "bytesEgress")]
    AnalyticsEventBytimeGetResponseQueryMetricBytesEgress,
    #[serde(rename = "durationAvg")]
    AnalyticsEventBytimeGetResponseQueryMetricDurationAvg,
    #[serde(rename = "durationMedian")]
    AnalyticsEventBytimeGetResponseQueryMetricDurationMedian,
    #[serde(rename = "duration90th")]
    AnalyticsEventBytimeGetResponseQueryMetricDuration90th,
    #[serde(rename = "duration99th")]
    AnalyticsEventBytimeGetResponseQueryMetricDuration99th,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalyticsEventSummaryGetResponseQueryMetric {
    #[serde(rename = "count")]
    AnalyticsEventSummaryGetResponseQueryMetricCount,
    #[serde(rename = "bytesIngress")]
    AnalyticsEventSummaryGetResponseQueryMetricBytesIngress,
    #[serde(rename = "bytesEgress")]
    AnalyticsEventSummaryGetResponseQueryMetricBytesEgress,
    #[serde(rename = "durationAvg")]
    AnalyticsEventSummaryGetResponseQueryMetricDurationAvg,
    #[serde(rename = "durationMedian")]
    AnalyticsEventSummaryGetResponseQueryMetricDurationMedian,
    #[serde(rename = "duration90th")]
    AnalyticsEventSummaryGetResponseQueryMetricDuration90th,
    #[serde(rename = "duration99th")]
    AnalyticsEventSummaryGetResponseQueryMetricDuration99th,
}

