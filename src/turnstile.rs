#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    /// If bot_fight_mode is set to `true`, Cloudflare issues computationally expensive
    /// challenges in response to malicious bots (ENT only).
    pub bot_fight_mode: bool,

    /// If Turnstile is embedded on a Cloudflare site and the widget should grant
    /// challenge clearance, this setting can determine the clearance level to be set
    pub clearance_level: WidgetClearanceLevel,

    /// When the widget was created.
    pub created_on: DateTime<Utc>,

    pub domains: Vec<String>,

    /// Return the Ephemeral ID in /siteverify (ENT only).
    pub ephemeral_id: bool,

    /// Widget Mode
    pub mode: WidgetMode,

    /// When the widget was modified.
    pub modified_on: DateTime<Utc>,

    /// Human readable widget name. Not unique. Cloudflare suggests that you set this to
    /// a meaningful string to make it easier to identify your widget, and where it is
    /// used.
    pub name: String,

    /// Do not show any Cloudflare branding on the widget (ENT only).
    pub offlabel: bool,

    /// Region where this widget can be used. This cannot be changed after creation.
    pub region: WidgetRegion,

    /// Secret key for this widget.
    pub secret: String,

    /// Widget item identifier tag.
    pub sitekey: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetListResponse {
    /// If bot_fight_mode is set to `true`, Cloudflare issues computationally expensive
    /// challenges in response to malicious bots (ENT only).
    pub bot_fight_mode: bool,

    /// If Turnstile is embedded on a Cloudflare site and the widget should grant
    /// challenge clearance, this setting can determine the clearance level to be set
    pub clearance_level: WidgetListResponseClearanceLevel,

    /// When the widget was created.
    pub created_on: DateTime<Utc>,

    pub domains: Vec<String>,

    /// Return the Ephemeral ID in /siteverify (ENT only).
    pub ephemeral_id: bool,

    /// Widget Mode
    pub mode: WidgetListResponseMode,

    /// When the widget was modified.
    pub modified_on: DateTime<Utc>,

    /// Human readable widget name. Not unique. Cloudflare suggests that you set this to
    /// a meaningful string to make it easier to identify your widget, and where it is
    /// used.
    pub name: String,

    /// Do not show any Cloudflare branding on the widget (ENT only).
    pub offlabel: bool,

    /// Region where this widget can be used. This cannot be changed after creation.
    pub region: WidgetListResponseRegion,

    /// Widget item identifier tag.
    pub sitekey: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub domains: String,

    /// Widget Mode
    pub mode: String,

    /// Human readable widget name. Not unique. Cloudflare suggests that you set this to
    /// a meaningful string to make it easier to identify your widget, and where it is
    /// used.
    pub name: String,

    /// Direction to order widgets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Filter widgets by field using case-insensitive substring matching. Format:
    /// `field:value`
    /// Supported fields:
    /// - `name` - Filter by widget name (e.g., `filter=name:login-form`)
    /// - `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)
    /// Returns 400 Bad Request if the field is unsupported or format is invalid. An
    /// empty filter value returns all results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Field to order widgets by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// If bot_fight_mode is set to `true`, Cloudflare issues computationally expensive
    /// challenges in response to malicious bots (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_fight_mode: Option<String>,

    /// If Turnstile is embedded on a Cloudflare site and the widget should grant
    /// challenge clearance, this setting can determine the clearance level to be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearance_level: Option<String>,

    /// Return the Ephemeral ID in /siteverify (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_id: Option<String>,

    /// Do not show any Cloudflare branding on the widget (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offlabel: Option<String>,

    /// Region where this widget can be used. This cannot be changed after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub domains: String,

    /// Widget Mode
    pub mode: String,

    /// Human readable widget name. Not unique. Cloudflare suggests that you set this to
    /// a meaningful string to make it easier to identify your widget, and where it is
    /// used.
    pub name: String,

    /// If bot_fight_mode is set to `true`, Cloudflare issues computationally expensive
    /// challenges in response to malicious bots (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_fight_mode: Option<String>,

    /// If Turnstile is embedded on a Cloudflare site and the widget should grant
    /// challenge clearance, this setting can determine the clearance level to be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearance_level: Option<String>,

    /// Return the Ephemeral ID in /siteverify (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_id: Option<String>,

    /// Do not show any Cloudflare branding on the widget (ENT only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offlabel: Option<String>,

    /// Region where this widget can be used. This cannot be changed after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Direction to order widgets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// Filter widgets by field using case-insensitive substring matching. Format:
    /// `field:value`
    /// Supported fields:
    /// - `name` - Filter by widget name (e.g., `filter=name:login-form`)
    /// - `sitekey` - Filter by sitekey (e.g., `filter=sitekey:0x4AAA`)
    /// Returns 400 Bad Request if the field is unsupported or format is invalid. An
    /// empty filter value returns all results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Field to order widgets by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Page number of paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetRotateSecretParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// If `invalidate_immediately` is set to `false`, the previous secret will remain
    /// valid for two hours. Otherwise, the secret is immediately invalidated, and
    /// requests using it will be rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidate_immediately: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetClearanceLevel {
    #[serde(rename = "no_clearance")]
    WidgetClearanceLevelNoClearance,
    #[serde(rename = "jschallenge")]
    WidgetClearanceLevelJschallenge,
    #[serde(rename = "managed")]
    WidgetClearanceLevelManaged,
    #[serde(rename = "interactive")]
    WidgetClearanceLevelInteractive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetMode {
    #[serde(rename = "non-interactive")]
    WidgetModeNonInteractive,
    #[serde(rename = "invisible")]
    WidgetModeInvisible,
    #[serde(rename = "managed")]
    WidgetModeManaged,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetRegion {
    #[serde(rename = "world")]
    WidgetRegionWorld,
    #[serde(rename = "china")]
    WidgetRegionChina,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetListResponseClearanceLevel {
    #[serde(rename = "no_clearance")]
    WidgetListResponseClearanceLevelNoClearance,
    #[serde(rename = "jschallenge")]
    WidgetListResponseClearanceLevelJschallenge,
    #[serde(rename = "managed")]
    WidgetListResponseClearanceLevelManaged,
    #[serde(rename = "interactive")]
    WidgetListResponseClearanceLevelInteractive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetListResponseMode {
    #[serde(rename = "non-interactive")]
    WidgetListResponseModeNonInteractive,
    #[serde(rename = "invisible")]
    WidgetListResponseModeInvisible,
    #[serde(rename = "managed")]
    WidgetListResponseModeManaged,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WidgetListResponseRegion {
    #[serde(rename = "world")]
    WidgetListResponseRegionWorld,
    #[serde(rename = "china")]
    WidgetListResponseRegionChina,
}

