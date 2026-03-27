//! cloudflare-api for rust

use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
};
use std::sync::Arc;
use std::time::Duration;

#[doc(hidden)]
pub use reqwest::Proxy;

/// Authentication methods
/// ```
/// // example 1:
/// // use email and password
/// // 'EMAIL' will be set to header 'x-auth-email'
/// //  'KEY' will be set to header 'x-auth-key'
/// ApiAuth::EmailAndKey("<EMAIL>".to_string(), "<KEY>".to_string())
/// ```
///
/// ```
/// // example 2:
/// ApiAuth::Token("<TOKEN>".to_string())
/// ```
///
/// ```
/// // example 3:
/// // read <CLOUDFLARE_TOKEN> from environment variable
/// ApiAuth::default()
/// ```
#[derive(Debug, Clone)]
pub enum ApiAuth {
    /// Use email and key
    EmailAndKey(String, String),
    /// Use token
    Token(String),
}

impl Default for ApiAuth {
    fn default() -> Self {
        Self::Token(::std::env::var("CLOUDFLARE_TOKEN").unwrap_or_default())
    }
}



/// Builder for Cloudflare API client
#[derive(Debug)]
pub struct ClientBuilder {
    auth: ApiAuth,
    proxies: Vec<Proxy>,
    timeout: Option<Duration>,
}

impl ClientBuilder {
    /// Create a new client builder
    ///
    /// Your must provide an authentication method
    ///
    /// ```
    /// let client = ClientBuilder::new(ApiAuth::default()).build();
    /// ```
    pub fn new(auth: ApiAuth) -> Self {
        Self {
            auth,
            proxies: vec![],
            timeout: None,
        }
    }

    /// Add a proxy to the client
    pub fn with_proxy(mut self, proxy: Proxy) -> Self {
        self.proxies.push(proxy);
        self
    }

    /// Set timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<Client> {
        let mut builder = reqwest::ClientBuilder::new();

        let mut headers = HeaderMap::new();

        match self.auth {
            ApiAuth::EmailAndKey(email, key) => {
                headers.insert("x-auth-email", HeaderValue::from_str(&email).unwrap());
                headers.insert("x-auth-key", HeaderValue::from_str(&key).unwrap());
                tracing::debug!("Using x-auth-email,x-auth-key auth");
            }
            ApiAuth::Token(token) => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                );
                tracing::debug!("Using token auth");
            }
        }

        builder = builder.default_headers(headers);

        for proxy in self.proxies {
            // add proxy
            builder = builder.proxy(proxy)
        }

        if let Some(v) = self.timeout {
            builder = builder.timeout(v);
        }

        let http = Arc::new(builder.build()?);

        Ok(Client(http))
    }
}

/// The Cloudflare API client.
#[derive(Clone, Debug)]
pub struct Client(Arc<reqwest::Client>);

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a new client with the default configuration.
    /// Will use the default configuration from the environment.
    pub fn new() -> Self {
        ClientBuilder::new(ApiAuth::default()).build().unwrap()
    }
}

