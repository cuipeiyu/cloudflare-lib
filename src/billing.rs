use serde::{Deserialize, Serialize};

use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-billing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGetResponse {
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

#[cfg(any(feature = "full", feature = "with-billing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
