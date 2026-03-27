#[cfg(any(feature = "full", feature = "with-fraud"))]
mod fraud_bindings {

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FraudSettings {
        /// Whether Fraud User Profiles is enabled for the zone.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_profiles: Option<FraudSettingsUserProfiles>,

        /// List of expressions to detect usernames in write HTTP requests.
        /// - Maximum of 10 expressions.
        /// - Omit or set to null to leave unchanged on update.
        /// - Provide an empty array `[]` to clear all expressions on update.
        /// - Invalid expressions will result in a 10400 Bad Request with details in the
        /// `messages` array.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username_expressions: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FraudSettingsParam {
        /// Whether Fraud User Profiles is enabled for the zone.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_profiles: Option<String>,

        /// List of expressions to detect usernames in write HTTP requests.
        /// - Maximum of 10 expressions.
        /// - Omit or set to null to leave unchanged on update.
        /// - Provide an empty array `[]` to clear all expressions on update.
        /// - Invalid expressions will result in a 10400 Bad Request with details in the
        /// `messages` array.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username_expressions: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FraudUpdateParams {
        /// Identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        pub fraud_settings: FraudSettingsParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FraudGetParams {
        /// Identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum FraudSettingsUserProfiles {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}

#[cfg(any(feature = "full", feature = "with-fraud"))]
pub use fraud_bindings::*;

#[cfg(any(feature = "full", feature = "with-fraud"))]
impl Client {}
