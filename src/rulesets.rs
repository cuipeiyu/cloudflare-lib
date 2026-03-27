#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseUpdateResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<PhaseUpdateResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseGetResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<PhaseGetResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseUpdateParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The list of rules in the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseVersionListResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseVersionGetResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<PhaseVersionGetResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseVersionListParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseVersionGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<BlockRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<BlockRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<BlockRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<BlockRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CompressResponseRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<CompressResponseRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<CompressResponseRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<CompressResponseRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDoSDynamicRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DDoSDynamicRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<DDoSDynamicRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<DDoSDynamicRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDoSDynamicRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ExecuteRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<ExecuteRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<ExecuteRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<ExecuteRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceConnectionCloseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ForceConnectionCloseRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<ForceConnectionCloseRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<ForceConnectionCloseRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceConnectionCloseRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LogCustomFieldRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<LogCustomFieldRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<LogCustomFieldRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<LogCustomFieldRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LogRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<LogRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<LogRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    /// Whether to generate a log when the rule matches.
    pub enabled: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingParam {
    /// Whether to generate a log when the rule matches.
    pub enabled: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedChallengeRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ManagedChallengeRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<ManagedChallengeRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<ManagedChallengeRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedChallengeRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RedirectRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<RedirectRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<RedirectRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RedirectRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RewriteRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<RewriteRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<RewriteRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RewriteRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RouteRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<RouteRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<RouteRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<RouteRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ScoreRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<ScoreRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<ScoreRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<ScoreRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeErrorRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ServeErrorRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<ServeErrorRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<ServeErrorRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<ServeErrorRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeErrorRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SetCacheSettingsRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<SetCacheSettingsRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<SetCacheSettingsRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<SetCacheSettingsRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SetConfigRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<SetConfigRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<SetConfigRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<SetConfigRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SkipRuleAction>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<SkipRuleActionParameters>,

    /// The categories of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<SkipRuleExposedCredentialCheck>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<SkipRuleRatelimit>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipRuleParam {
    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// The parameters configuring the rule's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// Configuration for exposed credential checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,

    /// An object configuring the rule's rate limit behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleNewResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RuleNewResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RuleDeleteResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEditResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RuleEditResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleNewParams {
    pub body: RuleNewParamsBodyUnion,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEditParams {
    pub body: RuleEditParamsBodyUnion,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetNewResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RulesetNewResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetUpdateResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RulesetUpdateResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetListResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetGetResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<RulesetGetResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetNewParams {
    /// The kind of the ruleset.
    pub kind: String,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The list of rules in the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetUpdateParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The kind of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The phase of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// The list of rules in the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetListParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The cursor to use for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The number of rulesets to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetDeleteParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGetResponse {
    /// The unique ID of the ruleset.
    pub id: String,

    /// The kind of the ruleset.
    pub kind: Kind,

    /// The timestamp of when the ruleset was last modified.
    pub last_updated: DateTime<Utc>,

    /// The human-readable name of the ruleset.
    pub name: String,

    /// The phase of the ruleset.
    pub phase: Phase,

    /// The list of rules in the ruleset.
    pub rules: Vec<VersionGetResponseRule>,

    /// The version of the ruleset.
    pub version: String,

    /// An informative description of the ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDeleteParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseUpdateResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PhaseUpdateResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [PhaseUpdateResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [PhaseUpdateResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [PhaseUpdateResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [PhaseUpdateResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseGetResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PhaseGetResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [PhaseGetResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [PhaseGetResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [PhaseGetResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [PhaseGetResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseVersionGetResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PhaseVersionGetResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [PhaseVersionGetResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [PhaseVersionGetResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [PhaseVersionGetResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [PhaseVersionGetResponseRulesRulesetsJSChallengeRuleRatelimit],
    /// [LogRuleRatelimit], [LogCustomFieldRuleRatelimit],
    /// [ManagedChallengeRuleRatelimit], [RedirectRuleRatelimit],
    /// [RewriteRuleRatelimit], [RouteRuleRatelimit], [ScoreRuleRatelimit],
    /// [ServeErrorRuleRatelimit], [SetCacheSettingsRuleRatelimit],
    /// [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRuleActionParameters {
    /// The response to show when the block is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<BlockRuleActionParametersResponse>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRuleActionParameters {
    /// Custom order for compression algorithms.
    pub algorithms: Vec<CompressResponseRuleActionParametersAlgorithm>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDoSDynamicRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DDoSDynamicRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleActionParameters {
    /// The ID of the ruleset to execute.
    pub id: String,

    /// The configuration to use for matched data logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_data: Option<ExecuteRuleActionParametersMatchedData>,

    /// A set of overrides to apply to the target ruleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ExecuteRuleActionParametersOverrides>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceConnectionCloseRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceConnectionCloseRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParameters {
    /// The cookie fields to log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_fields: Option<Vec<LogCustomFieldRuleActionParametersCookieField>>,

    /// The raw response fields to log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_response_fields: Option<Vec<LogCustomFieldRuleActionParametersRawResponseField>>,

    /// The raw request fields to log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_fields: Option<Vec<LogCustomFieldRuleActionParametersRequestField>>,

    /// The transformed response fields to log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_fields: Option<Vec<LogCustomFieldRuleActionParametersResponseField>>,

    /// The transformed request fields to log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformed_request_fields: Option<Vec<LogCustomFieldRuleActionParametersTransformedRequestField>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedChallengeRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedChallengeRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleActionParameters {
    /// A redirect based on a bulk list lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_list: Option<RedirectRuleActionParametersFromList>,

    /// A redirect based on the request properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_value: Option<RedirectRuleActionParametersFromValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleActionParameters {
    /// A map of headers to rewrite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, RewriteRuleActionParametersHeader>>,

    /// A URI path rewrite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<RewriteRuleActionParametersURI>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleActionParameters {
    /// A value to rewrite the HTTP host header to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_header: Option<String>,

    /// An origin to route to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<RouteRuleActionParametersOrigin>,

    /// A Server Name Indication (SNI) override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sni: Option<RouteRuleActionParametersSNI>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRuleActionParameters {
    /// A delta to change the score by, which can be either positive or negative.
    pub increment: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeErrorRuleActionParameters {
    /// The name of a custom asset to serve as the error response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,

    /// The response content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The content type header to set with the error response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ServeErrorRuleActionParametersContentType>,

    /// The status code to use for the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeErrorRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServeErrorRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParameters {
    /// A list of additional ports that caching should be enabled on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_cacheable_ports: Option<Vec<i64>>,

    /// How long client browsers should cache the response. Cloudflare cache purge will
    /// not purge content cached on client browsers, so high browser TTLs may lead to
    /// stale content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_ttl: Option<SetCacheSettingsRuleActionParametersBrowserTTL>,

    /// Whether the request's response from the origin is eligible for caching. Caching
    /// itself will still depend on the cache control header and your other caching
    /// configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,

    /// Which components of the request are included in or excluded from the cache key
    /// Cloudflare uses to store the response in cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<SetCacheSettingsRuleActionParametersCacheKey>,

    /// Settings to determine whether the request's response from origin is eligible for
    /// Cache Reserve (requires a Cache Reserve add-on plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_reserve: Option<SetCacheSettingsRuleActionParametersCacheReserve>,

    /// How long the Cloudflare edge network should cache the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_ttl: Option<SetCacheSettingsRuleActionParametersEdgeTTL>,

    /// Whether Cloudflare will aim to strictly adhere to RFC 7234.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_cache_control: Option<bool>,

    /// Whether to generate Cloudflare error pages for issues from the origin server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_error_page_passthru: Option<bool>,

    /// A timeout value between two successive read operations to use for your origin
    /// server. Historically, the timeout value between two read options from Cloudflare
    /// to an origin server is 100 seconds. If you are attempting to reduce HTTP 524
    /// errors because of timeouts from an origin server, try increasing this timeout
    /// value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<i64>,

    /// Whether Cloudflare should respect strong ETag (entity tag) headers. If false,
    /// Cloudflare converts strong ETag headers to weak ETag headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respect_strong_etags: Option<bool>,

    /// When to serve stale content from cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serve_stale: Option<SetCacheSettingsRuleActionParametersServeStale>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRuleActionParameters {
    /// Whether to enable Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_https_rewrites: Option<bool>,

    /// Which file extensions to minify automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autominify: Option<SetConfigRuleActionParametersAutominify>,

    /// Whether to enable Browser Integrity Check (BIC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<bool>,

    /// Whether to enable content conversion (e.g., HTML to Markdown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_converter: Option<bool>,

    /// Whether to disable Cloudflare Apps.
    /// Deprecated: Cloudflare Apps are deprected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_apps: Option<SetConfigRuleActionParametersDisableApps>,

    /// Whether to disable Pay Per Crawl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pay_per_crawl: Option<SetConfigRuleActionParametersDisablePayPerCrawl>,

    /// Whether to disable Real User Monitoring (RUM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rum: Option<SetConfigRuleActionParametersDisableRUM>,

    /// Whether to disable Zaraz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_zaraz: Option<SetConfigRuleActionParametersDisableZaraz>,

    /// Whether to enable Email Obfuscation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_obfuscation: Option<bool>,

    /// Whether to enable Cloudflare Fonts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fonts: Option<bool>,

    /// Whether to enable Hotlink Protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotlink_protection: Option<bool>,

    /// Whether to enable Mirage.
    /// Deprecated: Mirage is deprecated. More information at
    /// https://developers.cloudflare.com/speed/optimization/images/mirage/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirage: Option<bool>,

    /// Whether to enable Opportunistic Encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opportunistic_encryption: Option<bool>,

    /// The Polish level to configure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polish: Option<SetConfigRuleActionParametersPolish>,

    /// The request body buffering mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body_buffering: Option<SetConfigRuleActionParametersRequestBodyBuffering>,

    /// The response body buffering mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_buffering: Option<SetConfigRuleActionParametersResponseBodyBuffering>,

    /// Whether to enable Rocket Loader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rocket_loader: Option<bool>,

    /// The Security Level to configure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<SetConfigRuleActionParametersSecurityLevel>,

    /// Whether to enable Server-Side Excludes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_excludes: Option<bool>,

    /// The SSL level to configure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SetConfigRuleActionParametersSSL>,

    /// Whether to enable Signed Exchanges (SXG).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sxg: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipRuleActionParameters {
    /// A phase to skip the execution of. This option is only compatible with the
    /// products option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<SkipRuleActionParametersPhase>,

    /// A list of phases to skip the execution of. This option is incompatible with the
    /// rulesets option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<Phase>>,

    /// A list of legacy security products to skip the execution of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<SkipRuleActionParametersProduct>>,

    /// A mapping of ruleset IDs to a list of rule IDs in that ruleset to skip the
    /// execution of. This option is incompatible with the ruleset option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<::std::collections::HashMap<String, Vec<String>>>,

    /// A ruleset to skip the execution of. This option is incompatible with the
    /// rulesets option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<SkipRuleActionParametersRuleset>,

    /// A list of ruleset IDs to skip the execution of. This option is incompatible with
    /// the ruleset and phases options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesets: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipRuleExposedCredentialCheck {
    /// An expression that selects the password used in the credentials check.
    pub password_expression: String,

    /// An expression that selects the user ID used in the credentials check.
    pub username_expression: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipRuleRatelimit {
    /// Characteristics of the request on which the rate limit counter will be
    /// incremented.
    pub characteristics: Vec<String>,

    /// Period in seconds over which the counter is being incremented.
    pub period: i64,

    /// An expression that defines when the rate limit counter should be incremented. It
    /// defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counting_expression: Option<String>,

    /// Period of time in seconds after which the action will be disabled following its
    /// first execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_timeout: Option<i64>,

    /// The threshold of requests per period after which the action will be executed for
    /// the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_period: Option<i64>,

    /// Whether counting is only performed when an origin is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_to_origin: Option<bool>,

    /// The score threshold per period for which the action will be executed the first
    /// time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_per_period: Option<i64>,

    /// A response header name provided by the origin, which contains the score to
    /// increment rate limit counter with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_response_header_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleNewResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleNewResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RuleNewResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RuleNewResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RuleNewResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RuleNewResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleDeleteResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RuleDeleteResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RuleDeleteResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RuleDeleteResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RuleDeleteResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEditResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleEditResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RuleEditResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RuleEditResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RuleEditResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RuleEditResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetNewResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetNewResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RulesetNewResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RulesetNewResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RulesetNewResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RulesetNewResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetUpdateResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetUpdateResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RulesetUpdateResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RulesetUpdateResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RulesetUpdateResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RulesetUpdateResponseRulesRulesetsJSChallengeRuleRatelimit],
    /// [LogRuleRatelimit], [LogCustomFieldRuleRatelimit],
    /// [ManagedChallengeRuleRatelimit], [RedirectRuleRatelimit],
    /// [RewriteRuleRatelimit], [RouteRuleRatelimit], [ScoreRuleRatelimit],
    /// [ServeErrorRuleRatelimit], [SetCacheSettingsRuleRatelimit],
    /// [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetGetResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RulesetGetResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [RulesetGetResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [RulesetGetResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [RulesetGetResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [RulesetGetResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGetResponseRule {
    /// The timestamp of when the rule was last modified.
    pub last_updated: DateTime<Utc>,

    /// The version of the rule.
    pub version: String,

    /// The unique ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The action to perform when the rule matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<VersionGetResponseRulesAction>,

    /// This field can have the runtime type of [BlockRuleActionParameters],
    /// [interface{}], [CompressResponseRuleActionParameters],
    /// [ExecuteRuleActionParameters], [LogCustomFieldRuleActionParameters],
    /// [RedirectRuleActionParameters], [RewriteRuleActionParameters],
    /// [RouteRuleActionParameters], [ScoreRuleActionParameters],
    /// [ServeErrorRuleActionParameters], [SetCacheSettingsRuleActionParameters],
    /// [SetConfigRuleActionParameters], [SkipRuleActionParameters].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<String>,

    /// This field can have the runtime type of [[]string].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,

    /// An informative description of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the rule should be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This field can have the runtime type of [BlockRuleExposedCredentialCheck],
    /// [VersionGetResponseRulesRulesetsChallengeRuleExposedCredentialCheck],
    /// [CompressResponseRuleExposedCredentialCheck],
    /// [DDoSDynamicRuleExposedCredentialCheck], [ExecuteRuleExposedCredentialCheck],
    /// [ForceConnectionCloseRuleExposedCredentialCheck],
    /// [VersionGetResponseRulesRulesetsJSChallengeRuleExposedCredentialCheck],
    /// [LogRuleExposedCredentialCheck], [LogCustomFieldRuleExposedCredentialCheck],
    /// [ManagedChallengeRuleExposedCredentialCheck],
    /// [RedirectRuleExposedCredentialCheck], [RewriteRuleExposedCredentialCheck],
    /// [RouteRuleExposedCredentialCheck], [ScoreRuleExposedCredentialCheck],
    /// [ServeErrorRuleExposedCredentialCheck],
    /// [SetCacheSettingsRuleExposedCredentialCheck],
    /// [SetConfigRuleExposedCredentialCheck], [SkipRuleExposedCredentialCheck].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_credential_check: Option<String>,

    /// The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// An object configuring the rule's logging behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    /// This field can have the runtime type of [BlockRuleRatelimit],
    /// [VersionGetResponseRulesRulesetsChallengeRuleRatelimit],
    /// [CompressResponseRuleRatelimit], [DDoSDynamicRuleRatelimit],
    /// [ExecuteRuleRatelimit], [ForceConnectionCloseRuleRatelimit],
    /// [VersionGetResponseRulesRulesetsJSChallengeRuleRatelimit], [LogRuleRatelimit],
    /// [LogCustomFieldRuleRatelimit], [ManagedChallengeRuleRatelimit],
    /// [RedirectRuleRatelimit], [RewriteRuleRatelimit], [RouteRuleRatelimit],
    /// [ScoreRuleRatelimit], [ServeErrorRuleRatelimit],
    /// [SetCacheSettingsRuleRatelimit], [SetConfigRuleRatelimit], [SkipRuleRatelimit].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<String>,

    /// The reference of the rule (the rule's ID by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRuleActionParametersResponse {
    /// The content to return.
    pub content: String,

    /// The type of the content to return.
    pub content_type: String,

    /// The status code to return.
    pub status_code: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressResponseRuleActionParametersAlgorithm {
    /// Name of the compression algorithm to enable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompressResponseRuleActionParametersAlgorithmsName>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleActionParametersMatchedData {
    /// The public key to encrypt matched data logs with.
    pub public_key: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleActionParametersOverrides {
    /// An action to override all rules with. This option has lower precedence than rule
    /// and category overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// A list of category-level overrides. This option has the second-highest
    /// precedence after rule-level overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<ExecuteRuleActionParametersOverridesCategory>>,

    /// Whether to enable execution of all rules. This option has lower precedence than
    /// rule and category overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// A list of rule-level overrides. This option has the highest precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ExecuteRuleActionParametersOverridesRule>>,

    /// A sensitivity level to set for all rules. This option has lower precedence than
    /// rule and category overrides and is only applicable for DDoS phases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<ExecuteRuleActionParametersOverridesSensitivityLevel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParametersCookieField {
    /// The name of the cookie.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParametersRawResponseField {
    /// The name of the response header.
    pub name: String,

    /// Whether to log duplicate values of the same header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_duplicates: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParametersRequestField {
    /// The name of the header.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParametersResponseField {
    /// The name of the response header.
    pub name: String,

    /// Whether to log duplicate values of the same header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_duplicates: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogCustomFieldRuleActionParametersTransformedRequestField {
    /// The name of the header.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleActionParametersFromList {
    /// An expression that evaluates to the list lookup key.
    pub key: String,

    /// The name of the list to match against.
    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleActionParametersFromValue {
    /// A URL to redirect the request to.
    pub target_url: RedirectRuleActionParametersFromValueTargetURL,

    /// Whether to keep the query string of the original request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_query_string: Option<bool>,

    /// The status code to use for the redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<RedirectRuleActionParametersFromValueStatusCode>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleActionParametersHeader {
    /// The operation to perform on the header.
    pub operation: RewriteRuleActionParametersHeadersOperation,

    /// An expression that evaluates to a value for the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// A static value for the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRuleActionParametersURI {
    /// Whether to propagate the rewritten URI to origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<bool>,

    /// This field can have the runtime type of
    /// [RewriteRuleActionParametersURIURIPathPath].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// This field can have the runtime type of
    /// [RewriteRuleActionParametersURIURIQueryQuery].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleActionParametersOrigin {
    /// A resolved host to route to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// A destination port to route to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRuleActionParametersSNI {
    /// A value to override the SNI to.
    pub value: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersBrowserTTL {
    /// The browser TTL mode.
    pub mode: SetCacheSettingsRuleActionParametersBrowserTTLMode,

    /// The browser TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKey {
    /// Whether to separate cached content based on the visitor's device type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_by_device_type: Option<bool>,

    /// Whether to protect from web cache deception attacks, while allowing static
    /// assets to be cached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_deception_armor: Option<bool>,

    /// Which components of the request are included or excluded from the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKey>,

    /// Whether to treat requests with the same query parameters the same, regardless of
    /// the order those query parameters are in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_strings_order: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheReserve {
    /// Whether Cache Reserve is enabled. If this is true and a request meets
    /// eligibility criteria, Cloudflare will write the resource to Cache Reserve.
    pub eligible: bool,

    /// The minimum file size eligible for storage in Cache Reserve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_file_size: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersEdgeTTL {
    /// The edge TTL mode.
    pub mode: SetCacheSettingsRuleActionParametersEdgeTTLMode,

    /// The edge TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,

    /// A list of TTLs to apply to specific status codes or status code ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code_ttl: Option<Vec<SetCacheSettingsRuleActionParametersEdgeTTLStatusCodeTTL>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersServeStale {
    /// Whether Cloudflare should disable serving stale content while getting the latest
    /// content from the origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_stale_while_updating: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfigRuleActionParametersAutominify {
    /// Whether to minify CSS files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<bool>,

    /// Whether to minify HTML files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<bool>,

    /// Whether to minify JavaScript files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleActionParametersOverridesCategory {
    /// The name of the category to override.
    pub category: String,

    /// The action to override rules in the category with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Whether to enable execution of rules in the category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The sensitivity level to use for rules in the category. This option is only
    /// applicable for DDoS phases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<ExecuteRuleActionParametersOverridesCategoriesSensitivityLevel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRuleActionParametersOverridesRule {
    /// The ID of the rule to override.
    pub id: String,

    /// The action to override the rule with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Whether to enable execution of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The score threshold to use for the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<i64>,

    /// The sensitivity level to use for the rule. This option is only applicable for
    /// DDoS phases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_level: Option<ExecuteRuleActionParametersOverridesRulesSensitivityLevel>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectRuleActionParametersFromValueTargetURL {
    /// An expression that evaluates to a URL to redirect the request to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// A URL to redirect the request to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKey {
    /// Which cookies to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyCookie>,

    /// Which headers to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyHeader>,

    /// How to use the host in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyHost>,

    /// Which query string parameters to include in or exclude from the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryString>,

    /// How to use characteristics of the request user agent in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyUser>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersEdgeTTLStatusCodeTTL {
    /// The time to cache the response for (in seconds). A value of 0 is equivalent to
    /// setting the cache control header with the value "no-cache". A value of -1 is
    /// equivalent to setting the cache control header with the value of "no-store".
    pub value: i64,

    /// A single status code to apply the TTL to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,

    /// A range of status codes to apply the TTL to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code_range: Option<SetCacheSettingsRuleActionParametersEdgeTTLStatusCodeTTLStatusCodeRange>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyCookie {
    /// A list of cookies to check for the presence of. The presence of these cookies is
    /// included in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_presence: Option<Vec<String>>,

    /// A list of cookies to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyHeader {
    /// A list of headers to check for the presence of. The presence of these headers is
    /// included in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_presence: Option<Vec<String>>,

    /// A mapping of header names to a list of values. If a header is present in the
    /// request and contains any of the values provided, its value is included in the
    /// cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<::std::collections::HashMap<String, Vec<String>>>,

    /// Whether to exclude the origin header in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_origin: Option<bool>,

    /// A list of headers to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyHost {
    /// Whether to use the resolved host in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryString {
    /// Which query string parameters to exclude from the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringExclude>,

    /// Which query string parameters to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringInclude>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyUser {
    /// Whether to use the user agent's device type in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<bool>,

    /// Whether to use the user agents's country in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,

    /// Whether to use the user agent's language in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersEdgeTTLStatusCodeTTLStatusCodeRange {
    /// The lower bound of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// The upper bound of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringExclude {
    /// Whether to exclude all query string parameters from the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringExcludeAll>,

    /// A list of query string parameters to exclude from the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringInclude {
    /// Whether to include all query string parameters in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<SetCacheSettingsRuleActionParametersCacheKeyCustomKeyQueryStringIncludeAll>,

    /// A list of query string parameters to include in the cache key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
    #[serde(rename = "managed")]
    KindManaged,
    #[serde(rename = "custom")]
    KindCustom,
    #[serde(rename = "root")]
    KindRoot,
    #[serde(rename = "zone")]
    KindZone,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Phase {
    #[serde(rename = "ddos_l4")]
    PhaseDDoSL4,
    #[serde(rename = "ddos_l7")]
    PhaseDDoSL7,
    #[serde(rename = "http_config_settings")]
    PhaseHTTPConfigSettings,
    #[serde(rename = "http_custom_errors")]
    PhaseHTTPCustomErrors,
    #[serde(rename = "http_log_custom_fields")]
    PhaseHTTPLogCustomFields,
    #[serde(rename = "http_ratelimit")]
    PhaseHTTPRatelimit,
    #[serde(rename = "http_request_cache_settings")]
    PhaseHTTPRequestCacheSettings,
    #[serde(rename = "http_request_dynamic_redirect")]
    PhaseHTTPRequestDynamicRedirect,
    #[serde(rename = "http_request_firewall_custom")]
    PhaseHTTPRequestFirewallCustom,
    #[serde(rename = "http_request_firewall_managed")]
    PhaseHTTPRequestFirewallManaged,
    #[serde(rename = "http_request_late_transform")]
    PhaseHTTPRequestLateTransform,
    #[serde(rename = "http_request_origin")]
    PhaseHTTPRequestOrigin,
    #[serde(rename = "http_request_redirect")]
    PhaseHTTPRequestRedirect,
    #[serde(rename = "http_request_sanitize")]
    PhaseHTTPRequestSanitize,
    #[serde(rename = "http_request_sbfm")]
    PhaseHTTPRequestSBFM,
    #[serde(rename = "http_request_transform")]
    PhaseHTTPRequestTransform,
    #[serde(rename = "http_response_compression")]
    PhaseHTTPResponseCompression,
    #[serde(rename = "http_response_firewall_managed")]
    PhaseHTTPResponseFirewallManaged,
    #[serde(rename = "http_response_headers_transform")]
    PhaseHTTPResponseHeadersTransform,
    #[serde(rename = "magic_transit")]
    PhaseMagicTransit,
    #[serde(rename = "magic_transit_ids_managed")]
    PhaseMagicTransitIDsManaged,
    #[serde(rename = "magic_transit_managed")]
    PhaseMagicTransitManaged,
    #[serde(rename = "magic_transit_ratelimit")]
    PhaseMagicTransitRatelimit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockRuleAction {
    #[serde(rename = "block")]
    BlockRuleActionBlock,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressResponseRuleAction {
    #[serde(rename = "compress_response")]
    CompressResponseRuleActionCompressResponse,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DDoSDynamicRuleAction {
    #[serde(rename = "ddos_dynamic")]
    DDoSDynamicRuleActionDDoSDynamic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecuteRuleAction {
    #[serde(rename = "execute")]
    ExecuteRuleActionExecute,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForceConnectionCloseRuleAction {
    #[serde(rename = "force_connection_close")]
    ForceConnectionCloseRuleActionForceConnectionClose,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogCustomFieldRuleAction {
    #[serde(rename = "log_custom_field")]
    LogCustomFieldRuleActionLogCustomField,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogRuleAction {
    #[serde(rename = "log")]
    LogRuleActionLog,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ManagedChallengeRuleAction {
    #[serde(rename = "managed_challenge")]
    ManagedChallengeRuleActionManagedChallenge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RedirectRuleAction {
    #[serde(rename = "redirect")]
    RedirectRuleActionRedirect,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewriteRuleAction {
    #[serde(rename = "rewrite")]
    RewriteRuleActionRewrite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RouteRuleAction {
    #[serde(rename = "route")]
    RouteRuleActionRoute,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScoreRuleAction {
    #[serde(rename = "score")]
    ScoreRuleActionScore,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServeErrorRuleAction {
    #[serde(rename = "serve_error")]
    ServeErrorRuleActionServeError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetCacheSettingsRuleAction {
    #[serde(rename = "set_cache_settings")]
    SetCacheSettingsRuleActionSetCacheSettings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleAction {
    #[serde(rename = "set_config")]
    SetConfigRuleActionSetConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkipRuleAction {
    #[serde(rename = "skip")]
    SkipRuleActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhaseUpdateResponseRulesAction {
    #[serde(rename = "block")]
    PhaseUpdateResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    PhaseUpdateResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    PhaseUpdateResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    PhaseUpdateResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    PhaseUpdateResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    PhaseUpdateResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    PhaseUpdateResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    PhaseUpdateResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    PhaseUpdateResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    PhaseUpdateResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    PhaseUpdateResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    PhaseUpdateResponseRulesActionRewrite,
    #[serde(rename = "route")]
    PhaseUpdateResponseRulesActionRoute,
    #[serde(rename = "score")]
    PhaseUpdateResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    PhaseUpdateResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    PhaseUpdateResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    PhaseUpdateResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    PhaseUpdateResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhaseGetResponseRulesAction {
    #[serde(rename = "block")]
    PhaseGetResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    PhaseGetResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    PhaseGetResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    PhaseGetResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    PhaseGetResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    PhaseGetResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    PhaseGetResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    PhaseGetResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    PhaseGetResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    PhaseGetResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    PhaseGetResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    PhaseGetResponseRulesActionRewrite,
    #[serde(rename = "route")]
    PhaseGetResponseRulesActionRoute,
    #[serde(rename = "score")]
    PhaseGetResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    PhaseGetResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    PhaseGetResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    PhaseGetResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    PhaseGetResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhaseVersionGetResponseRulesAction {
    #[serde(rename = "block")]
    PhaseVersionGetResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    PhaseVersionGetResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    PhaseVersionGetResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    PhaseVersionGetResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    PhaseVersionGetResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    PhaseVersionGetResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    PhaseVersionGetResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    PhaseVersionGetResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    PhaseVersionGetResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    PhaseVersionGetResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    PhaseVersionGetResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    PhaseVersionGetResponseRulesActionRewrite,
    #[serde(rename = "route")]
    PhaseVersionGetResponseRulesActionRoute,
    #[serde(rename = "score")]
    PhaseVersionGetResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    PhaseVersionGetResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    PhaseVersionGetResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    PhaseVersionGetResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    PhaseVersionGetResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServeErrorRuleActionParametersContentType {
    #[serde(rename = "application/json")]
    ServeErrorRuleActionParametersContentTypeApplicationJson,
    #[serde(rename = "text/html")]
    ServeErrorRuleActionParametersContentTypeTextHTML,
    #[serde(rename = "text/plain")]
    ServeErrorRuleActionParametersContentTypeTextPlain,
    #[serde(rename = "text/xml")]
    ServeErrorRuleActionParametersContentTypeTextXml,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleActionParametersPolish {
    #[serde(rename = "off")]
    SetConfigRuleActionParametersPolishOff,
    #[serde(rename = "lossless")]
    SetConfigRuleActionParametersPolishLossless,
    #[serde(rename = "lossy")]
    SetConfigRuleActionParametersPolishLossy,
    #[serde(rename = "webp")]
    SetConfigRuleActionParametersPolishWebP,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleActionParametersRequestBodyBuffering {
    #[serde(rename = "none")]
    SetConfigRuleActionParametersRequestBodyBufferingNone,
    #[serde(rename = "standard")]
    SetConfigRuleActionParametersRequestBodyBufferingStandard,
    #[serde(rename = "full")]
    SetConfigRuleActionParametersRequestBodyBufferingFull,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleActionParametersResponseBodyBuffering {
    #[serde(rename = "none")]
    SetConfigRuleActionParametersResponseBodyBufferingNone,
    #[serde(rename = "standard")]
    SetConfigRuleActionParametersResponseBodyBufferingStandard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleActionParametersSecurityLevel {
    #[serde(rename = "off")]
    SetConfigRuleActionParametersSecurityLevelOff,
    #[serde(rename = "essentially_off")]
    SetConfigRuleActionParametersSecurityLevelEssentiallyOff,
    #[serde(rename = "low")]
    SetConfigRuleActionParametersSecurityLevelLow,
    #[serde(rename = "medium")]
    SetConfigRuleActionParametersSecurityLevelMedium,
    #[serde(rename = "high")]
    SetConfigRuleActionParametersSecurityLevelHigh,
    #[serde(rename = "under_attack")]
    SetConfigRuleActionParametersSecurityLevelUnderAttack,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetConfigRuleActionParametersSSL {
    #[serde(rename = "off")]
    SetConfigRuleActionParametersSSLOff,
    #[serde(rename = "flexible")]
    SetConfigRuleActionParametersSSLFlexible,
    #[serde(rename = "full")]
    SetConfigRuleActionParametersSSLFull,
    #[serde(rename = "strict")]
    SetConfigRuleActionParametersSSLStrict,
    #[serde(rename = "origin_pull")]
    SetConfigRuleActionParametersSSLOriginPull,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkipRuleActionParametersPhase {
    #[serde(rename = "current")]
    SkipRuleActionParametersPhaseCurrent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkipRuleActionParametersProduct {
    #[serde(rename = "bic")]
    SkipRuleActionParametersProductBIC,
    #[serde(rename = "hot")]
    SkipRuleActionParametersProductHot,
    #[serde(rename = "rateLimit")]
    SkipRuleActionParametersProductRateLimit,
    #[serde(rename = "securityLevel")]
    SkipRuleActionParametersProductSecurityLevel,
    #[serde(rename = "uaBlock")]
    SkipRuleActionParametersProductUABlock,
    #[serde(rename = "waf")]
    SkipRuleActionParametersProductWAF,
    #[serde(rename = "zoneLockdown")]
    SkipRuleActionParametersProductZoneLockdown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkipRuleActionParametersRuleset {
    #[serde(rename = "current")]
    SkipRuleActionParametersRulesetCurrent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleNewResponseRulesAction {
    #[serde(rename = "block")]
    RuleNewResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RuleNewResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RuleNewResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RuleNewResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RuleNewResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RuleNewResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RuleNewResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RuleNewResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RuleNewResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RuleNewResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RuleNewResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RuleNewResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RuleNewResponseRulesActionRoute,
    #[serde(rename = "score")]
    RuleNewResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RuleNewResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RuleNewResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RuleNewResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RuleNewResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleDeleteResponseRulesAction {
    #[serde(rename = "block")]
    RuleDeleteResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RuleDeleteResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RuleDeleteResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RuleDeleteResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RuleDeleteResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RuleDeleteResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RuleDeleteResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RuleDeleteResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RuleDeleteResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RuleDeleteResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RuleDeleteResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RuleDeleteResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RuleDeleteResponseRulesActionRoute,
    #[serde(rename = "score")]
    RuleDeleteResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RuleDeleteResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RuleDeleteResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RuleDeleteResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RuleDeleteResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleEditResponseRulesAction {
    #[serde(rename = "block")]
    RuleEditResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RuleEditResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RuleEditResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RuleEditResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RuleEditResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RuleEditResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RuleEditResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RuleEditResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RuleEditResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RuleEditResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RuleEditResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RuleEditResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RuleEditResponseRulesActionRoute,
    #[serde(rename = "score")]
    RuleEditResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RuleEditResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RuleEditResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RuleEditResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RuleEditResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RulesetNewResponseRulesAction {
    #[serde(rename = "block")]
    RulesetNewResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RulesetNewResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RulesetNewResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RulesetNewResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RulesetNewResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RulesetNewResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RulesetNewResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RulesetNewResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RulesetNewResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RulesetNewResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RulesetNewResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RulesetNewResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RulesetNewResponseRulesActionRoute,
    #[serde(rename = "score")]
    RulesetNewResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RulesetNewResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RulesetNewResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RulesetNewResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RulesetNewResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RulesetUpdateResponseRulesAction {
    #[serde(rename = "block")]
    RulesetUpdateResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RulesetUpdateResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RulesetUpdateResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RulesetUpdateResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RulesetUpdateResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RulesetUpdateResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RulesetUpdateResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RulesetUpdateResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RulesetUpdateResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RulesetUpdateResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RulesetUpdateResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RulesetUpdateResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RulesetUpdateResponseRulesActionRoute,
    #[serde(rename = "score")]
    RulesetUpdateResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RulesetUpdateResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RulesetUpdateResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RulesetUpdateResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RulesetUpdateResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RulesetGetResponseRulesAction {
    #[serde(rename = "block")]
    RulesetGetResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    RulesetGetResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    RulesetGetResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    RulesetGetResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    RulesetGetResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    RulesetGetResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    RulesetGetResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    RulesetGetResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    RulesetGetResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    RulesetGetResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    RulesetGetResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    RulesetGetResponseRulesActionRewrite,
    #[serde(rename = "route")]
    RulesetGetResponseRulesActionRoute,
    #[serde(rename = "score")]
    RulesetGetResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    RulesetGetResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    RulesetGetResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    RulesetGetResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    RulesetGetResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VersionGetResponseRulesAction {
    #[serde(rename = "block")]
    VersionGetResponseRulesActionBlock,
    #[serde(rename = "challenge")]
    VersionGetResponseRulesActionChallenge,
    #[serde(rename = "compress_response")]
    VersionGetResponseRulesActionCompressResponse,
    #[serde(rename = "ddos_dynamic")]
    VersionGetResponseRulesActionDDoSDynamic,
    #[serde(rename = "execute")]
    VersionGetResponseRulesActionExecute,
    #[serde(rename = "force_connection_close")]
    VersionGetResponseRulesActionForceConnectionClose,
    #[serde(rename = "js_challenge")]
    VersionGetResponseRulesActionJSChallenge,
    #[serde(rename = "log")]
    VersionGetResponseRulesActionLog,
    #[serde(rename = "log_custom_field")]
    VersionGetResponseRulesActionLogCustomField,
    #[serde(rename = "managed_challenge")]
    VersionGetResponseRulesActionManagedChallenge,
    #[serde(rename = "redirect")]
    VersionGetResponseRulesActionRedirect,
    #[serde(rename = "rewrite")]
    VersionGetResponseRulesActionRewrite,
    #[serde(rename = "route")]
    VersionGetResponseRulesActionRoute,
    #[serde(rename = "score")]
    VersionGetResponseRulesActionScore,
    #[serde(rename = "serve_error")]
    VersionGetResponseRulesActionServeError,
    #[serde(rename = "set_cache_settings")]
    VersionGetResponseRulesActionSetCacheSettings,
    #[serde(rename = "set_config")]
    VersionGetResponseRulesActionSetConfig,
    #[serde(rename = "skip")]
    VersionGetResponseRulesActionSkip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressResponseRuleActionParametersAlgorithmsName {
    #[serde(rename = "none")]
    CompressResponseRuleActionParametersAlgorithmsNameNone,
    #[serde(rename = "auto")]
    CompressResponseRuleActionParametersAlgorithmsNameAuto,
    #[serde(rename = "default")]
    CompressResponseRuleActionParametersAlgorithmsNameDefault,
    #[serde(rename = "gzip")]
    CompressResponseRuleActionParametersAlgorithmsNameGzip,
    #[serde(rename = "brotli")]
    CompressResponseRuleActionParametersAlgorithmsNameBrotli,
    #[serde(rename = "zstd")]
    CompressResponseRuleActionParametersAlgorithmsNameZstd,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecuteRuleActionParametersOverridesSensitivityLevel {
    #[serde(rename = "default")]
    ExecuteRuleActionParametersOverridesSensitivityLevelDefault,
    #[serde(rename = "medium")]
    ExecuteRuleActionParametersOverridesSensitivityLevelMedium,
    #[serde(rename = "low")]
    ExecuteRuleActionParametersOverridesSensitivityLevelLow,
    #[serde(rename = "eoff")]
    ExecuteRuleActionParametersOverridesSensitivityLevelEoff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RedirectRuleActionParametersFromValueStatusCode {
    RedirectRuleActionParametersFromValueStatusCode301 = 301,
    RedirectRuleActionParametersFromValueStatusCode302 = 302,
    RedirectRuleActionParametersFromValueStatusCode303 = 303,
    RedirectRuleActionParametersFromValueStatusCode307 = 307,
    RedirectRuleActionParametersFromValueStatusCode308 = 308,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewriteRuleActionParametersHeadersOperation {
    #[serde(rename = "add")]
    RewriteRuleActionParametersHeadersOperationAdd,
    #[serde(rename = "set")]
    RewriteRuleActionParametersHeadersOperationSet,
    #[serde(rename = "remove")]
    RewriteRuleActionParametersHeadersOperationRemove,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetCacheSettingsRuleActionParametersBrowserTTLMode {
    #[serde(rename = "respect_origin")]
    SetCacheSettingsRuleActionParametersBrowserTTLModeRespectOrigin,
    #[serde(rename = "bypass_by_default")]
    SetCacheSettingsRuleActionParametersBrowserTTLModeBypassByDefault,
    #[serde(rename = "override_origin")]
    SetCacheSettingsRuleActionParametersBrowserTTLModeOverrideOrigin,
    #[serde(rename = "bypass")]
    SetCacheSettingsRuleActionParametersBrowserTTLModeBypass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetCacheSettingsRuleActionParametersEdgeTTLMode {
    #[serde(rename = "respect_origin")]
    SetCacheSettingsRuleActionParametersEdgeTTLModeRespectOrigin,
    #[serde(rename = "bypass_by_default")]
    SetCacheSettingsRuleActionParametersEdgeTTLModeBypassByDefault,
    #[serde(rename = "override_origin")]
    SetCacheSettingsRuleActionParametersEdgeTTLModeOverrideOrigin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecuteRuleActionParametersOverridesCategoriesSensitivityLevel {
    #[serde(rename = "default")]
    ExecuteRuleActionParametersOverridesCategoriesSensitivityLevelDefault,
    #[serde(rename = "medium")]
    ExecuteRuleActionParametersOverridesCategoriesSensitivityLevelMedium,
    #[serde(rename = "low")]
    ExecuteRuleActionParametersOverridesCategoriesSensitivityLevelLow,
    #[serde(rename = "eoff")]
    ExecuteRuleActionParametersOverridesCategoriesSensitivityLevelEoff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecuteRuleActionParametersOverridesRulesSensitivityLevel {
    #[serde(rename = "default")]
    ExecuteRuleActionParametersOverridesRulesSensitivityLevelDefault,
    #[serde(rename = "medium")]
    ExecuteRuleActionParametersOverridesRulesSensitivityLevelMedium,
    #[serde(rename = "low")]
    ExecuteRuleActionParametersOverridesRulesSensitivityLevelLow,
    #[serde(rename = "eoff")]
    ExecuteRuleActionParametersOverridesRulesSensitivityLevelEoff,
}

