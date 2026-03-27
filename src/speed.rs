#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<AvailabilityQuota>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<LabeledRegion>>,

    /// Available regions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "regionsPerPlan")]
    pub regions_per_plan: Option<AvailabilityRegionsPerPlan>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageListResponse {
    /// A test region with a label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<LabeledRegion>,

    /// The frequency of the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduleFrequency")]
    pub schedule_frequency: Option<PageListResponseScheduleFrequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<Test>>,

    /// A URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTrendParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// The type of device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    /// A comma-separated list of metrics to include in the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// The timezone of the start and end timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    /// UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,

    /// The Lighthouse report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "desktopReport")]
    pub desktop_report: Option<LighthouseReport>,

    /// The Lighthouse report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mobileReport")]
    pub mobile_report: Option<LighthouseReport>,

    /// A test region with a label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<LabeledRegion>,

    /// The frequency of the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduleFrequency")]
    pub schedule_frequency: Option<TestScheduleFrequency>,

    /// A URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTestDeleteResponse {
    /// Number of items affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTestNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTestListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTestDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTestGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// The frequency of the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduleFrequency>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<ScheduleRegion>,

    /// A URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleNewResponse {
    /// The test schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Test>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleDeleteResponse {
    /// Number of items affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledRegion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// A test region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<LabeledRegionValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LighthouseReport {
    /// Cumulative Layout Shift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cls: Option<f64>,

    /// The type of device.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceType")]
    pub device_type: Option<LighthouseReportDeviceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<LighthouseReportError>,

    /// First Contentful Paint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcp: Option<f64>,

    /// The URL to the full Lighthouse JSON report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jsonReportUrl")]
    pub json_report_url: Option<String>,

    /// Largest Contentful Paint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcp: Option<f64>,

    /// The Lighthouse performance score.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "performanceScore")]
    pub performance_score: Option<f64>,

    /// Speed Index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<f64>,

    /// The state of the Lighthouse report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LighthouseReportState>,

    /// Total Blocking Time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbt: Option<f64>,

    /// Time To First Byte.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttfb: Option<f64>,

    /// Time To Interactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tti: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    /// Cumulative Layout Shift trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cls: Option<Vec<f64>>,

    /// First Contentful Paint trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcp: Option<Vec<f64>>,

    /// Largest Contentful Paint trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcp: Option<Vec<f64>>,

    /// The Lighthouse score trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "performanceScore")]
    pub performance_score: Option<Vec<f64>>,

    /// Speed Index trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<Vec<f64>>,

    /// Total Blocking Time trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbt: Option<Vec<f64>>,

    /// Time To First Byte trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttfb: Option<Vec<f64>>,

    /// Time To Interactive trend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tti: Option<Vec<f64>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityQuota {
    /// Cloudflare plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The number of tests available per plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quotasPerPlan")]
    pub quotas_per_plan: Option<AvailabilityQuotaQuotasPerPlan>,

    /// The number of remaining schedules available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remainingSchedules")]
    pub remaining_schedules: Option<f64>,

    /// The number of remaining tests available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remainingTests")]
    pub remaining_tests: Option<f64>,

    /// The number of schedules available per plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduleQuotasPerPlan")]
    pub schedule_quotas_per_plan: Option<AvailabilityQuotaScheduleQuotasPerPlan>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRegionsPerPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<Vec<LabeledRegion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Vec<LabeledRegion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<Vec<LabeledRegion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro: Option<Vec<LabeledRegion>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LighthouseReportError {
    /// The error code of the Lighthouse result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<LighthouseReportErrorCode>,

    /// Detailed error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// The final URL displayed to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finalDisplayedUrl")]
    pub final_displayed_url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityQuotaQuotasPerPlan {
    /// Counts per account plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AvailabilityQuotaQuotasPerPlanValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityQuotaScheduleQuotasPerPlan {
    /// Counts per account plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AvailabilityQuotaScheduleQuotasPerPlanValue>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityQuotaQuotasPerPlanValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityQuotaScheduleQuotasPerPlanValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro: Option<i64>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PageListResponseScheduleFrequency {
    #[serde(rename = "DAILY")]
    PageListResponseScheduleFrequencyDaily,
    #[serde(rename = "WEEKLY")]
    PageListResponseScheduleFrequencyWeekly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestScheduleFrequency {
    #[serde(rename = "DAILY")]
    TestScheduleFrequencyDaily,
    #[serde(rename = "WEEKLY")]
    TestScheduleFrequencyWeekly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduleFrequency {
    #[serde(rename = "DAILY")]
    ScheduleFrequencyDaily,
    #[serde(rename = "WEEKLY")]
    ScheduleFrequencyWeekly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduleRegion {
    #[serde(rename = "asia-east1")]
    ScheduleRegionAsiaEast1,
    #[serde(rename = "asia-northeast1")]
    ScheduleRegionAsiaNortheast1,
    #[serde(rename = "asia-northeast2")]
    ScheduleRegionAsiaNortheast2,
    #[serde(rename = "asia-south1")]
    ScheduleRegionAsiaSouth1,
    #[serde(rename = "asia-southeast1")]
    ScheduleRegionAsiaSoutheast1,
    #[serde(rename = "australia-southeast1")]
    ScheduleRegionAustraliaSoutheast1,
    #[serde(rename = "europe-north1")]
    ScheduleRegionEuropeNorth1,
    #[serde(rename = "europe-southwest1")]
    ScheduleRegionEuropeSouthwest1,
    #[serde(rename = "europe-west1")]
    ScheduleRegionEuropeWest1,
    #[serde(rename = "europe-west2")]
    ScheduleRegionEuropeWest2,
    #[serde(rename = "europe-west3")]
    ScheduleRegionEuropeWest3,
    #[serde(rename = "europe-west4")]
    ScheduleRegionEuropeWest4,
    #[serde(rename = "europe-west8")]
    ScheduleRegionEuropeWest8,
    #[serde(rename = "europe-west9")]
    ScheduleRegionEuropeWest9,
    #[serde(rename = "me-west1")]
    ScheduleRegionMeWest1,
    #[serde(rename = "southamerica-east1")]
    ScheduleRegionSouthamericaEast1,
    #[serde(rename = "us-central1")]
    ScheduleRegionUsCentral1,
    #[serde(rename = "us-east1")]
    ScheduleRegionUsEast1,
    #[serde(rename = "us-east4")]
    ScheduleRegionUsEast4,
    #[serde(rename = "us-south1")]
    ScheduleRegionUsSouth1,
    #[serde(rename = "us-west1")]
    ScheduleRegionUsWest1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LabeledRegionValue {
    #[serde(rename = "asia-east1")]
    LabeledRegionValueAsiaEast1,
    #[serde(rename = "asia-northeast1")]
    LabeledRegionValueAsiaNortheast1,
    #[serde(rename = "asia-northeast2")]
    LabeledRegionValueAsiaNortheast2,
    #[serde(rename = "asia-south1")]
    LabeledRegionValueAsiaSouth1,
    #[serde(rename = "asia-southeast1")]
    LabeledRegionValueAsiaSoutheast1,
    #[serde(rename = "australia-southeast1")]
    LabeledRegionValueAustraliaSoutheast1,
    #[serde(rename = "europe-north1")]
    LabeledRegionValueEuropeNorth1,
    #[serde(rename = "europe-southwest1")]
    LabeledRegionValueEuropeSouthwest1,
    #[serde(rename = "europe-west1")]
    LabeledRegionValueEuropeWest1,
    #[serde(rename = "europe-west2")]
    LabeledRegionValueEuropeWest2,
    #[serde(rename = "europe-west3")]
    LabeledRegionValueEuropeWest3,
    #[serde(rename = "europe-west4")]
    LabeledRegionValueEuropeWest4,
    #[serde(rename = "europe-west8")]
    LabeledRegionValueEuropeWest8,
    #[serde(rename = "europe-west9")]
    LabeledRegionValueEuropeWest9,
    #[serde(rename = "me-west1")]
    LabeledRegionValueMeWest1,
    #[serde(rename = "southamerica-east1")]
    LabeledRegionValueSouthamericaEast1,
    #[serde(rename = "us-central1")]
    LabeledRegionValueUsCentral1,
    #[serde(rename = "us-east1")]
    LabeledRegionValueUsEast1,
    #[serde(rename = "us-east4")]
    LabeledRegionValueUsEast4,
    #[serde(rename = "us-south1")]
    LabeledRegionValueUsSouth1,
    #[serde(rename = "us-west1")]
    LabeledRegionValueUsWest1,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LighthouseReportDeviceType {
    #[serde(rename = "DESKTOP")]
    LighthouseReportDeviceTypeDesktop,
    #[serde(rename = "MOBILE")]
    LighthouseReportDeviceTypeMobile,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LighthouseReportState {
    #[serde(rename = "RUNNING")]
    LighthouseReportStateRunning,
    #[serde(rename = "COMPLETE")]
    LighthouseReportStateComplete,
    #[serde(rename = "FAILED")]
    LighthouseReportStateFailed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LighthouseReportErrorCode {
    #[serde(rename = "NOT_REACHABLE")]
    LighthouseReportErrorCodeNotReachable,
    #[serde(rename = "DNS_FAILURE")]
    LighthouseReportErrorCodeDNSFailure,
    #[serde(rename = "NOT_HTML")]
    LighthouseReportErrorCodeNotHTML,
    #[serde(rename = "LIGHTHOUSE_TIMEOUT")]
    LighthouseReportErrorCodeLighthouseTimeout,
    #[serde(rename = "UNKNOWN")]
    LighthouseReportErrorCodeUnknown,
}

