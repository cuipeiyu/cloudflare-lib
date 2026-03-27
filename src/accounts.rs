#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-accounts"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Identifier
    pub id: String,

    /// Account name
    pub name: String,

    pub r#type: AccountType,

    /// Timestamp for the creation of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Parent container details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<AccountManagedBy>,

    /// Account settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdateBody {
    /// Identifier
    pub id: String,

    /// Account name
    pub name: String,

    pub r#type: AccountType,

    /// Parent container details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<AccountManagedByParam>,

    /// Account settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParam>,
}

/// Parent container details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountManagedByParam {}

/// Account settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingsParam {
    // Sets an abuse contact email to notify for abuse reports.
    pub abuse_contact_email: Option<String>,
    // Indicates whether membership in this account requires that Two-Factor
    // Authentication is enabled
    pub enforce_twofactor: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeleteResponse {
    /// Identifier
    pub id: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AccountUpdateParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,

//     pub account: AccountParam,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountListQuery {
    /// [query] Direction to order results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<AccountListQueryDirection>,

    /// [query] Name of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// [query] Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// [query] Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

pub type AccountListQueryDirection = crate::shared::SortDirection;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AccountDeleteParams {
//     /// [path] The account ID of the account to be deleted
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOrganizationNewResponse {
    pub account_id: String,

    pub destination_organization_id: String,

    pub source_organization_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOrganizationMoveBody {
    pub destination_organization_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfile {
    pub business_address: String,

    pub business_email: String,

    pub business_name: String,

    pub business_phone: String,

    pub external_metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfileParam {
    pub business_address: String,

    pub business_email: String,

    pub business_name: String,

    pub business_phone: String,

    pub external_metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfileUpdateParams {
    #[serde(skip)]
    pub account_id: String,

    pub account_profile: AccountProfileParam,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct AccountProfileGetParams {
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponse {
    /// A unique identifier for the audit log entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Contains account related information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<LogAuditListResponseAccount>,

    /// Provides information about the action performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LogAuditListResponseAction>,

    /// Provides details about the actor who performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<LogAuditListResponseActor>,

    /// Provides raw information about the request and response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<LogAuditListResponseRaw>,

    /// Provides details about the affected resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<LogAuditListResponseResource>,

    /// Provides details about the zone affected by the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<LogAuditListResponseZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParams {
    /// [query] Limits the returned results to logs older than the specified date. This can be a
    /// date string 2019-04-30 (interpreted in UTC) or an absolute timestamp that
    /// conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<Utc>>,

    /// [query] Limits the returned results to logs newer than the specified date. This can be a
    /// date string 2019-04-30 (interpreted in UTC) or an absolute timestamp that
    /// conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LogAuditListParamsID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<LogAuditListParamsAccountName>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_result: Option<LogAuditListParamsActionResult>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<LogAuditListParamsActionType>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_context: Option<LogAuditListParamsActorContext>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_email: Option<LogAuditListParamsActorEmail>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<LogAuditListParamsActorID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ip_address: Option<LogAuditListParamsActorIPAddress>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_token_id: Option<LogAuditListParamsActorTokenID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_token_name: Option<LogAuditListParamsActorTokenName>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<LogAuditListParamsActorType>,

    /// Deprecated: deprecated in favor of ID. Use ID to query by the audit log ID.
    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_id: Option<LogAuditListParamsAuditLogID>,

    /// [query] The cursor is an opaque token used to paginate through large sets of records. It
    /// indicates the position from which to continue when requesting the next set of
    /// records. A valid cursor value can be obtained from the cursor object in the
    /// result_info structure of a previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// [query] Sets sorting order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<LogAuditListParamsDirection>,

    /// [query] The number limits the objects to return. The cursor attribute may be used to
    /// iterate over the next batch of objects if there are more than the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_cf_ray_id: Option<LogAuditListParamsRawCfRayID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_method: Option<LogAuditListParamsRawMethod>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_status_code: Option<LogAuditListParamsRawStatusCode>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_uri: Option<LogAuditListParamsRawURI>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<LogAuditListParamsResourceID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_product: Option<LogAuditListParamsResourceProduct>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scope: Option<LogAuditListParamsResourceScope>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<LogAuditListParamsResourceType>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<LogAuditListParamsZoneID>,

    /// [query]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<LogAuditListParamsZoneName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsID {
    /// Filters out audit logs by their IDs.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsAccountName {
    /// Filters out audit logs by the account name.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActionResult {
    /// Filters out audit logs by whether the action was successful or not.
    pub not: Option<Vec<LogAuditListParamsActionResultNot>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActionType {
    /// Filters out audit logs by the action type.
    pub not: Option<Vec<LogAuditListParamsActionTypeNot>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorContext {
    /// Filters out audit logs by the actor context.
    pub not: Option<Vec<LogAuditListParamsActorContextNot>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorEmail {
    /// Filters out audit logs by the actor's email address.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorID {
    /// Filters out audit logs by the actor ID. This can be either the Account ID or
    /// User ID.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorIPAddress {
    /// Filters out audit logs IP address where the action was initiated.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorTokenID {
    /// Filters out audit logs by the API token ID when the actor context is an
    /// api_token or oauth.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorTokenName {
    /// Filters out audit logs by the API token name when the actor context is an
    /// api_token or oauth.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsActorType {
    /// Filters out audit logs by the actor type.
    pub not: Option<Vec<LogAuditListParamsActorTypeNot>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListParamsActorTypeNot {
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "cloudflare_admin")]
    CloudflareAdmin,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

/// Deprecated: deprecated in favor of ID. Use ID to query by the audit log ID.
#[deprecated]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsAuditLogID {
    /// Filters out audit logs by their IDs.
    pub not: Option<Vec<String>>,
}

type LogAuditListParamsDirection = crate::shared::SortDirection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsRawCfRayID {
    /// Filters out audit logs by the response CF Ray ID.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsRawMethod {
    /// Filters out audit logs by the HTTP method for the API call.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsRawStatusCode {
    /// Filters out audit logs by the response status code that was returned.
    pub not: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsRawURI {
    /// Filters out audit logs by the request URI.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsResourceID {
    /// Filters out audit logs by the resource ID.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsResourceProduct {
    /// Filters out audit logs by the Cloudflare product associated with the changed
    /// resource.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsResourceScope {
    /// Filters out audit logs by the resource scope, specifying whether the resource is
    /// associated with an user, an account, or a zone.
    pub not: Option<Vec<LogAuditListParamsResourceScopeNot>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListParamsResourceScopeNot {
    #[serde(rename = "accounts")]
    Accounts,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "zones")]
    Zones,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsResourceType {
    /// Filters out audit logs based on the unique type of resource changed by the
    /// action.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsZoneID {
    /// Filters out audit logs by the zone ID.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListParamsZoneName {
    /// Filters out audit logs by the zone name associated with the change.
    pub not: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListParamsActionResultNot {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListParamsActionTypeNot {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "update")]
    Update,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListParamsActorContextNot {
    #[serde(rename = "api_key")]
    APIKey,
    #[serde(rename = "api_token")]
    APIToken,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "oauth")]
    OAuth,
    #[serde(rename = "origin_ca_key")]
    OriginCAKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDeleteResponse {
    /// Identifier
    pub id: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberNewParams<T> {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,

//     /// Satisfied by [MemberNewParamsBodyIAMCreateMemberWithRoles],
//     /// [MemberNewParamsBodyIAMCreateMemberWithPolicies],
//     /// [MemberNewParamsBody].
//     pub body: T,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBody<P, R> {
	/// The contact email address of the user.
	pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<R>,
	/// Status of the member invitation. If not provided during creation, defaults to
	/// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
	/// of the member resource in Terraform.
    #[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<MemberNewParamsBodyStatus>,
}

/// Status of the member invitation. If not provided during creation, defaults to
/// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
/// of the member resource in Terraform.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberNewParamsBodyStatus {
    /// value is "accepted"
    #[serde(rename = "accepted")]
    Accepted,
    /// value is "pending"
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBodyIAMCreateMemberWithRoles {
    /// The contact email address of the user.
    pub email: String,
    /// Array of roles associated with this member.
    pub roles: Vec<String>,
    /// Status of the member invitation. If not provided during creation, defaults to
    /// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
    /// of the member resource in Terraform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemberNewParamsBodyIAMCreateMemberWithRolesStatus>,
}

/// Status of the member invitation. If not provided during creation, defaults to
/// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
/// of the member resource in Terraform.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberNewParamsBodyIAMCreateMemberWithRolesStatus {
    /// value is "accepted"
    #[serde(rename = "accepted")]
    Accepted,
    /// value is "pending"
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBodyIAMCreateMemberWithPolicies {
    /// The contact email address of the user.
    pub email: String,
    /// Array of policies associated with this member.
    pub policies: Vec<MemberNewParamsBodyIAMCreateMemberWithPoliciesPolicy>,
    /// Status of the member invitation. If not provided during creation, defaults to
    /// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
    /// of the member resource in Terraform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemberNewParamsBodyIAMCreateMemberWithPoliciesStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBodyIAMCreateMemberWithPoliciesPolicy {
    /// Allow or deny operations against the resources.
    pub access: MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesAccess,
    /// A set of permission groups that are specified to the policy.
    pub permission_groups:
        Vec<MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesPermissionGroup>,
    /// A list of resource groups that the policy applies to.
    pub resource_groups:
        Vec<MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesResourceGroup>,
}

/// Allow or deny operations against the resources.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesAccess {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

/// Status of the member invitation. If not provided during creation, defaults to
/// 'pending'. Changing from 'accepted' back to 'pending' will trigger a replacement
/// of the member resource in Terraform.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberNewParamsBodyIAMCreateMemberWithPoliciesStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
}

/// A group of permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesPermissionGroup {
    pub id: String,
}

/// A group of scoped resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberNewParamsBodyIAMCreateMemberWithPoliciesPoliciesResourceGroup {
    pub id: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberUpdateParams<T> {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,

//     /// Satisfied by [MemberUpdateParamsBodyIAMUpdateMemberWithRoles],
//     /// [MemberUpdateParamsBodyIAMUpdateMemberWithPolicies],
//     /// [MemberUpdateParamsBody].
//     pub body: T,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBodyIAMUpdateMemberWithRoles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::shared::RoleParam>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBodyIAMUpdateMemberWithPolicies {
    pub policies: Vec<MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBody<P, R, U> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<R>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<U>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPolicy {
    // Allow or deny operations against the resources.
    pub access: MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesAccess,
    // A set of permission groups that are specified to the policy.
    pub permission_groups:
        Vec<MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesPermissionGroup>,
    // A list of resource groups that the policy applies to.
    pub resource_groups:
        Vec<MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesResourceGroup>,
}

/// Allow or deny operations against the resources.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesAccess {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

/// A group of permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesPermissionGroup {
    /// Identifier of the group.
    pub id: String,
}

/// A group of scoped resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUpdateParamsBodyIAMUpdateMemberWithPoliciesPoliciesResourceGroup {
    /// Identifier of the group.
    pub id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberListQuery {
    /// [query] Direction to order results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<MemberListQueryDirection>,

    /// [query] Field to order results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<MemberListQueryOrder>,

    /// [query] Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// [query] Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,

    /// [query] A member's status in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MemberListQueryStatus>,
}

pub type MemberListQueryDirection = crate::shared::SortDirection;

/// Field to order results by.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberListQueryOrder {
    #[serde(rename = "user.first_name")]
    UserFirstName,
    #[serde(rename = "user.last_name")]
    UserLastName,
    #[serde(rename = "user.email")]
    UserEmail,
    #[serde(rename = "status")]
    Status,
}

/// A member's status in the account.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemberListQueryStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "rejected")]
    Rejected,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberDeleteParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MemberGetParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleListQuery {
    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// Number of roles per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct RoleGetParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteResponse {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SubscriptionNewParams {
//     /// [path] Identifier
//     #[serde(skip)]
//     pub account_id: String,

//     pub subscription: crate::shared::SubscriptionParam,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SubscriptionUpdateParams {
//     /// [path] Identifier
//     #[serde(skip)]
//     pub account_id: String,

//     pub subscription: crate::shared::SubscriptionParam,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SubscriptionDeleteParams {
//     /// [path] Identifier
//     #[serde(skip)]
//     pub account_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SubscriptionGetParams {
//     /// [path] Identifier
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewResponse<T> {
    /// Token identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenNewResponseCondition>,

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
    pub policies: Option<Vec<crate::shared::TokenPolicy<T>>>,

    /// Status of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenNewResponseStatus>,

    /// The token value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::shared::TokenValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDeleteResponse {
    /// Identifier
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVerifyResponse {
    /// Token identifier tag.
    pub id: String,

    /// Status of the token.
    pub status: TokenVerifyResponseStatus,

    /// The expiration time on or after which the JWT MUST NOT be accepted for
    /// processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// The time before which the token MUST NOT be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewBody {
    /// Token name.
    pub name: String,

    /// List of access policies assigned to the token.
    pub policies: Vec<crate::shared::TokenPolicyParam>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TokenNewBodyCondition>,

    /// The expiration time on or after which the JWT MUST NOT be accepted for
    /// processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// The time before which the token MUST NOT be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewBodyCondition {
    pub request_ip: Option<TokenNewBodyConditionRequestIP>,
}

/// Client IP restrictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewBodyConditionRequestIP {
    /// List of IPv4/IPv6 CIDR addresses.
    pub r#in: Option<Vec<crate::shared::TokenConditionCIDRListParam>>,
    /// List of IPv4/IPv6 CIDR addresses.
    pub not_in: Option<Vec<crate::shared::TokenConditionCIDRListParam>>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TokenUpdateParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,

//     pub token: crate::shared::TokenParam,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenListQuery {
    /// [query] Direction to order results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TokenListQueryDirection>,

    /// [query] Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// [query] Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

pub type TokenListQueryDirection = crate::shared::SortDirection;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TokenDeleteParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TokenGetParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TokenVerifyParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPermissionGroupListResponse {
    /// Public ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Permission Group Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resources to which the Permission Group is scoped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<TokenPermissionGroupListResponseScope>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPermissionGroupListQuery {
    /// [query] Filter by the name of the permission group. The value must be URL-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// [query] Filter by the scope of the permission group. The value must be URL-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TokenValueUpdateParams {
//     /// [path] Account identifier tag.
//     #[serde(skip)]
//     pub account_id: String,

//     pub body: (),
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountManagedBy {
    /// ID of the parent Organization, if one exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<String>,

    /// Name of the parent Organization, if one exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    /// Sets an abuse contact email to notify for abuse reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact_email: Option<String>,

    /// Indicates whether membership in this account requires that Two-Factor
    /// Authentication is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_twofactor: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseAccount {
    /// A unique identifier for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that identifies the account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseAction {
    /// A short description of the action performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The result of the action, indicating success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,

    /// A timestamp indicating when the action was logged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,

    /// A short string that describes the action that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseActor {
    /// The ID of the actor who performed the action. If a user performed the action,
    /// this will be their User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<LogAuditListResponseActorContext>,

    /// The email of the actor who performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The IP address of the request that performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Filters by the API token ID when the actor context is an api_token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,

    /// Filters by the API token name when the actor context is an api_token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,

    /// The type of actor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<LogAuditListResponseActorType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseRaw {
    /// The Cloudflare Ray ID for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf_ray_id: Option<String>,

    /// The HTTP method of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// The HTTP response status code returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,

    /// The URI of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The client's user agent string sent with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseResource {
    /// The unique identifier for the affected resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The Cloudflare product associated with the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,

    /// The scope of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// The type of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAuditListResponseZone {
    /// A string that identifies the zone id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string that identifies the zone name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewResponseCondition {
    /// Client IP restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ip: Option<TokenNewResponseConditionRequestIP>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewResponseConditionRequestIP {
    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<crate::shared::TokenConditionCIDRList>>,

    /// List of IPv4/IPv6 CIDR addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<crate::shared::TokenConditionCIDRList>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "member")]
    StatusMember,
    #[serde(rename = "invited")]
    StatusInvited,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    #[serde(rename = "standard")]
    AccountTypeStandard,
    #[serde(rename = "enterprise")]
    AccountTypeEnterprise,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenNewResponseStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenVerifyResponseStatus {
    /// value is "active"
    #[serde(rename = "active")]
    Active,
    /// value is "disabled"
    #[serde(rename = "disabled")]
    Disabled,
    /// value is "expired"
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenPermissionGroupListResponseScope {
    #[serde(rename = "com.cloudflare.api.account")]
    ComCloudflareAPIAccount,
    #[serde(rename = "com.cloudflare.api.account.zone")]
    ComCloudflareAPIAccountZone,
    #[serde(rename = "com.cloudflare.api.user")]
    ComCloudflareAPIUser,
    #[serde(rename = "com.cloudflare.edge.r2.bucket")]
    ComCloudflareEdgeR2Bucket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListResponseActorContext {
    #[serde(rename = "api_key")]
    APIKey,
    #[serde(rename = "api_token")]
    APIToken,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "oauth")]
    OAuth,
    #[serde(rename = "origin_ca_key")]
    OriginCAKey,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogAuditListResponseActorType {
    /// value is "account"
    #[serde(rename = "account")]
    Account,
    /// value is "cloudflare_admin"
    #[serde(rename = "cloudflare_admin")]
    CloudflareAdmin,
    /// value is "system"
    #[serde(rename = "system")]
    System,
    /// value is "user"
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountNewBody {
    /// Account name
    pub name: String,
    /// One of the following:
    /// - "standard"
    /// - "enterprise"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AccountNewBodyType>,
    /// information related to the tenant unit, and optionally, an id of the unit to
    /// create the account on. see
    /// https://developers.cloudflare.com/tenant/how-to/manage-accounts/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AccountNewBodyUnit>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountNewBodyType {
    /// value is "standard"
    #[serde(rename = "standard")]
    Standard,
    /// value is "enterprise"
    #[serde(rename = "enterprise")]
    Enterprise,
}

/// information related to the tenant unit, and optionally, an id of the unit to
/// create the account on. see
/// https://developers.cloudflare.com/tenant/how-to/manage-accounts/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountNewBodyUnit {
    /// Tenant unit ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
