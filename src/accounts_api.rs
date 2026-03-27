/// accounts bindings
///
/// add the feature "full" or "with-accounts" to enable
pub mod accounts;

#[cfg(any(feature = "full", feature = "with-accounts"))]
impl Client {
    /// List all accounts you have ownership or verified access to.
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn accounts_list(
        &self,
        query: Option<&accounts::AccountListQuery>,
    ) -> Result<ApiPagePagination<Vec<accounts::Account>>> {
        let resp = self
            .request(Method::GET, "/accounts", &query, None, &())
            .await?;
        let res: ApiResponse<Vec<accounts::Account>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Account Details
    ///
    /// Get information about a specific account that you are a member of.
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// - Trust and Safety Read
    /// - DNS View Write
    /// - DNS View Read
    /// - SCIM Provisioning
    /// - Load Balancers Account Write
    /// - Load Balancers Account Read
    /// - Zero Trust: PII Read
    /// - DDoS Botnet Feed Write
    /// - DDoS Botnet Feed Read
    /// - Workers R2 Storage Write
    /// - Workers R2 Storage Read
    /// - DDoS Protection Write
    /// - DDoS Protection Read
    /// - Workers Tail Read
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    /// - Workers Scripts Write
    /// - Workers Scripts Read
    /// - Load Balancing: Monitors and Pools Write
    /// - Load Balancing: Monitors and Pools Read
    /// - Account Firewall Access Rules Write
    /// - Account Firewall Access Rules Read
    /// - DNS Firewall Write
    /// - DNS Firewall Read
    /// - Billing Write
    /// - Billing Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn accounts_get(&self, account_id: &str) -> Result<Option<accounts::Account>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}", account_id,);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<accounts::Account>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Create an account (only available for tenant admins at this time)
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn accounts_new(&self, body: &accounts::AccountNewBody) -> Result<accounts::Account> {
        let resp = self
            .request(Method::POST, "/accounts", &(), None, body)
            .await?;
        let res: ApiResponse<accounts::Account> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Update an existing account.
    ///
    /// Accepted Permissions (at least one required):
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn accounts_update(&self, account_id: &str, body: &accounts::AccountUpdateBody) -> Result<accounts::Account> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}", account_id);
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<accounts::Account> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Delete a specific account (only available for tenant admins at this time). This
    /// is a permanent operation that will delete any zones or other resources under the
    /// account
    ///
    /// Accepted Permissions (at least one required):
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn accounts_delete(
        &self,
        account_id: &str,
    ) -> Result<accounts::AccountDeleteResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}", account_id);
        let resp = self.request(Method::DELETE, &path, &(), None, &()).await?;
        let res: ApiResponse<accounts::AccountDeleteResponse> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Move account
    ///
    /// Move an account within an organization hierarchy or an account outside an
    /// organization. (Currently in Closed Beta - see
    /// https://developers.cloudflare.com/fundamentals/organizations/)
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/account_organizations/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_organizations_move(
        &self,
        account_id: &str,
        body: &accounts::AccountOrganizationMoveBody,
    ) -> Result<accounts::AccountOrganizationNewResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/move", account_id);
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<accounts::AccountOrganizationNewResponse> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Get account profile
    ///
    /// Retrieves the profile information for a specific Cloudflare account, including
    /// organization details, settings, and metadata. This endpoint is commonly used
    /// to verify account access and retrieve account-level configuration.
    ///
    /// Accepted Permissions (at least one required):
    /// - Trust and Safety Write
    /// - Trust and Safety Read
    /// - DNS View Write
    /// - DNS View Read
    /// - SCIM Provisioning
    /// - Load Balancers Account Write
    /// - Load Balancers Account Read
    /// - Zero Trust: PII Read
    /// - DDoS Botnet Feed Write
    /// - DDoS Botnet Feed Read
    /// - Workers R2 Storage Write
    /// - Workers R2 Storage Read
    /// - DDoS Protection Write
    /// - DDoS Protection Read
    /// - Workers Tail Read
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    /// - Workers Scripts Write
    /// - Workers Scripts Read
    /// - Load Balancing: Monitors and Pools Write
    /// - Load Balancing: Monitors and Pools Read
    /// - Account Firewall Access Rules Write
    /// - Account Firewall Access Rules Read
    /// - DNS Firewall Write
    /// - DNS Firewall Read
    /// - Billing Write
    /// - Billing Read
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/account_profile/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_profile_get(
        &self,
        account_id: &str,
    ) -> Result<accounts::AccountProfile> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/profile", account_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<accounts::AccountProfile> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Modify account profile
    ///
    /// Updates the profile information for a Cloudflare account. Allows modification
    /// of account-level settings and organizational details. Requires Account Settings
    /// Write permission.
    ///
    /// Accepted Permissions (at least one required):
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/account_profile/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_profile_update(
        &self,
        params: &accounts::AccountProfileUpdateParams,
    ) -> Result<()> {
        if params.account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/profile", params.account_id,);
        let resp = self.request(Method::PUT, &path, &(), None, params).await?;
        let res: ApiResponse<()> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// List Members
    ///
    /// List all members of an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/members/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_members_list(
        &self,
        account_id: &str,
        query: Option<&accounts::MemberListQuery>,
    ) -> Result<ApiPagePagination<Vec<crate::shared::Member>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/members", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Vec<crate::shared::Member>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Member Details
    ///
    /// Get information about a specific member of an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/members/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_members_get(
        &self,
        account_id: &str,
        member_id: &str,
    ) -> Result<Option<crate::shared::Member>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if member_id.is_empty() {
            return Err(anyhow!("missing required member_id parameter"));
        }
        let path = format!("/accounts/{}/members/{}", account_id, member_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<crate::shared::Member>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Add Member
    ///
    /// Add a user to the list of members for this account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/members/methods/create" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [accounts::MemberNewParamsBodyIAMCreateMemberWithRoles],
    /// [accounts::MemberNewParamsBodyIAMCreateMemberWithPolicies],
    /// [accounts::MemberNewParamsBody].
    pub async fn accounts_account_members_new<T>(
        &self,
        account_id: &str,
        body: &T,
    ) -> Result<crate::shared::Member>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + ::std::fmt::Debug,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/members", account_id);
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<crate::shared::Member> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Update Member
    ///
    /// Modify an account member.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/members/methods/update" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [accounts::MemberUpdateParamsBodyIAMUpdateMemberWithRoles],
    /// [accounts::MemberUpdateParamsBodyIAMUpdateMemberWithPolicies],
    /// [accounts::MemberUpdateParamsBody].
    pub async fn accounts_account_members_update<T>(
        &self,
        account_id: &str,
        member_id: &str,
        body: &T,
    ) -> Result<crate::shared::Member>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + ::std::fmt::Debug,
    {
        if  account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if member_id.is_empty() {
            return Err(anyhow!("missing required member_id parameter"));
        }
        let path = format!("/accounts/{}/members/{}",  account_id, member_id);
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<crate::shared::Member> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Remove a member from an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/members/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_members_delete(
        &self,
        account_id: &str,
        member_id: &str,
    ) -> Result<Option<accounts::MemberDeleteResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if member_id.is_empty() {
            return Err(anyhow!("missing required member_id parameter"));
        }
        let path = format!("/accounts/{}/members/{}", account_id, member_id);
        let resp = self.request(Method::DELETE, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<accounts::MemberDeleteResponse>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// List Roles
    ///
    /// Get all available roles for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/roles/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_roles_list(
        &self,
        account_id: &str,
        query: Option<&accounts::RoleListQuery>,
    ) -> Result<ApiPagePagination<Option<Vec<crate::shared::Role>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/roles", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Option<Vec<crate::shared::Role>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Role Details
    ///
    /// Get information about a specific role for an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - SCIM Provisioning
    /// - Account Settings Write
    /// - Account Settings Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/roles/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn accounts_account_roles_get(
        &self,
        account_id: &str,
        role_id: &str,
    ) -> Result<Option<crate::shared::Role>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if role_id.is_empty() {
            return Err(anyhow!("missing required member_id parameter"));
        }
        let path = format!("/accounts/{}/roles/{}", account_id, role_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<crate::shared::Role>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Lists all of an account's subscriptions.
    ///
    /// Accepted Permissions (at least one required)
    /// - Billing Write
    /// - Billing Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/subscriptions/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn accounts_subscriptions_list(
        &self,
        account_id: &str,
    ) -> Result<ApiSinglePage<Vec<crate::shared::Subscription>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/subscriptions", account_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Vec<crate::shared::Subscription>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Creates an account subscription.
    ///
    /// Accepted Permissions (at least one required)
    /// - Billing Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/subscriptions/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn accounts_subscriptions_new(
        &self,
        account_id: &str,
        body: &crate::shared::SubscriptionParam,
    ) -> Result<crate::shared::Subscription> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/subscriptions", account_id);
        let resp = self
            .request(Method::POST, &path, &(), None, body)
            .await?;
        let res: ApiResponse<crate::shared::Subscription> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Updates an account subscription.
    ///
    /// Accepted Permissions (at least one required)
    /// - Billing Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/subscriptions/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn accounts_subscriptions_update(
        &self,
        account_id: &str,
        subscription_identifier: &str,
        body: &crate::shared::SubscriptionParam,
    ) -> Result<crate::shared::Subscription> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if subscription_identifier.is_empty() {
            return Err(anyhow!(
                "missing required subscription_identifier parameter"
            ));
        }
        let path = format!(
            "/accounts/{}/subscriptions/{}",
            account_id, subscription_identifier
        );
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<crate::shared::Subscription> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Deletes an account's subscription.
    ///
    /// Accepted Permissions (at least one required)
    /// - Billing Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/subscriptions/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn accounts_subscriptions_delete(
        &self,
        account_id: &str,
        subscription_identifier: &str,
    ) -> Result<accounts::SubscriptionDeleteResponse> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if subscription_identifier.is_empty() {
            return Err(anyhow!(
                "missing required subscription_identifier parameter"
            ));
        }
        let path = format!(
            "/accounts/{}/subscriptions/{}",
            account_id, subscription_identifier
        );
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<accounts::SubscriptionDeleteResponse> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// List all Account Owned API tokens created for this account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    /// - Account API Tokens Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/list" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectString],
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectNested].
    pub async fn accounts_tokens_list<T>(
        &self,
        account_id: &str,
        query: Option<&accounts::TokenListQuery>,
    ) -> Result<ApiPagePagination<Option<Vec<crate::shared::Token<T>>>>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/tokens", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Option<Vec<crate::shared::Token<T>>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Get information about a specific Account Owned API token.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    /// - Account API Tokens Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/get" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectString],
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectNested].
    pub async fn accounts_tokens_get<T>(
        &self,
        account_id: &str,
        token_id: &str,
    ) -> Result<Option<crate::shared::Token<T>>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if token_id.is_empty() {
            return Err(anyhow!("missing required token_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/{}", account_id, token_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<crate::shared::Token<T>>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Create a new Account Owned API token.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/create" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectString],
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectNested].
    pub async fn accounts_tokens_new<T>(
        &self,
        account_id: &str,
        body: &accounts::TokenNewBody,
    ) -> Result<accounts::TokenNewResponse<T>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/tokens", account_id);
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<accounts::TokenNewResponse<T>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Update an existing token.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/update" target="_blank">the API docs</a> for more information.
    ///
    /// The generic "T" should satisfied by
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectString],
    /// [shared::TokenPolicyResourcesIAMResourcesTypeObjectNested].
    pub async fn accounts_tokens_update<T>(
        &self,
        account_id: &str,
        token_id: &str,
        body: &crate::shared::TokenParam,
    ) -> Result<crate::shared::Token<T>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if token_id.is_empty() {
            return Err(anyhow!("missing required token_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/{}", account_id, token_id);
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<crate::shared::Token<T>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Destroy an Account Owned API token.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn accounts_tokens_delete(
        &self,
        account_id: &str,
        token_id: &str,
    ) -> Result<Option<accounts::TokenDeleteResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if token_id.is_empty() {
            return Err(anyhow!("missing required token_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/{}", account_id, token_id);
        let resp = self
            .request(Method::DELETE, &path, &(), None, &())
            .await?;
        let res: ApiResponse<Option<accounts::TokenDeleteResponse>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Test whether a token works.
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/methods/verify" target="_blank">the API docs</a> for more information.
    pub async fn accounts_tokens_verify(
        &self,
        account_id: &str,
    ) -> Result<Option<accounts::TokenVerifyResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/verify", account_id);
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<accounts::TokenVerifyResponse>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Find all available permission groups for Account Owned API Tokens
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    /// - Account API Tokens Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/subresources/permission_groups/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn accounts_tokens_permission_groups_list(
        &self,
        account_id: &str,
        query: Option<&accounts::TokenPermissionGroupListQuery>,
    ) -> Result<ApiSinglePage<Option<Vec<accounts::TokenPermissionGroupListResponse>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/permission_groups", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Option<Vec<accounts::TokenPermissionGroupListResponse>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Roll the Account Owned API token secret.
    ///
    /// Accepted Permissions (at least one required)
    /// - Account API Tokens Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/accounts/subresources/tokens/subresources/value/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn accounts_tokens_value_update<T>(
        &self,
        account_id: &str,
        token_id: &str,
        body: &T,
    ) -> Result<Option<crate::shared::TokenValue>>
    where
    T: serde::Serialize + serde::de::DeserializeOwned + ::std::fmt::Debug,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if token_id.is_empty() {
            return Err(anyhow!("missing required token_id parameter"));
        }
        let path = format!("/accounts/{}/tokens/{}/value", account_id, token_id);
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<Option<crate::shared::TokenValue>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Gets a list of audit logs for an account.
    /// This is the beta release of Audit Logs Version 2.
    /// Since this is a beta version, there may be gaps or
    /// missing entries in the available audit logs. Be aware of the following
    /// limitations.
    /// - Audit logs are available only for the past 30 days.
    /// - Error handling is not yet implemented.
    pub async fn accounts_logs_audit_list(
        &self,
        account_id: &str,
        query: Option<&accounts::LogAuditListParams>,
    ) -> Result<ApiCursorPaginationAfter<accounts::LogAuditListResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/logs/audit", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Vec<accounts::LogAuditListResponse>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }
}
