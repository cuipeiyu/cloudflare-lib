#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckParam {
    /// type of check to perform
    pub check_type: String,

    /// the IP address of the host to perform checks against
    pub endpoint: String,

    /// Optional name associated with this check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckNewResponse {
    /// type of check to perform
    pub check_type: EndpointHealthcheckNewResponseCheckType,

    /// the IP address of the host to perform checks against
    pub endpoint: String,

    /// UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional name associated with this check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckUpdateResponse {
    /// type of check to perform
    pub check_type: EndpointHealthcheckUpdateResponseCheckType,

    /// the IP address of the host to perform checks against
    pub endpoint: String,

    /// UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional name associated with this check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckListResponse {
    /// type of check to perform
    pub check_type: EndpointHealthcheckListResponseCheckType,

    /// the IP address of the host to perform checks against
    pub endpoint: String,

    /// UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional name associated with this check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteResponse {
    pub errors: Vec<EndpointHealthcheckDeleteResponseError>,

    pub messages: Vec<EndpointHealthcheckDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckGetResponse {
    /// type of check to perform
    pub check_type: EndpointHealthcheckGetResponseCheckType,

    /// the IP address of the host to perform checks against
    pub endpoint: String,

    /// UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional name associated with this check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub endpoint_healthcheck: EndpointHealthcheckParam,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub endpoint_healthcheck: EndpointHealthcheckParam,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traceroute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colos: Option<Vec<TracerouteColo>>,

    /// The target hostname, IPv6, or IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub targets: String,

    /// If no source colo names specified, all colos will be used. China colos are
    /// unavailable for traceroutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colos: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EndpointHealthcheckDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EndpointHealthcheckDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteColo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colo: Option<TracerouteColosColo>,

    /// Errors resulting from collecting traceroute from colo to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TracerouteColosError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hops: Option<Vec<TracerouteColosHop>>,

    /// Aggregated statistics from all hops about the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_summary: Option<String>,

    /// Total time of traceroute in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traceroute_time_ms: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealthcheckDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteColosColo {
    /// Source colo city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Source colo name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteColosHop {
    /// An array of node objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<TracerouteColosHopsNode>>,

    /// Number of packets where no response was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_lost: Option<i64>,

    /// Number of packets sent with specified TTL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_sent: Option<i64>,

    /// The time to live (TTL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_ttl: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteColosHopsNode {
    /// AS number associated with the node object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,

    /// IP address of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Field appears if there is an additional annotation printed when the probe
    /// returns. Field also appears when running a GRE+ICMP traceroute to denote which
    /// traceroute a node comes from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,

    /// Maximum RTT in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rtt_ms: Option<f64>,

    /// Mean RTT in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_rtt_ms: Option<f64>,

    /// Minimum RTT in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_rtt_ms: Option<f64>,

    /// Host name of the address, this may be the same as the IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of packets with a response from this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_count: Option<i64>,

    /// Standard deviation of the RTTs in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_dev_rtt_ms: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointHealthcheckNewResponseCheckType {
    #[serde(rename = "icmp")]
    Icmp,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointHealthcheckUpdateResponseCheckType {
    #[serde(rename = "icmp")]
    Icmp,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointHealthcheckListResponseCheckType {
    #[serde(rename = "icmp")]
    Icmp,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointHealthcheckGetResponseCheckType {
    #[serde(rename = "icmp")]
    Icmp,
}

#[cfg(any(feature = "full", feature = "with-diagnostics"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TracerouteColosError {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "Could not gather traceroute data: Code 1")]
    CouldNotGatherTracerouteDataCode1,
    #[serde(rename = "Could not gather traceroute data: Code 2")]
    CouldNotGatherTracerouteDataCode2,
    #[serde(rename = "Could not gather traceroute data: Code 3")]
    CouldNotGatherTracerouteDataCode3,
    #[serde(rename = "Could not gather traceroute data: Code 4")]
    CouldNotGatherTracerouteDataCode4,
}
