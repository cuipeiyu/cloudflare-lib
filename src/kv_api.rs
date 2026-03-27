/// KV bindings
///
/// add the feature "full" or "with-kv" to enable
pub mod kv;

#[cfg(any(feature = "full", feature = "with-kv"))]
impl Client {
    /// List Namespaces
    ///
    /// Returns the namespaces owned by an account.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_list(
        &self,
        account_id: &str,
        query: Option<&kv::NamespaceListQuery>,
    ) -> Result<ApiPagePagination<Option<Vec<kv::Namespace>>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/storage/kv/namespaces", account_id);
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Option<Vec<kv::Namespace>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Get the namespace corresponding to the given ID.
    ///
    /// Accepted Permissions (at least one required)
    /// -Workers KV Storage Write
    /// -Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_get(
        &self,
        account_id: &str,
        namespace_id: &str,
    ) -> Result<Option<kv::Namespace>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}",
            account_id, namespace_id
        );
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<kv::Namespace>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Creates a namespace under the given title. A `400` is returned if the account
    /// already owns a namespace with this title. A namespace must be explicitly deleted
    /// to be replaced.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/create" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_new(
        &self,
        account_id: &str,
        body: &kv::NamespaceNewBody,
    ) -> Result<Option<kv::Namespace>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        let path = format!("/accounts/{}/storage/kv/namespaces", account_id);
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<Option<kv::Namespace>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Modifies a namespace's title.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_update(
        &self,
        account_id: &str,
        namespace_id: &str,
        body: &kv::NamespaceUpdateBody,
    ) -> Result<kv::Namespace> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}",
            account_id, namespace_id
        );
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<kv::Namespace> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Deletes the namespace corresponding to the given ID.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_delete(&self, account_id: &str, namespace_id: &str) -> Result<()> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}",
            account_id, namespace_id
        );
        let resp = self.request(Method::DELETE, &path, &(), None, &()).await?;
        let res: ApiResponse<()> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Write multiple key-value pairs
    ///
    /// Write multiple keys and values at once. Body should be an array of up to 10,000
    /// key-value pairs to be stored, along with optional expiration information.
    /// Existing values and expirations will be overwritten. If neither `expiration` nor
    /// `expiration_ttl` is specified, the key-value pair will never expire. If both are
    /// set, `expiration_ttl` is used and `expiration` is ignored. The entire request
    /// size must be 100 megabytes or less.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk_update" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_bulk_update(
        &self,
        account_id: &str,
        namespace_id: &str,
        body: &Vec<kv::NamespaceBulkUpdateParamsBody>,
    ) -> Result<Option<kv::NamespaceBulkUpdateResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/bulk",
            account_id, namespace_id
        );
        let resp = self.request(Method::PUT, &path, &(), None, body).await?;
        let res: ApiResponse<Option<kv::NamespaceBulkUpdateResponse>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Remove multiple KV pairs from the namespace. Body should be an array of up to
    /// 10,000 keys to be removed.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk_delete" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_bulk_delete(
        &self,
        account_id: &str,
        namespace_id: &str,
        body: &Vec<String>,
    ) -> Result<Option<kv::NamespaceBulkDeleteResponse>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/bulk/delete",
            account_id, namespace_id
        );
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<Option<kv::NamespaceBulkDeleteResponse>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Get multiple key-value pairs
    ///
    /// Retrieve up to 100 KV pairs from the namespace. Keys must contain text-based
    /// values. JSON values can optionally be parsed instead of being returned as a
    /// string value. Metadata can be included if `withMetadata` is true.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk_get" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_bulk_get<T>(
        &self,
        account_id: &str,
        namespace_id: &str,
        body: &kv::NamespaceBulkGetBody,
    ) -> Result<Option<kv::NamespaceBulkGetResponse<T>>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/bulk/get",
            account_id, namespace_id
        );
        let resp = self.request(Method::POST, &path, &(), None, body).await?;
        let res: ApiResponse<Option<kv::NamespaceBulkGetResponse<T>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Lists a namespace's keys.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/list" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_keys_list(
        &self,
        account_id: &str,
        namespace_id: &str,
        query: Option<&kv::NamespaceKeyListCursorQuery>,
    ) -> Result<ApiCursorLimitPagination<kv::Key>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/keys",
            account_id, namespace_id
        );
        let resp = self.request(Method::GET, &path, &query, None, &()).await?;
        let res: ApiResponse<Vec<kv::Key>> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.into())
    }

    /// Write multiple keys and values at once. Body should be an array of up to 10,000
    /// key-value pairs to be stored, along with optional expiration information.
    /// Existing values and expirations will be overwritten. If neither `expiration` nor
    /// `expiration_ttl` is specified, the key-value pair will never expire. If both are
    /// set, `expiration_ttl` is used and `expiration` is ignored. The entire request
    /// size must be 100 megabytes or less.
    ///
    /// Deprecated: Please use kv_namespaces_bulk_update instead
    // #[deprecated]
    // pub async fn kv_namespaces_keys_bulk_update(
    //     &self,
    //     namespace_id: &str,
    //     params: &kv::NamespaceKeyBulkUpdateParams,
    // ) -> Result<kv::NamespaceKeyBulkUpdateResponse> {
    //     if params.account_id.is_empty() {
    //         return Err(anyhow!("missing required account_id parameter"));
    //     }
    //     if namespace_id.is_empty() {
    //         return Err(anyhow!("missing required namespace_id parameter"));
    //     }
    //     let path = format!(
    //         "/accounts/{}/storage/kv/namespaces/{}/bulk",
    //         params.account_id, namespace_id
    //     );
    //     let resp = self
    //         .request(Method::PUT, &path, &(), None, &params.body)
    //         .await?;
    //     let res: ApiResponse<kv::NamespaceKeyBulkUpdateResponse> =
    //         resp.json().await.map_err(|e| anyhow!(e))?;
    //     Ok(res.result)
    // }

    /// Remove multiple KV pairs from the namespace. Body should be an array of up to
    /// 10,000 keys to be removed.
    ///
    /// Deprecated: Please use kv.namespaces.bulk_delete instead
    // #[deprecated]
    // pub async fn kv_namespaces_keys_bulk_delete(
    //     &self,
    //     namespace_id: &str,
    //     params: &kv::NamespaceKeyBulkDeleteParams,
    // ) -> Result<kv::NamespaceKeyBulkDeleteResponse> {
    //     if params.account_id.is_empty() {
    //         return Err(anyhow!("missing required account_id parameter"));
    //     }
    //     if namespace_id.is_empty() {
    //         return Err(anyhow!("missing required namespace_id parameter"));
    //     }
    //     let path = format!(
    //         "/accounts/{}/storage/kv/namespaces/{}/bulk/delete",
    //         params.account_id, namespace_id
    //     );
    //     let resp = self
    //         .request(Method::POST, &path, &(), None, &params.body)
    //         .await?;
    //     let res: ApiResponse<kv::NamespaceKeyBulkDeleteResponse> =
    //         resp.json().await.map_err(|e| anyhow!(e))?;
    //     Ok(res.result)
    // }

    /// Retrieve up to 100 KV pairs from the namespace. Keys must contain text-based
    /// values. JSON values can optionally be parsed instead of being returned as a
    /// string value. Metadata can be included if `withMetadata` is true.
    ///
    /// Deprecated: Please use kv_namespaces_bulk_get instead
    // #[deprecated]
    // pub async fn kv_namespaces_keys_bulk_get<T>(
    //     &self,
    //     namespace_id: &str,
    //     params: &kv::NamespaceKeyBulkGetParams,
    // ) -> Result<kv::NamespaceKeyBulkGetResponse<T>>
    // where
    //     T: serde::Serialize + serde::de::DeserializeOwned,
    // {
    //     if params.account_id.is_empty() {
    //         return Err(anyhow!("missing required account_id parameter"));
    //     }
    //     if namespace_id.is_empty() {
    //         return Err(anyhow!("missing required namespace_id parameter"));
    //     }
    //     let path = format!(
    //         "/accounts/{}/storage/kv/namespaces/{}/bulk/get",
    //         params.account_id, namespace_id
    //     );
    //     let resp = self.request(Method::POST, &path, &(), None, params).await?;
    //     let res: ApiResponse<kv::NamespaceKeyBulkGetResponse<T>> =
    //         resp.json().await.map_err(|e| anyhow!(e))?;
    //     Ok(res.result)
    // }

    /// Read the metadata for a key
    ///
    /// Returns the metadata associated with the given key in the given namespace. Use
    /// URL-encoding to use special characters (for example, `:`, `!`, `%`) in the key
    /// name.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/metadata/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_metadata_get(
        &self,
        account_id: &str,
        namespace_id: &str,
        key_name: &str,
    ) -> Result<Option<::std::collections::HashMap<String, String>>> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        if key_name.is_empty() {
            return Err(anyhow!("missing required key_name parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/metadata/{}",
            account_id, namespace_id, key_name
        );
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        let res: ApiResponse<Option<::std::collections::HashMap<String, String>>> =
            resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Read key-value pair
    ///
    /// Returns the value associated with the given key in the given namespace. Use
    /// URL-encoding to use special characters (for example, `:`, `!`, `%`) in the key
    /// name. If the KV-pair is set to expire at some point, the expiration time as
    /// measured in seconds since the UNIX epoch will be returned in the `expiration`
    /// response header.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    /// - Workers KV Storage Read
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/get" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_values_get(
        &self,
        account_id: &str,
        namespace_id: &str,
        key_name: &str,
    ) -> Result<bytes::Bytes> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        if key_name.is_empty() {
            return Err(anyhow!("missing required key_name parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key_name
        );
        let resp = self.request(Method::GET, &path, &(), None, &()).await?;
        // let ex = resp.headers().get("expiration");
        let res = resp.bytes().await.map_err(|e| anyhow!(e))?;
        Ok(res)
    }

    /// Write key-value pair with optional metadata
    ///
    /// Write a value identified by a key. Use URL-encoding to use special characters
    /// (for example, `:`, `!`, `%`) in the key name. Body should be the value to be
    /// stored. If JSON metadata to be associated with the key/value pair is needed, use
    /// `multipart/form-data` content type for your PUT request (see dropdown below in
    /// `REQUEST BODY SCHEMA`). Existing values, expirations, and metadata will be
    /// overwritten. If neither `expiration` nor `expiration_ttl` is specified, the
    /// key-value pair will never expire. If both are set, `expiration_ttl` is used and
    /// `expiration` is ignored.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/update" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_values_update(
        &self,
        account_id: &str,
        namespace_id: &str,
        key_name: &str,
        query: Option<&kv::NamespaceValueUpdateQuery>,
        body: &kv::NamespaceValueUpdateBody,
    ) -> Result<()> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        if key_name.is_empty() {
            return Err(anyhow!("missing required key_name parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key_name
        );
        let resp = self.request(Method::PUT, &path, &query, None, body).await?;
        let res: ApiResponse<()> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }

    /// Delete key-value pair
    ///
    /// Remove a KV pair from the namespace. Use URL-encoding to use special characters
    /// (for example, `:`, `!`, `%`) in the key name.
    ///
    /// Accepted Permissions (at least one required)
    /// - Workers KV Storage Write
    ///
    /// See <a href="https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/delete" target="_blank">the API docs</a> for more information.
    pub async fn kv_namespaces_values_delete(
        &self,
        account_id: &str,
        namespace_id: &str,
        key_name: &str,
    ) -> Result<()> {
        if account_id.is_empty() {
            return Err(anyhow!("missing required account_id parameter"));
        }
        if namespace_id.is_empty() {
            return Err(anyhow!("missing required namespace_id parameter"));
        }
        if key_name.is_empty() {
            return Err(anyhow!("missing required key_name parameter"));
        }
        let path = format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            account_id, namespace_id, key_name
        );
        let resp = self.request(Method::DELETE, &path, &(), None, &()).await?;
        let res: ApiResponse<()> = resp.json().await.map_err(|e| anyhow!(e))?;
        Ok(res.result)
    }
}
