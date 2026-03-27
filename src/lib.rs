//! cloudflare api client for rust

#![warn(missing_docs, missing_debug_implementations)]

#![crate_type = "lib"]

use anyhow::Result;
use anyhow::anyhow;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Method, StatusCode,
};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::sync::Arc;
use std::time::Duration;

#[doc(hidden)]
pub use reqwest::Proxy;

pub mod shared;

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

    #[doc(hidden)]
    /// Execute request
    async fn request<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        raw_body: Option<Vec<u8>>,
        json_body: &B,
    ) -> Result<ResponseManage>
    where
        Q: Serialize,
        B: Serialize + ::std::fmt::Debug,
    {
        let path = format!("https://api.cloudflare.com/client/v4{}", path);
        tracing::debug!("requesting [{}] {}", method, path);

        let mut b = self.0.request(method.clone(), &path);

        b = b.query(query);

        if method == Method::PUT || method == Method::POST {
            if let Some(body) = raw_body {
                b = b.body(body)
            } else {
                tracing::debug!("request body: {:?}", json_body);
                b = b.json(json_body);
            }
        }

        let resp = b.send().await?;
        Ok(ResponseManage(resp))
    }
}

struct ResponseManage(reqwest::Response);

impl ResponseManage {
    async fn bytes(self) -> Result<bytes::Bytes> {
        self.0.bytes().await.map_err(|err| err.into())
    }

    async fn json<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = self.0.status();
        tracing::debug!("response status: {}", status);
        tracing::debug!("response headers: {:?}", self.0.headers());

        let buf = self.0.text().await?;
        tracing::debug!("response body: {}", buf);

        if status != StatusCode::OK {
            let body = serde_json::from_str::<ApiGotError>(&buf).map_err(|err| anyhow!(err))?;
            // let body: ApiGotError = resp.json().await?;

            if let Some(err) = body.errors.into_iter().next() {
                tracing::info!("pick err: {:?}", err);
                return Err(err.into());
            }

            if let Some(messages) = body.messages {
                if let Some(msg) = messages.into_iter().next() {
                    tracing::info!("pick msg: {:?}", msg);
                    return Err(msg.into());
                }
            }

            return Err(anyhow!("request error"));
        } else {
            let body = serde_json::from_str::<T>(&buf).map_err(|err| anyhow!(err))?;

            return Ok(body);
        }
    }
}

/// API response message
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponseMessage {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

impl ::std::fmt::Display for ApiResponseMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.code,
            self.message,
            self.documentation_url
                .as_ref()
                .map(|s| format!(", find more information: {}", s))
                .unwrap_or_default(),
        )
    }
}

impl Into<anyhow::Error> for ApiResponseMessage {
    fn into(self) -> anyhow::Error {
        anyhow!(self.to_string())
    }
}

/// API response error
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiResponseError {
    pub code: i64,
    pub message: String,
    /// [optional] documentation link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

impl ::std::fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.code,
            self.message,
            self.documentation_url
                .as_ref()
                .map(|s| format!(", find more information: {}", s))
                .unwrap_or_default(),
        )
    }
}

impl Into<anyhow::Error> for ApiResponseError {
    fn into(self) -> anyhow::Error {
        anyhow!(self.to_string())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiResponseResultInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ApiResponse<Res> {
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<ApiResponseMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ApiResponseError>>,
    success: bool,
    result: Res,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_info: Option<ApiResponseResultInfo>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ApiGotError {
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<ApiResponseMessage>>,
    errors: Vec<ApiResponseError>,
    success: bool,
}

impl<T> Into<ApiPagePagination<T>> for ApiResponse<T>
where
    T: DeserializeOwned + Serialize,
{
    fn into(self) -> ApiPagePagination<T> {
        ApiPagePagination {
            result: self.result,
            result_info: self.result_info.unwrap_or_default().into(),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiPagePagination<T> {
    pub result: T,
    pub result_info: ApiPagePaginationResultInfo,
}

impl<T> ApiPagePagination<T> {
    pub async fn get_next_page(&self) -> Result<ApiPagePagination<T>> {
        // TODO
        unimplemented!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCursorLimitPagination<T> {
    pub result: Vec<T>,
    pub result_info: ApiCursorLimitPaginationResultInfo,
}

impl<T> Into<ApiCursorLimitPagination<T>> for ApiResponse<Vec<T>> {
    fn into(self) -> ApiCursorLimitPagination<T> {
        ApiCursorLimitPagination {
            result: self.result,
            result_info: self.result_info.unwrap_or_default().into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCursorLimitPaginationResultInfo {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub per_page: Option<i64>,
}

impl Into<ApiCursorLimitPaginationResultInfo> for ApiResponseResultInfo {
    fn into(self) -> ApiCursorLimitPaginationResultInfo {
        ApiCursorLimitPaginationResultInfo {
            count: self.count,
            cursor: self.cursor,
            per_page: self.per_page,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCursorPaginationAfter<T> {
    pub result: Vec<T>,
    pub result_info: ApiCursorPaginationAfterResultInfo,
}

impl<T> Into<ApiCursorPaginationAfter<T>> for ApiResponse<Vec<T>>
where
    T: DeserializeOwned + Serialize,
{
    fn into(self) -> ApiCursorPaginationAfter<T> {
        ApiCursorPaginationAfter {
            result: self.result,
            result_info: self.result_info.unwrap_or_default().into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCursorPaginationAfterResultInfo {
    pub cursors: ApiCursorPaginationAfterResultInfoCursors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCursorPaginationAfterResultInfoCursors {
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSinglePage<T> {
    pub result: T,
}

impl<T> Into<ApiSinglePage<T>> for ApiResponse<T>
where
    T: DeserializeOwned + Serialize,
{
    fn into(self) -> ApiSinglePage<T> {
        ApiSinglePage {
            result: self.result,
        }
    }
}

impl<T> ApiSinglePage<T> {
    pub async fn get_next_page(&self) -> Result<ApiSinglePage<T>> {
        // TODO
        unimplemented!();
    }
}

// impl <T> ApiPagePagination<T>
// where
//     T: DeserializeOwned,
//     T: Serialize,
// {
//     /// call 类似 async move |page| {}
//     pub(crate) fn new<F, Fut>(result: Vec<T>, result_info: ApiPagePaginationResultInfo, call: F) -> Self
//     where
//         F: Fn(i64) -> Fut + Send + 'static,
//         Fut: std::future::Future<Output = Result<ApiPagePagination<T>>> + Send + 'static,
//     {
//         Self {
//             result,
//             result_info,
//             call: Box::pin(call),
//         }
//     }

//     pub fn no_more(&self) -> bool {
//         self.result_info.per_page > self.result.len() as i64
//     }

//     pub async fn next_page(self) -> Result<ApiPagePagination<T>> {
//         let next_page_num = self.result_info.page + 1;
//         (self.call)(next_page_num).await
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPagePaginationResultInfo {
    pub page: i64,
    pub per_page: i64,
}

impl Into<ApiPagePaginationResultInfo> for ApiResponseResultInfo {
    fn into(self) -> ApiPagePaginationResultInfo {
        ApiPagePaginationResultInfo {
            page: self.page.unwrap_or(1),
            per_page: self.per_page.unwrap_or(20),
        }
    }
}

impl Into<ApiCursorPaginationAfterResultInfo> for ApiResponseResultInfo {
    fn into(self) -> ApiCursorPaginationAfterResultInfo {
        ApiCursorPaginationAfterResultInfo {
            cursors: ApiCursorPaginationAfterResultInfoCursors {
                after: self.cursor.unwrap_or_default(),
            },
        }
    }
}

#[inline]
fn read_env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client() {
        let _ = Client::new();
    }
}

include!("abuse_reports_api.rs");
include!("accounts_api.rs");
include!("kv_api.rs");
