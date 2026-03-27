
#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmbConfig {
    /// Allow out of region access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_out_of_region_access: Option<bool>,

    /// Name of the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmbConfigParam {
    /// Allow out of region access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_out_of_region_access: Option<String>,

    /// Name of the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCmbConfigNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub cmb_config: CmbConfigParam,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCmbConfigDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCmbConfigGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRetentionNewResponse {
    /// The log retention flag for Logpull API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRetentionGetResponse {
    /// The log retention flag for Logpull API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRetentionNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The log retention flag for Logpull API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRetentionGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayIDGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The `/received` route by default returns a limited set of fields, and allows
    /// customers to override the default field set by specifying individual fields. The
    /// reasons for this are: 1. Most customers require only a small subset of fields,
    /// but that subset varies from customer to customer; 2. Flat schema is much easier
    /// to work with downstream (importing into BigTable etc); 3. Performance (time to
    /// process, file size). If `?fields=` is not specified, default field set is
    /// returned. This default field set may change at any time. When `?fields=` is
    /// provided, each record is returned with the specified fields. `fields` must be
    /// specified as a comma separated list without any whitespaces, and all fields must
    /// exist. The order in which fields are specified does not matter, and the order of
    /// fields in the response is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    /// By default, timestamps in responses are returned as Unix nanosecond integers.
    /// The `?timestamps=` argument can be set to change the format in which response
    /// timestamps are returned. Possible values are: `unix`, `unixnano`, `rfc3339`.
    /// Note that `unix` and `unixnano` return timestamps as integers; `rfc3339` returns
    /// timestamps as strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Sets the (exclusive) end of the requested time frame. This can be a unix
    /// timestamp (in seconds or nanoseconds), or an absolute timestamp that conforms to
    /// RFC 3339. `end` must be at least five minutes earlier than now and must be later
    /// than `start`. Difference between `start` and `end` must be not greater than one
    /// hour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// When `?count=` is provided, the response will contain up to `count` results.
    /// Since results are not sorted, you are likely to get different data for repeated
    /// requests. `count` must be an integer > 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,

    /// The `/received` route by default returns a limited set of fields, and allows
    /// customers to override the default field set by specifying individual fields. The
    /// reasons for this are: 1. Most customers require only a small subset of fields,
    /// but that subset varies from customer to customer; 2. Flat schema is much easier
    /// to work with downstream (importing into BigTable etc); 3. Performance (time to
    /// process, file size). If `?fields=` is not specified, default field set is
    /// returned. This default field set may change at any time. When `?fields=` is
    /// provided, each record is returned with the specified fields. `fields` must be
    /// specified as a comma separated list without any whitespaces, and all fields must
    /// exist. The order in which fields are specified does not matter, and the order of
    /// fields in the response is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    /// When `?sample=` is provided, a sample of matching records is returned. If
    /// `sample=0.1` then 10% of records will be returned. Sampling is random: repeated
    /// calls will not only return different records, but likely will also vary slightly
    /// in number of returned records. When `?count=` is also specified, `count` is
    /// applied to the number of returned records, not the sampled records. So, with
    /// `sample=0.05` and `count=7`, when there is a total of 100 records available,
    /// approximately five will be returned. When there are 1000 records, seven will be
    /// returned. When there are 10,000 records, seven will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,

    /// Sets the (inclusive) beginning of the requested time frame. This can be a unix
    /// timestamp (in seconds or nanoseconds), or an absolute timestamp that conforms to
    /// RFC 3339. At this point in time, it cannot exceed a time in the past greater
    /// than seven days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// By default, timestamps in responses are returned as Unix nanosecond integers.
    /// The `?timestamps=` argument can be set to change the format in which response
    /// timestamps are returned. Possible values are: `unix`, `unixnano`, `rfc3339`.
    /// Note that `unix` and `unixnano` return timestamps as integers; `rfc3339` returns
    /// timestamps as strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedFieldGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logs"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedFieldGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

