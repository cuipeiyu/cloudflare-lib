#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{DateTime, Duration, Utc};

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewResponseEnvelope {
    /// The identifier for the submitted abuse report.
    pub abuse_rand: String,
    pub request: AbuseReportNewResponseEnvelopeRequest,
    /// The result should be 'success' for successful response
    pub result: String,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewResponseEnvelopeRequest {
    /// The report type for submitted reports.
    pub act: String,
}

/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsDmcaReportAct {
    /// [default] value is "abuse_dmca"
    #[default]
    #[serde(rename = "abuse_dmca")]
    AbuseDmca,
}

/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsTrademarkReportAct {
    /// [default] value is "abuse_trademark"
    #[default]
    #[serde(rename = "abuse_trademark")]
    AbuseTrademark,
}

/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsGeneralReportAct {
    /// [default] value is "abuse_general"
    #[default]
    #[serde(rename = "abuse_general")]
    AbuseGeneral,
}

/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsCsamReportAct {
    /// [default] value is "abuse_children"
    #[default]
    #[serde(rename = "abuse_children")]
    AbuseChildren,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportsDmcaReportHostNotification {
    /// [default] value is "send"
    #[default]
    #[serde(rename = "send")]
    Send,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsGeneralReportHostNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsPhishingReportHostNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsPhishingReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsCsamReportHostNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsCsamReportNcmecNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsGeneralReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
    /// value is "none"
    #[serde(rename = "none")]
    None,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsCsamReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
    /// value is "none"
    #[serde(rename = "none")]
    None,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportNewParamsBodyAbuseReportsDmcaReportHostNotification = AbuseReportsDmcaReportHostNotification;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportNewParamsBodyAbuseReportsDmcaReportOwnerNotification = AbuseReportsDmcaReportHostNotification;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportNewParamsBodyAbuseReportsTrademarkReportHostNotification = AbuseReportsDmcaReportHostNotification;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportNewParamsBodyAbuseReportsTrademarkReportOwnerNotification = AbuseReportsDmcaReportHostNotification;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsDmcaReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsDmcaReportAct,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub address1: String,
	/// The name of the copyright holder. Text not exceeding 60 characters. This field
	/// may be released by Cloudflare to third parties such as the Lumen Database
	/// (https://lumendatabase.org/).
	pub agent_name: String,
	/// Can be `0` for false or `1` for true. Must be value: 1 for DMCA reports
	pub agree: i64,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub city: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub country: String,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsDmcaReportHostNotification,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub original_work: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsDmcaReportOwnerNotification,
	/// Required for DMCA reports, should be same as Name. An affirmation that all
	/// information in the report is true and accurate while agreeing to the policies of
	/// Cloudflare's abuse reports
	pub signature: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub state: String,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}


#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsTrademarkReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsTrademarkReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsTrademarkReportHostNotification,
	/// A detailed description of the infringement, including any necessary access
	/// details and the exact steps needed to view the content, not exceeding 5000
	/// characters.
	pub justification: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsTrademarkReportOwnerNotification,
	/// Text not exceeding 1000 characters
	pub trademark_number: String,
	/// Text not exceeding 1000 characters
	pub trademark_office: String,
	/// Text not exceeding 1000 characters
	pub trademark_symbol: String,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsCsamReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsCsamReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsCsamReportHostNotification,
	/// A detailed description of the infringement, including any necessary access
	/// details and the exact steps needed to view the content, not exceeding 5000
	/// characters.
	pub justification: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub ncmec_notification: AbuseReportNewParamsBodyAbuseReportsCsamReportNcmecNotification,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsCsamReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}


/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsThreatReportAct {
    /// [default] value is "abuse_threat"
    #[default]
    #[serde(rename = "abuse_threat")]
    AbuseThreat,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsThreatReportHostNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsThreatReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}


#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsThreatReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsThreatReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsThreatReportHostNotification,
	/// A detailed description of the infringement, including any necessary access
	/// details and the exact steps needed to view the content, not exceeding 5000
	/// characters.
	pub justification: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsThreatReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}


/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReportAct {
    /// [default] value is "abuse_registrar_whois"
    #[default]
    #[serde(rename = "abuse_registrar_whois")]
    AbuseRegistrarWhois,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
    /// value is "none"
    #[serde(rename = "none")]
    None,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

/// The report type for submitted reports.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsNcseiReportAct {
    /// [default] value is "abuse_ncsei"
    #[default]
    #[serde(rename = "abuse_ncsei")]
    AbuseNcsei,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsNcseiReportHostNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
}

/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
/// reports cannot be anonymous.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsNcseiReportOwnerNotification {
    /// value is "send"
    #[serde(rename = "send")]
    Send,
    /// value is "send-anon"
    #[serde(rename = "send-anon")]
    SendAnon,
    /// value is "none"
    #[serde(rename = "none")]
    None,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsNcseiReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsNcseiReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsNcseiReportHostNotification,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// If the submitter is the target of NCSEI in the URLs of the abuse report.
	pub ncsei_subject_representation: bool,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsNcseiReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportListQuery {
    /// [query] Returns reports created after the specified date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// [query] Returns reports created before the specified date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// [query] Filter by domain name related to the abuse report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// [query] Filter reports that have any mitigations in the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_status: Option<AbuseReportListParamsMitigationStatus>,
    /// [query] Where in pagination to start listing abuse reports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// [query] How many abuse reports per page to list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// [query] A property to sort by, followed by the order (id, cdate, domain, type, status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// [query] Filter by the status of the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AbuseReportListParamsStatus>,
    /// [query] Filter by the type of the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AbuseReportListParamsType>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportListResponse {
    pub reports: Vec<AbuseReportListResponseReport>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportListResponseReport {
    /// Public facing ID of abuse report, aka abuse_rand.
    pub id: String,
    /// Creation date of report. Time in RFC 3339 format
    /// (https://www.rfc-editor.org/rfc/rfc3339.html)
    pub cdate: DateTime<Utc>,
    /// Domain that relates to the report.
    pub domain: String,
    /// A summary of the mitigations related to this report.
    pub mitigation_summary: AbuseReportListResponseReportsMitigationSummary,
    /// An enum value that represents the status of an abuse record
    pub status: AbuseReportListResponseReportsStatus,
    /// The abuse report type
    pub r#type: AbuseReportListResponseReportsType,
    /// Justification for the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Original work / Targeted brand in the alleged abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_work: Option<String>,
    /// Information about the submitter of the report.
    pub submitter: Option<AbuseReportListResponseReportsSubmitter>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsGeneralReport   {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsGeneralReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsGeneralReportHostNotification,
	/// A detailed description of the infringement, including any necessary access
	/// details and the exact steps needed to view the content, not exceeding 5000
	/// characters.
	pub justification: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsGeneralReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// A list of IP addresses separated by ‘\n’ (new line character). The list of
	/// destination IPs should not exceed 30 IP addresses. Each one of the IP addresses
	/// ought to be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
	pub destination_ips: Option<String>,
	/// A comma separated list of ports and protocols e.g. 80/TCP, 22/UDP. The total
	/// size of the field should not exceed 2000 characters. Each individual
	/// port/protocol should not exceed 100 characters. The list should not have more
	/// than 30 unique ports and protocols.
    #[serde(skip_serializing_if = "Option::is_none")]
	pub ports_protocols: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// A list of IP addresses separated by ‘\n’ (new line character). The list of
	/// source IPs should not exceed 30 IP addresses. Each one of the IP addresses ought
	/// to be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
	pub source_ips: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportNewParamsBodyAbuseReportsPhishingReportAct {
    /// [default] value is "abuse_phishing"
    #[default]
    #[serde(rename = "abuse_phishing")]
    AbusePhishing,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportNewParamsBodyAbuseReportsPhishingReport {
	/// The report type for submitted reports.
	pub act: AbuseReportNewParamsBodyAbuseReportsPhishingReportAct,
	/// A valid email of the abuse reporter. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub email: String,
	/// Should match the value provided in `email`
	pub email2: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub host_notification: AbuseReportNewParamsBodyAbuseReportsPhishingReportHostNotification,
	/// A detailed description of the infringement, including any necessary access
	/// details and the exact steps needed to view the content, not exceeding 5000
	/// characters.
	pub justification: String,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
	pub name: String,
	/// Notification type based on the abuse type. NOTE: Copyright (DMCA) and Trademark
	/// reports cannot be anonymous.
	pub owner_notification: AbuseReportNewParamsBodyAbuseReportsPhishingReportOwnerNotification,
	/// A list of valid URLs separated by ‘\n’ (new line character). The list of the
	/// URLs should not exceed 250 URLs. All URLs should have the same hostname. Each
	/// URL should be unique. This field may be released by Cloudflare to third parties
	/// such as the Lumen Database (https://lumendatabase.org/).
	pub urls: String,
	/// Any additional comments about the infringement not exceeding 2000 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub comments: Option<String>,
	/// Text not exceeding 100 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub company: Option<String>,
	/// Text not exceeding 255 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub original_work: Option<String>,
	/// Text containing 2 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_country: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub reported_user_agent: Option<String>,
	/// Text not exceeding 20 characters. This field may be released by Cloudflare to
	/// third parties such as the Lumen Database (https://lumendatabase.org/).
    #[serde(skip_serializing_if = "Option::is_none")]
	pub tele: Option<String>,
	/// Text not exceeding 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

/// A summary of the mitigations related to this report.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportListResponseReportsMitigationSummary {
    /// How many of the reported URLs were confirmed as abusive.
    pub accepted_url_count: i64,
    /// How many mitigations are active.
    pub active_count: i64,
    /// Whether the report has been forwarded to an external hosting provider.
    pub external_host_notified: bool,
    /// How many mitigations are under review.
    pub in_review_count: i64,
    /// How many mitigations are pending their effective date.
    pub pending_count: i64,
}

/// Filter reports that have any mitigations in the given status.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportListParamsMitigationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "in_review")]
    InReview,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "removed")]
    Removed,
}

/// An enum value that represents the status of an abuse record
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportListParamsStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "in_review")]
    InReview,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportListResponseReportsStatus = AbuseReportListParamsStatus;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum AbuseReportListParamsType {
    #[serde(rename = "PHISH")]
    Phish,
    #[serde(rename = "GEN")]
    Gen,
    #[serde(rename = "THREAT")]
    Threat,
    #[serde(rename = "DMCA")]
    Dmca,
    #[serde(rename = "EMER")]
    Emer,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "REG_WHO")]
    RegWho,
    #[serde(rename = "NCSEI")]
    Ncsei,
    #[serde(rename = "NETWORK")]
    Network,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportListResponseReportsType = AbuseReportListParamsType;

// Information about the submitter of the report.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportListResponseReportsSubmitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportGetResponse {
    /// Public facing ID of abuse report, aka abuse_rand.
    pub id: String,
    /// Creation date of report. Time in RFC 3339 format
    /// (https://www.rfc-editor.org/rfc/rfc3339.html)
    pub cdate: String,
    /// Domain that relates to the report.
    pub domain: String,
    /// A summary of the mitigations related to this report.
    pub mitigation_summary: AbuseReportGetResponseMitigationSummary,
    /// An enum value that represents the status of an abuse record
    pub status: AbuseReportGetResponseStatus,
    /// The abuse report type
    pub r#type: AbuseReportGetResponseType,
    /// Justification for the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Original work / Targeted brand in the alleged abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_work: Option<String>,
    /// Information about the submitter of the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<AbuseReportGetResponseSubmitter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// A summary of the mitigations related to this report.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportGetResponseMitigationSummary {
    /// How many of the reported URLs were confirmed as abusive.
    pub accepted_url_count: i64,
    /// How many mitigations are active.
    pub active_count: i64,
    /// Whether the report has been forwarded to an external hosting provider.
    pub external_host_notified: bool,
    /// How many mitigations are under review.
    pub in_review_count: i64,
    /// How many mitigations are pending their effective date.
    pub pending_count: i64,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportGetResponseStatus = AbuseReportListParamsStatus;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
pub type AbuseReportGetResponseType = AbuseReportListParamsType;

/// Information about the submitter of the report.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReportGetResponseSubmitter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationListQuery {
    /// [query] Returns mitigation that were dispatched after the given date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_after: Option<String>,
    /// [query] Returns mitigations that were dispatched before the given date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_before: Option<String>,
    /// [query] Filter by the type of entity the mitigation impacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<MitigationListParamsEntityType>,
    /// [query] Where in pagination to start listing abuse reports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// [query] How many abuse reports per page to list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// [query] A property to sort by, followed by the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MitigationListParamsSort>,
    /// [query] Filter by the status of the mitigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MitigationListParamsStatus>,
    /// [query] Filter by the type of mitigation. This filter parameter can be specified
    /// multiple times to include multiple types of mitigations in the result set, e.g.
    /// ?type=rate_limit_cache&type=legal_block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MitigationListParamsType>,
}


#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListParamsEntityType {
    #[serde(rename = "url_pattern")]
    URLPattern,
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "zone")]
    Zone,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListParamsSort {
    #[serde(rename = "type,asc")]
    TypeAsc,
    #[serde(rename = "type,desc")]
    TypeDesc,
    #[serde(rename = "effective_date,asc")]
    EffectiveDateAsc,
    #[serde(rename = "effective_date,desc")]
    EffectiveDateDesc,
    #[serde(rename = "status,asc")]
    StatusAsc,
    #[serde(rename = "status,desc")]
    StatusDesc,
    #[serde(rename = "entity_type,asc")]
    EntityTypeAsc,
    #[serde(rename = "entity_type,desc")]
    EntityTypeDesc,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListParamsStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "in_review")]
    InReview,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "removed")]
    Removed,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListParamsType {
    #[serde(rename = "legal_block")]
    LegalBlock,
    #[serde(rename = "phishing_interstitial")]
    PhishingInterstitial,
    #[serde(rename = "network_block")]
    NetworkBlock,
    #[serde(rename = "rate_limit_cache")]
    RateLimitCache,
    #[serde(rename = "account_suspend")]
    AccountSuspend,
    #[serde(rename = "redirect_video_stream")]
    RedirectVideoStream,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigations: Option<Vec<MitigationListResponseMitigation>>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationListResponseMitigation {
    /// ID of remediation.
    pub id: String,
    /// Date when the mitigation will become active. Time in RFC 3339 format
    /// (https://www.rfc-editor.org/rfc/rfc3339.html)
    pub effective_date: String,
    pub entity_id: String,
    pub entity_type: MitigationListResponseMitigationsEntityType,
    /// The status of a mitigation
    pub status: MitigationListResponseMitigationsStatus,
    /// The type of mitigation
    pub r#type: MitigationListResponseMitigationsType,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListResponseMitigationsEntityType {
    #[serde(rename = "url_pattern")]
    URLPattern,
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "zone")]
    Zone,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListResponseMitigationsStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "in_review")]
    InReview,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "removed")]
    Removed,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationListResponseMitigationsType {
    #[serde(rename = "legal_block")]
    LegalBlock,
    #[serde(rename = "phishing_interstitial")]
    PhishingInterstitial,
    #[serde(rename = "network_block")]
    NetworkBlock,
    #[serde(rename = "rate_limit_cache")]
    RateLimitCache,
    #[serde(rename = "account_suspend")]
    AccountSuspend,
    #[serde(rename = "redirect_video_stream")]
    RedirectVideoStream,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationReviewBody {
	/// List of mitigations to appeal.
    pub appeals: Vec<MitigationReviewParamsAppeal>,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationReviewParamsAppeal {
    /// ID of the mitigation to appeal.
    pub id: String,
    /// Reason why the customer is appealing.
    pub reason: MitigationReviewParamsAppealsReason,
}

/// Reason why the customer is appealing.
#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationReviewParamsAppealsReason {
    /// value is "removed"
    #[serde(rename = "removed")]
    Removed,
    /// value is "misclassified"
    #[serde(rename = "misclassified")]
    Misclassified,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationReviewResponse {
    // ID of remediation.
    pub id: String,
    // Date when the mitigation will become active. Time in RFC 3339 format
    // (https://www.rfc-editor.org/rfc/rfc3339.html)
    pub effective_date: String,
    pub entity_id: String,
    pub entity_type: MitigationReviewResponseEntityType,
    // The status of a mitigation
    pub status: MitigationReviewResponseStatus,
    // The type of mitigation
    pub r#type: MitigationReviewResponseType,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationReviewResponseEntityType {
    /// value is "url_pattern"
    #[serde(rename = "url_pattern")]
    URLPattern,
    /// value is "account"
    #[serde(rename = "account")]
    Account,
    /// value is "zone"
    #[serde(rename = "zone")]
    Zone,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationReviewResponseStatus {
    /// value is "pending"
    #[serde(rename = "pending")]
    Pending,
    /// value is "active"
    #[serde(rename = "active")]
    Active,
    /// value is "in_review"
    #[serde(rename = "in_review")]
    InReview,
    /// value is "cancelled"
    #[serde(rename = "cancelled")]
    Cancelled,
    /// value is "removed"
    #[serde(rename = "removed")]
    Removed,
}

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MitigationReviewResponseType {
    /// value is "legal_block"
    #[serde(rename = "legal_block")]
    LegalBlock,
    /// value is "phishing_interstitial"
    #[serde(rename = "phishing_interstitial")]
    PhishingInterstitial,
    /// value is "network_block"
    #[serde(rename = "network_block")]
    NetworkBlock,
    /// value is "rate_limit_cache"
    #[serde(rename = "rate_limit_cache")]
    RateLimitCache,
    /// value is "account_suspend"
    #[serde(rename = "account_suspend")]
    AccountSuspend,
    /// value is "redirect_video_stream"
    #[serde(rename = "redirect_video_stream")]
    RedirectVideoStream,
}
