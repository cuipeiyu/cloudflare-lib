#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Domain identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Shows if a domain is available for transferring into Cloudflare Registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,

    /// Indicates if the domain can be registered as a new domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_register: Option<bool>,

    /// Shows time of creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Shows name of current registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_registrar: Option<String>,

    /// Shows when domain name registration expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    /// Shows whether a registrar lock is in place for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /// Shows contact information for domain registrant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<DomainRegistrantContact>,

    /// A comma-separated list of registry status codes. A full list of status codes can
    /// be found at
    /// [EPP Status Codes](https://www.icann.org/resources/pages/epp-status-codes-2014-06-16-en).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_statuses: Option<String>,

    /// Whether a particular TLD is currently supported by Cloudflare Registrar. Refer
    /// to [TLD Policies](https://www.cloudflare.com/tld-policies/) for a list of
    /// supported TLDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tld: Option<bool>,

    /// Statuses for domain transfers into Cloudflare Registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_in: Option<DomainTransferIn>,

    /// Last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Auto-renew controls whether subscription is automatically renewed upon domain
    /// expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,

    /// Shows whether a registrar lock is in place for a domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<String>,

    /// Privacy option controls redacting WHOIS information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRegistrantContact {
    /// Address.
    pub address: String,

    /// City.
    pub city: String,

    /// The country in which the user lives.
    pub country: String,

    /// User's first name
    pub first_name: String,

    /// User's last name
    pub last_name: String,

    /// Name of organization.
    pub organization: String,

    /// User's telephone number
    pub phone: String,

    /// State.
    pub state: String,

    /// The zipcode or postal code where the user lives.
    pub zip: String,

    /// Contact Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional address line for unit, floor, suite, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,

    /// The contact email address of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Contact fax number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainTransferIn {
    /// Form of authorization has been accepted by the registrant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_foa: Option<DomainTransferInAcceptFoa>,

    /// Shows transfer status with the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_transfer: Option<DomainTransferInApproveTransfer>,

    /// Indicates if cancellation is still possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_cancel_transfer: Option<bool>,

    /// Privacy guards are disabled at the foreign registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_privacy: Option<DomainTransferInDisablePrivacy>,

    /// Auth code has been entered and verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_auth_code: Option<DomainTransferInEnterAuthCode>,

    /// Domain is unlocked at the foreign registrar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_domain: Option<DomainTransferInUnlockDomain>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainTransferInAcceptFoa {
    #[serde(rename = "needed")]
    DomainTransferInAcceptFoaNeeded,
    #[serde(rename = "ok")]
    DomainTransferInAcceptFoaOk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainTransferInApproveTransfer {
    #[serde(rename = "needed")]
    DomainTransferInApproveTransferNeeded,
    #[serde(rename = "ok")]
    DomainTransferInApproveTransferOk,
    #[serde(rename = "pending")]
    DomainTransferInApproveTransferPending,
    #[serde(rename = "trying")]
    DomainTransferInApproveTransferTrying,
    #[serde(rename = "rejected")]
    DomainTransferInApproveTransferRejected,
    #[serde(rename = "unknown")]
    DomainTransferInApproveTransferUnknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainTransferInDisablePrivacy {
    #[serde(rename = "needed")]
    DomainTransferInDisablePrivacyNeeded,
    #[serde(rename = "ok")]
    DomainTransferInDisablePrivacyOk,
    #[serde(rename = "unknown")]
    DomainTransferInDisablePrivacyUnknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainTransferInEnterAuthCode {
    #[serde(rename = "needed")]
    DomainTransferInEnterAuthCodeNeeded,
    #[serde(rename = "ok")]
    DomainTransferInEnterAuthCodeOk,
    #[serde(rename = "pending")]
    DomainTransferInEnterAuthCodePending,
    #[serde(rename = "trying")]
    DomainTransferInEnterAuthCodeTrying,
    #[serde(rename = "rejected")]
    DomainTransferInEnterAuthCodeRejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainTransferInUnlockDomain {
    #[serde(rename = "needed")]
    DomainTransferInUnlockDomainNeeded,
    #[serde(rename = "ok")]
    DomainTransferInUnlockDomainOk,
    #[serde(rename = "pending")]
    DomainTransferInUnlockDomainPending,
    #[serde(rename = "trying")]
    DomainTransferInUnlockDomainTrying,
    #[serde(rename = "unknown")]
    DomainTransferInUnlockDomainUnknown,
}

