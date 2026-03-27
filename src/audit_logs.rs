#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{DateTime, Duration, Utc};

#[cfg(any(feature = "full", feature = "with-audit-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListQuery {
    /// Finds a specific log by its ID.
    pub id: Option<String>,
    pub action: Option<AuditLogListParamsAction>,
    pub actor: Option<AuditLogListParamsActor>,
    /// Limits the returned results to logs older than the specified date. A `full-date`
    /// that conforms to RFC3339.
    pub before: Option<DateTime<Utc>>,
    /// Changes the direction of the chronological sorting.
    pub direction: Option<AuditLogListParamsDirection>,
    /// Indicates that this request is an export of logs in CSV format.
    pub export: Option<bool>,
    /// Indicates whether or not to hide user level audit logs.
    pub hide_user_logs: Option<bool>,
    /// Defines which page of results to return.
    pub page: Option<i64>,
    /// Sets the number of results to return per page.
    pub per_page: Option<i64>,
    /// Limits the returned results to logs newer than the specified date. A `full-date`
    /// that conforms to RFC3339.
    pub since: Option<DateTime<Utc>>,
    pub zone: Option<AuditLogListParamsZone>,
}

#[cfg(any(feature = "full", feature = "with-audit-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListParamsAction {
    /// Filters by the action type.
    pub r#type: Option<String>,
}

#[cfg(any(feature = "full", feature = "with-audit-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListParamsActor {
    /// Filters by the email address of the actor that made the change.
    pub email: Option<String>,
    /// Filters by the IP address of the request that made the change by specific IP
    /// address or valid CIDR Range.
    pub ip: Option<String>,
}

// Changes the direction of the chronological sorting.
#[cfg(any(feature = "full", feature = "with-audit-logs"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLogListParamsDirection {
    /// value is "desc"
    #[serde(rename = "desc")]
    Desc,
    /// value is "asc"
    #[serde(rename = "asc")]
    Asc,
}

#[cfg(any(feature = "full", feature = "with-audit-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListParamsZone {
    /// Filters by the name of the zone associated to the change.
    pub name: Option<String>,
}
