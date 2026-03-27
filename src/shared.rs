#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{DateTime, Duration, Utc};


pub type UnionInt = i64;
pub type UnionString = String;

pub type ASN = i64;
pub type ASNParam = i64;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// A string that uniquely identifies the audit log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AuditLogAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<AuditLogActor>,

    /// The source of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,

    /// An object which can lend more context to the action being logged. This is a
    /// flexible value and varies between different actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,

    /// The new value of the resource that was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newValue")]
    pub new_value: Option<String>,

    /// The value of the resource before it was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oldValue")]
    pub old_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<AuditLogOwner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AuditLogResource>,

    /// A UTC RFC3339 timestamp that specifies when the action being logged occured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogAction {
    /// A boolean that indicates if the action attempted was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,

    /// A short string that describes the action that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogActor {
    /// The ID of the actor that performed the action. If a user performed the action,
    /// this will be their User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The email of the user that performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The IP address of the request that performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The type of actor, whether a User, Cloudflare Admin, or an Automated System.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AuditLogActorType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogOwner {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogResource {
    /// An identifier for the resource that was affected by the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A short string that describes the resource that was affected by the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareTunnel {
    /// UUID of the tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Cloudflare account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tag: Option<String>,

    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage
    /// the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the
    /// tunnel on the Zero Trust dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_src: Option<CloudflareTunnelConfigSrc>,

    /// The Cloudflare Tunnel connections between your origin and Cloudflare's edge.
    /// Deprecated: This field will start returning an empty array. To fetch the
    /// connections of a given tunnel, please use the dedicated endpoint
    /// `/accounts/{account_id}/{tunnel_type}/{tunnel_id}/connections`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<CloudflareTunnelConnection>>,

    /// Timestamp of when the tunnel established at least one connection to Cloudflare's
    /// edge. If `null`, the tunnel is inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns_active_at: Option<DateTime<Utc>>,

    /// Timestamp of when the tunnel became inactive (no connections to Cloudflare's
    /// edge). If `null`, the tunnel is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns_inactive_at: Option<DateTime<Utc>>,

    /// Timestamp of when the resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Timestamp of when the resource was deleted. If `null`, the resource has not been
    /// deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,

    /// Metadata associated with the tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,

    /// A user-friendly name for a tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If `true`, the tunnel can be configured remotely from the Zero Trust dashboard.
    /// If `false`, the tunnel must be configured locally on the origin machine.
    /// Deprecated: Use the config_src field instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_config: Option<bool>,

    /// The status of the tunnel. Valid values are `inactive` (tunnel has never been
    /// run), `degraded` (tunnel is active and able to serve traffic but in an unhealthy
    /// state), `healthy` (tunnel is active and able to serve traffic), or `down`
    /// (tunnel can not serve traffic as it has no connections to the Cloudflare Edge).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CloudflareTunnelStatus>,

    /// The type of tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tun_type: Option<CloudflareTunnelTunType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareTunnelConnection {
    /// UUID of the Cloudflare Tunnel connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// UUID of the Cloudflare Tunnel connector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// The cloudflared version used to establish this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,

    /// The Cloudflare data center used for this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colo_name: Option<String>,

    /// Cloudflare continues to track connections for several minutes after they
    /// disconnect. This is an optimization to improve latency and reliability of
    /// reconnecting. If `true`, the connection has disconnected but is still being
    /// tracked. If `false`, the connection is actively serving traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pending_reconnect: Option<bool>,

    /// Timestamp of when the connection was established.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<DateTime<Utc>>,

    /// The public IP address of the host running cloudflared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ip: Option<String>,

    /// UUID of the Cloudflare Tunnel connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorDataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDataSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// Membership identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The contact email address of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Access policy for the membership
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<MemberPolicy>>,

    /// Roles assigned to this Member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,

    /// A member's status in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemberStatus>,

    /// Details of the user associated to the membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MemberUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPolicy {
    /// Policy identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Allow or deny operations against the resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<MemberPoliciesAccess>,

    /// A set of permission groups that are specified to the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_groups: Option<Vec<MemberPoliciesPermissionGroup>>,

    /// A list of resource groups that the policy applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<Vec<MemberPoliciesResourceGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesPermissionGroup {
    /// Identifier of the permission group.
    pub id: String,

    /// Attributes associated to the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MemberPoliciesPermissionGroupsMeta>,

    /// Name of the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesPermissionGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesResourceGroup {
    /// Identifier of the resource group.
    pub id: String,

    /// The scope associated to the resource group
    pub scope: Vec<MemberPoliciesResourceGroupsScope>,

    /// Attributes associated to the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MemberPoliciesResourceGroupsMeta>,

    /// Name of the resource group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesResourceGroupsScope {
    /// This is a combination of pre-defined resource name and identifier (like Account
    /// ID etc.)
    pub key: String,

    /// A list of scope objects for additional context.
    pub objects: Vec<MemberPoliciesResourceGroupsScopeObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesResourceGroupsScopeObject {
    /// This is a combination of pre-defined resource name and identifier (like Zone ID
    /// etc.)
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPoliciesResourceGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUser {
    /// The contact email address of the user.
    pub email: String,

    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// User's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// User's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Indicates whether two-factor authentication is enabled for the user account.
    /// Does not apply to API authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGrant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGrantParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlan {
    /// The ID of the rate plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RatePlanID>,

    /// The currency applied to the rate plan subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Whether this rate plan is managed externally from Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,

    /// Whether a rate plan is enterprise-based (or newly adopted term contract).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract: Option<bool>,

    /// The full name of the rate plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,

    /// The scope that this rate plan applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// The list of sets this rate plan applies to. Returns array of strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanParam {
    /// The ID of the rate plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RatePlanID>,

    /// The currency applied to the rate plan subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Whether this rate plan is managed externally from Cloudflare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,

    /// Whether a rate plan is enterprise-based (or newly adopted term contract).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract: Option<bool>,

    /// The full name of the rate plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,

    /// The scope that this rate plan applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// The list of sets this rate plan applies to. Returns array of strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseInfo {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ResponseInfoSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseInfoSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role identifier tag.
    pub id: String,

    /// Description of role's permissions.
    pub description: String,

    /// Role name.
    pub name: String,

    pub permissions: RolePermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_purge: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_settings: Option<PermissionGrant>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<PermissionGrant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleParam {
    /// Role identifier tag.
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermissionsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_purge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_settings: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The monetary unit in which pricing information is displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The end of the current period and also when the next billing is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<DateTime<Utc>>,

    /// When the current billing period started. May match initial_period_start if this
    /// is the first period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<DateTime<Utc>>,

    /// How often the subscription is renewed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SubscriptionFrequency>,

    /// The price of the subscription that will be billed, in US dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// The rate plan applied to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_plan: Option<RatePlan>,

    /// The state that the subscription is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SubscriptionState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionParam {
    /// How often the subscription is renewed automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<SubscriptionFrequency>,

    /// The rate plan applied to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_plan: Option<RatePlanParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token<T> {
    /// Token identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenCondition>,

    /// The expiration time on or after which the JWT MUST NOT be accepted for
    /// processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// The time on which the token was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<DateTime<Utc>>,

    /// Last time the token was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_on: Option<DateTime<Utc>>,

    /// Last time the token was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// Token name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The time before which the token MUST NOT be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<DateTime<Utc>>,

    /// List of access policies assigned to the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<TokenPolicy<T>>>,

    /// Status of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenStatus>,
}

pub type TokenValue = String;
pub type TokenConditionCIDRList = String;
pub type TokenConditionCIDRListParam = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicy<T> {
    /// Policy identifier.
    pub id: String,
    /// Allow or deny operations against the resources.
    pub effect: TokenPolicyEffect,
    /// A set of permission groups that are specified to the policy.
    pub permission_groups: Vec<TokenPolicyPermissionGroup>,
    /// A list of resource names that the policy applies to.
    /// [TokenPolicyResourcesIAMResourcesTypeObjectString] or
    /// [TokenPolicyResourcesIAMResourcesTypeObjectNested]
    pub resources: T,
}

pub type TokenPolicyResourcesIAMResourcesTypeObjectString =
    ::std::collections::HashMap<String, String>;

pub type TokenPolicyResourcesIAMResourcesTypeObjectNested =
    ::std::collections::HashMap<String, TokenPolicyResourcesIAMResourcesTypeObjectString>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCondition {
    /// Client IP restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ip: Option<TokenConditionRequestIP>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConditionRequestIP {
    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<String>>,

    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenConditionParam>,

    /// The expiration time on or after which the JWT MUST NOT be accepted for
    /// processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// Token name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The time before which the token MUST NOT be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<DateTime<Utc>>,

    /// List of access policies assigned to the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<TokenPolicyParam>>,

    /// Status of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConditionParam {
    /// Client IP restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ip: Option<TokenConditionRequestIPParam>,
}

/// Client IP restrictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConditionRequestIPParam {
    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<String>,

    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicyPermissionGroup {
    /// Identifier of the permission group.
    pub id: String,

    /// Attributes associated to the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<TokenPolicyPermissionGroupsMeta>,

    /// Name of the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicyPermissionGroupsMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicyParam {
    /// Allow or deny operations against the resources.
    pub effect: String,

    /// A set of permission groups that are specified to the policy.
    pub permission_groups: String,

    /// A list of resource names that the policy applies to.
    pub resources: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicyPermissionGroupParam {
    /// Identifier of the permission group.
    pub id: String,

    /// Attributes associated to the permission group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPolicyPermissionGroupsMetaParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLogActorType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "Cloudflare")]
    Cloudflare,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateCA {
    #[serde(rename = "digicert")]
    Digicert,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
    #[serde(rename = "ssl_com")]
    SSLCom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateRequestType {
    #[serde(rename = "origin-rsa")]
    OriginRSA,
    #[serde(rename = "origin-ecc")]
    OriginECC,
    #[serde(rename = "keyless-certificate")]
    KeylessCertificate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudflareTunnelConfigSrc {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "cloudflare")]
    Cloudflare,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudflareTunnelStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "down")]
    Down,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudflareTunnelTunType {
    #[serde(rename = "cfd_tunnel")]
    CfdTunnel,
    #[serde(rename = "warp_connector")]
    WARPConnector,
    #[serde(rename = "warp")]
    WARP,
    #[serde(rename = "magic")]
    Magic,
    #[serde(rename = "ip_sec")]
    IPSec,
    #[serde(rename = "gre")]
    GRE,
    #[serde(rename = "cni")]
    CNI,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberPoliciesAccess {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
}

/// The ID of the rate plan.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RatePlanID {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "lite")]
    Lite,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "pro_plus")]
    ProPlus,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "enterprise")]
    Enterprise,
    #[serde(rename = "partners_free")]
    PartnersFree,
    #[serde(rename = "partners_pro")]
    PartnersPro,
    #[serde(rename = "partners_business")]
    PartnersBusiness,
    #[serde(rename = "partners_enterprise")]
    PartnersEnterprise,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// How often the subscription is renewed automatically.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionFrequency {
    /// value is "weekly"
    #[serde(rename = "weekly")]
    Weekly,
    /// value is "monthly"
    #[serde(rename = "monthly")]
    Monthly,
    /// value is "quarterly"
    #[serde(rename = "quarterly")]
    Quarterly,
    /// value is "yearly"
    #[serde(rename = "yearly")]
    Yearly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionState {
    /// value is "Trial"
    #[serde(rename = "Trial")]
    Trial,
    /// value is "Provisioned"
    #[serde(rename = "Provisioned")]
    Provisioned,
    /// value is "Paid"
    #[serde(rename = "Paid")]
    Paid,
    /// value is "AwaitingPayment"
    #[serde(rename = "AwaitingPayment")]
    AwaitingPayment,
    /// value is "Cancelled"
    #[serde(rename = "Cancelled")]
    Cancelled,
    /// value is "Failed"
    #[serde(rename = "Failed")]
    Failed,
    /// value is "Expired"
    #[serde(rename = "Expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenPolicyEffect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}
