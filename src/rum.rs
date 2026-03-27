#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RUMRule {
    /// The Web Analytics rule identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The hostname the rule will be applied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Whether the rule includes or excludes traffic from being measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,

    /// Whether the rule is paused or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,

    /// The paths the rule will be applied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleListResponse {
    /// A list of rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RUMRule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<RuleListResponseRuleset>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteResponse {
    /// The Web Analytics rule identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleBulkNewResponse {
    /// A list of rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RUMRule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<RuleBulkNewResponseRuleset>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Whether the rule includes or excludes traffic from being measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<String>,

    /// Whether the rule is paused or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Whether the rule includes or excludes traffic from being measured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<String>,

    /// Whether the rule is paused or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleBulkNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A list of rule identifiers to delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rules: Option<String>,

    /// A list of rules to create or update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    /// If enabled, the JavaScript snippet is automatically injected for orange-clouded
    /// sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_install: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// A list of rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RUMRule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<SiteRuleset>,

    /// The Web Analytics site identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_tag: Option<String>,

    /// The Web Analytics site token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_token: Option<String>,

    /// Encoded JavaScript snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoDeleteResponse {
    /// The Web Analytics site identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// If enabled, the JavaScript snippet is automatically injected for orange-clouded
    /// sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_install: Option<String>,

    /// The hostname to use for gray-clouded sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// The zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// If enabled, the JavaScript snippet is automatically injected for orange-clouded
    /// sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_install: Option<String>,

    /// Enables or disables RUM. This option can be used only when auto_install is set
    /// to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// The hostname to use for gray-clouded sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// If enabled, the JavaScript snippet will not be injected for visitors from the
    /// EU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite: Option<String>,

    /// The zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The property used to sort the list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,

    /// Current page within the paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of items to return per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfoGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleListResponseRuleset {
    /// The Web Analytics ruleset identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Whether the ruleset is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

    /// The zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleBulkNewResponseRuleset {
    /// The Web Analytics ruleset identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Whether the ruleset is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

    /// The zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteRuleset {
    /// The Web Analytics ruleset identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Whether the ruleset is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,

    /// The zone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

