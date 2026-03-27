#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMap {
    /// Identifier of an Address Map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If set to false, then the Address Map cannot be deleted via API. This is true
    /// for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// If set to false, then the IPs on the Address Map cannot be modified via the API.
    /// This is true for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "can_modify_ips")]
    pub can_modify_i_ps: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// If you have legacy TLS clients which do not send the TLS server name indicator,
    /// then you can specify one default SNI on the map. If Cloudflare receives a TLS
    /// handshake from a client without an SNI, it will respond with the default SNI on
    /// those IPs. The default SNI can be any valid zone or subdomain owned by the
    /// account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sni: Option<String>,

    /// An optional description field which may be used to describe the types of IPs or
    /// zones on the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the Address Map is enabled or not. Cloudflare's DNS will not respond
    /// with IP addresses on an Address Map until the map is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapNewResponse {
    /// Identifier of an Address Map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If set to false, then the Address Map cannot be deleted via API. This is true
    /// for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// If set to false, then the IPs on the Address Map cannot be modified via the API.
    /// This is true for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "can_modify_ips")]
    pub can_modify_i_ps: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// If you have legacy TLS clients which do not send the TLS server name indicator,
    /// then you can specify one default SNI on the map. If Cloudflare receives a TLS
    /// handshake from a client without an SNI, it will respond with the default SNI on
    /// those IPs. The default SNI can be any valid zone or subdomain owned by the
    /// account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sni: Option<String>,

    /// An optional description field which may be used to describe the types of IPs or
    /// zones on the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the Address Map is enabled or not. Cloudflare's DNS will not respond
    /// with IP addresses on an Address Map until the map is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The set of IPs on the Address Map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<crate::ips::IPs>,

    /// Zones and Accounts which will be assigned IPs on this Address Map. A zone
    /// membership will take priority over an account membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<AddressMapNewResponseMembership>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponse {
    pub errors: Vec<AddressMapDeleteResponseError>,

    pub messages: Vec<AddressMapDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapDeleteResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapGetResponse {
    /// Identifier of an Address Map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If set to false, then the Address Map cannot be deleted via API. This is true
    /// for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// If set to false, then the IPs on the Address Map cannot be modified via the API.
    /// This is true for Cloudflare-managed maps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "can_modify_ips")]
    pub can_modify_i_ps: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// If you have legacy TLS clients which do not send the TLS server name indicator,
    /// then you can specify one default SNI on the map. If Cloudflare receives a TLS
    /// handshake from a client without an SNI, it will respond with the default SNI on
    /// those IPs. The default SNI can be any valid zone or subdomain owned by the
    /// account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sni: Option<String>,

    /// An optional description field which may be used to describe the types of IPs or
    /// zones on the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the Address Map is enabled or not. Cloudflare's DNS will not respond
    /// with IP addresses on an Address Map until the map is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The set of IPs on the Address Map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<IPsItem>>,

    /// Zones and Accounts which will be assigned IPs on this Address Map. A zone
    /// membership will take priority over an account membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<AddressMapGetResponseMembership>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// An optional description field which may be used to describe the types of IPs or
    /// zones on the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the Address Map is enabled or not. Cloudflare's DNS will not respond
    /// with IP addresses on an Address Map until the map is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ips")]
    pub ips: Option<String>,

    /// Zones and Accounts which will be assigned IPs on this Address Map. A zone
    /// membership will take priority over an account membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapEditParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// If you have legacy TLS clients which do not send the TLS server name indicator,
    /// then you can specify one default SNI on the map. If Cloudflare receives a TLS
    /// handshake from a client without an SNI, it will respond with the default SNI on
    /// those IPs. The default SNI can be any valid zone or subdomain owned by the
    /// account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sni: Option<String>,

    /// An optional description field which may be used to describe the types of IPs or
    /// zones on the map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the Address Map is enabled or not. Cloudflare's DNS will not respond
    /// with IP addresses on an Address Map until the map is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponse {
    pub errors: Vec<AddressMapAccountUpdateResponseError>,

    pub messages: Vec<AddressMapAccountUpdateResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapAccountUpdateResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponse {
    pub errors: Vec<AddressMapAccountDeleteResponseError>,

    pub messages: Vec<AddressMapAccountDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapAccountDeleteResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponse {
    pub errors: Vec<AddressMapIPUpdateResponseError>,

    pub messages: Vec<AddressMapIPUpdateResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapIPUpdateResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponse {
    pub errors: Vec<AddressMapIPDeleteResponseError>,

    pub messages: Vec<AddressMapIPDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapIPDeleteResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponse {
    pub errors: Vec<AddressMapZoneUpdateResponseError>,

    pub messages: Vec<AddressMapZoneUpdateResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapZoneUpdateResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponse {
    pub errors: Vec<AddressMapZoneDeleteResponseError>,

    pub messages: Vec<AddressMapZoneDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<AddressMapZoneDeleteResponseResultInfo>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateParams {
    /// Identifier of a zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteParams {
    /// Identifier of a zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOADocumentNewResponse {
    /// Identifier for the uploaded LOA document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Whether the LOA has been auto-generated for the prefix owner by Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Name of LOA document. Max file size 10MB, and supported filetype is pdf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// File size of the uploaded LOA document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,

    /// Whether the LOA has been verified by Cloudflare staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// Timestamp of the moment the LOA was marked as validated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOADocumentNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// LOA document to upload.
    pub loa_document: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOADocumentGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prefix {
    /// Identifier of an IP Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Prefix advertisement status to the Internet. This field is only not 'null' if on
    /// demand is enabled.
    /// Deprecated: Prefer the
    /// [BGP Prefixes API](https://developers.cloudflare.com/api/resources/addressing/subresources/prefixes/subresources/bgp_prefixes/)
    /// instead, which allows for advertising multiple BGP routes within a single IP
    /// Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised: Option<bool>,

    /// Last time the advertisement status was changed. This field is only not 'null' if
    /// on demand is enabled.
    /// Deprecated: Prefer the
    /// [BGP Prefixes API](https://developers.cloudflare.com/api/resources/addressing/subresources/prefixes/subresources/bgp_prefixes/)
    /// instead, which allows for advertising multiple BGP routes within a single IP
    /// Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised_modified_at: Option<DateTime<Utc>>,

    /// Approval state of the prefix (P = pending, V = active).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<String>,

    /// Autonomous System Number (ASN) the prefix will be advertised under.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Whether Cloudflare is allowed to generate the LOA document on behalf of the
    /// prefix owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_loa_creation: Option<bool>,

    /// Description of the prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// State of one kind of validation for an IP prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irr_validation_state: Option<String>,

    /// Identifier for the uploaded LOA document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_document_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,

    /// Whether advertisement of the prefix to the Internet may be dynamically enabled
    /// or disabled.
    /// Deprecated: Prefer the
    /// [BGP Prefixes API](https://developers.cloudflare.com/api/resources/addressing/subresources/prefixes/subresources/bgp_prefixes/)
    /// instead, which allows for advertising multiple BGP routes within a single IP
    /// Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_enabled: Option<bool>,

    /// Whether advertisement status of the prefix is locked, meaning it cannot be
    /// changed.
    /// Deprecated: Prefer the
    /// [BGP Prefixes API](https://developers.cloudflare.com/api/resources/addressing/subresources/prefixes/subresources/bgp_prefixes/)
    /// instead, which allows for advertising multiple BGP routes within a single IP
    /// Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_locked: Option<bool>,

    /// State of one kind of validation for an IP prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_validation_state: Option<String>,

    /// Token provided to demonstrate ownership of the prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_validation_token: Option<String>,

    /// State of one kind of validation for an IP prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpki_validation_state: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteResponse {
    pub errors: Vec<PrefixDeleteResponseError>,

    pub messages: Vec<PrefixDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Autonomous System Number (ASN) the prefix will be advertised under.
    pub asn: String,

    /// IP Prefix in Classless Inter-Domain Routing format.
    pub cidr: String,

    /// Whether Cloudflare is allowed to generate the LOA document on behalf of the
    /// prefix owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_loa_creation: Option<String>,

    /// Description of the prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Identifier for the uploaded LOA document.
    /// Deprecated: The LOA API is deprecated and will be removed in a future release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_document_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixEditParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Description of the prefix.
    pub description: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixAdvertisementStatusEditResponse {
    /// Advertisement status of the prefix. If `true`, the BGP route for the prefix is
    /// advertised to the Internet. If `false`, the BGP route is withdrawn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised: Option<bool>,

    /// Last time the advertisement status was changed. This field is only not 'null' if
    /// on demand is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised_modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixAdvertisementStatusGetResponse {
    /// Advertisement status of the prefix. If `true`, the BGP route for the prefix is
    /// advertised to the Internet. If `false`, the BGP route is withdrawn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised: Option<bool>,

    /// Last time the advertisement status was changed. This field is only not 'null' if
    /// on demand is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised_modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixAdvertisementStatusEditParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Advertisement status of the prefix. If `true`, the BGP route for the prefix is
    /// advertised to the Internet. If `false`, the BGP route is withdrawn.
    pub advertised: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixAdvertisementStatusGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BGPPrefix {
    /// Identifier of BGP Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Autonomous System Number (ASN) the prefix will be advertised under.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,

    /// Number of times to prepend the Cloudflare ASN to the BGP AS-Path attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_prepend_count: Option<i64>,

    /// Determines if Cloudflare advertises a BYOIP BGP prefix even when there is no
    /// matching BGP prefix in the Magic routing table. When true, Cloudflare will
    /// automatically withdraw the BGP prefix when there are no matching BGP routes, and
    /// will resume advertising when there is at least one matching BGP route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advertise_withdraw: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_signal_opts: Option<BGPPrefixBGPSignalOpts>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<BGPPrefixOnDemand>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixBGPPrefixNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    pub cidr: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixBGPPrefixListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixBGPPrefixEditParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Number of times to prepend the Cloudflare ASN to the BGP AS-Path attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_prepend_count: Option<String>,

    /// Determines if Cloudflare advertises a BYOIP BGP prefix even when there is no
    /// matching BGP prefix in the Magic routing table. When true, Cloudflare will
    /// automatically withdraw the BGP prefix when there are no matching BGP routes, and
    /// will resume advertising when there is at least one matching BGP route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advertise_withdraw: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixBGPPrefixGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegations {
    /// Identifier of a Delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Account identifier for the account to which prefix is being delegated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,

    /// Identifier of an IP Prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_prefix_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDelegationDeleteResponse {
    /// Identifier of a Delegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDelegationNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    pub cidr: String,

    /// Account identifier for the account to which prefix is being delegated.
    pub delegated_account_id: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDelegationListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDelegationDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBinding {
    /// Identifier of a Service Binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,

    /// Status of a Service Binding's deployment to the Cloudflare network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<ServiceBindingProvisioning>,

    /// Identifier of a Service on the Cloudflare network. Available services and their
    /// IDs may be found in the **List Services** endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,

    /// Name of a service running on the Cloudflare network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteResponse {
    pub errors: Vec<PrefixServiceBindingDeleteResponseError>,

    pub messages: Vec<PrefixServiceBindingDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingNewParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// IP Prefix in Classless Inter-Domain Routing format.
    pub cidr: String,

    /// Identifier of a Service on the Cloudflare network. Available services and their
    /// IDs may be found in the **List Services** endpoint.
    pub service_id: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingGetParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameNewResponse {
    /// When the regional hostname was created
    pub created_on: DateTime<Utc>,

    /// DNS hostname to be regionalized, must be a subdomain of the zone. Wildcards are
    /// supported for one level, e.g `*.example.com`
    pub hostname: String,

    /// Identifying key for the region
    pub region_key: String,

    /// Configure which routing method to use for the regional hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameListResponse {
    /// When the regional hostname was created
    pub created_on: DateTime<Utc>,

    /// DNS hostname to be regionalized, must be a subdomain of the zone. Wildcards are
    /// supported for one level, e.g `*.example.com`
    pub hostname: String,

    /// Identifying key for the region
    pub region_key: String,

    /// Configure which routing method to use for the regional hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteResponse {
    pub errors: Vec<RegionalHostnameDeleteResponseError>,

    pub messages: Vec<RegionalHostnameDeleteResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameEditResponse {
    /// When the regional hostname was created
    pub created_on: DateTime<Utc>,

    /// DNS hostname to be regionalized, must be a subdomain of the zone. Wildcards are
    /// supported for one level, e.g `*.example.com`
    pub hostname: String,

    /// Identifying key for the region
    pub region_key: String,

    /// Configure which routing method to use for the regional hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameGetResponse {
    /// When the regional hostname was created
    pub created_on: DateTime<Utc>,

    /// DNS hostname to be regionalized, must be a subdomain of the zone. Wildcards are
    /// supported for one level, e.g `*.example.com`
    pub hostname: String,

    /// Identifying key for the region
    pub region_key: String,

    /// Configure which routing method to use for the regional hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// DNS hostname to be regionalized, must be a subdomain of the zone. Wildcards are
    /// supported for one level, e.g `*.example.com`
    pub hostname: String,

    /// Identifying key for the region
    pub region_key: String,

    /// Configure which routing method to use for the regional hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Identifying key for the region
    pub region_key: String,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameRegionListResponse {
    /// Identifying key for the region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Human-readable text label for the region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameRegionListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceListResponse {
    /// Identifier of a Service on the Cloudflare network. Available services and their
    /// IDs may be found in the **List Services** endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of a service running on the Cloudflare network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceListParams {
    /// Identifier of a Cloudflare account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapNewResponseMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The identifier for the membership (eg. a zone or account tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// The type of the membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapGetResponseMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The identifier for the membership (eg. a zone or account tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// The type of the membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapAccountUpdateResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapAccountUpdateResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapAccountDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapAccountDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapIPUpdateResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapIPUpdateResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapIPDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapIPDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapZoneUpdateResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapZoneUpdateResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapZoneDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressMapZoneDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponseResultInfo {
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

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PrefixDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PrefixDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BGPPrefixBGPSignalOpts {
    /// Whether control of advertisement of the prefix to the Internet is enabled to be
    /// performed via BGP signal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Last time BGP signaling control was toggled. This field is null if BGP signaling
    /// has never been enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BGPPrefixOnDemand {
    /// Prefix advertisement status to the Internet. This field is only not 'null' if on
    /// demand is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised: Option<bool>,

    /// Last time the advertisement status was changed. This field is only not 'null' if
    /// on demand is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertised_modified_at: Option<DateTime<Utc>>,

    /// Whether advertisement of the prefix to the Internet may be dynamically enabled
    /// or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_enabled: Option<bool>,

    /// Whether the advertisement status of the prefix is locked, meaning it cannot be
    /// changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_locked: Option<bool>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBindingProvisioning {
    /// When a binding has been deployed to a majority of Cloudflare datacenters, the
    /// binding will become active and can be used with its associated service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceBindingProvisioningState>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PrefixServiceBindingDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PrefixServiceBindingDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RegionalHostnameDeleteResponseErrorsSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RegionalHostnameDeleteResponseMessagesSource>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountUpdateResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapAccountDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPUpdateResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapIPDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneUpdateResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMapZoneDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixServiceBindingDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalHostnameDeleteResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
    #[serde(rename = "zone")]
    KindZone,
    #[serde(rename = "account")]
    KindAccount,
}

#[cfg(any(feature = "full", feature = "with-addressing"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceBindingProvisioningState {
    #[serde(rename = "provisioning")]
    ServiceBindingProvisioningStateProvisioning,
    #[serde(rename = "active")]
    ServiceBindingProvisioningStateActive,
}
