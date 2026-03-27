#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceNewResponse {
    pub host: DirectoryServiceNewResponseHost,

    pub name: String,

    pub r#type: DirectoryServiceNewResponseType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceUpdateResponse {
    pub host: DirectoryServiceUpdateResponseHost,

    pub name: String,

    pub r#type: DirectoryServiceUpdateResponseType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceListResponse {
    pub host: DirectoryServiceListResponseHost,

    pub name: String,

    pub r#type: DirectoryServiceListResponseType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceGetResponse {
    pub host: DirectoryServiceGetResponseHost,

    pub name: String,

    pub r#type: DirectoryServiceGetResponseType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceNewParams {
    /// Account identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub host: String,

    pub name: String,

    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub host: String,

    pub name: String,

    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceListParams {
    /// Account identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Current page in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Max amount of entries returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceNewResponseHost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceNewResponseHostInfraIPv4HostNetwork],
    /// [DirectoryServiceNewResponseHostInfraIPv6HostNetwork],
    /// [DirectoryServiceNewResponseHostInfraDualStackHostNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceNewResponseHostInfraHostnameHostResolverNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_network: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceUpdateResponseHost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceUpdateResponseHostInfraIPv4HostNetwork],
    /// [DirectoryServiceUpdateResponseHostInfraIPv6HostNetwork],
    /// [DirectoryServiceUpdateResponseHostInfraDualStackHostNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceUpdateResponseHostInfraHostnameHostResolverNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_network: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceListResponseHost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceListResponseHostInfraIPv4HostNetwork],
    /// [DirectoryServiceListResponseHostInfraIPv6HostNetwork],
    /// [DirectoryServiceListResponseHostInfraDualStackHostNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceListResponseHostInfraHostnameHostResolverNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_network: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryServiceGetResponseHost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceGetResponseHostInfraIPv4HostNetwork],
    /// [DirectoryServiceGetResponseHostInfraIPv6HostNetwork],
    /// [DirectoryServiceGetResponseHostInfraDualStackHostNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// This field can have the runtime type of
    /// [DirectoryServiceGetResponseHostInfraHostnameHostResolverNetwork].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_network: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectoryServiceNewResponseType {
    #[serde(rename = "http")]
    DirectoryServiceNewResponseTypeHTTP,
}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectoryServiceUpdateResponseType {
    #[serde(rename = "http")]
    DirectoryServiceUpdateResponseTypeHTTP,
}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectoryServiceListResponseType {
    #[serde(rename = "http")]
    DirectoryServiceListResponseTypeHTTP,
}

#[cfg(any(feature = "full", feature = "with-connectivity"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectoryServiceGetResponseType {
    #[serde(rename = "http")]
    DirectoryServiceGetResponseTypeHTTP,
}
