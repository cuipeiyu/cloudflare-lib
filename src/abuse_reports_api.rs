/// abuse reports bindings
///
/// add the feature "full" or "with-abuse-reports" to enable
pub mod abuse_reports;

#[cfg(any(feature = "full", feature = "with-abuse-reports"))]
impl Client {
    /// Submit the Abuse Report of a particular type
    ///
    /// "T" satisfied by [abuse_reports::AbuseReportNewParamsBodyAbuseReportsDmcaReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsTrademarkReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsGeneralReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsPhishingReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsCsamReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsThreatReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsRegistrarWhoisReport],
    /// [abuse_reports::AbuseReportNewParamsBodyAbuseReportsNcseiReport],
    /// [AbuseReportNewParamsBody].
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/abuse_reports/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn abuse_reports_new<T>(
        &self,
        account_id: &str,
        report_param: &str,
        body: &T,
    ) -> Result<String>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + ::std::fmt::Debug,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if report_param.is_empty() {
            return Err(anyhow!("missing required report_param parameter"));
        }
        let path = format!(
            "/accounts/{}/abuse-reports/{}",
            account_id, report_param
        );
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: abuse_reports::AbuseReportNewResponseEnvelope = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// List the abuse reports for a given account
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// - Trust and Safety Read
    /// See <a href="https://developers.cloudflare.com/api/resources/abuse_reports/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn abuse_reports_list(
        &self,
        account_id: &str,
        query: Option<&abuse_reports::AbuseReportListQuery>,
    ) -> Result<ApiPagePagination<abuse_reports::AbuseReportListResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/abuse-reports", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<abuse_reports::AbuseReportListResponse> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Retrieve the details of an abuse report.
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// - Trust and Safety Read
    /// See <a href="https://developers.cloudflare.com/api/resources/abuse_reports/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn abuse_reports_get(
        &self,
        account_id: &str,
        report_id: &str,
    ) -> Result<abuse_reports::AbuseReportGetResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if report_id.is_empty() {
            return Err(anyhow!("missing required report_id parameter"));
        }
        let path = format!("/accounts/{}/abuse-reports/{}", account_id, report_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<abuse_reports::AbuseReportGetResponse> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// List mitigations done to remediate the abuse report.
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// - Trust and Safety Read
    /// See <a href="https://developers.cloudflare.com/api/resources/abuse_reports/subresources/mitigations/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn abuse_reports_mitigations_list(
        &self,
        account_id: &str,
        report_id: &str,
        query: Option<&abuse_reports::MitigationListQuery>,
    ) -> Result<ApiPagePagination<abuse_reports::MitigationListResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if report_id.is_empty() {
            return Err(anyhow!("missing required report_id parameter"));
        }
        let path = format!(
            "/accounts/{}/abuse-reports/{}/mitigations",
            account_id, report_id,
        );
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<abuse_reports::MitigationListResponse> =
            resp.json().await.map_err(|e| anyhow!(e))?;

        Ok(res.into())
    }

    /// Request a review for mitigations on an account.
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// See <a href="https://developers.cloudflare.com/api/resources/abuse_reports/subresources/mitigations/methods/review" target="_blank">the API docs</a> for more information.
    pub async fn abuse_reports_mitigations_review(
        &self,
        account_id: &str,
        report_id: &str,
        body: &abuse_reports::MitigationReviewBody,
    ) -> Result<ApiSinglePage<Vec<abuse_reports::MitigationReviewResponse>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if report_id.is_empty() {
            return Err(anyhow!("missing required report_id parameter"));
        }
        let path = format!(
            "/accounts/{}/abuse-reports/{}/mitigations/appeal",
            account_id, report_id,
        );
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<Vec<abuse_reports::MitigationReviewResponse>> =
            resp.json().await.map_err(|e| anyhow!(e))?;

        Ok(res.into())
    }
}
