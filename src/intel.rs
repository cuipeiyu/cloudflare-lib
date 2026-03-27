#[cfg(any(feature = "full", feature = "with-intel"))]
mod intel_bindings {


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASNGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASNSubnetGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<crate::shared::ASN>,

    /// Total results returned based on your search parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_count_total: Option<i64>,

    /// Current page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,

    /// Number of results per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ASNSubnetGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueListResponse {
    /// Indicates the total number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<AttackSurfaceReportIssueListResponseIssue>>,

    /// Specifies the current page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,

    /// Sets the number of results per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueClassResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissResponse {
    pub errors: Vec<AttackSurfaceReportIssueDismissResponseError>,

    pub messages: Vec<AttackSurfaceReportIssueDismissResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueSeverityResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueTypeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type_neq: Option<String>,

    /// Specifies the current page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Sets the number of results per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_neq: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueClassParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_neq: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueSeverityParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_neq: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueTypeParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_neq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_neq: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueTypeGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNS {
    /// Total results returned based on your search parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

    /// Current page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,

    /// Number of results per page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<f64>,

    /// Reverse DNS look-ups observed during the time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_records: Option<Vec<DNSReverseRecord>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    /// Requested page within paginated list of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// Maximum number of results requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_end_params: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Additional information related to the host name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<DomainAdditionalInformation>,

    /// Application that the hostname belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<DomainApplication>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_categories: Option<Vec<DomainContentCategory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_content_categories: Option<Vec<DomainInheritedContentCategory>>,

    /// Domain from which `inherited_content_categories` and `inherited_risk_types` are
    /// inherited, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_risk_types: Option<Vec<DomainInheritedRiskType>>,

    /// Global Cloudflare 100k ranking for the last 30 days, if available for the
    /// hostname. The top ranked domain is 1, the lowest ranked domain is 100,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity_rank: Option<i64>,

    /// Specifies a list of references to one or more IP addresses or domain names that
    /// the domain name currently resolves to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolves_to_refs: Option<Vec<DomainResolvesToRef>>,

    /// Hostname risk score, which is a value between 0 (lowest risk) to 1 (highest
    /// risk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_types: Option<Vec<DomainRiskType>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponse {
    /// Additional information related to the host name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<DomainBulkGetResponseAdditionalInformation>,

    /// Application that the hostname belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<DomainBulkGetResponseApplication>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_categories: Option<Vec<DomainBulkGetResponseContentCategory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_content_categories: Option<Vec<DomainBulkGetResponseInheritedContentCategory>>,

    /// Domain from which `inherited_content_categories` and `inherited_risk_types` are
    /// inherited, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_risk_types: Option<Vec<DomainBulkGetResponseInheritedRiskType>>,

    /// Global Cloudflare 100k ranking for the last 30 days, if available for the
    /// hostname. The top ranked domain is 1, the lowest ranked domain is 100,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity_rank: Option<i64>,

    /// Hostname risk score, which is a value between 0 (lowest risk) to 1 (highest
    /// risk).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_types: Option<Vec<DomainBulkGetResponseRiskType>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Accepts multiple values like `?domain=cloudflare.com&domain=example.com`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorizations: Option<Vec<DomainHistoryCategorization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainHistoryGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedNewResponse {
    /// The unique identifier for the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The date and time when the data entry was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the indicator feed can be attributed to a provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<bool>,

    /// Whether the indicator feed can be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,

    /// Whether the indicator feed is exposed to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// The date and time when the data entry was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedUpdateResponse {
    /// The unique identifier for the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The date and time when the data entry was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the indicator feed can be attributed to a provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<bool>,

    /// Whether the indicator feed can be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,

    /// Whether the indicator feed is exposed to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// The date and time when the data entry was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedListResponse {
    /// The unique identifier for the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The date and time when the data entry was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the indicator feed can be attributed to a provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<bool>,

    /// Whether the indicator feed can be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,

    /// Whether the indicator feed is exposed to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// The date and time when the data entry was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedGetResponse {
    /// The unique identifier for the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The date and time when the data entry was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the indicator feed can be attributed to a provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<bool>,

    /// Whether the indicator feed can be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,

    /// Whether the indicator feed is exposed to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// Status of the latest snapshot uploaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_upload_status: Option<IndicatorFeedGetResponseLatestUploadStatus>,

    /// The date and time when the data entry was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The unique identifier for the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// The provider of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The new description of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The new is_attributable value of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<String>,

    /// The new is_downloadable value of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<String>,

    /// The new is_public value of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,

    /// The new name of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedDataParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedGetParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionNewResponse {
    /// Whether the update succeeded or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionListResponse {
    /// The unique identifier for the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The description of the example test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the indicator feed can be attributed to a provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attributable: Option<bool>,

    /// Whether the indicator feed can be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,

    /// Whether the indicator feed is exposed to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// The name of the indicator feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionDeleteResponse {
    /// Whether the update succeeded or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionNewParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Cloudflare account tag of the account to change permissions on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tag: Option<String>,

    /// The ID of the feed to add/remove permissions on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedPermissionDeleteParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The Cloudflare account tag of the account to change permissions on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tag: Option<String>,

    /// The ID of the feed to add/remove permissions on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedSnapshotUpdateResponse {
    /// Feed id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i64>,

    /// Name of the file unified in our system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// Current status of upload, should be unified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorFeedSnapshotUpdateParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The file to upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IP {
    /// Specifies a reference to the autonomous systems (AS) that the IP address belongs
    /// to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belongs_to_ref: Option<IPBelongsToRef>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_types: Option<Vec<IPRiskType>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewResponse {
    pub errors: Vec<MiscategorizationNewResponseError>,

    pub messages: Vec<MiscategorizationNewResponseMessage>,

    /// Whether the API call was successful.
    pub success: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Content category IDs to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_adds: Option<String>,

    /// Content category IDs to remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_removes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_type: Option<String>,

    /// Provide only if indicator_type is `ipv4` or `ipv6`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Security category IDs to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_adds: Option<String>,

    /// Security category IDs to remove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_removes: Option<String>,

    /// Provide only if indicator_type is `domain` or `url`. Example if indicator_type
    /// is `domain`: `example.com`. Example if indicator_type is `url`:
    /// `https://example.com/news/`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sinkhole {
    /// The unique identifier for the sinkhole
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The account tag that owns this sinkhole
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tag: Option<String>,

    /// The date and time when the sinkhole was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<DateTime<Utc>>,

    /// The date and time when the sinkhole was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<DateTime<Utc>>,

    /// The name of the sinkhole
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The name of the R2 bucket to store results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2_bucket: Option<String>,

    /// The id of the R2 instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkholeListParams {
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisGetResponse {
    pub dnssec: bool,

    pub domain: String,

    pub extension: String,

    pub found: bool,

    pub nameservers: Vec<String>,

    pub punycode: String,

    pub registrant: String,

    pub registrar: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_fax_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_org: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_phone_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_referral_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fax_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_org: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_referral_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_raw: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_raw: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_fax_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_org: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_phone_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_referral_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_fax_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_org: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_phone_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_referral_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_fax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_fax_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_org: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_phone_ext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_referral_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_raw: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois_server: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisGetParams {
    /// Use to uniquely identify or reference the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueListResponseIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<IssueType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<AttackSurfaceReportIssueListResponseIssuesPayload>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<AttackSurfaceReportIssueListResponseIssuesSeverity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AttackSurfaceReportIssueDismissResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AttackSurfaceReportIssueDismissResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSReverseRecord {
    /// First seen date of the DNS record during the time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<DateTime<Utc>>,

    /// Hostname that the IP was observed resolving to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Last seen date of the DNS record during the time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAdditionalInformation {
    /// Suspected DGA malware family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspected_malware_family: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainContentCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainInheritedContentCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainInheritedRiskType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainResolvesToRef {
    /// STIX 2.1 identifier:
    /// https://docs.oasis-open.org/cti/stix/v2.1/cs02/stix-v2.1-cs02.html#_64yvzeku5a5c.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// IP address or domain name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRiskType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseAdditionalInformation {
    /// Suspected DGA malware family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspected_malware_family: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseContentCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseInheritedContentCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseInheritedRiskType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainBulkGetResponseRiskType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainHistoryCategorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<DomainHistoryCategorizationsCategory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPBelongsToRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Infrastructure type of this ASN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IPBelongsToRefType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPRiskType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_category_id: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewResponseError {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MiscategorizationNewResponseErrorsSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewResponseMessage {
    pub code: i64,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MiscategorizationNewResponseMessagesSource>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueListResponseIssuesPayload {
    /// Describes the method used to detect insight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_tag: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceReportIssueDismissResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainHistoryCategorizationsCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewResponseErrorsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscategorizationNewResponseMessagesSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueType {
    #[serde(rename = "compliance_violation")]
    IssueTypeComplianceViolation,
    #[serde(rename = "email_security")]
    IssueTypeEmailSecurity,
    #[serde(rename = "exposed_infrastructure")]
    IssueTypeExposedInfrastructure,
    #[serde(rename = "insecure_configuration")]
    IssueTypeInsecureConfiguration,
    #[serde(rename = "weak_authentication")]
    IssueTypeWeakAuthentication,
    #[serde(rename = "configuration_suggestion")]
    IssueTypeConfigurationSuggestion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SeverityQueryParam {
    #[serde(rename = "low")]
    SeverityQueryParamLow,
    #[serde(rename = "moderate")]
    SeverityQueryParamModerate,
    #[serde(rename = "critical")]
    SeverityQueryParamCritical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndicatorFeedGetResponseLatestUploadStatus {
    #[serde(rename = "Mirroring")]
    IndicatorFeedGetResponseLatestUploadStatusMirroring,
    #[serde(rename = "Unifying")]
    IndicatorFeedGetResponseLatestUploadStatusUnifying,
    #[serde(rename = "Loading")]
    IndicatorFeedGetResponseLatestUploadStatusLoading,
    #[serde(rename = "Provisioning")]
    IndicatorFeedGetResponseLatestUploadStatusProvisioning,
    #[serde(rename = "Complete")]
    IndicatorFeedGetResponseLatestUploadStatusComplete,
    #[serde(rename = "Error")]
    IndicatorFeedGetResponseLatestUploadStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttackSurfaceReportIssueListResponseIssuesSeverity {
    #[serde(rename = "Low")]
    AttackSurfaceReportIssueListResponseIssuesSeverityLow,
    #[serde(rename = "Moderate")]
    AttackSurfaceReportIssueListResponseIssuesSeverityModerate,
    #[serde(rename = "Critical")]
    AttackSurfaceReportIssueListResponseIssuesSeverityCritical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IPBelongsToRefType {
    #[serde(rename = "hosting_provider")]
    IPBelongsToRefTypeHostingProvider,
    #[serde(rename = "isp")]
    IPBelongsToRefTypeISP,
    #[serde(rename = "organization")]
    IPBelongsToRefTypeOrganization,
}

}

#[cfg(any(feature = "full", feature = "with-intel"))]
pub use intel_bindings::*;

#[cfg(any(feature = "full", feature = "with-intel"))]
impl Client {}
