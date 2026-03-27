#[cfg(any(feature = "full", feature = "with-firewall"))]
mod firewall_bindings {

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleCIDRConfiguration {
        /// The configuration target. You must set the target to `ip_range` when specifying
        /// an IP address range in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleCIDRConfigurationTarget>,

        /// The IP address range to match. You can only use prefix lengths `/16` and `/24`
        /// for IPv4 ranges, and prefix lengths `/32`, `/48`, and `/64` for IPv6 ranges.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleCIDRConfigurationParam {
        /// The configuration target. You must set the target to `ip_range` when specifying
        /// an IP address range in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The IP address range to match. You can only use prefix lengths `/16` and `/24`
        /// for IPv4 ranges, and prefix lengths `/32`, `/48`, and `/64` for IPv6 ranges.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleIPConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleIPConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleIPConfigurationParam {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ASNConfiguration {
        /// The configuration target. You must set the target to `asn` when specifying an
        /// Autonomous System Number (ASN) in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<ASNConfigurationTarget>,

        /// The AS number to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ASNConfigurationParam {
        /// The configuration target. You must set the target to `asn` when specifying an
        /// Autonomous System Number (ASN) in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The AS number to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CountryConfiguration {
        /// The configuration target. You must set the target to `country` when specifying a
        /// country code in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<CountryConfigurationTarget>,

        /// The two-letter ISO-3166-1 alpha-2 code to match. For more information, refer to
        /// [IP Access rules: Parameters](https://developers.cloudflare.com/waf/tools/ip-access-rules/parameters/#country).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CountryConfigurationParam {
        /// The configuration target. You must set the target to `country` when specifying a
        /// country code in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The two-letter ISO-3166-1 alpha-2 code to match. For more information, refer to
        /// [IP Access rules: Parameters](https://developers.cloudflare.com/waf/tools/ip-access-rules/parameters/#country).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleNewResponse {
        /// The unique identifier of the IP Access rule.
        pub id: String,

        /// The available actions that a rule can apply to a matched request.
        pub allowed_modes: Vec<AccessRuleNewResponseAllowedMode>,

        /// The rule configuration.
        pub configuration: AccessRuleNewResponseConfiguration,

        /// The action to apply to a matched request.
        pub mode: AccessRuleNewResponseMode,

        /// The timestamp of when the rule was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// The timestamp of when the rule was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,

        /// All zones owned by the user will have the rule applied.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scope: Option<AccessRuleNewResponseScope>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleListResponse {
        /// The unique identifier of the IP Access rule.
        pub id: String,

        /// The available actions that a rule can apply to a matched request.
        pub allowed_modes: Vec<AccessRuleListResponseAllowedMode>,

        /// The rule configuration.
        pub configuration: AccessRuleListResponseConfiguration,

        /// The action to apply to a matched request.
        pub mode: AccessRuleListResponseMode,

        /// The timestamp of when the rule was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// The timestamp of when the rule was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,

        /// All zones owned by the user will have the rule applied.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scope: Option<AccessRuleListResponseScope>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleDeleteResponse {
        /// Defines an identifier.
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleEditResponse {
        /// The unique identifier of the IP Access rule.
        pub id: String,

        /// The available actions that a rule can apply to a matched request.
        pub allowed_modes: Vec<AccessRuleEditResponseAllowedMode>,

        /// The rule configuration.
        pub configuration: AccessRuleEditResponseConfiguration,

        /// The action to apply to a matched request.
        pub mode: AccessRuleEditResponseMode,

        /// The timestamp of when the rule was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// The timestamp of when the rule was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,

        /// All zones owned by the user will have the rule applied.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scope: Option<AccessRuleEditResponseScope>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleGetResponse {
        /// The unique identifier of the IP Access rule.
        pub id: String,

        /// The available actions that a rule can apply to a matched request.
        pub allowed_modes: Vec<AccessRuleGetResponseAllowedMode>,

        /// The rule configuration.
        pub configuration: AccessRuleGetResponseConfiguration,

        /// The action to apply to a matched request.
        pub mode: AccessRuleGetResponseMode,

        /// The timestamp of when the rule was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// The timestamp of when the rule was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,

        /// All zones owned by the user will have the rule applied.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scope: Option<AccessRuleGetResponseScope>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleNewParams {
        /// The rule configuration.
        pub configuration: String,

        /// The action to apply to a matched request.
        pub mode: String,

        /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleListParams {
        /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<String>,

        /// Defines the direction used to sort returned rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<String>,

        /// Defines the search requirements. When set to `all`, all the search requirements
        /// must match. When set to `any`, only one of the search requirements has to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#match: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,

        /// Defines the string to search for in the notes of existing IP Access rules.
        /// Notes: For example, the string 'attack' would match IP Access rules with notes
        /// 'Attack 26/02' and 'Attack 27/02'. The search is case insensitive.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,

        /// Defines the field used to sort returned rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<String>,

        /// Defines the requested page within paginated list of results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Defines the maximum number of results requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleDeleteParams {
        /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleEditParams {
        /// The rule configuration.
        pub configuration: String,

        /// The action to apply to a matched request.
        pub mode: String,

        /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// An informative summary of the rule, typically used as a reminder or explanation.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub notes: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleGetParams {
        /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Lockdown {
        /// The unique identifier of the Zone Lockdown rule.
        pub id: String,

        /// A list of IP addresses or CIDR ranges that will be allowed to access the URLs
        /// specified in the Zone Lockdown rule. You can include any number of `ip` or
        /// `ip_range` configurations.
        pub configurations: Vec<ConfigurationItem>,

        /// The timestamp of when the rule was created.
        pub created_on: DateTime<Utc>,

        /// An informative summary of the rule.
        pub description: String,

        /// The timestamp of when the rule was last modified.
        pub modified_on: DateTime<Utc>,

        /// When true, indicates that the rule is currently paused.
        pub paused: bool,

        /// The URLs to include in the rule definition. You can use wildcards. Each entered
        /// URL will be escaped before use, which means you can only use simple wildcard
        /// patterns.
        #[serde(rename = "urls")]
        pub ur_ls: Vec<LockdownURL>,
    }

    pub type LockdownURL = String;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigurationItem {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the Zone Lockdown rule.
        pub target: Option<ConfigurationItemTarget>,
        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        pub value: Option<String>,
    }

    /// The configuration target. You must set the target to `ip` when specifying an IP
    /// address in the Zone Lockdown rule.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConfigurationItemTarget {
        #[serde(rename = "ip")]
        IP,
        #[serde(rename = "ip_range")]
        IPRange,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownCIDRConfiguration {
        /// The configuration target. You must set the target to `ip_range` when specifying
        /// an IP address range in the Zone Lockdown rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<LockdownCIDRConfigurationTarget>,

        /// The IP address range to match. You can only use prefix lengths `/16` and `/24`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownCIDRConfigurationParam {
        /// The configuration target. You must set the target to `ip_range` when specifying
        /// an IP address range in the Zone Lockdown rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The IP address range to match. You can only use prefix lengths `/16` and `/24`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownIPConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the Zone Lockdown rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<LockdownIPConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownIPConfigurationParam {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the Zone Lockdown rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownDeleteResponse {
        /// The unique identifier of the Zone Lockdown rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownNewParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// A list of IP addresses or CIDR ranges that will be allowed to access the URLs
        /// specified in the Zone Lockdown rule. You can include any number of `ip` or
        /// `ip_range` configurations.
        pub configurations: String,

        /// The URLs to include in the current WAF override. You can use wildcards. Each
        /// entered URL will be escaped before use, which means you can only use simple
        /// wildcard patterns.
        #[serde(rename = "urls")]
        pub ur_ls: String,

        /// An informative summary of the rule. This value is sanitized and any tags will be
        /// removed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<String>,

        /// The priority of the rule to control the processing order. A lower number
        /// indicates higher priority. If not provided, any rules with a configured priority
        /// will be processed before rules without a priority.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownUpdateParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// A list of IP addresses or CIDR ranges that will be allowed to access the URLs
        /// specified in the Zone Lockdown rule. You can include any number of `ip` or
        /// `ip_range` configurations.
        pub configurations: String,

        /// The URLs to include in the current WAF override. You can use wildcards. Each
        /// entered URL will be escaped before use, which means you can only use simple
        /// wildcard patterns.
        #[serde(rename = "urls")]
        pub ur_ls: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownListParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The timestamp of when the rule was created.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<String>,

        /// A string to search for in the description of existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// A string to search for in the description of existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description_search: Option<String>,

        /// A single IP address to search for in existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip: Option<String>,

        /// A single IP address range to search for in existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip_range_search: Option<String>,

        /// A single IP address to search for in existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip_search: Option<String>,

        /// The timestamp of when the rule was last modified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// The maximum number of results per page. You can only set the value to `1` or to
        /// a multiple of 5 such as `5`, `10`, `15`, or `20`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,

        /// The priority of the rule to control the processing order. A lower number
        /// indicates higher priority. If not provided, any rules with a configured priority
        /// will be processed before rules without a priority.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,

        /// A single URI to search for in the list of URLs of existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uri_search: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownDeleteParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LockdownGetParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeletedFilter {
        /// The unique identifier of the filter.
        pub id: String,

        /// When true, indicates that the firewall rule was deleted.
        pub deleted: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FirewallRule {
        /// The unique identifier of the firewall rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The action to apply to a matched request. The `log` action is only available on
        /// an Enterprise plan.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<crate::rate_limits::Action>,

        /// An informative summary of the firewall rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter: Option<FirewallRuleFilter>,

        /// When true, indicates that the firewall rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,

        /// The priority of the rule. Optional value used to define the processing order. A
        /// lower number indicates a higher priority. If not provided, rules with a defined
        /// priority will be processed before rules without a priority.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub products: Option<Vec<Product>>,

        /// A short reference tag. Allows you to select related firewall rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#ref: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleNewParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The action to perform when the threshold of matched traffic within the
        /// configured period is exceeded.
        pub action: String,

        pub filter: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleUpdateParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The action to perform when the threshold of matched traffic within the
        /// configured period is exceeded.
        pub action: String,

        pub filter: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleListParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The unique identifier of the firewall rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The action to search for. Must be an exact match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,

        /// A case-insensitive string to find in the description.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// When true, indicates that the firewall rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<String>,

        /// Number of firewall rules per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleDeleteParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleBulkDeleteParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleBulkEditParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub body: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleBulkUpdateParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub body: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleEditParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RuleGetParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleNewResponse {
        /// The unique identifier of the User Agent Blocking rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The configuration object for the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<UARuleNewResponseConfiguration>,

        /// An informative summary of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<UARuleNewResponseMode>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleUpdateResponse {
        /// The unique identifier of the User Agent Blocking rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The configuration object for the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<UARuleUpdateResponseConfiguration>,

        /// An informative summary of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<UARuleUpdateResponseMode>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleListResponse {
        /// The unique identifier of the User Agent Blocking rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The configuration object for the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<UARuleListResponseConfiguration>,

        /// An informative summary of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<UARuleListResponseMode>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleDeleteResponse {
        /// The unique identifier of the User Agent Blocking rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The configuration object for the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<UARuleDeleteResponseConfiguration>,

        /// An informative summary of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<UARuleDeleteResponseMode>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleGetResponse {
        /// The unique identifier of the User Agent Blocking rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The configuration object for the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configuration: Option<UARuleGetResponseConfiguration>,

        /// An informative summary of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The action to apply to a matched request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<UARuleGetResponseMode>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleNewParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub configuration: String,

        /// The action to apply to a matched request.
        pub mode: String,

        /// An informative summary of the rule. This value is sanitized and any tags will be
        /// removed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleUpdateParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The rule configuration.
        pub configuration: String,

        /// The action to apply to a matched request.
        pub mode: String,

        /// An informative summary of the rule. This value is sanitized and any tags will be
        /// removed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleListParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// A string to search for in the description of existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// Page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<String>,

        /// The maximum number of results per page. You can only set the value to `1` or to
        /// a multiple of 5 such as `5`, `10`, `15`, or `20`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,

        /// A string to search for in the user agent values of existing rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_agent: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleDeleteParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleGetParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Override {
        /// The unique identifier of the WAF override.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// An informative summary of the current URI-based WAF override.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// An object that allows you to enable or disable WAF rule groups for the current
        /// WAF override. Each key of this object must be the ID of a WAF rule group, and
        /// each value must be a valid WAF action (usually `default` or `disable`). When
        /// creating a new URI-based WAF override, you must provide a `groups` object or a
        /// `rules` object.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub groups: Option<::std::collections::HashMap<String, String>>,

        /// When true, indicates that the rule is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,

        /// The relative priority of the current URI-based WAF override when multiple
        /// overrides match a single URL. A lower number indicates higher priority. Higher
        /// priority overrides may overwrite values set by lower priority overrides.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<f64>,

        /// Specifies that, when a WAF rule matches, its configured action will be replaced
        /// by the action configured in this object.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rewrite_action: Option<RewriteAction>,

        /// An object that allows you to override the action of specific WAF rules. Each key
        /// of this object must be the ID of a WAF rule, and each value must be a valid WAF
        /// action. Unless you are disabling a rule, ensure that you also enable the rule
        /// group that this WAF rule belongs to. When creating a new URI-based WAF override,
        /// you must provide a `groups` object or a `rules` object.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rules: Option<::std::collections::HashMap<String, WAFRuleItem>>,

        /// The URLs to include in the current WAF override. You can use wildcards. Each
        /// entered URL will be escaped before use, which means you can only use simple
        /// wildcard patterns.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "urls")]
        pub ur_ls: Option<Vec<String>>,
    }

    /// The WAF rule action to apply.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum WAFRuleItem {
        #[serde(rename = "challenge")]
        Challenge,
        #[serde(rename = "block")]
        Block,
        #[serde(rename = "simulate")]
        Simulate,
        #[serde(rename = "disable")]
        Disable,
        #[serde(rename = "default")]
        Default,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewriteAction {
        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block: Option<RewriteActionBlock>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub challenge: Option<RewriteActionChallenge>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default: Option<RewriteActionDefault>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disable: Option<RewriteActionDisable>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub simulate: Option<RewriteActionSimulate>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewriteActionParam {
        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block: Option<String>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub challenge: Option<String>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default: Option<String>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disable: Option<String>,

        /// The WAF rule action to apply.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub simulate: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideDeleteResponse {
        /// The unique identifier of the WAF override.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideNewParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The URLs to include in the current WAF override. You can use wildcards. Each
        /// entered URL will be escaped before use, which means you can only use simple
        /// wildcard patterns.
        #[serde(rename = "urls")]
        pub ur_ls: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideUpdateParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Defines an identifier.
        pub id: String,

        /// Specifies that, when a WAF rule matches, its configured action will be replaced
        /// by the action configured in this object.
        pub rewrite_action: String,

        /// An object that allows you to override the action of specific WAF rules. Each key
        /// of this object must be the ID of a WAF rule, and each value must be a valid WAF
        /// action. Unless you are disabling a rule, ensure that you also enable the rule
        /// group that this WAF rule belongs to. When creating a new URI-based WAF override,
        /// you must provide a `groups` object or a `rules` object.
        pub rules: String,

        /// The URLs to include in the current WAF override. You can use wildcards. Each
        /// entered URL will be escaped before use, which means you can only use simple
        /// wildcard patterns.
        #[serde(rename = "urls")]
        pub ur_ls: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideListParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// The number of WAF overrides per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideDeleteParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFOverrideGetParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageGetResponse {
        /// This field can have the runtime type of [[]shared.ResponseInfo].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub errors: Option<String>,

        /// This field can have the runtime type of [[]shared.ResponseInfo].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub messages: Option<String>,

        /// This field can have the runtime type of [interface{}].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result: Option<String>,

        /// Defines whether the API call was successful.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageListParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The direction used to sort returned packages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<String>,

        /// When set to `all`, all the search requirements must match. When set to `any`,
        /// only one of the search requirements has to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#match: Option<String>,

        /// The name of the WAF package.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// The field used to sort returned packages.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<String>,

        /// The page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// The number of packages per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageGetParams {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Group {
        /// Defines the unique identifier of the rule group.
        pub id: String,

        /// Defines an informative summary of what the rule group does.
        pub description: String,

        /// Defines the state of the rules contained in the rule group. When `on`, the rules
        /// in the group are configurable/usable.
        pub mode: GroupMode,

        /// Defines the name of the rule group.
        pub name: String,

        /// Defines the number of rules in the current rule group.
        pub rules_count: f64,

        /// Defines the available states for the rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allowed_modes: Option<Vec<GroupAllowedMode>>,

        /// Defines the number of rules within the group that have been modified from their
        /// default configuration.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_rules_count: Option<f64>,

        /// Defines the unique identifier of a WAF package.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub package_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageGroupListParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Defines the direction used to sort returned rule groups.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<String>,

        /// Defines the condition for search requirements. When set to `all`, all the search
        /// requirements must match. When set to `any`, only one of the search requirements
        /// has to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#match: Option<String>,

        /// Defines the state of the rules contained in the rule group. When `on`, the rules
        /// in the group are configurable/usable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,

        /// Defines the name of the rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        /// Defines the field used to sort returned rule groups.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<String>,

        /// Defines the page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Defines the number of rule groups per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,

        /// Defines the number of rules in the current rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rules_count: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageGroupEditParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Defines the state of the rules contained in the rule group. When `on`, the rules
        /// in the group are configurable/usable.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageGroupGetParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFRuleGroup {
        /// Defines the unique identifier of the rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// Defines the name of the rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageRuleListResponse {
        /// Defines the unique identifier of the WAF rule.
        pub id: String,

        /// This field can have the runtime type of [[]AllowedModesAnomaly],
        /// [[]WAFPackageRuleListResponseWAFManagedRulesTraditionalDenyRuleAllowedMode],
        /// [[]WAFPackageRuleListResponseWAFManagedRulesTraditionalAllowRuleAllowedMode].
        pub allowed_modes: String,

        /// Defines the public description of the WAF rule.
        pub description: String,

        /// Defines the rule group to which the current WAF rule belongs.
        pub group: WAFRuleGroup,

        /// Defines the mode anomaly. When set to `on`, the current WAF rule will be used
        /// when evaluating the request. Applies to anomaly detection WAF rules.
        pub mode: AllowedModesAnomaly,

        /// Defines the unique identifier of a WAF package.
        pub package_id: String,

        /// Defines the order in which the individual WAF rule is executed within its rule
        /// group.
        pub priority: String,

        /// Defines the default action/mode of a rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode: Option<WAFPackageRuleListResponseDefaultMode>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageRuleEditResponse {
        /// Defines the unique identifier of the WAF rule.
        pub id: String,

        /// This field can have the runtime type of [[]AllowedModesAnomaly],
        /// [[]WAFPackageRuleEditResponseWAFManagedRulesTraditionalDenyRuleAllowedMode],
        /// [[]WAFPackageRuleEditResponseWAFManagedRulesTraditionalAllowRuleAllowedMode].
        pub allowed_modes: String,

        /// Defines the public description of the WAF rule.
        pub description: String,

        /// Defines the rule group to which the current WAF rule belongs.
        pub group: WAFRuleGroup,

        /// Defines the mode anomaly. When set to `on`, the current WAF rule will be used
        /// when evaluating the request. Applies to anomaly detection WAF rules.
        pub mode: AllowedModesAnomaly,

        /// Defines the unique identifier of a WAF package.
        pub package_id: String,

        /// Defines the order in which the individual WAF rule is executed within its rule
        /// group.
        pub priority: String,

        /// Defines the default action/mode of a rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode: Option<WAFPackageRuleEditResponseDefaultMode>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageRuleListParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Defines the public description of the WAF rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// Defines the direction used to sort returned rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub direction: Option<String>,

        /// Defines the unique identifier of the rule group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group_id: Option<String>,

        /// Defines the search requirements. When set to `all`, all the search requirements
        /// must match. When set to `any`, only one of the search requirements has to match.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#match: Option<String>,

        /// Defines the action/mode a rule has been overridden to perform.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,

        /// Defines the field used to sort returned rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<String>,

        /// Defines the page number of paginated results.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<String>,

        /// Defines the number of rules per page.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub per_page: Option<String>,

        /// Defines the order in which the individual WAF rule is executed within its rule
        /// group.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageRuleEditParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// Defines the mode/action of the rule when triggered. You must use a value from
        /// the `allowed_modes` array of the current rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WAFPackageRuleGetParams {
        /// Defines an identifier of a schema.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleNewResponseConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleNewResponseConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleNewResponseScope {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// Defines the scope of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AccessRuleNewResponseScopeType>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleListResponseConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleListResponseConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleListResponseScope {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// Defines the scope of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AccessRuleListResponseScopeType>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleEditResponseConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleEditResponseConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleEditResponseScope {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// Defines the scope of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AccessRuleEditResponseScopeType>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleGetResponseConfiguration {
        /// The configuration target. You must set the target to `ip` when specifying an IP
        /// address in the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<AccessRuleGetResponseConfigurationTarget>,

        /// The IP address to match. This address will be compared to the IP address of
        /// incoming requests.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessRuleGetResponseScope {
        /// Defines an identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// The contact email address of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,

        /// Defines the scope of the rule.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AccessRuleGetResponseScopeType>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FirewallRuleFilter {
        /// The unique identifier of the filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,

        /// When true, indicates that the firewall rule was deleted.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,

        /// An informative summary of the filter.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        /// The filter expression. For more information, refer to
        /// [Expressions](https://developers.cloudflare.com/ruleset-engine/rules-language/expressions/).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expression: Option<String>,

        /// When true, indicates that the filter is currently paused.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,

        /// A short reference tag. Allows you to select related filters.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#ref: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleNewResponseConfiguration {
        /// The configuration target for this rule. You must set the target to `ua` for User
        /// Agent Blocking rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The exact user agent string to match. This value will be compared to the
        /// received `User-Agent` HTTP header value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleUpdateResponseConfiguration {
        /// The configuration target for this rule. You must set the target to `ua` for User
        /// Agent Blocking rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The exact user agent string to match. This value will be compared to the
        /// received `User-Agent` HTTP header value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleListResponseConfiguration {
        /// The configuration target for this rule. You must set the target to `ua` for User
        /// Agent Blocking rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The exact user agent string to match. This value will be compared to the
        /// received `User-Agent` HTTP header value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleDeleteResponseConfiguration {
        /// The configuration target for this rule. You must set the target to `ua` for User
        /// Agent Blocking rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The exact user agent string to match. This value will be compared to the
        /// received `User-Agent` HTTP header value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UARuleGetResponseConfiguration {
        /// The configuration target for this rule. You must set the target to `ua` for User
        /// Agent Blocking rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,

        /// The exact user agent string to match. This value will be compared to the
        /// received `User-Agent` HTTP header value.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum Product {
        #[serde(rename = "zoneLockdown")]
        ProductZoneLockdown,
        #[serde(rename = "uaBlock")]
        ProductUABlock,
        #[serde(rename = "bic")]
        ProductBIC,
        #[serde(rename = "hot")]
        ProductHot,
        #[serde(rename = "securityLevel")]
        ProductSecurityLevel,
        #[serde(rename = "rateLimit")]
        ProductRateLimit,
        #[serde(rename = "waf")]
        ProductWAF,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AllowedModesAnomaly {
        #[serde(rename = "on")]
        AllowedModesAnomalyOn,
        #[serde(rename = "off")]
        AllowedModesAnomalyOff,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleCIDRConfigurationTarget {
        #[serde(rename = "ip_range")]
        AccessRuleCIDRConfigurationTargetIPRange,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleIPConfigurationTarget {
        #[serde(rename = "ip")]
        AccessRuleIPConfigurationTargetIP,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ASNConfigurationTarget {
        #[serde(rename = "asn")]
        ASNConfigurationTargetASN,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum CountryConfigurationTarget {
        #[serde(rename = "country")]
        CountryConfigurationTargetCountry,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleNewResponseAllowedMode {
        #[serde(rename = "block")]
        AccessRuleNewResponseAllowedModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleNewResponseAllowedModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleNewResponseAllowedModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleNewResponseAllowedModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleNewResponseAllowedModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleNewResponseMode {
        #[serde(rename = "block")]
        AccessRuleNewResponseModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleNewResponseModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleNewResponseModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleNewResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleNewResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleListResponseAllowedMode {
        #[serde(rename = "block")]
        AccessRuleListResponseAllowedModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleListResponseAllowedModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleListResponseAllowedModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleListResponseAllowedModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleListResponseAllowedModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleListResponseMode {
        #[serde(rename = "block")]
        AccessRuleListResponseModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleListResponseModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleListResponseModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleListResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleListResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleEditResponseAllowedMode {
        #[serde(rename = "block")]
        AccessRuleEditResponseAllowedModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleEditResponseAllowedModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleEditResponseAllowedModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleEditResponseAllowedModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleEditResponseAllowedModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleEditResponseMode {
        #[serde(rename = "block")]
        AccessRuleEditResponseModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleEditResponseModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleEditResponseModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleEditResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleEditResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleGetResponseAllowedMode {
        #[serde(rename = "block")]
        AccessRuleGetResponseAllowedModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleGetResponseAllowedModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleGetResponseAllowedModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleGetResponseAllowedModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleGetResponseAllowedModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleGetResponseMode {
        #[serde(rename = "block")]
        AccessRuleGetResponseModeBlock,
        #[serde(rename = "challenge")]
        AccessRuleGetResponseModeChallenge,
        #[serde(rename = "whitelist")]
        AccessRuleGetResponseModeWhitelist,
        #[serde(rename = "js_challenge")]
        AccessRuleGetResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        AccessRuleGetResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum LockdownCIDRConfigurationTarget {
        #[serde(rename = "ip_range")]
        LockdownCIDRConfigurationTargetIPRange,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum LockdownIPConfigurationTarget {
        #[serde(rename = "ip")]
        LockdownIPConfigurationTargetIP,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UARuleNewResponseMode {
        #[serde(rename = "block")]
        UARuleNewResponseModeBlock,
        #[serde(rename = "challenge")]
        UARuleNewResponseModeChallenge,
        #[serde(rename = "js_challenge")]
        UARuleNewResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        UARuleNewResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UARuleUpdateResponseMode {
        #[serde(rename = "block")]
        UARuleUpdateResponseModeBlock,
        #[serde(rename = "challenge")]
        UARuleUpdateResponseModeChallenge,
        #[serde(rename = "js_challenge")]
        UARuleUpdateResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        UARuleUpdateResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UARuleListResponseMode {
        #[serde(rename = "block")]
        UARuleListResponseModeBlock,
        #[serde(rename = "challenge")]
        UARuleListResponseModeChallenge,
        #[serde(rename = "js_challenge")]
        UARuleListResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        UARuleListResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UARuleDeleteResponseMode {
        #[serde(rename = "block")]
        UARuleDeleteResponseModeBlock,
        #[serde(rename = "challenge")]
        UARuleDeleteResponseModeChallenge,
        #[serde(rename = "js_challenge")]
        UARuleDeleteResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        UARuleDeleteResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum UARuleGetResponseMode {
        #[serde(rename = "block")]
        UARuleGetResponseModeBlock,
        #[serde(rename = "challenge")]
        UARuleGetResponseModeChallenge,
        #[serde(rename = "js_challenge")]
        UARuleGetResponseModeJSChallenge,
        #[serde(rename = "managed_challenge")]
        UARuleGetResponseModeManagedChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum RewriteActionBlock {
        #[serde(rename = "challenge")]
        RewriteActionBlockChallenge,
        #[serde(rename = "block")]
        RewriteActionBlockBlock,
        #[serde(rename = "simulate")]
        RewriteActionBlockSimulate,
        #[serde(rename = "disable")]
        RewriteActionBlockDisable,
        #[serde(rename = "default")]
        RewriteActionBlockDefault,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum RewriteActionChallenge {
        #[serde(rename = "challenge")]
        RewriteActionChallengeChallenge,
        #[serde(rename = "block")]
        RewriteActionChallengeBlock,
        #[serde(rename = "simulate")]
        RewriteActionChallengeSimulate,
        #[serde(rename = "disable")]
        RewriteActionChallengeDisable,
        #[serde(rename = "default")]
        RewriteActionChallengeDefault,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum RewriteActionDefault {
        #[serde(rename = "challenge")]
        RewriteActionDefaultChallenge,
        #[serde(rename = "block")]
        RewriteActionDefaultBlock,
        #[serde(rename = "simulate")]
        RewriteActionDefaultSimulate,
        #[serde(rename = "disable")]
        RewriteActionDefaultDisable,
        #[serde(rename = "default")]
        RewriteActionDefaultDefault,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum RewriteActionDisable {
        #[serde(rename = "challenge")]
        RewriteActionDisableChallenge,
        #[serde(rename = "block")]
        RewriteActionDisableBlock,
        #[serde(rename = "simulate")]
        RewriteActionDisableSimulate,
        #[serde(rename = "disable")]
        RewriteActionDisableDisable,
        #[serde(rename = "default")]
        RewriteActionDisableDefault,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum RewriteActionSimulate {
        #[serde(rename = "challenge")]
        RewriteActionSimulateChallenge,
        #[serde(rename = "block")]
        RewriteActionSimulateBlock,
        #[serde(rename = "simulate")]
        RewriteActionSimulateSimulate,
        #[serde(rename = "disable")]
        RewriteActionSimulateDisable,
        #[serde(rename = "default")]
        RewriteActionSimulateDefault,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum GroupMode {
        #[serde(rename = "on")]
        GroupModeOn,
        #[serde(rename = "off")]
        GroupModeOff,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum GroupAllowedMode {
        #[serde(rename = "on")]
        GroupAllowedModeOn,
        #[serde(rename = "off")]
        GroupAllowedModeOff,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum WAFPackageRuleListResponseDefaultMode {
        #[serde(rename = "disable")]
        WAFPackageRuleListResponseDefaultModeDisable,
        #[serde(rename = "simulate")]
        WAFPackageRuleListResponseDefaultModeSimulate,
        #[serde(rename = "block")]
        WAFPackageRuleListResponseDefaultModeBlock,
        #[serde(rename = "challenge")]
        WAFPackageRuleListResponseDefaultModeChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum WAFPackageRuleEditResponseDefaultMode {
        #[serde(rename = "disable")]
        WAFPackageRuleEditResponseDefaultModeDisable,
        #[serde(rename = "simulate")]
        WAFPackageRuleEditResponseDefaultModeSimulate,
        #[serde(rename = "block")]
        WAFPackageRuleEditResponseDefaultModeBlock,
        #[serde(rename = "challenge")]
        WAFPackageRuleEditResponseDefaultModeChallenge,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleNewResponseConfigurationTarget {
        #[serde(rename = "ip")]
        AccessRuleNewResponseConfigurationTargetIP,
        #[serde(rename = "ip6")]
        AccessRuleNewResponseConfigurationTargetIp6,
        #[serde(rename = "ip_range")]
        AccessRuleNewResponseConfigurationTargetIPRange,
        #[serde(rename = "asn")]
        AccessRuleNewResponseConfigurationTargetASN,
        #[serde(rename = "country")]
        AccessRuleNewResponseConfigurationTargetCountry,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleNewResponseScopeType {
        #[serde(rename = "user")]
        AccessRuleNewResponseScopeTypeUser,
        #[serde(rename = "organization")]
        AccessRuleNewResponseScopeTypeOrganization,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleListResponseConfigurationTarget {
        #[serde(rename = "ip")]
        AccessRuleListResponseConfigurationTargetIP,
        #[serde(rename = "ip6")]
        AccessRuleListResponseConfigurationTargetIp6,
        #[serde(rename = "ip_range")]
        AccessRuleListResponseConfigurationTargetIPRange,
        #[serde(rename = "asn")]
        AccessRuleListResponseConfigurationTargetASN,
        #[serde(rename = "country")]
        AccessRuleListResponseConfigurationTargetCountry,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleListResponseScopeType {
        #[serde(rename = "user")]
        AccessRuleListResponseScopeTypeUser,
        #[serde(rename = "organization")]
        AccessRuleListResponseScopeTypeOrganization,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleEditResponseConfigurationTarget {
        #[serde(rename = "ip")]
        AccessRuleEditResponseConfigurationTargetIP,
        #[serde(rename = "ip6")]
        AccessRuleEditResponseConfigurationTargetIp6,
        #[serde(rename = "ip_range")]
        AccessRuleEditResponseConfigurationTargetIPRange,
        #[serde(rename = "asn")]
        AccessRuleEditResponseConfigurationTargetASN,
        #[serde(rename = "country")]
        AccessRuleEditResponseConfigurationTargetCountry,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleEditResponseScopeType {
        #[serde(rename = "user")]
        AccessRuleEditResponseScopeTypeUser,
        #[serde(rename = "organization")]
        AccessRuleEditResponseScopeTypeOrganization,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleGetResponseConfigurationTarget {
        #[serde(rename = "ip")]
        AccessRuleGetResponseConfigurationTargetIP,
        #[serde(rename = "ip6")]
        AccessRuleGetResponseConfigurationTargetIp6,
        #[serde(rename = "ip_range")]
        AccessRuleGetResponseConfigurationTargetIPRange,
        #[serde(rename = "asn")]
        AccessRuleGetResponseConfigurationTargetASN,
        #[serde(rename = "country")]
        AccessRuleGetResponseConfigurationTargetCountry,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessRuleGetResponseScopeType {
        #[serde(rename = "user")]
        AccessRuleGetResponseScopeTypeUser,
        #[serde(rename = "organization")]
        AccessRuleGetResponseScopeTypeOrganization,
    }
}

#[cfg(any(feature = "full", feature = "with-firewall"))]
pub use firewall_bindings::*;

#[cfg(any(feature = "full", feature = "with-firewall"))]
impl Client {}
