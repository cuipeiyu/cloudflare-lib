/// audit logs bindings
///
/// add the feature "full" or "with-audit-logs" to enable
pub mod audit_logs;

#[cfg(any(feature = "full", feature = "with-audit-logs"))]
impl Client {
    /// Get account audit logs
    ///
    /// Gets a list of audit logs for an account. Can be filtered by who made the
    /// change, on which zone, and the timeframe of the change.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/audit_logs/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn audit_logs_list(
        &self,
        account_id: &str,
        query: Option<&audit_logs::AuditLogListQuery>,
    ) -> Result<ApiPagePagination<Option<Vec<crate::shared::AuditLog>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/audit_logs", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Option<Vec<crate::shared::AuditLog>>> = resp.json().await?;
        Ok(res.into())
    }
}
