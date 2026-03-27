#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Data layer compatibility mode enabled.
    #[serde(rename = "dataLayer")]
    pub data_layer: bool,

    /// The key for Zaraz debug mode.
    #[serde(rename = "debugKey")]
    pub debug_key: String,

    /// General Zaraz settings.
    pub settings: ConfigurationSettings,

    /// Tools set up under Zaraz configuration, where key is the alpha-numeric tool ID
    /// and value is the tool configuration object.
    pub tools: ::std::collections::HashMap<String, ConfigurationTool>,

    /// Triggers set up under Zaraz configuration, where key is the trigger
    /// alpha-numeric ID and value is the trigger configuration.
    pub triggers: ::std::collections::HashMap<String, ConfigurationTrigger>,

    /// Variables set up under Zaraz configuration, where key is the variable
    /// alpha-numeric ID and value is the variable configuration. Values of variables of
    /// type secret are not included.
    pub variables: ::std::collections::HashMap<String, ConfigurationVariable>,

    /// Zaraz internal version of the config.
    #[serde(rename = "zarazVersion")]
    pub zaraz_version: i64,

    /// Cloudflare Monitoring settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<ConfigurationAnalytics>,

    /// Consent management configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<ConfigurationConsent>,

    /// Single Page Application support enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "historyChange")]
    pub history_change: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Data layer compatibility mode enabled.
    #[serde(rename = "dataLayer")]
    pub data_layer: String,

    /// The key for Zaraz debug mode.
    #[serde(rename = "debugKey")]
    pub debug_key: String,

    /// General Zaraz settings.
    pub settings: String,

    /// Tools set up under Zaraz configuration, where key is the alpha-numeric tool ID
    /// and value is the tool configuration object.
    pub tools: String,

    /// Triggers set up under Zaraz configuration, where key is the trigger
    /// alpha-numeric ID and value is the trigger configuration.
    pub triggers: String,

    /// Variables set up under Zaraz configuration, where key is the variable
    /// alpha-numeric ID and value is the variable configuration. Values of variables of
    /// type secret are not included.
    pub variables: String,

    /// Zaraz internal version of the config.
    #[serde(rename = "zarazVersion")]
    pub zaraz_version: String,

    /// Cloudflare Monitoring settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<String>,

    /// Consent management configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<String>,

    /// Single Page Application support enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "historyChange")]
    pub history_change: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListResponse {
    /// ID of the configuration
    pub id: i64,

    /// Date and time the configuration was created
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Configuration description provided by the user who published this configuration
    pub description: String,

    /// Date and time the configuration was last updated
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    /// Alpha-numeric ID of the account user who published the configuration
    #[serde(rename = "userId")]
    pub user_id: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// ID of the Zaraz configuration to restore.
    pub body: i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Maximum amount of results to list. Default value is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Ordinal number to start listing the results with. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,

    /// The field to sort by. Default is updated_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,

    /// Sorting order. Default is DESC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfigGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Comma separated list of Zaraz configuration IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_ds: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Zaraz configuration description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTextTranslation {
    /// Object where keys are language codes
    pub accept_all: ::std::collections::HashMap<String, String>,

    /// Object where keys are language codes
    pub confirm_my_choices: ::std::collections::HashMap<String, String>,

    /// Object where keys are language codes
    pub reject_all: ::std::collections::HashMap<String, String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTextTranslationParam {
    /// Object where keys are language codes
    pub accept_all: String,

    /// Object where keys are language codes
    pub confirm_my_choices: String,

    /// Object where keys are language codes
    pub reject_all: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoEvent {
    /// Tool event type
    #[serde(rename = "actionType")]
    pub action_type: String,

    /// List of blocking triggers IDs
    #[serde(rename = "blockingTriggers")]
    pub blocking_triggers: Vec<String>,

    /// Event payload
    pub data: String,

    /// List of firing triggers IDs
    #[serde(rename = "firingTriggers")]
    pub firing_triggers: Vec<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoEventParam {
    /// Tool event type
    #[serde(rename = "actionType")]
    pub action_type: String,

    /// List of blocking triggers IDs
    #[serde(rename = "blockingTriggers")]
    pub blocking_triggers: String,

    /// Event payload
    pub data: String,

    /// List of firing triggers IDs
    #[serde(rename = "firingTriggers")]
    pub firing_triggers: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZarazUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Zaraz workflow
    pub workflow: Workflow,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSettings {
    /// Automatic injection of Zaraz scripts enabled.
    #[serde(rename = "autoInjectScript")]
    pub auto_inject_script: bool,

    /// Details of the worker that receives and edits Zaraz Context object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextEnricher")]
    pub context_enricher: Option<ConfigurationSettingsContextEnricher>,

    /// The domain Zaraz will use for writing and reading its cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookieDomain")]
    pub cookie_domain: Option<String>,

    /// Ecommerce API enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<bool>,

    /// Custom endpoint for server-side track events.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventsApiPath")]
    pub events_api_path: Option<String>,

    /// Hiding external referrer URL enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideExternalReferer")]
    pub hide_external_referer: Option<bool>,

    /// Trimming IP address enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideIPAddress")]
    pub hide_ip_address: Option<bool>,

    /// Removing URL query params enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideQueryParams")]
    pub hide_query_params: Option<bool>,

    /// Removing sensitive data from User Aagent string enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideUserAgent")]
    pub hide_user_agent: Option<bool>,

    /// Custom endpoint for Zaraz init script.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initPath")]
    pub init_path: Option<String>,

    /// Injection of Zaraz scripts into iframes enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "injectIframes")]
    pub inject_iframes: Option<bool>,

    /// Custom path for Managed Components server functionalities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mcRootPath")]
    pub mc_root_path: Option<String>,

    /// Custom endpoint for Zaraz main script.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptPath")]
    pub script_path: Option<String>,

    /// Custom endpoint for Zaraz tracking requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trackPath")]
    pub track_path: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTool {
    /// This field can have the runtime type of [[]string].
    #[serde(rename = "blockingTriggers")]
    pub blocking_triggers: String,

    /// Tool's internal name
    pub component: String,

    /// This field can have the runtime type of
    /// [map[string]ConfigurationToolsZarazManagedComponentDefaultFieldsUnion],
    /// [map[string]ConfigurationToolsWorkerDefaultFieldsUnion].
    #[serde(rename = "defaultFields")]
    pub default_fields: String,

    /// Whether tool is enabled
    pub enabled: bool,

    /// Tool's name defined by the user
    pub name: String,

    /// This field can have the runtime type of [[]string].
    pub permissions: String,

    /// This field can have the runtime type of
    /// [map[string]ConfigurationToolsZarazManagedComponentSettingsUnion],
    /// [map[string]ConfigurationToolsWorkerSettingsUnion].
    pub settings: String,

    pub r#type: ConfigurationToolsType,

    /// This field can have the runtime type of [map[string]NeoEvent].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,

    /// Default consent purpose ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultPurpose")]
    pub default_purpose: Option<String>,

    /// This field can have the runtime type of [[]NeoEvent].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "neoEvents")]
    pub neo_events: Option<String>,

    /// Vendor name for TCF compliant consent modal, required for Custom Managed
    /// Components and Custom HTML tool with a defaultPurpose assigned
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vendorName")]
    pub vendor_name: Option<String>,

    /// Vendor's Privacy Policy URL for TCF compliant consent modal, required for Custom
    /// Managed Components and Custom HTML tool with a defaultPurpose assigned
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vendorPolicyUrl")]
    pub vendor_policy_url: Option<String>,

    /// This field can have the runtime type of [ConfigurationToolsWorkerWorker].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTrigger {
    /// Rules defining when the trigger is not fired.
    #[serde(rename = "excludeRules")]
    pub exclude_rules: Vec<ConfigurationTriggersExcludeRule>,

    /// Rules defining when the trigger is fired.
    #[serde(rename = "loadRules")]
    pub load_rules: Vec<ConfigurationTriggersLoadRule>,

    /// Trigger name.
    pub name: String,

    /// Trigger description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<ConfigurationTriggersSystem>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationVariable {
    pub name: String,

    pub r#type: ConfigurationVariablesType,

    /// This field can have the runtime type of [string],
    /// [ConfigurationVariablesZarazWorkerVariableValue].
    pub value: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationAnalytics {
    /// Consent purpose assigned to Monitoring.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultPurpose")]
    pub default_purpose: Option<String>,

    /// Whether Advanced Monitoring reports are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Session expiration time (seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sessionExpTime")]
    pub session_exp_time: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationConsent {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buttonTextTranslations")]
    pub button_text_translations: Option<ButtonTextTranslation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companyEmail")]
    pub company_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companyStreetAddress")]
    pub company_street_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "consentModalIntroHTML")]
    pub consent_modal_intro_html: Option<String>,

    /// Object where keys are language codes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "consentModalIntroHTMLWithTranslations")]
    pub consent_modal_intro_html_with_translations: Option<::std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cookieName")]
    pub cookie_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customCSS")]
    pub custom_css: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customIntroDisclaimerDismissed")]
    pub custom_intro_disclaimer_dismissed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultLanguage")]
    pub default_language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideModal")]
    pub hide_modal: Option<bool>,

    /// Object where keys are purpose alpha-numeric IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purposes: Option<::std::collections::HashMap<String, ConfigurationConsentPurpose>>,

    /// Object where keys are purpose alpha-numeric IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "purposesWithTranslations")]
    pub purposes_with_translations: Option<::std::collections::HashMap<String, ConfigurationConsentPurposesWithTranslation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tcfCompliant")]
    pub tcf_compliant: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSettingsContextEnricher {
    #[serde(rename = "escapedWorkerName")]
    pub escaped_worker_name: String,

    #[serde(rename = "workerTag")]
    pub worker_tag: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTriggersExcludeRule {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ConfigurationTriggersExcludeRulesAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<ConfigurationTriggersExcludeRulesOp>,

    /// This field can have the runtime type of
    /// [ConfigurationTriggersExcludeRulesZarazClickListenerRuleSettings],
    /// [ConfigurationTriggersExcludeRulesZarazTimerRuleSettings],
    /// [ConfigurationTriggersExcludeRulesZarazFormSubmissionRuleSettings],
    /// [ConfigurationTriggersExcludeRulesZarazVariableMatchRuleSettings],
    /// [ConfigurationTriggersExcludeRulesZarazScrollDepthRuleSettings],
    /// [ConfigurationTriggersExcludeRulesZarazElementVisibilityRuleSettings].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTriggersLoadRule {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ConfigurationTriggersLoadRulesAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<ConfigurationTriggersLoadRulesOp>,

    /// This field can have the runtime type of
    /// [ConfigurationTriggersLoadRulesZarazClickListenerRuleSettings],
    /// [ConfigurationTriggersLoadRulesZarazTimerRuleSettings],
    /// [ConfigurationTriggersLoadRulesZarazFormSubmissionRuleSettings],
    /// [ConfigurationTriggersLoadRulesZarazVariableMatchRuleSettings],
    /// [ConfigurationTriggersLoadRulesZarazScrollDepthRuleSettings],
    /// [ConfigurationTriggersLoadRulesZarazElementVisibilityRuleSettings].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationConsentPurpose {
    pub description: String,

    pub name: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationConsentPurposesWithTranslation {
    /// Object where keys are language codes
    pub description: ::std::collections::HashMap<String, String>,

    /// Object where keys are language codes
    pub name: ::std::collections::HashMap<String, String>,

    pub order: i64,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Workflow {
    #[serde(rename = "realtime")]
    WorkflowRealtime,
    #[serde(rename = "preview")]
    WorkflowPreview,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationToolsType {
    #[serde(rename = "component")]
    ConfigurationToolsTypeComponent,
    #[serde(rename = "custom-mc")]
    ConfigurationToolsTypeCustomMc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationTriggersSystem {
    #[serde(rename = "pageload")]
    ConfigurationTriggersSystemPageload,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationVariablesType {
    #[serde(rename = "string")]
    ConfigurationVariablesTypeString,
    #[serde(rename = "secret")]
    ConfigurationVariablesTypeSecret,
    #[serde(rename = "worker")]
    ConfigurationVariablesTypeWorker,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationTriggersExcludeRulesAction {
    #[serde(rename = "clickListener")]
    ConfigurationTriggersExcludeRulesActionClickListener,
    #[serde(rename = "timer")]
    ConfigurationTriggersExcludeRulesActionTimer,
    #[serde(rename = "formSubmission")]
    ConfigurationTriggersExcludeRulesActionFormSubmission,
    #[serde(rename = "variableMatch")]
    ConfigurationTriggersExcludeRulesActionVariableMatch,
    #[serde(rename = "scrollDepth")]
    ConfigurationTriggersExcludeRulesActionScrollDepth,
    #[serde(rename = "elementVisibility")]
    ConfigurationTriggersExcludeRulesActionElementVisibility,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationTriggersExcludeRulesOp {
    #[serde(rename = "CONTAINS")]
    ConfigurationTriggersExcludeRulesOpContains,
    #[serde(rename = "EQUALS")]
    ConfigurationTriggersExcludeRulesOpEquals,
    #[serde(rename = "STARTS_WITH")]
    ConfigurationTriggersExcludeRulesOpStartsWith,
    #[serde(rename = "ENDS_WITH")]
    ConfigurationTriggersExcludeRulesOpEndsWith,
    #[serde(rename = "MATCH_REGEX")]
    ConfigurationTriggersExcludeRulesOpMatchRegex,
    #[serde(rename = "NOT_MATCH_REGEX")]
    ConfigurationTriggersExcludeRulesOpNotMatchRegex,
    #[serde(rename = "GREATER_THAN")]
    ConfigurationTriggersExcludeRulesOpGreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    ConfigurationTriggersExcludeRulesOpGreaterThanOrEqual,
    #[serde(rename = "LESS_THAN")]
    ConfigurationTriggersExcludeRulesOpLessThan,
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    ConfigurationTriggersExcludeRulesOpLessThanOrEqual,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationTriggersLoadRulesAction {
    #[serde(rename = "clickListener")]
    ConfigurationTriggersLoadRulesActionClickListener,
    #[serde(rename = "timer")]
    ConfigurationTriggersLoadRulesActionTimer,
    #[serde(rename = "formSubmission")]
    ConfigurationTriggersLoadRulesActionFormSubmission,
    #[serde(rename = "variableMatch")]
    ConfigurationTriggersLoadRulesActionVariableMatch,
    #[serde(rename = "scrollDepth")]
    ConfigurationTriggersLoadRulesActionScrollDepth,
    #[serde(rename = "elementVisibility")]
    ConfigurationTriggersLoadRulesActionElementVisibility,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationTriggersLoadRulesOp {
    #[serde(rename = "CONTAINS")]
    ConfigurationTriggersLoadRulesOpContains,
    #[serde(rename = "EQUALS")]
    ConfigurationTriggersLoadRulesOpEquals,
    #[serde(rename = "STARTS_WITH")]
    ConfigurationTriggersLoadRulesOpStartsWith,
    #[serde(rename = "ENDS_WITH")]
    ConfigurationTriggersLoadRulesOpEndsWith,
    #[serde(rename = "MATCH_REGEX")]
    ConfigurationTriggersLoadRulesOpMatchRegex,
    #[serde(rename = "NOT_MATCH_REGEX")]
    ConfigurationTriggersLoadRulesOpNotMatchRegex,
    #[serde(rename = "GREATER_THAN")]
    ConfigurationTriggersLoadRulesOpGreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    ConfigurationTriggersLoadRulesOpGreaterThanOrEqual,
    #[serde(rename = "LESS_THAN")]
    ConfigurationTriggersLoadRulesOpLessThan,
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    ConfigurationTriggersLoadRulesOpLessThanOrEqual,
}

