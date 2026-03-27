#[cfg(any(feature = "full", feature = "with-hyperdrive"))]
mod hyperdrive_bindings {

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigNewParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        pub hyperdrive: HyperdriveParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigUpdateParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        pub hyperdrive: HyperdriveParam,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigListParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigDeleteParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigEditParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub caching: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mtls: Option<String>,

        /// The name of the Hyperdrive configuration. Used to identify the configuration in
        /// the Cloudflare dashboard and API.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,

        /// The (soft) maximum number of connections the Hyperdrive is allowed to make to
        /// the origin database.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_connection_limit: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConfigGetParams {
        /// Define configurations using a unique string identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Hyperdrive {
        /// Define configurations using a unique string identifier.
        pub id: String,

        /// The name of the Hyperdrive configuration. Used to identify the configuration in
        /// the Cloudflare dashboard and API.
        pub name: String,

        pub origin: HyperdriveOrigin,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub caching: Option<HyperdriveCaching>,

        /// Defines the creation time of the Hyperdrive configuration.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created_on: Option<DateTime<Utc>>,

        /// Defines the last modified time of the Hyperdrive configuration.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub modified_on: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mtls: Option<HyperdriveMTLS>,

        /// The (soft) maximum number of connections the Hyperdrive is allowed to make to
        /// the origin database.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_connection_limit: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperdriveParam {
        /// The name of the Hyperdrive configuration. Used to identify the configuration in
        /// the Cloudflare dashboard and API.
        pub name: String,

        pub origin: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub caching: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub mtls: Option<String>,

        /// The (soft) maximum number of connections the Hyperdrive is allowed to make to
        /// the origin database.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_connection_limit: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperdriveOrigin {
        /// Set the name of your origin database.
        pub database: String,

        /// Defines the host (hostname or IP) of your origin database.
        pub host: String,

        /// Specifies the URL scheme used to connect to your origin database.
        pub scheme: HyperdriveOriginScheme,

        /// Set the user of your origin database.
        pub user: String,

        /// Defines the Client ID of the Access token to use when connecting to the origin
        /// database.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_client_id: Option<String>,

        /// Defines the port of your origin database. Defaults to 5432 for PostgreSQL or
        /// 3306 for MySQL if not specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperdriveCaching {
        /// Set to true to disable caching of SQL responses. Default is false.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,

        /// Specify the maximum duration (in seconds) items should persist in the cache.
        /// Defaults to 60 seconds if not specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_age: Option<i64>,

        /// Specify the number of seconds the cache may serve a stale response. Defaults to
        /// 15 seconds if not specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stale_while_revalidate: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperdriveMTLS {
        /// Define CA certificate ID obtained after uploading CA cert.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ca_certificate_id: Option<String>,

        /// Define mTLS certificate ID obtained after uploading client cert.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mtls_certificate_id: Option<String>,

        /// Set SSL mode to 'require', 'verify-ca', or 'verify-full' to verify the CA.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sslmode: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HyperdriveOriginScheme {
        #[serde(rename = "postgres")]
        HyperdriveOriginSchemePostgres,
        #[serde(rename = "postgresql")]
        HyperdriveOriginSchemePostgresql,
        #[serde(rename = "mysql")]
        HyperdriveOriginSchemeMysql,
    }
}

#[cfg(any(feature = "full", feature = "with-hyperdrive"))]
pub use hyperdrive_bindings::*;

#[cfg(any(feature = "full", feature = "with-hyperdrive"))]
impl Client {}
