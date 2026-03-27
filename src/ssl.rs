#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A ubiquitous bundle has the highest probability of being verified everywhere,
    /// even by clients using outdated or unusual trust stores. An optimal bundle uses
    /// the shortest chain and newest intermediates. And the force bundle verifies the
    /// chain, but does not otherwise modify it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,

    /// The zone's SSL certificate or certificate and the intermediate(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponse {
    /// Identifier.
    pub id: String,

    /// Array of certificates in this pack.
    pub certificates: Vec<CertificatePackNewResponseCertificate>,

    /// Comma separated list of valid host names for the certificate packs. Must contain
    /// the zone apex, may not contain more than 50 hosts, and may not be empty.
    pub hosts: Vec<String>,

    /// Status of certificate pack.
    pub status: Status,

    /// Type of certificate pack.
    pub r#type: CertificatePackNewResponseType,

    /// Certificate Authority selected for the order. For information on any certificate
    /// authority specific details or restrictions
    /// [see this page for more details.](https://developers.cloudflare.com/ssl/reference/certificate-authorities)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificatePackNewResponseCertificateAuthority>,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<bool>,

    /// DCV Delegation records for domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcv_delegation_records: Option<Vec<CertificatePackNewResponseDCVDelegationRecord>>,

    /// Identifier of the primary certificate in a pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_certificate: Option<String>,

    /// Domain validation errors that have been received by the certificate authority
    /// (CA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<CertificatePackNewResponseValidationError>>,

    /// Validation Method selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<CertificatePackNewResponseValidationMethod>,

    /// Certificates' validation records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_records: Option<Vec<CertificatePackNewResponseValidationRecord>>,

    /// Validity Days selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_days: Option<CertificatePackNewResponseValidityDays>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponse {
    /// Identifier.
    pub id: String,

    /// Array of certificates in this pack.
    pub certificates: Vec<CertificatePackListResponseCertificate>,

    /// Comma separated list of valid host names for the certificate packs. Must contain
    /// the zone apex, may not contain more than 50 hosts, and may not be empty.
    pub hosts: Vec<String>,

    /// Status of certificate pack.
    pub status: Status,

    /// Type of certificate pack.
    pub r#type: CertificatePackListResponseType,

    /// Certificate Authority selected for the order. For information on any certificate
    /// authority specific details or restrictions
    /// [see this page for more details.](https://developers.cloudflare.com/ssl/reference/certificate-authorities)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificatePackListResponseCertificateAuthority>,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<bool>,

    /// DCV Delegation records for domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcv_delegation_records: Option<Vec<CertificatePackListResponseDCVDelegationRecord>>,

    /// Identifier of the primary certificate in a pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_certificate: Option<String>,

    /// Domain validation errors that have been received by the certificate authority
    /// (CA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<CertificatePackListResponseValidationError>>,

    /// Validation Method selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<CertificatePackListResponseValidationMethod>,

    /// Certificates' validation records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_records: Option<Vec<CertificatePackListResponseValidationRecord>>,

    /// Validity Days selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_days: Option<CertificatePackListResponseValidityDays>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackDeleteResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponse {
    /// Identifier.
    pub id: String,

    /// Array of certificates in this pack.
    pub certificates: Vec<CertificatePackEditResponseCertificate>,

    /// Comma separated list of valid host names for the certificate packs. Must contain
    /// the zone apex, may not contain more than 50 hosts, and may not be empty.
    pub hosts: Vec<String>,

    /// Status of certificate pack.
    pub status: Status,

    /// Type of certificate pack.
    pub r#type: CertificatePackEditResponseType,

    /// Certificate Authority selected for the order. For information on any certificate
    /// authority specific details or restrictions
    /// [see this page for more details.](https://developers.cloudflare.com/ssl/reference/certificate-authorities)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificatePackEditResponseCertificateAuthority>,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<bool>,

    /// DCV Delegation records for domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcv_delegation_records: Option<Vec<CertificatePackEditResponseDCVDelegationRecord>>,

    /// Identifier of the primary certificate in a pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_certificate: Option<String>,

    /// Domain validation errors that have been received by the certificate authority
    /// (CA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<CertificatePackEditResponseValidationError>>,

    /// Validation Method selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<CertificatePackEditResponseValidationMethod>,

    /// Certificates' validation records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_records: Option<Vec<CertificatePackEditResponseValidationRecord>>,

    /// Validity Days selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_days: Option<CertificatePackEditResponseValidityDays>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponse {
    /// Identifier.
    pub id: String,

    /// Array of certificates in this pack.
    pub certificates: Vec<CertificatePackGetResponseCertificate>,

    /// Comma separated list of valid host names for the certificate packs. Must contain
    /// the zone apex, may not contain more than 50 hosts, and may not be empty.
    pub hosts: Vec<String>,

    /// Status of certificate pack.
    pub status: Status,

    /// Type of certificate pack.
    pub r#type: CertificatePackGetResponseType,

    /// Certificate Authority selected for the order. For information on any certificate
    /// authority specific details or restrictions
    /// [see this page for more details.](https://developers.cloudflare.com/ssl/reference/certificate-authorities)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificatePackGetResponseCertificateAuthority>,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<bool>,

    /// DCV Delegation records for domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcv_delegation_records: Option<Vec<CertificatePackGetResponseDCVDelegationRecord>>,

    /// Identifier of the primary certificate in a pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_certificate: Option<String>,

    /// Domain validation errors that have been received by the certificate authority
    /// (CA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<CertificatePackGetResponseValidationError>>,

    /// Validation Method selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<CertificatePackGetResponseValidationMethod>,

    /// Certificates' validation records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_records: Option<Vec<CertificatePackGetResponseValidationRecord>>,

    /// Validity Days selected for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_days: Option<CertificatePackGetResponseValidityDays>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Certificate Authority selected for the order. For information on any certificate
    /// authority specific details or restrictions
    /// [see this page for more details.](https://developers.cloudflare.com/ssl/reference/certificate-authorities)
    pub certificate_authority: String,

    /// Comma separated list of valid host names for the certificate packs. Must contain
    /// the zone apex, may not contain more than 50 hosts, and may not be empty.
    pub hosts: String,

    /// Type of certificate pack.
    pub r#type: String,

    /// Validation Method selected for the order.
    pub validation_method: String,

    /// Validity Days selected for the order.
    pub validity_days: String,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Specify the deployment environment for the certificate packs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of certificate packs per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Include Certificate Packs of all statuses, not just active ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Whether or not to add Cloudflare Branding for the order. This will add a
    /// subdomain of sni.cloudflaressl.com as the Common Name if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_branding: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackQuotaGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<CertificatePackQuotaGetResponseAdvanced>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackQuotaGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationGetResponse {
    pub id: String,

    /// Whether this setting can be updated or not.
    pub editable: bool,

    /// Last time this setting was modified.
    pub modified_on: DateTime<Utc>,

    /// Current setting of the automatic SSL/TLS.
    pub value: RecommendationGetResponseValue,

    /// Next time this zone will be scanned by the Automatic SSL/TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_scheduled_scan: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSSLSettings {
    /// Disabling Universal SSL removes any currently active Universal SSL certificates
    /// for your zone from the edge and prevents any future Universal SSL certificates
    /// from being ordered. If there are no advanced certificates or custom certificates
    /// uploaded for the domain, visitors will be unable to access the domain over
    /// HTTPS.
    /// By disabling Universal SSL, you understand that the following Cloudflare
    /// settings and preferences will result in visitors being unable to visit your
    /// domain unless you have uploaded a custom certificate or purchased an advanced
    /// certificate.
    /// - HSTS
    /// - Always Use HTTPS
    /// - Opportunistic Encryption
    /// - Onion Routing
    /// - Any Page Rules redirecting traffic to HTTPS
    /// Similarly, any HTTP redirect to HTTPS at the origin while the Cloudflare proxy
    /// is enabled will result in users being unable to visit your site without a valid
    /// certificate at Cloudflare's edge.
    /// If you do not have a valid custom or advanced certificate at Cloudflare's edge
    /// and are unsure if any of the above Cloudflare settings are enabled, or if any
    /// HTTP redirects exist at your origin, we advise leaving Universal SSL enabled for
    /// your domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSSLSettingsParam {
    /// Disabling Universal SSL removes any currently active Universal SSL certificates
    /// for your zone from the edge and prevents any future Universal SSL certificates
    /// from being ordered. If there are no advanced certificates or custom certificates
    /// uploaded for the domain, visitors will be unable to access the domain over
    /// HTTPS.
    /// By disabling Universal SSL, you understand that the following Cloudflare
    /// settings and preferences will result in visitors being unable to visit your
    /// domain unless you have uploaded a custom certificate or purchased an advanced
    /// certificate.
    /// - HSTS
    /// - Always Use HTTPS
    /// - Opportunistic Encryption
    /// - Onion Routing
    /// - Any Page Rules redirecting traffic to HTTPS
    /// Similarly, any HTTP redirect to HTTPS at the origin while the Cloudflare proxy
    /// is enabled will result in users being unable to visit your site without a valid
    /// certificate at Cloudflare's edge.
    /// If you do not have a valid custom or advanced certificate at Cloudflare's edge
    /// and are unsure if any of the above Cloudflare settings are enabled, or if any
    /// HTTP redirects exist at your origin, we advise leaving Universal SSL enabled for
    /// your domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSettingEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub universal_ssl_settings: UniversalSSLSettingsParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalSettingGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    /// Current status of certificate.
    pub certificate_status: VerificationCertificateStatus,

    /// Certificate Authority is manually reviewing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_check: Option<bool>,

    /// Certificate Pack UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_pack_uuid: Option<String>,

    /// Certificate's signature algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<VerificationSignature>,

    /// Validation method in use for a certificate pack order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<ValidationMethod>,

    /// Certificate's required verification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_info: Option<VerificationVerificationInfo>,

    /// Status of the required verification information, omitted if verification status
    /// is unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<bool>,

    /// Method of verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_type: Option<VerificationVerificationType>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationEditResponse {
    /// Result status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Desired validation method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<VerificationEditResponseValidationMethod>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Desired validation method.
    pub validation_method: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Immediately retry SSL Verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponseCertificate {
    /// Certificate identifier.
    pub id: String,

    /// Hostnames covered by this certificate.
    pub hosts: Vec<String>,

    /// Certificate status.
    pub status: String,

    /// Certificate bundle method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,

    /// When the certificate from the authority expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// Specify the region where your private key can be held locally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_restrictions: Option<CertificatePackNewResponseCertificatesGeoRestrictions>,

    /// The certificate authority that issued the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// When the certificate was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The order/priority in which the certificate will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// The type of hash used for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// When the certificate was uploaded to Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_on: Option<DateTime<Utc>>,

    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponseDCVDelegationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponseValidationError {
    /// A domain validation error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponseValidationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponseCertificate {
    /// Certificate identifier.
    pub id: String,

    /// Hostnames covered by this certificate.
    pub hosts: Vec<String>,

    /// Certificate status.
    pub status: String,

    /// Certificate bundle method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,

    /// When the certificate from the authority expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// Specify the region where your private key can be held locally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_restrictions: Option<CertificatePackListResponseCertificatesGeoRestrictions>,

    /// The certificate authority that issued the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// When the certificate was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The order/priority in which the certificate will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// The type of hash used for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// When the certificate was uploaded to Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_on: Option<DateTime<Utc>>,

    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponseDCVDelegationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponseValidationError {
    /// A domain validation error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponseValidationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponseCertificate {
    /// Certificate identifier.
    pub id: String,

    /// Hostnames covered by this certificate.
    pub hosts: Vec<String>,

    /// Certificate status.
    pub status: String,

    /// Certificate bundle method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,

    /// When the certificate from the authority expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// Specify the region where your private key can be held locally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_restrictions: Option<CertificatePackEditResponseCertificatesGeoRestrictions>,

    /// The certificate authority that issued the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// When the certificate was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The order/priority in which the certificate will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// The type of hash used for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// When the certificate was uploaded to Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_on: Option<DateTime<Utc>>,

    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponseDCVDelegationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponseValidationError {
    /// A domain validation error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponseValidationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponseCertificate {
    /// Certificate identifier.
    pub id: String,

    /// Hostnames covered by this certificate.
    pub hosts: Vec<String>,

    /// Certificate status.
    pub status: String,

    /// Certificate bundle method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,

    /// When the certificate from the authority expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// Specify the region where your private key can be held locally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_restrictions: Option<CertificatePackGetResponseCertificatesGeoRestrictions>,

    /// The certificate authority that issued the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// When the certificate was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The order/priority in which the certificate will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// The type of hash used for the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// When the certificate was uploaded to Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_on: Option<DateTime<Utc>>,

    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponseDCVDelegationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponseValidationError {
    /// A domain validation error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponseValidationRecord {
    /// The CNAME record hostname for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,

    /// The CNAME record target value for DCV delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,

    /// The set of email addresses that the certificate authority (CA) will use to
    /// complete domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,

    /// The content that the certificate authority (CA) will expect to find at the
    /// http_url during the domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_body: Option<String>,

    /// The url that will be checked during domain validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "http_url")]
    pub httpurl: Option<String>,

    /// Status of the validation record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The hostname that the certificate authority (CA) will check for a TXT record
    /// during domain validation .
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    /// The TXT record that the certificate authority (CA) will check during domain
    /// validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackQuotaGetResponseAdvanced {
    /// Quantity Allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated: Option<i64>,

    /// Quantity Used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationVerificationInfo {
    /// Name of CNAME record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_name: Option<VerificationVerificationInfoRecordName>,

    /// Target of CNAME record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_target: Option<VerificationVerificationInfoRecordTarget>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackNewResponseCertificatesGeoRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CertificatePackNewResponseCertificatesGeoRestrictionsLabel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackListResponseCertificatesGeoRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CertificatePackListResponseCertificatesGeoRestrictionsLabel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackEditResponseCertificatesGeoRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CertificatePackEditResponseCertificatesGeoRestrictionsLabel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePackGetResponseCertificatesGeoRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CertificatePackGetResponseCertificatesGeoRestrictionsLabel>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "initializing")]
    StatusInitializing,
    #[serde(rename = "pending_validation")]
    StatusPendingValidation,
    #[serde(rename = "deleted")]
    StatusDeleted,
    #[serde(rename = "pending_issuance")]
    StatusPendingIssuance,
    #[serde(rename = "pending_deployment")]
    StatusPendingDeployment,
    #[serde(rename = "pending_deletion")]
    StatusPendingDeletion,
    #[serde(rename = "pending_expiration")]
    StatusPendingExpiration,
    #[serde(rename = "expired")]
    StatusExpired,
    #[serde(rename = "active")]
    StatusActive,
    #[serde(rename = "initializing_timed_out")]
    StatusInitializingTimedOut,
    #[serde(rename = "validation_timed_out")]
    StatusValidationTimedOut,
    #[serde(rename = "issuance_timed_out")]
    StatusIssuanceTimedOut,
    #[serde(rename = "deployment_timed_out")]
    StatusDeploymentTimedOut,
    #[serde(rename = "deletion_timed_out")]
    StatusDeletionTimedOut,
    #[serde(rename = "pending_cleanup")]
    StatusPendingCleanup,
    #[serde(rename = "staging_deployment")]
    StatusStagingDeployment,
    #[serde(rename = "staging_active")]
    StatusStagingActive,
    #[serde(rename = "deactivating")]
    StatusDeactivating,
    #[serde(rename = "inactive")]
    StatusInactive,
    #[serde(rename = "backup_issued")]
    StatusBackupIssued,
    #[serde(rename = "holding_deployment")]
    StatusHoldingDeployment,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationMethod {
    #[serde(rename = "http")]
    ValidationMethodHTTP,
    #[serde(rename = "cname")]
    ValidationMethodCNAME,
    #[serde(rename = "txt")]
    ValidationMethodTXT,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackNewResponseType {
    #[serde(rename = "mh_custom")]
    CertificatePackNewResponseTypeMhCustom,
    #[serde(rename = "managed_hostname")]
    CertificatePackNewResponseTypeManagedHostname,
    #[serde(rename = "sni_custom")]
    CertificatePackNewResponseTypeSNICustom,
    #[serde(rename = "universal")]
    CertificatePackNewResponseTypeUniversal,
    #[serde(rename = "advanced")]
    CertificatePackNewResponseTypeAdvanced,
    #[serde(rename = "total_tls")]
    CertificatePackNewResponseTypeTotalTLS,
    #[serde(rename = "keyless")]
    CertificatePackNewResponseTypeKeyless,
    #[serde(rename = "legacy_custom")]
    CertificatePackNewResponseTypeLegacyCustom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackNewResponseCertificateAuthority {
    #[serde(rename = "google")]
    CertificatePackNewResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    CertificatePackNewResponseCertificateAuthorityLetsEncrypt,
    #[serde(rename = "ssl_com")]
    CertificatePackNewResponseCertificateAuthoritySSLCom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackNewResponseValidationMethod {
    #[serde(rename = "txt")]
    CertificatePackNewResponseValidationMethodTXT,
    #[serde(rename = "http")]
    CertificatePackNewResponseValidationMethodHTTP,
    #[serde(rename = "email")]
    CertificatePackNewResponseValidationMethodEmail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackNewResponseValidityDays {
    CertificatePackNewResponseValidityDays14 = 14,
    CertificatePackNewResponseValidityDays30 = 30,
    CertificatePackNewResponseValidityDays90 = 90,
    CertificatePackNewResponseValidityDays365 = 365,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackListResponseType {
    #[serde(rename = "mh_custom")]
    CertificatePackListResponseTypeMhCustom,
    #[serde(rename = "managed_hostname")]
    CertificatePackListResponseTypeManagedHostname,
    #[serde(rename = "sni_custom")]
    CertificatePackListResponseTypeSNICustom,
    #[serde(rename = "universal")]
    CertificatePackListResponseTypeUniversal,
    #[serde(rename = "advanced")]
    CertificatePackListResponseTypeAdvanced,
    #[serde(rename = "total_tls")]
    CertificatePackListResponseTypeTotalTLS,
    #[serde(rename = "keyless")]
    CertificatePackListResponseTypeKeyless,
    #[serde(rename = "legacy_custom")]
    CertificatePackListResponseTypeLegacyCustom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackListResponseCertificateAuthority {
    #[serde(rename = "google")]
    CertificatePackListResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    CertificatePackListResponseCertificateAuthorityLetsEncrypt,
    #[serde(rename = "ssl_com")]
    CertificatePackListResponseCertificateAuthoritySSLCom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackListResponseValidationMethod {
    #[serde(rename = "txt")]
    CertificatePackListResponseValidationMethodTXT,
    #[serde(rename = "http")]
    CertificatePackListResponseValidationMethodHTTP,
    #[serde(rename = "email")]
    CertificatePackListResponseValidationMethodEmail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackListResponseValidityDays {
    CertificatePackListResponseValidityDays14 = 14,
    CertificatePackListResponseValidityDays30 = 30,
    CertificatePackListResponseValidityDays90 = 90,
    CertificatePackListResponseValidityDays365 = 365,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackEditResponseType {
    #[serde(rename = "mh_custom")]
    CertificatePackEditResponseTypeMhCustom,
    #[serde(rename = "managed_hostname")]
    CertificatePackEditResponseTypeManagedHostname,
    #[serde(rename = "sni_custom")]
    CertificatePackEditResponseTypeSNICustom,
    #[serde(rename = "universal")]
    CertificatePackEditResponseTypeUniversal,
    #[serde(rename = "advanced")]
    CertificatePackEditResponseTypeAdvanced,
    #[serde(rename = "total_tls")]
    CertificatePackEditResponseTypeTotalTLS,
    #[serde(rename = "keyless")]
    CertificatePackEditResponseTypeKeyless,
    #[serde(rename = "legacy_custom")]
    CertificatePackEditResponseTypeLegacyCustom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackEditResponseCertificateAuthority {
    #[serde(rename = "google")]
    CertificatePackEditResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    CertificatePackEditResponseCertificateAuthorityLetsEncrypt,
    #[serde(rename = "ssl_com")]
    CertificatePackEditResponseCertificateAuthoritySSLCom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackEditResponseValidationMethod {
    #[serde(rename = "txt")]
    CertificatePackEditResponseValidationMethodTXT,
    #[serde(rename = "http")]
    CertificatePackEditResponseValidationMethodHTTP,
    #[serde(rename = "email")]
    CertificatePackEditResponseValidationMethodEmail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackEditResponseValidityDays {
    CertificatePackEditResponseValidityDays14 = 14,
    CertificatePackEditResponseValidityDays30 = 30,
    CertificatePackEditResponseValidityDays90 = 90,
    CertificatePackEditResponseValidityDays365 = 365,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackGetResponseType {
    #[serde(rename = "mh_custom")]
    CertificatePackGetResponseTypeMhCustom,
    #[serde(rename = "managed_hostname")]
    CertificatePackGetResponseTypeManagedHostname,
    #[serde(rename = "sni_custom")]
    CertificatePackGetResponseTypeSNICustom,
    #[serde(rename = "universal")]
    CertificatePackGetResponseTypeUniversal,
    #[serde(rename = "advanced")]
    CertificatePackGetResponseTypeAdvanced,
    #[serde(rename = "total_tls")]
    CertificatePackGetResponseTypeTotalTLS,
    #[serde(rename = "keyless")]
    CertificatePackGetResponseTypeKeyless,
    #[serde(rename = "legacy_custom")]
    CertificatePackGetResponseTypeLegacyCustom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackGetResponseCertificateAuthority {
    #[serde(rename = "google")]
    CertificatePackGetResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    CertificatePackGetResponseCertificateAuthorityLetsEncrypt,
    #[serde(rename = "ssl_com")]
    CertificatePackGetResponseCertificateAuthoritySSLCom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackGetResponseValidationMethod {
    #[serde(rename = "txt")]
    CertificatePackGetResponseValidationMethodTXT,
    #[serde(rename = "http")]
    CertificatePackGetResponseValidationMethodHTTP,
    #[serde(rename = "email")]
    CertificatePackGetResponseValidationMethodEmail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackGetResponseValidityDays {
    CertificatePackGetResponseValidityDays14 = 14,
    CertificatePackGetResponseValidityDays30 = 30,
    CertificatePackGetResponseValidityDays90 = 90,
    CertificatePackGetResponseValidityDays365 = 365,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationGetResponseValue {
    #[serde(rename = "auto")]
    RecommendationGetResponseValueAuto,
    #[serde(rename = "custom")]
    RecommendationGetResponseValueCustom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationCertificateStatus {
    #[serde(rename = "initializing")]
    VerificationCertificateStatusInitializing,
    #[serde(rename = "authorizing")]
    VerificationCertificateStatusAuthorizing,
    #[serde(rename = "active")]
    VerificationCertificateStatusActive,
    #[serde(rename = "expired")]
    VerificationCertificateStatusExpired,
    #[serde(rename = "issuing")]
    VerificationCertificateStatusIssuing,
    #[serde(rename = "timing_out")]
    VerificationCertificateStatusTimingOut,
    #[serde(rename = "pending_deployment")]
    VerificationCertificateStatusPendingDeployment,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationSignature {
    #[serde(rename = "ECDSAWithSHA256")]
    VerificationSignatureEcdsaWithSha256,
    #[serde(rename = "SHA1WithRSA")]
    VerificationSignatureSha1WithRSA,
    #[serde(rename = "SHA256WithRSA")]
    VerificationSignatureSha256WithRSA,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationVerificationType {
    #[serde(rename = "cname")]
    VerificationVerificationTypeCNAME,
    #[serde(rename = "meta tag")]
    VerificationVerificationTypeMetaTag,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationEditResponseValidationMethod {
    #[serde(rename = "http")]
    VerificationEditResponseValidationMethodHTTP,
    #[serde(rename = "cname")]
    VerificationEditResponseValidationMethodCNAME,
    #[serde(rename = "txt")]
    VerificationEditResponseValidationMethodTXT,
    #[serde(rename = "email")]
    VerificationEditResponseValidationMethodEmail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationVerificationInfoRecordName {
    #[serde(rename = "record_name")]
    VerificationVerificationInfoRecordNameRecordName,
    #[serde(rename = "http_url")]
    VerificationVerificationInfoRecordNameHTTPURL,
    #[serde(rename = "cname")]
    VerificationVerificationInfoRecordNameCNAME,
    #[serde(rename = "txt_name")]
    VerificationVerificationInfoRecordNameTXTName,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationVerificationInfoRecordTarget {
    #[serde(rename = "record_value")]
    VerificationVerificationInfoRecordTargetRecordValue,
    #[serde(rename = "http_body")]
    VerificationVerificationInfoRecordTargetHTTPBody,
    #[serde(rename = "cname_target")]
    VerificationVerificationInfoRecordTargetCNAMETarget,
    #[serde(rename = "txt_value")]
    VerificationVerificationInfoRecordTargetTXTValue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackNewResponseCertificatesGeoRestrictionsLabel {
    #[serde(rename = "us")]
    CertificatePackNewResponseCertificatesGeoRestrictionsLabelUs,
    #[serde(rename = "eu")]
    CertificatePackNewResponseCertificatesGeoRestrictionsLabelEu,
    #[serde(rename = "highest_security")]
    CertificatePackNewResponseCertificatesGeoRestrictionsLabelHighestSecurity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackListResponseCertificatesGeoRestrictionsLabel {
    #[serde(rename = "us")]
    CertificatePackListResponseCertificatesGeoRestrictionsLabelUs,
    #[serde(rename = "eu")]
    CertificatePackListResponseCertificatesGeoRestrictionsLabelEu,
    #[serde(rename = "highest_security")]
    CertificatePackListResponseCertificatesGeoRestrictionsLabelHighestSecurity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackEditResponseCertificatesGeoRestrictionsLabel {
    #[serde(rename = "us")]
    CertificatePackEditResponseCertificatesGeoRestrictionsLabelUs,
    #[serde(rename = "eu")]
    CertificatePackEditResponseCertificatesGeoRestrictionsLabelEu,
    #[serde(rename = "highest_security")]
    CertificatePackEditResponseCertificatesGeoRestrictionsLabelHighestSecurity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificatePackGetResponseCertificatesGeoRestrictionsLabel {
    #[serde(rename = "us")]
    CertificatePackGetResponseCertificatesGeoRestrictionsLabelUs,
    #[serde(rename = "eu")]
    CertificatePackGetResponseCertificatesGeoRestrictionsLabelEu,
    #[serde(rename = "highest_security")]
    CertificatePackGetResponseCertificatesGeoRestrictionsLabelHighestSecurity,
}

