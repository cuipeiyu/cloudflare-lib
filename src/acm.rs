#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStore {
    /// Identifier.
    pub id: String,

    /// The zone's SSL certificate or certificate and the intermediate(s).
    pub certificate: String,

    /// When the certificate expires.
    pub expires_on: DateTime<Utc>,

    /// The certificate authority that issued the certificate.
    pub issuer: String,

    /// The type of hash used for the certificate.
    pub signature: String,

    /// Status of the zone's custom SSL.
    pub status: CustomTrustStoreStatus,

    /// When the certificate was last modified.
    pub updated_at: DateTime<Utc>,

    /// When the certificate was uploaded to Cloudflare.
    pub uploaded_on: DateTime<Utc>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStoreDeleteResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStoreNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The zone's SSL certificate or certificate and the intermediate(s).
    pub certificate: String,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStoreListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Limit to the number of records returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Offset the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of records per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStoreDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTrustStoreGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSUpdateResponse {
    /// The Certificate Authority that Total TLS certificates will be issued through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,

    /// If enabled, Total TLS will order a hostname specific TLS certificate for any
    /// proxied A, AAAA, or CNAME record in your zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The validity period in days for the certificates ordered via Total TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TotalTLSUpdateResponseValidityPeriod>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSEditResponse {
    /// The Certificate Authority that Total TLS certificates will be issued through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,

    /// If enabled, Total TLS will order a hostname specific TLS certificate for any
    /// proxied A, AAAA, or CNAME record in your zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The validity period in days for the certificates ordered via Total TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TotalTLSEditResponseValidityPeriod>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSGetResponse {
    /// The Certificate Authority that Total TLS certificates will be issued through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,

    /// If enabled, Total TLS will order a hostname specific TLS certificate for any
    /// proxied A, AAAA, or CNAME record in your zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The validity period in days for the certificates ordered via Total TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TotalTLSGetResponseValidityPeriod>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If enabled, Total TLS will order a hostname specific TLS certificate for any
    /// proxied A, AAAA, or CNAME record in your zone.
    pub enabled: String,

    /// The Certificate Authority that Total TLS certificates will be issued through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If enabled, Total TLS will order a hostname specific TLS certificate for any
    /// proxied A, AAAA, or CNAME record in your zone.
    pub enabled: String,

    /// The Certificate Authority that Total TLS certificates will be issued through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalTLSGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateAuthority {
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
    #[serde(rename = "ssl_com")]
    SSLCom,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CustomTrustStoreStatus {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "pending_deployment")]
    PendingDeployment,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending_deletion")]
    PendingDeletion,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "expired")]
    Expired,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TotalTLSUpdateResponseValidityPeriod {
    TotalTLSUpdateResponseValidityPeriod90 = 90,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TotalTLSEditResponseValidityPeriod {
    TotalTLSEditResponseValidityPeriod90 = 90,
}

#[cfg(any(feature = "full", feature = "with-acm"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TotalTLSGetResponseValidityPeriod {
    TotalTLSGetResponseValidityPeriod90 = 90,
}
