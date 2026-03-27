/// alerting bindings
///
/// add the feature "full" or "with-alerting" to enable
pub mod alerting;

#[cfg(any(feature = "full", feature = "with-alerting"))]
impl Client {
    /// Get Alert Types
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/available_alerts/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn alerting_available_alerts_list(
        &self,
        account_id: &str,
        query: Option<&accounts::AccountListQuery>,
    ) -> Result<alerting::AvailableAlertListResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/available_alerts", account_id);
        let resp = self
            .request(Method::GET, &path, &query, None, &())
            .await?;
        let res: ApiResponse<alerting::AvailableAlertListResponse> = resp.json().await?;
        Ok(res.result)
    }

    /// Get a list of all delivery mechanism types for which an account is eligible.
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/eligible/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_eligible_list(
        &self,
        account_id: &str,
    ) -> Result<alerting::DestinationEligibleListResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/eligible", account_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<alerting::DestinationEligibleListResponse> = resp.json().await?;
        Ok(res.result)
    }

    /// Get a list of all configured PagerDuty services.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/pagerduty/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_pagerduty_list(
        &self,
        account_id: &str,
    ) -> Result<ApiSinglePage<Vec<alerting::Pagerduty>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/pagerduty", account_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Vec<alerting::Pagerduty>> = resp.json().await?;
        Ok(res.into())
    }

    /// Creates a new token for integrating with PagerDuty.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/pagerduty/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_pagerduty_new(
        &self,
        account_id: &str,
    ) -> Result<Option<alerting::DestinationPagerdutyNewResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/pagerduty/connect", account_id);
        let resp = self
            .request(Method::POST, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<alerting::DestinationPagerdutyNewResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Deletes all the PagerDuty Services connected to the account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/pagerduty/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_pagerduty_delete(
        &self,
        account_id: &str,
    ) -> Result<bool> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/pagerduty", account_id);
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<bool> = resp.json().await?;
        Ok(res.result)
    }

    /// Links PagerDuty with the account using the integration token.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/pagerduty/methods/link" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_pagerduty_link(
        &self,
        account_id: &str,
        token_id: &str,
    ) -> Result<Option<alerting::DestinationPagerdutyLinkResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if token_id.is_empty() {
            return Err(anyhow!("missing required token_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/pagerduty/connect/{}", account_id, token_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<alerting::DestinationPagerdutyLinkResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Gets a list of all configured webhook destinations.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/webhooks/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_webhooks_list(
        &self,
        account_id: &str,
    ) -> Result<ApiSinglePage<Vec<alerting::Webhooks>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/webhooks", account_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Vec<alerting::Webhooks>> = resp.json().await?;
        Ok(res.into())
    }

    /// Get details for a single webhooks destination.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/webhooks/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_webhooks_get(
        &self,
        account_id: &str,
        webhook_id: &str,
    ) -> Result<Option<alerting::Webhooks>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if webhook_id.is_empty() {
            return Err(anyhow!("missing required webhook_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/webhooks/{}", account_id, webhook_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<alerting::Webhooks>> = resp.json().await?;
        Ok(res.result)
    }

    /// Creates a new webhook destination.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/webhooks/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_webhooks_new(
        &self,
        account_id: &str,
        body: &alerting::DestinationWebhookNewBody,
    ) -> Result<Option<alerting::Webhooks>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/webhooks", account_id);
        let resp = self
            .request(Method::POST, &path, &(), None, body)
            .await?;
        let res: ApiResponse<Option<alerting::Webhooks>> = resp.json().await?;
        Ok(res.result)
    }

    /// Update a webhook destination.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/webhooks/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_webhooks_update(
        &self,
        account_id: &str,
        webhook_id: &str,
        body: &alerting::DestinationWebhookUpdateBody,
    ) -> Result<Option<alerting::DestinationWebhookUpdateResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if webhook_id.is_empty() {
            return Err(anyhow!("missing required webhook_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/webhooks/{}", account_id, webhook_id);
        let resp = self
            .request(Method::PUT, &path, &(), None, body)
            .await?;
        let res: ApiResponse<Option<alerting::DestinationWebhookUpdateResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Delete a configured webhook destination.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/destinations/subresources/webhooks/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn alerting_destinations_webhooks_delete(
        &self,
        account_id: &str,
        webhook_id: &str,
    ) -> Result<bool> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if webhook_id.is_empty() {
            return Err(anyhow!("missing required webhook_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/destinations/webhooks/{}", account_id, webhook_id);
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<bool> = resp.json().await?;
        Ok(res.result)
    }

    /// Gets a list of history records for notifications sent to an account. The
    /// records are displayed for last x number of days based on the zone plan
    /// (free = 30, pro = 30, biz = 30, ent = 90).
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/history/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn alerting_history_list(
        &self,
        account_id: &str,
        query: Option<&alerting::HistoryListQuery>
    ) -> Result<ApiPagePagination<Option<Vec<alerting::History>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/history", account_id);
        let resp = self
            .request(Method::GET, &path, &query, None, &())
            .await?;
        let res: ApiResponse<Option<Vec<alerting::History>>> = resp.json().await?;
        Ok(res.into())
    }

    /// Get a list of all Notification policies.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/policies/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn alerting_policy_list(
        &self,
        account_id: &str,
    ) -> Result<ApiSinglePage<Option<Vec<alerting::Policy>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/policies", account_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<Vec<alerting::Policy>>> = resp.json().await?;
        Ok(res.into())
    }

    /// Get details for a single policy.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/policies/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn alerting_policy_get(
        &self,
        account_id: &str,
        policy_id: &str,
    ) -> Result<ApiSinglePage<Option<alerting::Policy>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if policy_id.is_empty() {
            return Err(anyhow!("missing required policy_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/policies/{}", account_id, policy_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<alerting::Policy>> = resp.json().await?;
        Ok(res.into())
    }

    /// Creates a new Notification policy.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/policies/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn alerting_policy_new(
        &self,
        account_id: &str,
        body: &alerting::PolicyNewBody,
    ) -> Result<Option<alerting::PolicyNewResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/policies", account_id);
        let resp = self
            .request(Method::POST, &path, &(), None, body)
            .await?;
        let res: ApiResponse<Option<alerting::PolicyNewResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Update a Notification policy.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/policies/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn alerting_policy_update(
        &self,
        account_id: &str,
        policy_id: &str,
        body: &alerting::PolicyUpdateBody,
    ) -> Result<Option<alerting::PolicyUpdateResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if policy_id.is_empty() {
            return Err(anyhow!("missing required policy_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/policies/{}", account_id, policy_id);
        let resp = self
            .request(Method::GET, &path, &(), None, body)
            .await?;
        let res: ApiResponse<Option<alerting::PolicyUpdateResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Delete a Notification policy.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/policies/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn alerting_policy_delete(
        &self,
        account_id: &str,
        policy_id: &str,
    ) -> Result<bool> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if policy_id.is_empty() {
            return Err(anyhow!("missing required policy_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/policies/{}", account_id, policy_id);
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<bool> = resp.json().await?;
        Ok(res.result)
    }

    /// Gets a list of silences for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/silences/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn alerting_silence_list(
        &self,
        account_id: &str,
    ) -> Result<ApiSinglePage<Vec<alerting::SilenceListResponse>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/silences", account_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Vec<alerting::SilenceListResponse>> = resp.json().await?;
        Ok(res.into())
    }

    /// Gets a specific silence for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Zero Trust: PII Read
    /// - Notifications Write
    /// - Notifications Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/silences/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn alerting_silence_get(
        &self,
        account_id: &str,
        silence_id: &str,
    ) -> Result<Option<alerting::SilenceGetResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if silence_id.is_empty() {
            return Err(anyhow!("missing required silence_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/silences/{}", account_id, silence_id);
        let resp = self
            .request(Method::GET, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<alerting::SilenceGetResponse>> = resp.json().await?;
        Ok(res.result)
    }

    /// Creates a new silence for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/silences/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn alerting_silence_new(
        &self,
        account_id: &str,
        body: &Vec<alerting::SilenceNewParamsBody>,
    ) -> Result<bool> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/silences", account_id);
        let resp = self
            .request(Method::POST, &path, &(), None, body)
            .await?;
        let res: ApiResponse<bool> = resp.json().await?;
        Ok(res.result)
    }

    /// Updates existing silences for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/silences/methods/update" target="_blank">the API docs</a> for more information.
     pub async fn alerting_silence_update(
        &self,
        account_id: &str,
        body: &Vec<alerting::SilenceUpdateParamsBody>,
    ) -> Result<ApiSinglePage<Option<Vec<alerting::SilenceUpdateResponse>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/silences", account_id);
        let resp = self
            .request(Method::PUT, &path, &(), None, body)
            .await?;
        let res: ApiResponse<Option<Vec<alerting::SilenceUpdateResponse>>> = resp.json().await?;
        Ok(res.into())
    }

    /// Deletes an existing silence for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Notifications Write
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/alerting/subresources/silences/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn alerting_silence_delete(
        &self,
        account_id: &str,
        silence_id: &str,
    ) -> Result<bool> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if silence_id.is_empty() {
            return Err(anyhow!("missing required silence_id parameter"));
        }
        let path = format!("/accounts/{}/alerting/v3/silences/{}", account_id, silence_id);
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<bool> = resp.json().await?;
        Ok(res.result)
    }
}
