#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    /// Membership identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::accounts::Account>,

    /// Enterprise only. Indicates whether or not API access is enabled specifically for
    /// this user on a given account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_access_enabled: Option<bool>,

    /// All access permissions for the user at the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MembershipPermissions>,

    /// List of role names the membership has for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Status of this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MembershipStatus>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponse {
    /// Membership identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::accounts::Account>,

    /// Enterprise only. Indicates whether or not API access is enabled specifically for
    /// this user on a given account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_access_enabled: Option<bool>,

    /// All access permissions for the user at the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MembershipUpdateResponsePermissions>,

    /// Access policy for the membership
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<MembershipUpdateResponsePolicy>>,

    /// List of role names the membership has for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Status of this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MembershipUpdateResponseStatus>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipDeleteResponse {
    /// Membership identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponse {
    /// Membership identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::accounts::Account>,

    /// Enterprise only. Indicates whether or not API access is enabled specifically for
    /// this user on a given account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_access_enabled: Option<bool>,

    /// All access permissions for the user at the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MembershipGetResponsePermissions>,

    /// Access policy for the membership
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<MembershipGetResponsePolicy>>,

    /// List of role names the membership has for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Status of this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MembershipGetResponseStatus>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateParams {
    /// Whether to accept or reject this account invitation.
    pub status: String,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Direction to order memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to order memberships by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of memberships per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Status of this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_purge: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_settings: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<crate::shared::PermissionGrant>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_purge: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_settings: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<crate::shared::PermissionGrant>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePolicy {
    /// Policy identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Allow or deny operations against the resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<MembershipUpdateResponsePoliciesAccess>,

    /// A set of permission groups that are specified to the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_groups: Option<Vec<MembershipUpdateResponsePoliciesPermissionGroup>>,

    /// A list of resource groups that the policy applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<Vec<MembershipUpdateResponsePoliciesResourceGroup>>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_purge: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_settings: Option<crate::shared::PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<crate::shared::PermissionGrant>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePolicy {
    /// Policy identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Allow or deny operations against the resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<MembershipGetResponsePoliciesAccess>,

    /// A set of permission groups that are specified to the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_groups: Option<Vec<MembershipGetResponsePoliciesPermissionGroup>>,

    /// A list of resource groups that the policy applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<Vec<MembershipGetResponsePoliciesResourceGroup>>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesPermissionGroup {
    /// Identifier of the permission group.
    pub id: String,

    /// Attributes associated to the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MembershipUpdateResponsePoliciesPermissionGroupsMeta>,

    /// Name of the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesResourceGroup {
    /// Identifier of the resource group.
    pub id: String,

    /// The scope associated to the resource group
    pub scope: Vec<MembershipUpdateResponsePoliciesResourceGroupsScope>,

    /// Attributes associated to the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MembershipUpdateResponsePoliciesResourceGroupsMeta>,

    /// Name of the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesPermissionGroup {
    /// Identifier of the permission group.
    pub id: String,

    /// Attributes associated to the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MembershipGetResponsePoliciesPermissionGroupsMeta>,

    /// Name of the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesResourceGroup {
    /// Identifier of the resource group.
    pub id: String,

    /// The scope associated to the resource group
    pub scope: Vec<MembershipGetResponsePoliciesResourceGroupsScope>,

    /// Attributes associated to the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MembershipGetResponsePoliciesResourceGroupsMeta>,

    /// Name of the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesPermissionGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesResourceGroupsScope {
    /// This is a combination of pre-defined resource name and identifier (like Account
    /// ID etc.)
    pub key: String,

    /// A list of scope objects for additional context.
    pub objects: Vec<MembershipUpdateResponsePoliciesResourceGroupsScopeObject>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesResourceGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesPermissionGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesResourceGroupsScope {
    /// This is a combination of pre-defined resource name and identifier (like Account
    /// ID etc.)
    pub key: String,

    /// A list of scope objects for additional context.
    pub objects: Vec<MembershipGetResponsePoliciesResourceGroupsScopeObject>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesResourceGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipUpdateResponsePoliciesResourceGroupsScopeObject {
    /// This is a combination of pre-defined resource name and identifier (like Zone ID
    /// etc.)
    pub key: String,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipGetResponsePoliciesResourceGroupsScopeObject {
    /// This is a combination of pre-defined resource name and identifier (like Zone ID
    /// etc.)
    pub key: String,

}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipUpdateResponseStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipGetResponseStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipUpdateResponsePoliciesAccess {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

#[cfg(any(feature = "full", feature = "with-memberships"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipGetResponsePoliciesAccess {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

