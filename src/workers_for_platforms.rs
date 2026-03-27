#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// When the script was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// Name of the Workers for Platforms dispatch namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_namespace: Option<String>,

    /// When the script was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::workers::Script>,

}

