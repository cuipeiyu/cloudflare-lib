#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetFieldGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetJobGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantLogpushJob {
    /// Unique WebSocket address that will receive messages from Cloudflare’s edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_conf: Option<String>,

    /// Comma-separated list of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    /// Filters to drill down into specific events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// The sample parameter is the sample rate of the records set by the client:
    /// "sample": 1 is 100% of records "sample": 10 is 10% and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<i64>,

    /// Unique session id of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Comma-separated list of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    /// Filters to drill down into specific events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// The sample parameter is the sample rate of the records set by the client:
    /// "sample": 1 is 100% of records "sample": 10 is 10% and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogpushJob {
    /// Unique id of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Name of the dataset. A list of supported datasets can be found on the
    /// [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<LogpushJobDataset>,

    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_conf: Option<String>,

    /// Flag that indicates if the job is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// If not null, the job is currently failing. Failures are usually. repetitive
    /// (example: no permissions to write to destination bucket). Only the last failure
    /// is recorded. On successful execution of a job the error_message and last_error
    /// are set to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// This field is deprecated. Please use `max_upload_*` parameters instead. . The
    /// frequency at which Cloudflare sends batches of logs to your destination. Setting
    /// frequency to high sends your logs in larger quantities of smaller files. Setting
    /// frequency to low sends logs in smaller quantities of larger files.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<LogpushJobFrequency>,

    /// The kind parameter (optional) is used to differentiate between Logpush and Edge
    /// Log Delivery jobs (when supported by the dataset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<LogpushJobKind>,

    /// Records the last time for which logs have been successfully pushed. If the last
    /// successful push was for logs range 2018-07-23T10:00:00Z to 2018-07-23T10:01:00Z
    /// then the value of this field will be 2018-07-23T10:01:00Z. If the job has never
    /// run or has just been enabled and hasn't run yet then the field will be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_complete: Option<DateTime<Utc>>,

    /// Records the last time the job failed. If not null, the job is currently.
    /// failing. If null, the job has either never failed or has run successfully at
    /// least once since last failure. See also the error_message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<DateTime<Utc>>,

    /// This field is deprecated. Use `output_options` instead. Configuration string. It
    /// specifies things like requested fields and timestamp formats. If migrating from
    /// the logpull api, copy the url (full url or just the query string) of your call
    /// here, and logpush will keep on making this call for you, setting start and end
    /// times appropriately.
    /// Deprecated: deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpull_options: Option<String>,

    /// The maximum uncompressed file size of a batch of logs. This setting value must
    /// be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a
    /// minimum file size; this means that log files may be much smaller than this batch
    /// size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_bytes: Option<LogpushJobMaxUploadBytes>,

    /// The maximum interval in seconds for log batches. This setting must be between 30
    /// and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify
    /// a minimum interval for log batches; this means that log files may be sent in
    /// shorter intervals than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_interval_seconds: Option<LogpushJobMaxUploadIntervalSeconds>,

    /// The maximum number of log lines per batch. This setting must be between 1000 and
    /// 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum
    /// number of log lines per batch; this means that log files may contain many fewer
    /// lines than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_records: Option<LogpushJobMaxUploadRecords>,

    /// Optional human readable job name. Not unique. Cloudflare suggests. that you set
    /// this to a meaningful string, like the domain name, to make it easier to identify
    /// your job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The structured replacement for `logpull_options`. When including this field, the
    /// `logpull_option` field will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_options: Option<OutputOptions>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptions {
    /// String to be prepended before each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prefix: Option<String>,

    /// String to be appended after each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_suffix: Option<String>,

    /// If set to true, will cause all occurrences of `${` in the generated files to be
    /// replaced with `x{`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CVE-2021-44228")]
    pub cve2021_44228: Option<bool>,

    /// String to join fields. This field be ignored when `record_template` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,

    /// List of field names to be included in the Logpush output. For the moment, there
    /// is no option to add all fields at once, so you must specify all the fields names
    /// you are interested in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Vec<String>>,

    /// Specifies the output type, such as `ndjson` or `csv`. This sets default values
    /// for the rest of the settings, depending on the chosen output type. Some
    /// formatting rules, like string quoting, are different between output types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<OutputOptionsOutputType>,

    /// String to be inserted in-between the records as separator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,

    /// String to be prepended before each record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_prefix: Option<String>,

    /// String to be appended after each record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_suffix: Option<String>,

    /// String to use as template for each record instead of the default json key value
    /// mapping. All fields used in the template must be present in `field_names` as
    /// well, otherwise they will end up as null. Format as a Go `text/template` without
    /// any standard functions, like conditionals, loops, sub-templates, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_template: Option<String>,

    /// Floating number to specify sampling rate. Sampling is applied on top of
    /// filtering, and regardless of the current `sample_interval` of the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,

    /// String to specify the format for timestamps, such as `unixnano`, `unix`, or
    /// `rfc3339`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<OutputOptionsTimestampFormat>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOptionsParam {
    /// String to be prepended before each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_prefix: Option<String>,

    /// String to be appended after each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_suffix: Option<String>,

    /// If set to true, will cause all occurrences of `${` in the generated files to be
    /// replaced with `x{`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CVE-2021-44228")]
    pub cve2021_44228: Option<String>,

    /// String to join fields. This field be ignored when `record_template` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,

    /// List of field names to be included in the Logpush output. For the moment, there
    /// is no option to add all fields at once, so you must specify all the fields names
    /// you are interested in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<String>,

    /// Specifies the output type, such as `ndjson` or `csv`. This sets default values
    /// for the rest of the settings, depending on the chosen output type. Some
    /// formatting rules, like string quoting, are different between output types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,

    /// String to be inserted in-between the records as separator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,

    /// String to be prepended before each record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_prefix: Option<String>,

    /// String to be appended after each record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_suffix: Option<String>,

    /// String to use as template for each record instead of the default json key value
    /// mapping. All fields used in the template must be present in `field_names` as
    /// well, otherwise they will end up as null. Format as a Go `text/template` without
    /// any standard functions, like conditionals, loops, sub-templates, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_template: Option<String>,

    /// Floating number to specify sampling rate. Sampling is applied on top of
    /// filtering, and regardless of the current `sample_interval` of the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,

    /// String to specify the format for timestamps, such as `unixnano`, `unix`, or
    /// `rfc3339`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDeleteResponse {
    /// Unique id of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobNewParams {
    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    pub destination_conf: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Name of the dataset. A list of supported datasets can be found on the
    /// [Developer Docs](https://developers.cloudflare.com/logs/reference/log-fields/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,

    /// Flag that indicates if the job is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// The filters to select the events to include and/or remove from your logs. For
    /// more information, refer to
    /// [Filters](https://developers.cloudflare.com/logs/reference/filters/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// This field is deprecated. Please use `max_upload_*` parameters instead. . The
    /// frequency at which Cloudflare sends batches of logs to your destination. Setting
    /// frequency to high sends your logs in larger quantities of smaller files. Setting
    /// frequency to low sends logs in smaller quantities of larger files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,

    /// The kind parameter (optional) is used to differentiate between Logpush and Edge
    /// Log Delivery jobs (when supported by the dataset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// This field is deprecated. Use `output_options` instead. Configuration string. It
    /// specifies things like requested fields and timestamp formats. If migrating from
    /// the logpull api, copy the url (full url or just the query string) of your call
    /// here, and logpush will keep on making this call for you, setting start and end
    /// times appropriately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpull_options: Option<String>,

    /// The maximum uncompressed file size of a batch of logs. This setting value must
    /// be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a
    /// minimum file size; this means that log files may be much smaller than this batch
    /// size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_bytes: Option<String>,

    /// The maximum interval in seconds for log batches. This setting must be between 30
    /// and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify
    /// a minimum interval for log batches; this means that log files may be sent in
    /// shorter intervals than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_interval_seconds: Option<String>,

    /// The maximum number of log lines per batch. This setting must be between 1000 and
    /// 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum
    /// number of log lines per batch; this means that log files may contain many fewer
    /// lines than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_records: Option<String>,

    /// Optional human readable job name. Not unique. Cloudflare suggests. that you set
    /// this to a meaningful string, like the domain name, to make it easier to identify
    /// your job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The structured replacement for `logpull_options`. When including this field, the
    /// `logpull_option` field will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_options: Option<String>,

    /// Ownership challenge token to prove destination ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_challenge: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobUpdateParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_conf: Option<String>,

    /// Flag that indicates if the job is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// The filters to select the events to include and/or remove from your logs. For
    /// more information, refer to
    /// [Filters](https://developers.cloudflare.com/logs/reference/filters/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// This field is deprecated. Please use `max_upload_*` parameters instead. . The
    /// frequency at which Cloudflare sends batches of logs to your destination. Setting
    /// frequency to high sends your logs in larger quantities of smaller files. Setting
    /// frequency to low sends logs in smaller quantities of larger files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,

    /// The kind parameter (optional) is used to differentiate between Logpush and Edge
    /// Log Delivery jobs (when supported by the dataset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// This field is deprecated. Use `output_options` instead. Configuration string. It
    /// specifies things like requested fields and timestamp formats. If migrating from
    /// the logpull api, copy the url (full url or just the query string) of your call
    /// here, and logpush will keep on making this call for you, setting start and end
    /// times appropriately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logpull_options: Option<String>,

    /// The maximum uncompressed file size of a batch of logs. This setting value must
    /// be between `5 MB` and `1 GB`, or `0` to disable it. Note that you cannot set a
    /// minimum file size; this means that log files may be much smaller than this batch
    /// size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_bytes: Option<String>,

    /// The maximum interval in seconds for log batches. This setting must be between 30
    /// and 300 seconds (5 minutes), or `0` to disable it. Note that you cannot specify
    /// a minimum interval for log batches; this means that log files may be sent in
    /// shorter intervals than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_interval_seconds: Option<String>,

    /// The maximum number of log lines per batch. This setting must be between 1000 and
    /// 1,000,000 lines, or `0` to disable it. Note that you cannot specify a minimum
    /// number of log lines per batch; this means that log files may contain many fewer
    /// lines than this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_upload_records: Option<String>,

    /// Optional human readable job name. Not unique. Cloudflare suggests. that you set
    /// this to a meaningful string, like the domain name, to make it easier to identify
    /// your job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The structured replacement for `logpull_options`. When including this field, the
    /// `logpull_option` field will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_options: Option<String>,

    /// Ownership challenge token to prove destination ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_challenge: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobListParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDeleteParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobGetParams {
    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipValidation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipNewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipNewParams {
    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    pub destination_conf: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipValidateParams {
    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    pub destination_conf: String,

    /// Ownership challenge token to prove destination ownership.
    pub ownership_challenge: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateDestinationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateDestinationExistsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOriginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateDestinationParams {
    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    pub destination_conf: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateDestinationExistsParams {
    /// Uniquely identifies a resource (such as an s3 bucket) where data. will be
    /// pushed. Additional configuration parameters supported by the destination may be
    /// included.
    pub destination_conf: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOriginParams {
    /// This field is deprecated. Use `output_options` instead. Configuration string. It
    /// specifies things like requested fields and timestamp formats. If migrating from
    /// the logpull api, copy the url (full url or just the query string) of your call
    /// here, and logpush will keep on making this call for you, setting start and end
    /// times appropriately.
    pub logpull_options: String,

    /// The Account ID to use for this endpoint. Mutually exclusive with the Zone ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Zone ID to use for this endpoint. Mutually exclusive with the Account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatasetFieldGetParamsDatasetID {
    #[serde(rename = "access_requests")]
    DatasetFieldGetParamsDatasetIDAccessRequests,
    #[serde(rename = "audit_logs")]
    DatasetFieldGetParamsDatasetIDAuditLogs,
    #[serde(rename = "audit_logs_v2")]
    DatasetFieldGetParamsDatasetIDAuditLogsV2,
    #[serde(rename = "biso_user_actions")]
    DatasetFieldGetParamsDatasetIDBISOUserActions,
    #[serde(rename = "casb_findings")]
    DatasetFieldGetParamsDatasetIDCasbFindings,
    #[serde(rename = "device_posture_results")]
    DatasetFieldGetParamsDatasetIDDevicePostureResults,
    #[serde(rename = "dex_application_tests")]
    DatasetFieldGetParamsDatasetIDDEXApplicationTests,
    #[serde(rename = "dex_device_state_events")]
    DatasetFieldGetParamsDatasetIDDEXDeviceStateEvents,
    #[serde(rename = "dlp_forensic_copies")]
    DatasetFieldGetParamsDatasetIDDLPForensicCopies,
    #[serde(rename = "dns_firewall_logs")]
    DatasetFieldGetParamsDatasetIDDNSFirewallLogs,
    #[serde(rename = "dns_logs")]
    DatasetFieldGetParamsDatasetIDDNSLogs,
    #[serde(rename = "email_security_alerts")]
    DatasetFieldGetParamsDatasetIDEmailSecurityAlerts,
    #[serde(rename = "firewall_events")]
    DatasetFieldGetParamsDatasetIDFirewallEvents,
    #[serde(rename = "gateway_dns")]
    DatasetFieldGetParamsDatasetIDGatewayDNS,
    #[serde(rename = "gateway_http")]
    DatasetFieldGetParamsDatasetIDGatewayHTTP,
    #[serde(rename = "gateway_network")]
    DatasetFieldGetParamsDatasetIDGatewayNetwork,
    #[serde(rename = "http_requests")]
    DatasetFieldGetParamsDatasetIDHTTPRequests,
    #[serde(rename = "ipsec_logs")]
    DatasetFieldGetParamsDatasetIDIPSECLogs,
    #[serde(rename = "magic_ids_detections")]
    DatasetFieldGetParamsDatasetIDMagicIDsDetections,
    #[serde(rename = "nel_reports")]
    DatasetFieldGetParamsDatasetIDNELReports,
    #[serde(rename = "network_analytics_logs")]
    DatasetFieldGetParamsDatasetIDNetworkAnalyticsLogs,
    #[serde(rename = "page_shield_events")]
    DatasetFieldGetParamsDatasetIDPageShieldEvents,
    #[serde(rename = "sinkhole_http_logs")]
    DatasetFieldGetParamsDatasetIDSinkholeHTTPLogs,
    #[serde(rename = "spectrum_events")]
    DatasetFieldGetParamsDatasetIDSpectrumEvents,
    #[serde(rename = "ssh_logs")]
    DatasetFieldGetParamsDatasetIDSSHLogs,
    #[serde(rename = "warp_config_changes")]
    DatasetFieldGetParamsDatasetIDWARPConfigChanges,
    #[serde(rename = "warp_toggle_changes")]
    DatasetFieldGetParamsDatasetIDWARPToggleChanges,
    #[serde(rename = "workers_trace_events")]
    DatasetFieldGetParamsDatasetIDWorkersTraceEvents,
    #[serde(rename = "zaraz_events")]
    DatasetFieldGetParamsDatasetIDZarazEvents,
    #[serde(rename = "zero_trust_network_sessions")]
    DatasetFieldGetParamsDatasetIDZeroTrustNetworkSessions,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatasetJobGetParamsDatasetID {
    #[serde(rename = "access_requests")]
    DatasetJobGetParamsDatasetIDAccessRequests,
    #[serde(rename = "audit_logs")]
    DatasetJobGetParamsDatasetIDAuditLogs,
    #[serde(rename = "audit_logs_v2")]
    DatasetJobGetParamsDatasetIDAuditLogsV2,
    #[serde(rename = "biso_user_actions")]
    DatasetJobGetParamsDatasetIDBISOUserActions,
    #[serde(rename = "casb_findings")]
    DatasetJobGetParamsDatasetIDCasbFindings,
    #[serde(rename = "device_posture_results")]
    DatasetJobGetParamsDatasetIDDevicePostureResults,
    #[serde(rename = "dex_application_tests")]
    DatasetJobGetParamsDatasetIDDEXApplicationTests,
    #[serde(rename = "dex_device_state_events")]
    DatasetJobGetParamsDatasetIDDEXDeviceStateEvents,
    #[serde(rename = "dlp_forensic_copies")]
    DatasetJobGetParamsDatasetIDDLPForensicCopies,
    #[serde(rename = "dns_firewall_logs")]
    DatasetJobGetParamsDatasetIDDNSFirewallLogs,
    #[serde(rename = "dns_logs")]
    DatasetJobGetParamsDatasetIDDNSLogs,
    #[serde(rename = "email_security_alerts")]
    DatasetJobGetParamsDatasetIDEmailSecurityAlerts,
    #[serde(rename = "firewall_events")]
    DatasetJobGetParamsDatasetIDFirewallEvents,
    #[serde(rename = "gateway_dns")]
    DatasetJobGetParamsDatasetIDGatewayDNS,
    #[serde(rename = "gateway_http")]
    DatasetJobGetParamsDatasetIDGatewayHTTP,
    #[serde(rename = "gateway_network")]
    DatasetJobGetParamsDatasetIDGatewayNetwork,
    #[serde(rename = "http_requests")]
    DatasetJobGetParamsDatasetIDHTTPRequests,
    #[serde(rename = "ipsec_logs")]
    DatasetJobGetParamsDatasetIDIPSECLogs,
    #[serde(rename = "magic_ids_detections")]
    DatasetJobGetParamsDatasetIDMagicIDsDetections,
    #[serde(rename = "nel_reports")]
    DatasetJobGetParamsDatasetIDNELReports,
    #[serde(rename = "network_analytics_logs")]
    DatasetJobGetParamsDatasetIDNetworkAnalyticsLogs,
    #[serde(rename = "page_shield_events")]
    DatasetJobGetParamsDatasetIDPageShieldEvents,
    #[serde(rename = "sinkhole_http_logs")]
    DatasetJobGetParamsDatasetIDSinkholeHTTPLogs,
    #[serde(rename = "spectrum_events")]
    DatasetJobGetParamsDatasetIDSpectrumEvents,
    #[serde(rename = "ssh_logs")]
    DatasetJobGetParamsDatasetIDSSHLogs,
    #[serde(rename = "warp_config_changes")]
    DatasetJobGetParamsDatasetIDWARPConfigChanges,
    #[serde(rename = "warp_toggle_changes")]
    DatasetJobGetParamsDatasetIDWARPToggleChanges,
    #[serde(rename = "workers_trace_events")]
    DatasetJobGetParamsDatasetIDWorkersTraceEvents,
    #[serde(rename = "zaraz_events")]
    DatasetJobGetParamsDatasetIDZarazEvents,
    #[serde(rename = "zero_trust_network_sessions")]
    DatasetJobGetParamsDatasetIDZeroTrustNetworkSessions,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobDataset {
    #[serde(rename = "access_requests")]
    LogpushJobDatasetAccessRequests,
    #[serde(rename = "audit_logs")]
    LogpushJobDatasetAuditLogs,
    #[serde(rename = "audit_logs_v2")]
    LogpushJobDatasetAuditLogsV2,
    #[serde(rename = "biso_user_actions")]
    LogpushJobDatasetBISOUserActions,
    #[serde(rename = "casb_findings")]
    LogpushJobDatasetCasbFindings,
    #[serde(rename = "device_posture_results")]
    LogpushJobDatasetDevicePostureResults,
    #[serde(rename = "dex_application_tests")]
    LogpushJobDatasetDEXApplicationTests,
    #[serde(rename = "dex_device_state_events")]
    LogpushJobDatasetDEXDeviceStateEvents,
    #[serde(rename = "dlp_forensic_copies")]
    LogpushJobDatasetDLPForensicCopies,
    #[serde(rename = "dns_firewall_logs")]
    LogpushJobDatasetDNSFirewallLogs,
    #[serde(rename = "dns_logs")]
    LogpushJobDatasetDNSLogs,
    #[serde(rename = "email_security_alerts")]
    LogpushJobDatasetEmailSecurityAlerts,
    #[serde(rename = "firewall_events")]
    LogpushJobDatasetFirewallEvents,
    #[serde(rename = "gateway_dns")]
    LogpushJobDatasetGatewayDNS,
    #[serde(rename = "gateway_http")]
    LogpushJobDatasetGatewayHTTP,
    #[serde(rename = "gateway_network")]
    LogpushJobDatasetGatewayNetwork,
    #[serde(rename = "http_requests")]
    LogpushJobDatasetHTTPRequests,
    #[serde(rename = "ipsec_logs")]
    LogpushJobDatasetIPSECLogs,
    #[serde(rename = "magic_ids_detections")]
    LogpushJobDatasetMagicIDsDetections,
    #[serde(rename = "nel_reports")]
    LogpushJobDatasetNELReports,
    #[serde(rename = "network_analytics_logs")]
    LogpushJobDatasetNetworkAnalyticsLogs,
    #[serde(rename = "page_shield_events")]
    LogpushJobDatasetPageShieldEvents,
    #[serde(rename = "sinkhole_http_logs")]
    LogpushJobDatasetSinkholeHTTPLogs,
    #[serde(rename = "spectrum_events")]
    LogpushJobDatasetSpectrumEvents,
    #[serde(rename = "ssh_logs")]
    LogpushJobDatasetSSHLogs,
    #[serde(rename = "warp_config_changes")]
    LogpushJobDatasetWARPConfigChanges,
    #[serde(rename = "warp_toggle_changes")]
    LogpushJobDatasetWARPToggleChanges,
    #[serde(rename = "workers_trace_events")]
    LogpushJobDatasetWorkersTraceEvents,
    #[serde(rename = "zaraz_events")]
    LogpushJobDatasetZarazEvents,
    #[serde(rename = "zero_trust_network_sessions")]
    LogpushJobDatasetZeroTrustNetworkSessions,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobFrequency {
    #[serde(rename = "high")]
    LogpushJobFrequencyHigh,
    #[serde(rename = "low")]
    LogpushJobFrequencyLow,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobKind {
    #[serde(rename = "")]
    LogpushJobKindEmpty,
    #[serde(rename = "edge")]
    LogpushJobKindEdge,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobMaxUploadBytes {
    LogpushJobMaxUploadBytes0 = 0,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobMaxUploadIntervalSeconds {
    LogpushJobMaxUploadIntervalSeconds0 = 0,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogpushJobMaxUploadRecords {
    LogpushJobMaxUploadRecords0 = 0,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputOptionsOutputType {
    #[serde(rename = "ndjson")]
    OutputOptionsOutputTypeNdjson,
    #[serde(rename = "csv")]
    OutputOptionsOutputTypeCsv,
}

#[cfg(any(feature = "full", feature = "with-logpush"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputOptionsTimestampFormat {
    #[serde(rename = "unixnano")]
    OutputOptionsTimestampFormatUnixnano,
    #[serde(rename = "unix")]
    OutputOptionsTimestampFormatUnix,
    #[serde(rename = "rfc3339")]
    OutputOptionsTimestampFormatRfc3339,
}

