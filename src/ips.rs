use serde::{Deserialize, Serialize};

use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-ips"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPListResponse {
    /// A digest of the IP data. Useful for determining if the data has changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ipv4_cidrs")]
    pub ipv4_cid_rs: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ipv6_cidrs")]
    pub ipv6_cid_rs: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jdcloud_cidrs")]
    pub jd_cloud_cid_rs: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-ips"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPListParams {
    /// Specified as `jdcloud` to list IPs used by JD Cloud data centers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-ips"))]
pub type IPs = Vec<IPsItem>;

#[cfg(any(feature = "full", feature = "with-ips"))]
pub struct IPsItem {
    pub created_at: Option<DateTime<Utc>>,
	/// An IPv4 or IPv6 address.
	pub ip  : Option<String>,
}
