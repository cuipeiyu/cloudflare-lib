#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListParams {
    /// Finds a specific log by its ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,

    /// Limits the returned results to logs older than the specified date. A `full-date`
    /// that conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Changes the direction of the chronological sorting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Indicates that this request is an export of logs in CSV format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<String>,

    /// Indicates whether or not to hide user level audit logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_user_logs: Option<String>,

    /// Defines which page of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Sets the number of results to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Limits the returned results to logs newer than the specified date. A `full-date`
    /// that conforms to RFC3339.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingHistory {
    /// Billing item identifier tag.
    pub id: String,

    /// The billing item action.
    pub action: String,

    /// The amount associated with this billing item.
    pub amount: f64,

    /// The monetary unit in which pricing information is displayed.
    pub currency: String,

    /// The billing item description.
    pub description: String,

    /// When the billing item was created.
    pub occurred_at: DateTime<Utc>,

    /// The billing item type.
    pub r#type: String,

    pub zone: BillingHistoryZone,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingHistoryListParams {
    /// The billing item action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// When the billing item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,

    /// Field to order billing history by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// The billing item type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingProfileGetResponse {
    /// Billing item identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_expiry_month: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_expiry_year: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_billing_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_primary_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partner: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_bill_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_address2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_gateway: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_nonce: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_zipcode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invite {
    /// ID of the user to add to the organization.
    pub invited_member_id: String,

    /// ID of the organization the user will be added to.
    pub organization_id: String,

    /// Invite identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When the invite is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<DateTime<Utc>>,

    /// The email address of the user who created the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,

    /// Email address of the user to add to the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_member_email: Option<String>,

    /// When the invite was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_is_enforcing_twofactor: Option<bool>,

    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,

    /// List of role names the membership has for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Current status of the invitation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InviteStatus>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteEditParams {
    /// Status of your response to the invitation (rejected or accepted).
    pub status: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Access permissions for this User.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    /// List of roles that a user has within an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    /// Whether the user is a member of the organization or has an invitation pending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::accounts::Status>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDeleteResponse {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationListParams {
    /// Direction to order organizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Whether to match all search requirements or at least one (any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Field to order organizations by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of organizations per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// Whether the user is a member of the organization or has an inivitation pending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDeleteResponse {
    /// Subscription identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdateParams {
    pub subscription: crate::shared::SubscriptionParam,

}

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
pub struct TokenNewParams {
    /// Token name.
    pub name: String,

    /// List of access policies assigned to the token.
    pub policies: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// The expiration time on or after which the JWT MUST NOT be accepted for
    /// processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,

    /// The time before which the token MUST NOT be accepted for processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUpdateParams {
    pub token: crate::shared::TokenParam,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenListParams {
    /// Direction to order results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

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
pub struct TokenPermissionGroupListParams {
    /// Filter by the name of the permission group. The value must be URL-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by the scope of the permission group. The value must be URL-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValueUpdateParams {
    pub body: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEditResponse {
    /// Identifier of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Lists the betas that the user is participating in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betas: Option<Vec<String>>,

    /// The country in which the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// User's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Indicates whether user has any business zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_business_zones: Option<bool>,

    /// Indicates whether user has any enterprise zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_enterprise_zones: Option<bool>,

    /// Indicates whether user has any pro zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pro_zones: Option<bool>,

    /// User's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<Organization>>,

    /// Indicates whether user has been suspended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,

    /// User's telephone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    /// Indicates whether two-factor authentication is enabled for the user account.
    /// Does not apply to API authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_enabled: Option<bool>,

    /// Indicates whether two-factor authentication is required by one of the accounts
    /// that the user is a member of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_locked: Option<bool>,

    /// The zipcode or postal code where the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGetResponse {
    /// Identifier of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Lists the betas that the user is participating in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betas: Option<Vec<String>>,

    /// The country in which the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// User's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Indicates whether user has any business zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_business_zones: Option<bool>,

    /// Indicates whether user has any enterprise zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_enterprise_zones: Option<bool>,

    /// Indicates whether user has any pro zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pro_zones: Option<bool>,

    /// User's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<Organization>>,

    /// Indicates whether user has been suspended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,

    /// User's telephone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    /// Indicates whether two-factor authentication is enabled for the user account.
    /// Does not apply to API authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_enabled: Option<bool>,

    /// Indicates whether two-factor authentication is required by one of the accounts
    /// that the user is a member of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_authentication_locked: Option<bool>,

    /// The zipcode or postal code where the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEditParams {
    /// The country in which the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// User's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// User's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// User's telephone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    /// The zipcode or postal code where the user lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingHistoryZone {
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
pub enum InviteStatus {
    #[serde(rename = "pending")]
    InviteStatusPending,
    #[serde(rename = "accepted")]
    InviteStatusAccepted,
    #[serde(rename = "rejected")]
    InviteStatusRejected,
    #[serde(rename = "expired")]
    InviteStatusExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenNewResponseStatus {
    #[serde(rename = "active")]
    TokenNewResponseStatusActive,
    #[serde(rename = "disabled")]
    TokenNewResponseStatusDisabled,
    #[serde(rename = "expired")]
    TokenNewResponseStatusExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenVerifyResponseStatus {
    #[serde(rename = "active")]
    TokenVerifyResponseStatusActive,
    #[serde(rename = "disabled")]
    TokenVerifyResponseStatusDisabled,
    #[serde(rename = "expired")]
    TokenVerifyResponseStatusExpired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenPermissionGroupListResponseScope {
    #[serde(rename = "com.cloudflare.api.account")]
    TokenPermissionGroupListResponseScopeComCloudflareAPIAccount,
    #[serde(rename = "com.cloudflare.api.account.zone")]
    TokenPermissionGroupListResponseScopeComCloudflareAPIAccountZone,
    #[serde(rename = "com.cloudflare.api.user")]
    TokenPermissionGroupListResponseScopeComCloudflareAPIUser,
    #[serde(rename = "com.cloudflare.edge.r2.bucket")]
    TokenPermissionGroupListResponseScopeComCloudflareEdgeR2Bucket,
}

