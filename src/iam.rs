#[cfg(any(feature = "full", feature = "with-iam"))]
mod iam_bindings {

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupListResponse {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<PermissionGroupListResponseMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupGetResponse {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<PermissionGroupGetResponseMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupListParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// ID of the permission group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Label of the permission group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,

        /// Name of the permission group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Maximum number of results per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupGetParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupNewResponse {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<ResourceGroupNewResponseScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<ResourceGroupNewResponseMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupUpdateResponse {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<ResourceGroupUpdateResponseScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<ResourceGroupUpdateResponseMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupListResponse {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<ResourceGroupListResponseScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<ResourceGroupListResponseMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupDeleteResponse {
        /// Identifier
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupGetResponse {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<ResourceGroupGetResponseScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<ResourceGroupGetResponseMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupNewParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Name of the resource group
        pub name: String,

        /// A scope is a combination of scope objects which provides additional context.
        pub scope: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupUpdateParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Name of the resource group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// A scope is a combination of scope objects which provides additional context.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scope: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupListParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// ID of the resource group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Name of the resource group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupDeleteParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupGetParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSONewResponse {
        /// SSO Connector identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Timestamp for the creation of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_domain: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,

        /// Timestamp for the last update of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_on: Option<DateTime<Utc>>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<SSONewResponseVerification>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOUpdateResponse {
        /// SSO Connector identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Timestamp for the creation of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_domain: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,

        /// Timestamp for the last update of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_on: Option<DateTime<Utc>>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<SSOUpdateResponseVerification>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOListResponse {
        /// SSO Connector identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Timestamp for the creation of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_domain: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,

        /// Timestamp for the last update of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_on: Option<DateTime<Utc>>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<SSOListResponseVerification>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSODeleteResponse {
        /// Identifier
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationResponse {
        pub errors: Vec<SSOBeginVerificationResponseError>,

        pub messages: Vec<SSOBeginVerificationResponseMessage>,

        /// Whether the API call was successful.
        pub success: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOGetResponse {
        /// SSO Connector identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Timestamp for the creation of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_domain: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,

        /// Timestamp for the last update of the SSO connector
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_on: Option<DateTime<Utc>>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<SSOGetResponseVerification>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSONewParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Email domain of the new SSO connector
        pub email_domain: String,

        /// Begin the verification process after creation
        #[serde(skip_serializing_if = "Option::is_none")]
        pub begin_verification: Option<String>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOUpdateParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// SSO Connector enabled state
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enabled: Option<String>,

        /// Controls the display of FedRAMP language to the user during SSO login
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_fedramp_language: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOListParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSODeleteParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOGetParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponse {
        /// User Group identifier tag.
        pub id: String,

        /// Timestamp for the creation of the user group
        pub created_on: DateTime<Utc>,

        /// Last time the user group was modified.
        pub modified_on: DateTime<Utc>,

        /// Name of the user group.
        pub name: String,

        /// Policies attached to the User group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<UserGroupNewResponsePolicy>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponse {
        /// User Group identifier tag.
        pub id: String,

        /// Timestamp for the creation of the user group
        pub created_on: DateTime<Utc>,

        /// Last time the user group was modified.
        pub modified_on: DateTime<Utc>,

        /// Name of the user group.
        pub name: String,

        /// Policies attached to the User group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<UserGroupUpdateResponsePolicy>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponse {
        /// User Group identifier tag.
        pub id: String,

        /// Timestamp for the creation of the user group
        pub created_on: DateTime<Utc>,

        /// Last time the user group was modified.
        pub modified_on: DateTime<Utc>,

        /// Name of the user group.
        pub name: String,

        /// Policies attached to the User group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<UserGroupListResponsePolicy>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupDeleteResponse {
        /// Identifier
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponse {
        /// User Group identifier tag.
        pub id: String,

        /// Timestamp for the creation of the user group
        pub created_on: DateTime<Utc>,

        /// Last time the user group was modified.
        pub modified_on: DateTime<Utc>,

        /// Name of the user group.
        pub name: String,

        /// Policies attached to the User group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<UserGroupGetResponsePolicy>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Name of the User group.
        pub name: String,

        /// Policies attached to the User group
        pub policies: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Name of the User group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Policies attached to the User group
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policies: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// ID of the user group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The sort order of returned user groups by name. Default sort order is ascending.
        /// To switch to descending, set this parameter to "desc"
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<String>,

        /// A string used for searching for user groups containing that substring.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fuzzy_name: Option<String>,

        /// Name of the user group to be fetched.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Maximum number of results per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupDeleteParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberNewResponse {
        /// Account member identifier.
        pub id: String,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// The member's status in the account.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserGroupMemberNewResponseStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberUpdateResponse {
        /// Account member identifier.
        pub id: String,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// The member's status in the account.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserGroupMemberUpdateResponseStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberListResponse {
        /// Account member identifier.
        pub id: String,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// The member's status in the account.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserGroupMemberListResponseStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberDeleteResponse {
        /// Account member identifier.
        pub id: String,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// The member's status in the account.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserGroupMemberDeleteResponseStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberNewParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        pub body: Vec<UserGroupMemberNewParamsBody>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberUpdateParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Set/Replace members to a user group.
        pub body: Vec<UserGroupMemberUpdateParamsBody>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberListParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Maximum number of results per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberDeleteParams {
        /// Account identifier tag.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupListResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissionGroupGetResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupNewResponseScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<ResourceGroupNewResponseScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupNewResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupUpdateResponseScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<ResourceGroupUpdateResponseScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupUpdateResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupListResponseScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<ResourceGroupListResponseScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupListResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupGetResponseScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<ResourceGroupGetResponseScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupGetResponseMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSONewResponseVerification {
        /// DNS verification code. Add this entire string to the DNS TXT record of the email
        /// domain to validate ownership.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,

        /// The status of the verification code from the verification process.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SSONewResponseVerificationStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOUpdateResponseVerification {
        /// DNS verification code. Add this entire string to the DNS TXT record of the email
        /// domain to validate ownership.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,

        /// The status of the verification code from the verification process.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SSOUpdateResponseVerificationStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOListResponseVerification {
        /// DNS verification code. Add this entire string to the DNS TXT record of the email
        /// domain to validate ownership.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,

        /// The status of the verification code from the verification process.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SSOListResponseVerificationStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationResponseError {
        pub code: i64,

        pub message: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub documentation_url: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub source: Option<SSOBeginVerificationResponseErrorsSource>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationResponseMessage {
        pub code: i64,

        pub message: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub documentation_url: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub source: Option<SSOBeginVerificationResponseMessagesSource>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOGetResponseVerification {
        /// DNS verification code. Add this entire string to the DNS TXT record of the email
        /// domain to validate ownership.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,

        /// The status of the verification code from the verification process.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SSOGetResponseVerificationStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePolicy {
        /// Policy identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Allow or deny operations against the resources.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access: Option<UserGroupNewResponsePoliciesAccess>,

        /// A set of permission groups that are specified to the policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permission_groups: Option<Vec<UserGroupNewResponsePoliciesPermissionGroup>>,

        /// A list of resource groups that the policy applies to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<Vec<UserGroupNewResponsePoliciesResourceGroup>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePolicy {
        /// Policy identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Allow or deny operations against the resources.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access: Option<UserGroupUpdateResponsePoliciesAccess>,

        /// A set of permission groups that are specified to the policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permission_groups: Option<Vec<UserGroupUpdateResponsePoliciesPermissionGroup>>,

        /// A list of resource groups that the policy applies to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<Vec<UserGroupUpdateResponsePoliciesResourceGroup>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePolicy {
        /// Policy identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Allow or deny operations against the resources.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access: Option<UserGroupListResponsePoliciesAccess>,

        /// A set of permission groups that are specified to the policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permission_groups: Option<Vec<UserGroupListResponsePoliciesPermissionGroup>>,

        /// A list of resource groups that the policy applies to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<Vec<UserGroupListResponsePoliciesResourceGroup>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePolicy {
        /// Policy identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Allow or deny operations against the resources.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access: Option<UserGroupGetResponsePoliciesAccess>,

        /// A set of permission groups that are specified to the policy.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permission_groups: Option<Vec<UserGroupGetResponsePoliciesPermissionGroup>>,

        /// A list of resource groups that the policy applies to.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_groups: Option<Vec<UserGroupGetResponsePoliciesResourceGroup>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberNewParamsBody {
        /// The identifier of an existing account Member.
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupMemberUpdateParamsBody {
        /// The identifier of an existing account Member.
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupNewResponseScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupUpdateResponseScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupListResponseScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResourceGroupGetResponseScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationResponseErrorsSource {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pointer: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SSOBeginVerificationResponseMessagesSource {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pointer: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesPermissionGroup {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupNewResponsePoliciesPermissionGroupsMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesResourceGroup {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<UserGroupNewResponsePoliciesResourceGroupsScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupNewResponsePoliciesResourceGroupsMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesPermissionGroup {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupUpdateResponsePoliciesPermissionGroupsMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesResourceGroup {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<UserGroupUpdateResponsePoliciesResourceGroupsScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupUpdateResponsePoliciesResourceGroupsMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesPermissionGroup {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupListResponsePoliciesPermissionGroupsMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesResourceGroup {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<UserGroupListResponsePoliciesResourceGroupsScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupListResponsePoliciesResourceGroupsMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesPermissionGroup {
        /// Identifier of the permission group.
        pub id: String,

        /// Attributes associated to the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupGetResponsePoliciesPermissionGroupsMeta>,

        /// Name of the permission group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesResourceGroup {
        /// Identifier of the resource group.
        pub id: String,

        /// The scope associated to the resource group
        pub scope: Vec<UserGroupGetResponsePoliciesResourceGroupsScope>,

        /// Attributes associated to the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meta: Option<UserGroupGetResponsePoliciesResourceGroupsMeta>,

        /// Name of the resource group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesPermissionGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesResourceGroupsScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<UserGroupNewResponsePoliciesResourceGroupsScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesResourceGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesPermissionGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesResourceGroupsScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<UserGroupUpdateResponsePoliciesResourceGroupsScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesResourceGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesPermissionGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesResourceGroupsScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<UserGroupListResponsePoliciesResourceGroupsScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesResourceGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesPermissionGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesResourceGroupsScope {
        /// This is a combination of pre-defined resource name and identifier (like Account
        /// ID etc.)
        pub key: String,

        /// A list of scope objects for additional context.
        pub objects: Vec<UserGroupGetResponsePoliciesResourceGroupsScopeObject>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesResourceGroupsMeta {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupNewResponsePoliciesResourceGroupsScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupUpdateResponsePoliciesResourceGroupsScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupListResponsePoliciesResourceGroupsScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserGroupGetResponsePoliciesResourceGroupsScopeObject {
        /// This is a combination of pre-defined resource name and identifier (like Zone ID
        /// etc.)
        pub key: String,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupMemberNewResponseStatus {
        #[serde(rename = "accepted")]
        UserGroupMemberNewResponseStatusAccepted,
        #[serde(rename = "pending")]
        UserGroupMemberNewResponseStatusPending,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupMemberUpdateResponseStatus {
        #[serde(rename = "accepted")]
        UserGroupMemberUpdateResponseStatusAccepted,
        #[serde(rename = "pending")]
        UserGroupMemberUpdateResponseStatusPending,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupMemberListResponseStatus {
        #[serde(rename = "accepted")]
        UserGroupMemberListResponseStatusAccepted,
        #[serde(rename = "pending")]
        UserGroupMemberListResponseStatusPending,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupMemberDeleteResponseStatus {
        #[serde(rename = "accepted")]
        UserGroupMemberDeleteResponseStatusAccepted,
        #[serde(rename = "pending")]
        UserGroupMemberDeleteResponseStatusPending,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SSONewResponseVerificationStatus {
        #[serde(rename = "awaiting")]
        SSONewResponseVerificationStatusAwaiting,
        #[serde(rename = "pending")]
        SSONewResponseVerificationStatusPending,
        #[serde(rename = "failed")]
        SSONewResponseVerificationStatusFailed,
        #[serde(rename = "verified")]
        SSONewResponseVerificationStatusVerified,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SSOUpdateResponseVerificationStatus {
        #[serde(rename = "awaiting")]
        SSOUpdateResponseVerificationStatusAwaiting,
        #[serde(rename = "pending")]
        SSOUpdateResponseVerificationStatusPending,
        #[serde(rename = "failed")]
        SSOUpdateResponseVerificationStatusFailed,
        #[serde(rename = "verified")]
        SSOUpdateResponseVerificationStatusVerified,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SSOListResponseVerificationStatus {
        #[serde(rename = "awaiting")]
        SSOListResponseVerificationStatusAwaiting,
        #[serde(rename = "pending")]
        SSOListResponseVerificationStatusPending,
        #[serde(rename = "failed")]
        SSOListResponseVerificationStatusFailed,
        #[serde(rename = "verified")]
        SSOListResponseVerificationStatusVerified,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SSOGetResponseVerificationStatus {
        #[serde(rename = "awaiting")]
        SSOGetResponseVerificationStatusAwaiting,
        #[serde(rename = "pending")]
        SSOGetResponseVerificationStatusPending,
        #[serde(rename = "failed")]
        SSOGetResponseVerificationStatusFailed,
        #[serde(rename = "verified")]
        SSOGetResponseVerificationStatusVerified,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupNewResponsePoliciesAccess {
        #[serde(rename = "allow")]
        UserGroupNewResponsePoliciesAccessAllow,
        #[serde(rename = "deny")]
        UserGroupNewResponsePoliciesAccessDeny,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupUpdateResponsePoliciesAccess {
        #[serde(rename = "allow")]
        UserGroupUpdateResponsePoliciesAccessAllow,
        #[serde(rename = "deny")]
        UserGroupUpdateResponsePoliciesAccessDeny,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupListResponsePoliciesAccess {
        #[serde(rename = "allow")]
        UserGroupListResponsePoliciesAccessAllow,
        #[serde(rename = "deny")]
        UserGroupListResponsePoliciesAccessDeny,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UserGroupGetResponsePoliciesAccess {
        #[serde(rename = "allow")]
        UserGroupGetResponsePoliciesAccessAllow,
        #[serde(rename = "deny")]
        UserGroupGetResponsePoliciesAccessDeny,
    }
}

#[cfg(any(feature = "full", feature = "with-iam"))]
pub use iam_bindings::*;

#[cfg(any(feature = "full", feature = "with-iam"))]
impl Client {}
