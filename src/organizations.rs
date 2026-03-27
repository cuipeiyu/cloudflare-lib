#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMember {
    /// Organization Member ID
    pub id: String,

    pub create_time: DateTime<Utc>,

    pub meta: ::std::collections::HashMap<String, String>,

    pub status: OrganizationMemberStatus,

    pub update_time: DateTime<Utc>,

    pub user: OrganizationMemberUser,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParams {
    pub member: String,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListParams {
    /// The amount of items to return. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,

    /// An opaque token returned from the last list response that when provided will
    /// retrieve the next page.
    /// Parameters used to filter the retrieved list must remain in subsequent requests
    /// with a page token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// Filter the list of memberships by membership status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,

    pub create_time: DateTime<Utc>,

    pub meta: OrganizationMeta,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<OrganizationParent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::accounts::AccountProfile>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationParam {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDeleteResponse {
    pub id: String,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationNewParams {
    /// References an Organization in the Cloudflare data model.
    pub organization: OrganizationParam,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationUpdateParams {
    /// References an Organization in the Cloudflare data model.
    pub organization: OrganizationParam,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationListParams {
    /// Only return organizations with the specified IDs (ex. id=foo&id=bar). Send
    /// multiple elements by repeating the query value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub containing: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The amount of items to return. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,

    /// An opaque token returned from the last list response that when provided will
    /// retrieve the next page.
    /// Parameters used to filter the retrieved list must remain in subsequent requests
    /// with a page token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAccountGetResponse {
    pub id: String,

    pub created_on: DateTime<Utc>,

    pub name: String,

    pub settings: OrganizationAccountGetResponseSettings,

    pub r#type: OrganizationAccountGetResponseType,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAccountGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_pubname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The amount of items to return. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,

    /// An opaque token returned from the last list response that when provided will
    /// retrieve the next page.
    /// Parameters used to filter the retrieved list must remain in subsequent requests
    /// with a page token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationProfileUpdateParams {
    pub account_profile: crate::accounts::AccountProfileParam,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMemberUser {
    pub id: String,

    pub email: String,

    pub name: String,

    pub two_factor_authentication_enabled: bool,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMeta {
    /// Enable features for Organizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<OrganizationMetaFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "-,extras")]
    pub extra_fields: Option<::std::collections::HashMap<String, String>>,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationParent {
    pub id: String,

    pub name: String,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAccountGetResponseSettings {
    pub abuse_contact_email: String,

    pub access_approval_expiry: DateTime<Utc>,

    pub api_access_enabled: bool,

    /// Use
    /// [DNS Settings](https://developers.cloudflare.com/api/operations/dns-settings-for-an-account-list-dns-settings)
    /// instead. Deprecated.
    /// Deprecated: deprecated
    pub default_nameservers: String,

    pub enforce_twofactor: bool,

    /// Use
    /// [DNS Settings](https://developers.cloudflare.com/api/operations/dns-settings-for-an-account-list-dns-settings)
    /// instead. Deprecated.
    /// Deprecated: deprecated
    pub use_account_custom_ns_by_default: bool,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMetaFlags {
    pub account_creation: String,

    pub account_deletion: String,

    pub account_migration: String,

    pub account_mobility: String,

    pub sub_org_creation: String,

}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrganizationMemberStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
}


#[cfg(any(feature = "full", feature = "with-organizations"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrganizationAccountGetResponseType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "enterprise")]
    Enterprise,
}

