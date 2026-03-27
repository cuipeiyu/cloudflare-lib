#[cfg(any(feature = "full", feature = "with-hostnames"))]
mod hostnames_bindings {
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SettingValueString {
        #[serde(rename = "1.0")]
        V1_0 ,
        #[serde(rename = "1.1")]
        V1_1 ,
        #[serde(rename = "1.2")]
        V1_2 ,
        #[serde(rename = "1.3")]
        V1_3 ,
        #[serde(rename = "on")]
        On  ,
        #[serde(rename = "off")]
        Off ,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Setting<T> {
        /// This is the time the tls setting was originally created for this hostname.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateTime<Utc>>,

        /// The hostname for which the tls settings are set.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,

        /// Deployment status for the given tls setting.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,

        /// This is the time the tls setting was updated.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateTime<Utc>>,

        /// The TLS setting value. The type depends on the `setting_id` used in the request
        /// path:
        /// - `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g.,
        /// `["ECDHE-RSA-AES128-GCM-SHA256", "AES128-GCM-SHA256"]`)
        /// - `min_tls_version`: a string indicating the minimum TLS version — one of
        /// `"1.0"`, `"1.1"`, `"1.2"`, or `"1.3"` (e.g., `"1.2"`)
        /// - `http2`: a string indicating whether HTTP/2 is enabled — `"on"` or `"off"`
        /// (e.g., `"on"`)
        ///
        /// Union satisfied by [SettingValueString] or [SettingValueArray].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<T>,
    }

    pub type SettingValueArray = Vec<String>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingTLSDeleteResponse<T> {
        /// This is the time the tls setting was originally created for this hostname.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateTime<Utc>>,

        /// The hostname for which the tls settings are set.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,

        /// Deployment status for the given tls setting.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,

        /// This is the time the tls setting was updated.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateTime<Utc>>,

        /// The TLS setting value. The type depends on the `setting_id` used in the request
        /// path:
        /// - `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g.,
        /// `["ECDHE-RSA-AES128-GCM-SHA256", "AES128-GCM-SHA256"]`)
        /// - `min_tls_version`: a string indicating the minimum TLS version — one of
        /// `"1.0"`, `"1.1"`, `"1.2"`, or `"1.3"` (e.g., `"1.2"`)
        /// - `http2`: a string indicating whether HTTP/2 is enabled — `"on"` or `"off"`
        /// (e.g., `"on"`)
        ///
        /// Union satisfied by [SettingValueString] or [SettingValueArray].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<T>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingTLSGetResponse<T> {
        /// This is the time the tls setting was originally created for this hostname.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_at: Option<DateTime<Utc>>,

        /// The hostname for which the tls settings are set.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,

        /// Deployment status for the given tls setting.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,

        /// This is the time the tls setting was updated.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<DateTime<Utc>>,

        /// The TLS setting value. The type depends on the `setting_id` used in the request
        /// path:
        /// - `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g.,
        /// `["ECDHE-RSA-AES128-GCM-SHA256", "AES128-GCM-SHA256"]`)
        /// - `min_tls_version`: a string indicating the minimum TLS version — one of
        /// `"1.0"`, `"1.1"`, `"1.2"`, or `"1.3"` (e.g., `"1.2"`)
        /// - `http2`: a string indicating whether HTTP/2 is enabled — `"on"` or `"off"`
        /// (e.g., `"on"`)
        ///
        /// Union satisfied by [SettingValueString] or [SettingValueArray].
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<T>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingTLSUpdateParams {
        /// Identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,

        /// The TLS setting value. The type depends on the `setting_id` used in the request
        /// path:
        /// - `ciphers`: an array of allowed cipher suite strings in BoringSSL format (e.g.,
        /// `["ECDHE-RSA-AES128-GCM-SHA256", "AES128-GCM-SHA256"]`)
        /// - `min_tls_version`: a string indicating the minimum TLS version — one of
        /// `"1.0"`, `"1.1"`, `"1.2"`, or `"1.3"` (e.g., `"1.2"`)
        /// - `http2`: a string indicating whether HTTP/2 is enabled — `"on"` or `"off"`
        /// (e.g., `"on"`)
        pub value: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingTLSDeleteParams {
        /// Identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SettingTLSGetParams {
        /// Identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_id: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SettingTLSUpdateParamsSettingID {
        #[serde(rename = "ciphers")]
        SettingTLSUpdateParamsSettingIDCiphers,
        #[serde(rename = "min_tls_version")]
        SettingTLSUpdateParamsSettingIDMinTLSVersion,
        #[serde(rename = "http2")]
        SettingTLSUpdateParamsSettingIDHTTP2,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SettingTLSDeleteParamsSettingID {
        #[serde(rename = "ciphers")]
        SettingTLSDeleteParamsSettingIDCiphers,
        #[serde(rename = "min_tls_version")]
        SettingTLSDeleteParamsSettingIDMinTLSVersion,
        #[serde(rename = "http2")]
        SettingTLSDeleteParamsSettingIDHTTP2,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum SettingTLSGetParamsSettingID {
        #[serde(rename = "ciphers")]
        SettingTLSGetParamsSettingIDCiphers,
        #[serde(rename = "min_tls_version")]
        SettingTLSGetParamsSettingIDMinTLSVersion,
        #[serde(rename = "http2")]
        SettingTLSGetParamsSettingIDHTTP2,
    }
}

#[cfg(any(feature = "full", feature = "with-hostnames"))]
pub use hostnames_bindings::*;

#[cfg(any(feature = "full", feature = "with-hostnames"))]
impl Client {}
