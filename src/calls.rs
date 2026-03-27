#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};


#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUNewResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Bearer token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUUpdateResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUListResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUDeleteResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUGetResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUUpdateParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUListParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFUGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNNewResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Bearer token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of a TURN key, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNUpdateResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNListResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNDeleteResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNGetResponse {
    /// The date and time the item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The date and time the item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A short description of Calls app, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A Cloudflare-generated unique identifier for a item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A short description of a TURN key, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNUpdateParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A short description of a TURN key, not shown to end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNListParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-calls"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TURNGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}
