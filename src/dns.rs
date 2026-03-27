#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-dns"))]
pub type RecordTags = String;

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Array with one row per combination of dimension values.
    pub data: Vec<ReportData>,

    /// Number of seconds between current time and last processed event, in another
    /// words how many seconds of data could be missing.
    pub data_lag: f64,

    /// Maximum results for each metric (object mapping metric names to values).
    /// Currently always an empty object.
    pub max: String,

    /// Minimum results for each metric (object mapping metric names to values).
    /// Currently always an empty object.
    pub min: String,

    pub query: ReportQuery,

    /// Total number of rows in the result.
    pub rows: f64,

    /// Total results for metrics across all data (object mapping metric names to
    /// values).
    pub totals: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReportGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A comma-separated list of dimensions to group results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,

    /// Segmentation filter in 'attribute operator value' format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Limit number of returned metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// A comma-separated list of metrics to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,

    /// Start date and time of requesting data period in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// A comma-separated list of dimensions to sort by, where each dimension may be
    /// prefixed by - (descending) or + (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// End date and time of requesting data period in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByTime {
    /// Array with one row per combination of dimension values.
    pub data: Vec<ByTimeData>,

    /// Number of seconds between current time and last processed event, in another
    /// words how many seconds of data could be missing.
    pub data_lag: f64,

    /// Maximum results for each metric (object mapping metric names to values).
    /// Currently always an empty object.
    pub max: String,

    /// Minimum results for each metric (object mapping metric names to values).
    /// Currently always an empty object.
    pub min: String,

    pub query: ByTimeQuery,

    /// Total number of rows in the result.
    pub rows: f64,

    /// Array of time intervals in the response data. Each interval is represented as an
    /// array containing two values: the start time, and the end time.
    pub time_intervals: Vec<Vec<DateTime<Utc>>>,

    /// Total results for metrics across all data (object mapping metric names to
    /// values).
    pub totals: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReportBytimeGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A comma-separated list of dimensions to group results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,

    /// Segmentation filter in 'attribute operator value' format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Limit number of returned metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// A comma-separated list of metrics to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,

    /// Start date and time of requesting data period in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// A comma-separated list of dimensions to sort by, where each dimension may be
    /// prefixed by - (descending) or + (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Unit of time to group data by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delta: Option<String>,

    /// End date and time of requesting data period in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSSEC {
    /// Algorithm key code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Digest hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Type of digest algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_algorithm: Option<String>,

    /// Coded type for digest algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_type: Option<String>,

    /// If true, multi-signer DNSSEC is enabled on the zone, allowing multiple providers
    /// to serve a DNSSEC-signed zone at the same time. This is required for DNSKEY
    /// records (except those automatically generated by Cloudflare) to be added to the
    /// zone.
    /// See
    /// [Multi-signer DNSSEC](https://developers.cloudflare.com/dns/dnssec/multi-signer-dnssec/)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_multi_signer: Option<bool>,

    /// If true, allows Cloudflare to transfer in a DNSSEC-signed zone including
    /// signatures from an external provider, without requiring Cloudflare to sign any
    /// records on the fly.
    /// Note that this feature has some limitations. See
    /// [Cloudflare as Secondary](https://developers.cloudflare.com/dns/zone-setups/zone-transfers/cloudflare-as-secondary/setup/#dnssec)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_presigned: Option<bool>,

    /// If true, enables the use of NSEC3 together with DNSSEC on the zone. Combined
    /// with setting dnssec_presigned to true, this enables the use of NSEC3 records
    /// when transferring in from an external provider. If dnssec_presigned is instead
    /// set to false (default), NSEC3 records will be generated and signed at request
    /// time.
    /// See
    /// [DNSSEC with NSEC3](https://developers.cloudflare.com/dns/dnssec/enable-nsec3/)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_use_nsec3: Option<bool>,

    /// Full DS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds: Option<String>,

    /// Flag for DNSSEC record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<f64>,

    /// Code for key tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<f64>,

    /// Algorithm key type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,

    /// When DNSSEC was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Public key for DS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// Status of DNSSEC, based on user-desired state and presence of necessary records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DNSSECStatus>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSSECDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSSECEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// If true, multi-signer DNSSEC is enabled on the zone, allowing multiple providers
    /// to serve a DNSSEC-signed zone at the same time. This is required for DNSKEY
    /// records (except those automatically generated by Cloudflare) to be added to the
    /// zone.
    /// See
    /// [Multi-signer DNSSEC](https://developers.cloudflare.com/dns/dnssec/multi-signer-dnssec/)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_multi_signer: Option<String>,

    /// If true, allows Cloudflare to transfer in a DNSSEC-signed zone including
    /// signatures from an external provider, without requiring Cloudflare to sign any
    /// records on the fly.
    /// Note that this feature has some limitations. See
    /// [Cloudflare as Secondary](https://developers.cloudflare.com/dns/zone-setups/zone-transfers/cloudflare-as-secondary/setup/#dnssec)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_presigned: Option<String>,

    /// If true, enables the use of NSEC3 together with DNSSEC on the zone. Combined
    /// with setting dnssec_presigned to true, this enables the use of NSEC3 records
    /// when transferring in from an external provider. If dnssec_presigned is instead
    /// set to false (default), NSEC3 records will be generated and signed at request
    /// time.
    /// See
    /// [DNSSEC with NSEC3](https://developers.cloudflare.com/dns/dnssec/enable-nsec3/)
    /// for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_use_nsec3: Option<String>,

    /// Status of DNSSEC, based on user-desired state and presence of necessary records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSSECGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: ARecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ARecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AAAARecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: AAAARecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AAAARecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AAAARecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAARecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: CAARecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted CAA content. See 'data' to set CAA properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a CAA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CAARecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CAARecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAARecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a CAA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CERTRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: CERTRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted CERT content. See 'data' to set CERT properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a CERT record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CERTRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CERTRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CERTRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a CERT record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CNAMERecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: CNAMERecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid hostname. Must not match the record's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CNAMERecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CNAMERecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid hostname. Must not match the record's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSKEYRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: DNSKEYRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted DNSKEY content. See 'data' to set DNSKEY properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a DNSKEY record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DNSKEYRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DNSKEYRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSKEYRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a DNSKEY record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: DSRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted DS content. See 'data' to set DS properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a DS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DSRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DSRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a DS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: HTTPSRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted HTTPS content. See 'data' to set HTTPS properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a HTTPS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HTTPSRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HTTPSRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a HTTPS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOCRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: LOCRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted LOC content. See 'data' to set LOC properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a LOC record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<LOCRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LOCRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOCRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a LOC record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MXRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: MXRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid mail server hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MXRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MXRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid mail server hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NAPTRRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: NAPTRRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted NAPTR content. See 'data' to set NAPTR properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a NAPTR record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<NAPTRRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<NAPTRRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NAPTRRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a NAPTR record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NSRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: NSRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid name server host name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<NSRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NSRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// A valid name server host name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTRRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: PTRRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Domain name pointing to the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PTRRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTRRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Domain name pointing to the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// When the record comment was last modified. Omitted if there is no comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_modified_on: Option<DateTime<Utc>>,

    /// A valid IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// When the record was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// This field can have the runtime type of [CAARecordData], [CERTRecordData],
    /// [DNSKEYRecordData], [DSRecordData], [HTTPSRecordData], [LOCRecordData],
    /// [NAPTRRecordData], [SMIMEARecordData], [SRVRecordData], [SSHFPRecordData],
    /// [SVCBRecordData], [TLSARecordData], [URIRecordData].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// This field can have the runtime type of [interface{}].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// When the record was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Complete DNS record name, including the zone name, in Punycode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// Whether the record can be proxied by Cloudflare or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxiable: Option<bool>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// This field can have the runtime type of [ARecordSettings], [AAAARecordSettings],
    /// [CNAMERecordSettings], [MXRecordSettings], [NSRecordSettings],
    /// [RecordResponseOpenpgpkeySettings], [PTRRecordSettings], [TXTRecordSettings],
    /// [CAARecordSettings], [CERTRecordSettings], [DNSKEYRecordSettings],
    /// [DSRecordSettings], [HTTPSRecordSettings], [LOCRecordSettings],
    /// [NAPTRRecordSettings], [SMIMEARecordSettings], [SRVRecordSettings],
    /// [SSHFPRecordSettings], [SVCBRecordSettings], [TLSARecordSettings],
    /// [URIRecordSettings].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// This field can have the runtime type of [[]RecordTags].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,

    /// When the record tags were last modified. Omitted if there are no tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_modified_on: Option<DateTime<Utc>>,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,

    /// Record type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RecordResponseType>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMIMEARecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: SMIMEARecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted SMIMEA content. See 'data' to set SMIMEA properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a SMIMEA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SMIMEARecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SMIMEARecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMIMEARecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a SMIMEA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRVRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: SRVRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Priority, weight, port, and SRV target. See 'data' for setting the individual
    /// component values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a SRV record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SRVRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SRVRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRVRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a SRV record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHFPRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: SSHFPRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted SSHFP content. See 'data' to set SSHFP properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a SSHFP record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SSHFPRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SSHFPRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHFPRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a SSHFP record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVCBRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: SVCBRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted SVCB content. See 'data' to set SVCB properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a SVCB record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SVCBRecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SVCBRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVCBRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a SVCB record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSARecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: TLSARecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted TLSA content. See 'data' to set TLSA properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a TLSA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TLSARecordData>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<TLSARecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSARecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a TLSA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXTRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: TXTRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Text content for the record. The content must consist of quoted "character
    /// strings" (RFC 1035), each with a length of up to 255 bytes. Strings exceeding
    /// this allowed maximum length are automatically split.
    /// Learn more at
    /// <https://www.cloudflare.com/learning/dns/dns-records/dns-txt-record/>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<TXTRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXTRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Text content for the record. The content must consist of quoted "character
    /// strings" (RFC 1035), each with a length of up to 255 bytes. Strings exceeding
    /// this allowed maximum length are automatically split.
    /// Learn more at
    /// <https://www.cloudflare.com/learning/dns/dns-records/dns-txt-record/>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URIRecord {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: i32,

    /// Record type.
    pub r#type: URIRecordType,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Formatted URI content. See 'data' to set URI properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Components of a URI record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<URIRecordData>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<URIRecordSettings>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RecordTags>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URIRecordParam {
    /// Complete DNS record name, including the zone name, in Punycode.
    pub name: String,

    /// Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
    /// Value must be between 60 and 86400, with the minimum reduced to 30 for
    /// Enterprise zones.
    pub ttl: String,

    /// Record type.
    pub r#type: String,

    /// Comments or notes about the DNS record. This field has no effect on DNS
    /// responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Components of a URI record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Settings for the DNS record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    /// Custom tags for the DNS record. This field has no effect on DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDeleteResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordBatchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Vec<RecordResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<RecordResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<RecordResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub puts: Option<Vec<RecordResponse>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordImportResponse {
    /// Number of DNS records added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recs_added: Option<f64>,

    /// Total number of DNS records parsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records_parsed: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanResponse {
    /// Number of DNS records added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recs_added: Option<f64>,

    /// Total number of DNS records parsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records_parsed: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanReviewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<RecordResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejects: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerResponse {
    pub errors: Vec<RecordScanTriggerResponseError>,

    pub messages: Vec<RecordScanTriggerResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordNewParams<T> {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    // Satisfied by [dns.ARecordParam], [dns.AAAARecordParam], [dns.CNAMERecordParam],
    // [dns.MXRecordParam], [dns.NSRecordParam],
    // [dns.RecordNewParamsBodyDNSRecordsOpenpgpkeyRecord], [dns.PTRRecordParam],
    // [dns.TXTRecordParam], [dns.CAARecordParam], [dns.CERTRecordParam],
    // [dns.DNSKEYRecordParam], [dns.DSRecordParam], [dns.HTTPSRecordParam],
    // [dns.LOCRecordParam], [dns.NAPTRRecordParam], [dns.SMIMEARecordParam],
    // [dns.SRVRecordParam], [dns.SSHFPRecordParam], [dns.SVCBRecordParam],
    // [dns.TLSARecordParam], [dns.URIRecordParam], [RecordNewParamsBody].
    pub body: T,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordUpdateParams<T> {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    // Satisfied by [dns.ARecordParam], [dns.AAAARecordParam], [dns.CNAMERecordParam],
    // [dns.MXRecordParam], [dns.NSRecordParam],
    // [dns.RecordUpdateParamsBodyDNSRecordsOpenpgpkeyRecord], [dns.PTRRecordParam],
    // [dns.TXTRecordParam], [dns.CAARecordParam], [dns.CERTRecordParam],
    // [dns.DNSKEYRecordParam], [dns.DSRecordParam], [dns.HTTPSRecordParam],
    // [dns.LOCRecordParam], [dns.NAPTRRecordParam], [dns.SMIMEARecordParam],
    // [dns.SRVRecordParam], [dns.SSHFPRecordParam], [dns.SVCBRecordParam],
    // [dns.TLSARecordParam], [dns.URIRecordParam], [RecordUpdateParamsBody].
    pub body: T,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Direction to order DNS records in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Whether to match all search requirements or at least one (any). If set to `all`,
    /// acts like a logical AND between filters. If set to `any`, acts like a logical OR
    /// instead. Note that the interaction between tag filters is controlled by the
    /// `tag-match` parameter instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to order DNS records by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of DNS records per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Whether the record is receiving the performance and security benefits of
    /// Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,

    /// Allows searching in multiple properties of a DNS record simultaneously. This
    /// parameter is intended for human users, not automation. Its exact behavior is
    /// intentionally left unspecified and is subject to change in the future. This
    /// parameter works independently of the `match` setting. For automated searches,
    /// please use the other available parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Whether to match all tag search requirements or at least one (any). If set to
    /// `all`, acts like a logical AND between tag filters. If set to `any`, acts like a
    /// logical OR instead. Note that the regular `match` parameter is still used to
    /// combine the resulting condition with other filters that aren't related to tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_match: Option<String>,

    /// Record type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordBatchParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub puts: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordEditParams<T> {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    // Satisfied by [dns.ARecordParam], [dns.AAAARecordParam], [dns.CNAMERecordParam],
    // [dns.MXRecordParam], [dns.NSRecordParam],
    // [dns.RecordEditParamsBodyDNSRecordsOpenpgpkeyRecord], [dns.PTRRecordParam],
    // [dns.TXTRecordParam], [dns.CAARecordParam], [dns.CERTRecordParam],
    // [dns.DNSKEYRecordParam], [dns.DSRecordParam], [dns.HTTPSRecordParam],
    // [dns.LOCRecordParam], [dns.NAPTRRecordParam], [dns.SMIMEARecordParam],
    // [dns.SRVRecordParam], [dns.SSHFPRecordParam], [dns.SVCBRecordParam],
    // [dns.TLSARecordParam], [dns.URIRecordParam], [RecordEditParamsBody].
    pub body: T,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordExportParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordImportParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// BIND config to import.
    /// **Tip:** When using cURL, a file can be uploaded using
    /// `--form 'file=@bind_config.txt'`.
    pub file: String,

    /// Whether or not proxiable records should receive the performance and security
    /// benefits of Cloudflare.
    /// The value should be either `true` or `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanReviewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejects: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditResponse {
    pub zone_defaults: SettingAccountEditResponseZoneDefaults,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetResponse {
    pub zone_defaults: SettingAccountGetResponseZoneDefaults,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_defaults: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewNewResponse {
    /// Identifier.
    pub id: String,

    /// When the view was created.
    pub created_time: DateTime<Utc>,

    /// When the view was last modified.
    pub modified_time: DateTime<Utc>,

    /// The name of the view.
    pub name: String,

    /// The list of zones linked to this view.
    pub zones: Vec<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewListResponse {
    /// Identifier.
    pub id: String,

    /// When the view was created.
    pub created_time: DateTime<Utc>,

    /// When the view was last modified.
    pub modified_time: DateTime<Utc>,

    /// The name of the view.
    pub name: String,

    /// The list of zones linked to this view.
    pub zones: Vec<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewDeleteResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewEditResponse {
    /// Identifier.
    pub id: String,

    /// When the view was created.
    pub created_time: DateTime<Utc>,

    /// When the view was last modified.
    pub modified_time: DateTime<Utc>,

    /// The name of the view.
    pub name: String,

    /// The list of zones linked to this view.
    pub zones: Vec<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewGetResponse {
    /// Identifier.
    pub id: String,

    /// When the view was created.
    pub created_time: DateTime<Utc>,

    /// When the view was last modified.
    pub modified_time: DateTime<Utc>,

    /// The name of the view.
    pub name: String,

    /// The list of zones linked to this view.
    pub zones: Vec<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The name of the view.
    pub name: String,

    /// The list of zones linked to this view.
    pub zones: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Direction to order DNS views in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Whether to match all search requirements or at least one (any). If set to `all`,
    /// acts like a logical AND between filters. If set to `any`, acts like a logical OR
    /// instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to order DNS views by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of DNS views per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// A zone ID that exists in the zones list for the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A zone name that exists in the zones list for the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The name of the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The list of zones linked to this view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountViewGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneEditResponse {
    /// Whether to flatten all CNAME records in the zone. Note that, due to DNS
    /// limitations, a CNAME record at the zone apex will always be flattened.
    #[serde(rename = "flatten_all_cnames")]
    pub flatten_all_cnam_es: bool,

    /// Whether to enable Foundation DNS Advanced Nameservers on the zone.
    pub foundation_dns: bool,

    /// Settings for this internal zone.
    pub internal_dns: SettingZoneEditResponseInternalDNS,

    /// Whether to enable multi-provider DNS, which causes Cloudflare to activate the
    /// zone even when non-Cloudflare NS records exist, and to respect NS records at the
    /// zone apex during outbound zone transfers.
    pub multi_provider: bool,

    /// Settings determining the nameservers through which the zone should be available.
    pub nameservers: SettingZoneEditResponseNameservers,

    /// The time to live (TTL) of the zone's nameserver (NS) records.
    #[serde(rename = "ns_ttl")]
    pub nsttl: f64,

    /// Allows a Secondary DNS zone to use (proxied) override records and CNAME
    /// flattening at the zone apex.
    pub secondary_overrides: bool,

    /// Components of the zone's SOA record.
    pub soa: SettingZoneEditResponseSOA,

    /// Whether the zone mode is a regular or CDN/DNS only zone.
    pub zone_mode: SettingZoneEditResponseZoneMode,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneGetResponse {
    /// Whether to flatten all CNAME records in the zone. Note that, due to DNS
    /// limitations, a CNAME record at the zone apex will always be flattened.
    #[serde(rename = "flatten_all_cnames")]
    pub flatten_all_cnam_es: bool,

    /// Whether to enable Foundation DNS Advanced Nameservers on the zone.
    pub foundation_dns: bool,

    /// Settings for this internal zone.
    pub internal_dns: SettingZoneGetResponseInternalDNS,

    /// Whether to enable multi-provider DNS, which causes Cloudflare to activate the
    /// zone even when non-Cloudflare NS records exist, and to respect NS records at the
    /// zone apex during outbound zone transfers.
    pub multi_provider: bool,

    /// Settings determining the nameservers through which the zone should be available.
    pub nameservers: SettingZoneGetResponseNameservers,

    /// The time to live (TTL) of the zone's nameserver (NS) records.
    #[serde(rename = "ns_ttl")]
    pub nsttl: f64,

    /// Allows a Secondary DNS zone to use (proxied) override records and CNAME
    /// flattening at the zone apex.
    pub secondary_overrides: bool,

    /// Components of the zone's SOA record.
    pub soa: SettingZoneGetResponseSOA,

    /// Whether the zone mode is a regular or CDN/DNS only zone.
    pub zone_mode: SettingZoneGetResponseZoneMode,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Whether to flatten all CNAME records in the zone. Note that, due to DNS
    /// limitations, a CNAME record at the zone apex will always be flattened.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flatten_all_cnames")]
    pub flatten_all_cnam_es: Option<String>,

    /// Whether to enable Foundation DNS Advanced Nameservers on the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_dns: Option<String>,

    /// Settings for this internal zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_dns: Option<String>,

    /// Whether to enable multi-provider DNS, which causes Cloudflare to activate the
    /// zone even when non-Cloudflare NS records exist, and to respect NS records at the
    /// zone apex during outbound zone transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_provider: Option<String>,

    /// Settings determining the nameservers through which the zone should be available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<String>,

    /// The time to live (TTL) of the zone's nameserver (NS) records.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ns_ttl")]
    pub nsttl: Option<String>,

    /// Allows a Secondary DNS zone to use (proxied) override records and CNAME
    /// flattening at the zone apex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_overrides: Option<String>,

    /// Components of the zone's SOA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa: Option<String>,

    /// Whether the zone mode is a regular or CDN/DNS only zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_mode: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACL {
    pub id: String,

    /// Allowed IPv4/IPv6 address range of primary or secondary nameservers. This will
    /// be applied for the entire account. The IP range is used to allow additional
    /// NOTIFY IPs for secondary zones and IPs Cloudflare allows AXFR/IXFR requests from
    /// for primary zones. CIDRs are limited to a maximum of /24 for IPv4 and /64 for
    /// IPv6 respectively.
    pub ip_range: String,

    /// The name of the acl.
    pub name: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLParam {
    /// Allowed IPv4/IPv6 address range of primary or secondary nameservers. This will
    /// be applied for the entire account. The IP range is used to allow additional
    /// NOTIFY IPs for secondary zones and IPs Cloudflare allows AXFR/IXFR requests from
    /// for primary zones. CIDRs are limited to a maximum of /24 for IPv4 and /64 for
    /// IPv6 respectively.
    pub ip_range: String,

    /// The name of the acl.
    pub name: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Allowed IPv4/IPv6 address range of primary or secondary nameservers. This will
    /// be applied for the entire account. The IP range is used to allow additional
    /// NOTIFY IPs for secondary zones and IPs Cloudflare allows AXFR/IXFR requests from
    /// for primary zones. CIDRs are limited to a maximum of /24 for IPv4 and /64 for
    /// IPv6 respectively.
    pub ip_range: String,

    /// The name of the acl.
    pub name: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub acl: ACLParam,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferACLGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferForceAXFRNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingNewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// How often should a secondary zone auto refresh regardless of DNS NOTIFY. Not
    /// applicable for primary zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_seconds: Option<f64>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// How often should a secondary zone auto refresh regardless of DNS NOTIFY. Not
    /// applicable for primary zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_seconds: Option<f64>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// How often should a secondary zone auto refresh regardless of DNS NOTIFY. Not
    /// applicable for primary zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_seconds: Option<f64>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// How often should a secondary zone auto refresh regardless of DNS NOTIFY. Not
    /// applicable for primary zones.
    pub auto_refresh_seconds: String,

    /// Zone name.
    pub name: String,

    /// A list of peer tags.
    pub peers: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// How often should a secondary zone auto refresh regardless of DNS NOTIFY. Not
    /// applicable for primary zones.
    pub auto_refresh_seconds: String,

    /// Zone name.
    pub name: String,

    /// A list of peer tags.
    pub peers: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferIncomingGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingNewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transferred_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transferred_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// The time for a specific event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transferred_time: Option<String>,

    /// Zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of peer tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<String>>,

    /// The serial number of the SOA for the given zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_serial: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Zone name.
    pub name: String,

    /// A list of peer tags.
    pub peers: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Zone name.
    pub name: String,

    /// A list of peer tags.
    pub peers: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingDisableParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingEnableParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingForceNotifyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferOutgoingStatusGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,

    /// The name of the peer.
    pub name: String,

    /// IPv4/IPv6 address of primary or secondary nameserver, depending on what zone
    /// this peer is linked to. For primary zones this IP defines the IP of the
    /// secondary nameserver Cloudflare will NOTIFY upon zone changes. For secondary
    /// zones this IP defines the IP of the primary nameserver Cloudflare will send
    /// AXFR/IXFR requests to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Enable IXFR transfer protocol, default is AXFR. Only applicable to secondary
    /// zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ixfr_enable: Option<bool>,

    /// DNS port of primary or secondary nameserver, depending on what zone this peer is
    /// linked to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,

    /// TSIG authentication will be used for zone transfer if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tsig_id")]
    pub tsigid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerParam {
    /// The name of the peer.
    pub name: String,

    /// IPv4/IPv6 address of primary or secondary nameserver, depending on what zone
    /// this peer is linked to. For primary zones this IP defines the IP of the
    /// secondary nameserver Cloudflare will NOTIFY upon zone changes. For secondary
    /// zones this IP defines the IP of the primary nameserver Cloudflare will send
    /// AXFR/IXFR requests to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Enable IXFR transfer protocol, default is AXFR. Only applicable to secondary
    /// zones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ixfr_enable: Option<String>,

    /// DNS port of primary or secondary nameserver, depending on what zone this peer is
    /// linked to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    /// TSIG authentication will be used for zone transfer if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tsig_id")]
    pub tsigid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The name of the peer.
    pub name: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub peer: PeerParam,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferPeerGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSIG {
    pub id: String,

    /// TSIG algorithm.
    pub algo: String,

    /// TSIG key name.
    pub name: String,

    /// TSIG secret.
    pub secret: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSIGParam {
    /// TSIG algorithm.
    pub algo: String,

    /// TSIG key name.
    pub name: String,

    /// TSIG secret.
    pub secret: String,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGDeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub tsig: TSIGParam,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub tsig: TSIGParam,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGDeleteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransferTSIGGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Array of dimension values, representing the combination of dimension values
    /// corresponding to this row.
    pub dimensions: Vec<String>,

    /// Array with one item per requested metric. Each item is a single value.
    pub metrics: Vec<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportQuery {
    /// Array of dimension names.
    pub dimensions: Vec<String>,

    /// Limit number of returned metrics.
    pub limit: i64,

    /// Array of metric names.
    pub metrics: Vec<String>,

    /// Start date and time of requesting data period in ISO 8601 format.
    pub since: DateTime<Utc>,

    /// End date and time of requesting data period in ISO 8601 format.
    pub until: DateTime<Utc>,

    /// Segmentation filter in 'attribute operator value' format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Array of dimensions to sort by, where each dimension may be prefixed by -
    /// (descending) or + (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByTimeData {
    /// Array of dimension values, representing the combination of dimension values
    /// corresponding to this row.
    pub dimensions: Vec<String>,

    /// Array with one item per requested metric. Each item is an array of values,
    /// broken down by time interval.
    pub metrics: Vec<Vec<f64>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByTimeQuery {
    /// Array of dimension names.
    pub dimensions: Vec<String>,

    /// Limit number of returned metrics.
    pub limit: i64,

    /// Array of metric names.
    pub metrics: Vec<String>,

    /// Start date and time of requesting data period in ISO 8601 format.
    pub since: DateTime<Utc>,

    /// Unit of time to group data by.
    pub time_delta: ByTimeQueryTimeDelta,

    /// End date and time of requesting data period in ISO 8601 format.
    pub until: DateTime<Utc>,

    /// Segmentation filter in 'attribute operator value' format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Array of dimensions to sort by, where each dimension may be prefixed by -
    /// (descending) or + (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AAAARecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAARecordData {
    /// Flags for the CAA record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<f64>,

    /// Name of the property controlled by this record (e.g.: issue, issuewild, iodef).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Value of the record. This field's semantics depend on the chosen tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAARecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CERTRecordData {
    /// Algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<f64>,

    /// Certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Key Tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<f64>,

    /// Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CERTRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CNAMERecordSettings {
    /// If enabled, causes the CNAME record to be resolved externally and the resulting
    /// address records (e.g., A and AAAA) to be returned instead of the CNAME record
    /// itself. This setting is unavailable for proxied records, since they are always
    /// flattened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_cname: Option<bool>,

    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSKEYRecordData {
    /// Algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<f64>,

    /// Flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<f64>,

    /// Protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<f64>,

    /// Public Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSKEYRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSRecordData {
    /// Algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<f64>,

    /// Digest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    /// Digest Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_type: Option<f64>,

    /// Key Tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSRecordData {
    /// Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOCRecordData {
    /// Altitude of location in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,

    /// Degrees of latitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_degrees: Option<f64>,

    /// Latitude direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_direction: Option<LOCRecordDataLatDirection>,

    /// Minutes of latitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_minutes: Option<f64>,

    /// Seconds of latitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat_seconds: Option<f64>,

    /// Degrees of longitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_degrees: Option<f64>,

    /// Longitude direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_direction: Option<LOCRecordDataLongDirection>,

    /// Minutes of longitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_minutes: Option<f64>,

    /// Seconds of longitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_seconds: Option<f64>,

    /// Horizontal precision of location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision_horz: Option<f64>,

    /// Vertical precision of location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision_vert: Option<f64>,

    /// Size of location in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOCRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MXRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NAPTRRecordData {
    /// Flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,

    /// Order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,

    /// Preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<f64>,

    /// Regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    /// Replacement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,

    /// Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NAPTRRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NSRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTRRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMIMEARecordData {
    /// Certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Matching Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_type: Option<f64>,

    /// Selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<f64>,

    /// Usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMIMEARecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRVRecordData {
    /// The port of the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<f64>,

    /// Required for MX, SRV and URI records; unused by other record types. Records with
    /// lower priorities are preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// A valid hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// The record weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRVRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHFPRecordData {
    /// Algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<f64>,

    /// Fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHFPRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVCBRecordData {
    /// Priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

    /// Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVCBRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSARecordData {
    /// Certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Matching Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_type: Option<f64>,

    /// Selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<f64>,

    /// Usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSARecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXTRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URIRecordData {
    /// The record content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// The record weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URIRecordSettings {
    /// When enabled, only A records will be generated, and AAAA records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,

    /// When enabled, only AAAA records will be generated, and A records will not be
    /// created. This setting is intended for exceptional cases. Note that this option
    /// only applies to proxied records and it has no effect on whether Cloudflare
    /// communicates with the origin using IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RecordScanTriggerResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RecordScanTriggerResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditResponseZoneDefaults {
    /// Whether to flatten all CNAME records in the zone. Note that, due to DNS
    /// limitations, a CNAME record at the zone apex will always be flattened.
    #[serde(rename = "flatten_all_cnames")]
    pub flatten_all_cnam_es: bool,

    /// Whether to enable Foundation DNS Advanced Nameservers on the zone.
    pub foundation_dns: bool,

    /// Settings for this internal zone.
    pub internal_dns: SettingAccountEditResponseZoneDefaultsInternalDNS,

    /// Whether to enable multi-provider DNS, which causes Cloudflare to activate the
    /// zone even when non-Cloudflare NS records exist, and to respect NS records at the
    /// zone apex during outbound zone transfers.
    pub multi_provider: bool,

    /// Settings determining the nameservers through which the zone should be available.
    pub nameservers: SettingAccountEditResponseZoneDefaultsNameservers,

    /// The time to live (TTL) of the zone's nameserver (NS) records.
    #[serde(rename = "ns_ttl")]
    pub nsttl: f64,

    /// Allows a Secondary DNS zone to use (proxied) override records and CNAME
    /// flattening at the zone apex.
    pub secondary_overrides: bool,

    /// Components of the zone's SOA record.
    pub soa: SettingAccountEditResponseZoneDefaultsSOA,

    /// Whether the zone mode is a regular or CDN/DNS only zone.
    pub zone_mode: SettingAccountEditResponseZoneDefaultsZoneMode,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetResponseZoneDefaults {
    /// Whether to flatten all CNAME records in the zone. Note that, due to DNS
    /// limitations, a CNAME record at the zone apex will always be flattened.
    #[serde(rename = "flatten_all_cnames")]
    pub flatten_all_cnam_es: bool,

    /// Whether to enable Foundation DNS Advanced Nameservers on the zone.
    pub foundation_dns: bool,

    /// Settings for this internal zone.
    pub internal_dns: SettingAccountGetResponseZoneDefaultsInternalDNS,

    /// Whether to enable multi-provider DNS, which causes Cloudflare to activate the
    /// zone even when non-Cloudflare NS records exist, and to respect NS records at the
    /// zone apex during outbound zone transfers.
    pub multi_provider: bool,

    /// Settings determining the nameservers through which the zone should be available.
    pub nameservers: SettingAccountGetResponseZoneDefaultsNameservers,

    /// The time to live (TTL) of the zone's nameserver (NS) records.
    #[serde(rename = "ns_ttl")]
    pub nsttl: f64,

    /// Allows a Secondary DNS zone to use (proxied) override records and CNAME
    /// flattening at the zone apex.
    pub secondary_overrides: bool,

    /// Components of the zone's SOA record.
    pub soa: SettingAccountGetResponseZoneDefaultsSOA,

    /// Whether the zone mode is a regular or CDN/DNS only zone.
    pub zone_mode: SettingAccountGetResponseZoneDefaultsZoneMode,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneEditResponseInternalDNS {
    /// The ID of the zone to fallback to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneEditResponseNameservers {
    /// Nameserver type
    pub r#type: SettingZoneEditResponseNameserversType,

    /// Configured nameserver set to be used for this zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_set: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneEditResponseSOA {
    /// Time in seconds of being unable to query the primary server after which
    /// secondary servers should stop serving the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<f64>,

    /// The time to live (TTL) for negative caching of records within the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    /// The primary nameserver, which may be used for outbound zone transfers. If null,
    /// a Cloudflare-assigned value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mname: Option<String>,

    /// Time in seconds after which secondary servers should re-check the SOA record to
    /// see if the zone has been updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<f64>,

    /// Time in seconds after which secondary servers should retry queries after the
    /// primary server was unresponsive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<f64>,

    /// The email address of the zone administrator, with the first label representing
    /// the local part of the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rname: Option<String>,

    /// The time to live (TTL) of the SOA record itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneGetResponseInternalDNS {
    /// The ID of the zone to fallback to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneGetResponseNameservers {
    /// Nameserver type
    pub r#type: SettingZoneGetResponseNameserversType,

    /// Configured nameserver set to be used for this zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_set: Option<i64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingZoneGetResponseSOA {
    /// Time in seconds of being unable to query the primary server after which
    /// secondary servers should stop serving the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<f64>,

    /// The time to live (TTL) for negative caching of records within the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    /// The primary nameserver, which may be used for outbound zone transfers. If null,
    /// a Cloudflare-assigned value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mname: Option<String>,

    /// Time in seconds after which secondary servers should re-check the SOA record to
    /// see if the zone has been updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<f64>,

    /// Time in seconds after which secondary servers should retry queries after the
    /// primary server was unresponsive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<f64>,

    /// The email address of the zone administrator, with the first label representing
    /// the local part of the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rname: Option<String>,

    /// The time to live (TTL) of the SOA record itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordScanTriggerResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditResponseZoneDefaultsInternalDNS {
    /// The ID of the zone to fallback to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditResponseZoneDefaultsNameservers {
    /// Nameserver type
    pub r#type: SettingAccountEditResponseZoneDefaultsNameserversType,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountEditResponseZoneDefaultsSOA {
    /// Time in seconds of being unable to query the primary server after which
    /// secondary servers should stop serving the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<f64>,

    /// The time to live (TTL) for negative caching of records within the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    /// The primary nameserver, which may be used for outbound zone transfers. If null,
    /// a Cloudflare-assigned value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mname: Option<String>,

    /// Time in seconds after which secondary servers should re-check the SOA record to
    /// see if the zone has been updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<f64>,

    /// Time in seconds after which secondary servers should retry queries after the
    /// primary server was unresponsive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<f64>,

    /// The email address of the zone administrator, with the first label representing
    /// the local part of the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rname: Option<String>,

    /// The time to live (TTL) of the SOA record itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetResponseZoneDefaultsInternalDNS {
    /// The ID of the zone to fallback to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetResponseZoneDefaultsNameservers {
    /// Nameserver type
    pub r#type: SettingAccountGetResponseZoneDefaultsNameserversType,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAccountGetResponseZoneDefaultsSOA {
    /// Time in seconds of being unable to query the primary server after which
    /// secondary servers should stop serving the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<f64>,

    /// The time to live (TTL) for negative caching of records within the zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ttl: Option<f64>,

    /// The primary nameserver, which may be used for outbound zone transfers. If null,
    /// a Cloudflare-assigned value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mname: Option<String>,

    /// Time in seconds after which secondary servers should re-check the SOA record to
    /// see if the zone has been updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<f64>,

    /// Time in seconds after which secondary servers should retry queries after the
    /// primary server was unresponsive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<f64>,

    /// The email address of the zone administrator, with the first label representing
    /// the local part of the email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rname: Option<String>,

    /// The time to live (TTL) of the SOA record itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<f64>,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DNSSECStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "pending-disabled")]
    PendingDisabled,
    #[serde(rename = "error")]
    Error,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ARecordType {
    #[serde(rename = "A")]
    A,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AAAARecordType {
    #[serde(rename = "AAAA")]
    AAAA,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CAARecordType {
    #[serde(rename = "CAA")]
    CAA,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CERTRecordType {
    #[serde(rename = "CERT")]
    CERT,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CNAMERecordType {
    #[serde(rename = "CNAME")]
    CNAME,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DNSKEYRecordType {
    #[serde(rename = "DNSKEY")]
    DNSKEY,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DSRecordType {
    #[serde(rename = "DS")]
    DS,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HTTPSRecordType {
    #[serde(rename = "HTTPS")]
    HTTPS,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LOCRecordType {
    #[serde(rename = "LOC")]
    LOC,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MXRecordType {
    #[serde(rename = "MX")]
    MX,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NAPTRRecordType {
    #[serde(rename = "NAPTR")]
    NAPTR,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NSRecordType {
    #[serde(rename = "NS")]
    NS,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PTRRecordType {
    #[serde(rename = "PTR")]
    PTR,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordResponseType {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "AAAA")]
    AAAA,
    #[serde(rename = "CNAME")]
    CNAME,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "NS")]
    NS,
    #[serde(rename = "OPENPGPKEY")]
    Openpgpkey,
    #[serde(rename = "PTR")]
    PTR,
    #[serde(rename = "TXT")]
    TXT,
    #[serde(rename = "CAA")]
    CAA,
    #[serde(rename = "CERT")]
    CERT,
    #[serde(rename = "DNSKEY")]
    DNSKEY,
    #[serde(rename = "DS")]
    DS,
    #[serde(rename = "HTTPS")]
    HTTPS,
    #[serde(rename = "LOC")]
    LOC,
    #[serde(rename = "NAPTR")]
    NAPTR,
    #[serde(rename = "SMIMEA")]
    SMIMEA,
    #[serde(rename = "SRV")]
    SRV,
    #[serde(rename = "SSHFP")]
    SSHFP,
    #[serde(rename = "SVCB")]
    SVCB,
    #[serde(rename = "TLSA")]
    TLSA,
    #[serde(rename = "URI")]
    URI,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SMIMEARecordType {
    #[serde(rename = "SMIMEA")]
    SMIMEA,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SRVRecordType {
    #[serde(rename = "SRV")]
    SRV,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SSHFPRecordType {
    #[serde(rename = "SSHFP")]
    SSHFP,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SVCBRecordType {
    #[serde(rename = "SVCB")]
    SVCB,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TLSARecordType {
    #[serde(rename = "TLSA")]
    TLSA,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TXTRecordType {
    #[serde(rename = "TXT")]
    TXT,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum URIRecordType {
    #[serde(rename = "URI")]
    URI,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingZoneEditResponseZoneMode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "cdn_only")]
    CDNOnly,
    #[serde(rename = "dns_only")]
    DNSOnly,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingZoneGetResponseZoneMode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "cdn_only")]
    CDNOnly,
    #[serde(rename = "dns_only")]
    DNSOnly,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ByTimeQueryTimeDelta {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "dekaminute")]
    Dekaminute,
    #[serde(rename = "minute")]
    Minute,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LOCRecordDataLatDirection {
    #[serde(rename = "N")]
    N,
    #[serde(rename = "S")]
    S,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LOCRecordDataLongDirection {
    #[serde(rename = "E")]
    E,
    #[serde(rename = "W")]
    W,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingAccountEditResponseZoneDefaultsZoneMode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "cdn_only")]
    CDNOnly,
    #[serde(rename = "dns_only")]
    DNSOnly,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingAccountGetResponseZoneDefaultsZoneMode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "cdn_only")]
    CDNOnly,
    #[serde(rename = "dns_only")]
    DNSOnly,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingZoneEditResponseNameserversType {
    #[serde(rename = "cloudflare.standard")]
    CloudflareStandard,
    #[serde(rename = "custom.account")]
    CustomAccount,
    #[serde(rename = "custom.tenant")]
    CustomTenant,
    #[serde(rename = "custom.zone")]
    CustomZone,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingZoneGetResponseNameserversType {
    #[serde(rename = "cloudflare.standard")]
    CloudflareStandard,
    #[serde(rename = "custom.account")]
    CustomAccount,
    #[serde(rename = "custom.tenant")]
    CustomTenant,
    #[serde(rename = "custom.zone")]
    CustomZone,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingAccountEditResponseZoneDefaultsNameserversType {
    #[serde(rename = "cloudflare.standard")]
    CloudflareStandard,
    #[serde(rename = "cloudflare.standard.random")]
    CloudflareStandardRandom,
    #[serde(rename = "custom.account")]
    CustomAccount,
    #[serde(rename = "custom.tenant")]
    CustomTenant,
}

#[cfg(any(feature = "full", feature = "with-dns"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingAccountGetResponseZoneDefaultsNameserversType {
    #[serde(rename = "cloudflare.standard")]
    CloudflareStandard,
    #[serde(rename = "cloudflare.standard.random")]
    CloudflareStandardRandom,
    #[serde(rename = "custom.account")]
    CustomAccount,
    #[serde(rename = "custom.tenant")]
    CustomTenant,
}
