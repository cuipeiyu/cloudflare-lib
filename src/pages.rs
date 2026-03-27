#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    /// Id of the deployment.
    pub id: String,

    /// A list of alias URLs pointing to this deployment.
    pub aliases: Vec<String>,

    /// Configs for the project build process.
    pub build_config: DeploymentBuildConfig,

    /// When the deployment was created.
    pub created_on: DateTime<Utc>,

    /// Info about what caused the deployment.
    pub deployment_trigger: DeploymentDeploymentTrigger,

    /// Environment variables used for builds and Pages Functions.
    pub env_vars: ::std::collections::HashMap<String, DeploymentEnvVar>,

    /// Type of deploy.
    pub environment: DeploymentEnvironment,

    /// If the deployment has been skipped.
    pub is_skipped: bool,

    /// The status of the deployment.
    pub latest_stage: Stage,

    /// When the deployment was last modified.
    pub modified_on: DateTime<Utc>,

    /// Id of the project.
    pub project_id: String,

    /// Name of the project.
    pub project_name: String,

    /// Short Id (8 character) of the deployment.
    pub short_id: String,

    /// Configs for the project source control.
    pub source: DeploymentSource,

    /// List of past stages.
    pub stages: Vec<Stage>,

    /// The live URL to view this deployment.
    pub url: String,

    /// Whether the deployment uses functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_functions: Option<bool>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// ID of the project.
    pub id: String,

    /// Most recent production deployment of the project.
    pub canonical_deployment: Deployment,

    /// When the project was created.
    pub created_on: DateTime<Utc>,

    /// Configs for deployments in a project.
    pub deployment_configs: ProjectDeploymentConfigs,

    /// Framework the project is using.
    pub framework: String,

    /// Version of the framework the project is using.
    pub framework_version: String,

    /// Most recent deployment of the project.
    pub latest_deployment: Deployment,

    /// Name of the project.
    pub name: String,

    /// Name of the preview script.
    pub preview_script_name: String,

    /// Production branch of the project. Used to identify production deployments.
    pub production_branch: String,

    /// Name of the production script.
    pub production_script_name: String,

    /// Whether the project uses functions.
    pub uses_functions: bool,

    /// Configs for the project build process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_config: Option<ProjectBuildConfig>,

    /// A list of associated custom domains for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,

    /// Configs for the project source control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSource>,

    /// The Cloudflare subdomain associated with the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    /// When the stage ended.
    pub ended_on: DateTime<Utc>,

    /// The current build stage.
    pub name: StageName,

    /// When the stage started.
    pub started_on: DateTime<Utc>,

    /// State of the current stage.
    pub status: StageStatus,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Name of the project.
    pub name: String,

    /// Production branch of the project. Used to identify production deployments.
    pub production_branch: String,

    /// Configs for the project build process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_config: Option<String>,

    /// Configs for deployments in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configs: Option<String>,

    /// Configs for the project source control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Which page of projects to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// How many projects to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Configs for the project build process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_config: Option<String>,

    /// Configs for deployments in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configs: Option<String>,

    /// Name of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Production branch of the project. Used to identify production deployments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<String>,

    /// Configs for the project source control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPurgeBuildCacheParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Headers configuration file for the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_headers")]
    pub headers: Option<String>,

    /// Redirects configuration file for the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_redirects")]
    pub redirects: Option<String>,

    /// Routes configuration file defining routing rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_routes.json")]
    pub routes_json: Option<String>,

    /// Worker bundle file in multipart/form-data format. Mutually exclusive with
    /// `_worker.js`. Cannot specify both `_worker.js` and `_worker.bundle` in the same
    /// request. Maximum size: 25 MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_worker.bundle")]
    pub worker_bundle: Option<String>,

    /// Worker JavaScript file. Mutually exclusive with `_worker.bundle`. Cannot specify
    /// both `_worker.js` and `_worker.bundle` in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_worker.js")]
    pub worker_js: Option<String>,

    /// The branch to build the new deployment from. The `HEAD` of the branch will be
    /// used. If omitted, the production branch will be used by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Boolean string indicating if the working directory has uncommitted changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_dirty: Option<String>,

    /// Git commit SHA associated with this deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,

    /// Git commit message associated with this deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,

    /// Functions routing configuration file.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functions-filepath-routing-config.json")]
    pub functions_filepath_routing_config_json: Option<String>,

    /// JSON string containing a manifest of files to deploy. Maps file paths to their
    /// content hashes. Required for direct upload deployments. Maximum 20,000 entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,

    /// The build output directory path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_build_output_dir: Option<String>,

    /// Hash of the Wrangler configuration file used for this deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrangler_config_hash: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// What type of deployments to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<String>,

    /// Which page of deployments to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    /// How many deployments to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentRetryParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentRollbackParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentHistoryLogGetResponse {
    pub data: Vec<ProjectDeploymentHistoryLogGetResponseData>,

    pub includes_container_logs: bool,

    pub total: i64,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentHistoryLogGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainNewResponse {
    pub id: String,

    pub certificate_authority: ProjectDomainNewResponseCertificateAuthority,

    pub created_on: String,

    pub domain_id: String,

    /// The domain name.
    pub name: String,

    pub status: ProjectDomainNewResponseStatus,

    pub validation_data: ProjectDomainNewResponseValidationData,

    pub verification_data: ProjectDomainNewResponseVerificationData,

    pub zone_tag: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainListResponse {
    pub id: String,

    pub certificate_authority: ProjectDomainListResponseCertificateAuthority,

    pub created_on: String,

    pub domain_id: String,

    /// The domain name.
    pub name: String,

    pub status: ProjectDomainListResponseStatus,

    pub validation_data: ProjectDomainListResponseValidationData,

    pub verification_data: ProjectDomainListResponseVerificationData,

    pub zone_tag: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainEditResponse {
    pub id: String,

    pub certificate_authority: ProjectDomainEditResponseCertificateAuthority,

    pub created_on: String,

    pub domain_id: String,

    /// The domain name.
    pub name: String,

    pub status: ProjectDomainEditResponseStatus,

    pub validation_data: ProjectDomainEditResponseValidationData,

    pub verification_data: ProjectDomainEditResponseVerificationData,

    pub zone_tag: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainGetResponse {
    pub id: String,

    pub certificate_authority: ProjectDomainGetResponseCertificateAuthority,

    pub created_on: String,

    pub domain_id: String,

    /// The domain name.
    pub name: String,

    pub status: ProjectDomainGetResponseStatus,

    pub validation_data: ProjectDomainGetResponseValidationData,

    pub verification_data: ProjectDomainGetResponseVerificationData,

    pub zone_tag: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The domain name.
    pub name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainEditParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentBuildConfig {
    /// The classifying tag for analytics.
    pub web_analytics_tag: String,

    /// The auth token for analytics.
    pub web_analytics_token: String,

    /// Enable build caching for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_caching: Option<bool>,

    /// Command used to build project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,

    /// Assets output directory of the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_dir: Option<String>,

    /// Directory to run the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentDeploymentTrigger {
    /// Additional info about the trigger.
    pub metadata: DeploymentDeploymentTriggerMetadata,

    /// What caused the deployment.
    pub r#type: DeploymentDeploymentTriggerType,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEnvVar {
    pub r#type: DeploymentEnvVarsType,

    /// Environment variable value.
    pub value: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSource {
    pub config: DeploymentSourceConfig,

    /// The source control management provider.
    pub r#type: DeploymentSourceType,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigs {
    /// Configs for preview deploys.
    pub preview: ProjectDeploymentConfigsPreview,

    /// Configs for production deploys.
    pub production: ProjectDeploymentConfigsProduction,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBuildConfig {
    /// The classifying tag for analytics.
    pub web_analytics_tag: String,

    /// The auth token for analytics.
    pub web_analytics_token: String,

    /// Enable build caching for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_caching: Option<bool>,

    /// Command used to build project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,

    /// Assets output directory of the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_dir: Option<String>,

    /// Directory to run the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSource {
    pub config: ProjectSourceConfig,

    /// The source control management provider.
    pub r#type: ProjectSourceType,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentHistoryLogGetResponseData {
    pub line: String,

    pub ts: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainNewResponseValidationData {
    pub method: ProjectDomainNewResponseValidationDataMethod,

    pub status: ProjectDomainNewResponseValidationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainNewResponseVerificationData {
    pub status: ProjectDomainNewResponseVerificationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainListResponseValidationData {
    pub method: ProjectDomainListResponseValidationDataMethod,

    pub status: ProjectDomainListResponseValidationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainListResponseVerificationData {
    pub status: ProjectDomainListResponseVerificationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainEditResponseValidationData {
    pub method: ProjectDomainEditResponseValidationDataMethod,

    pub status: ProjectDomainEditResponseValidationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainEditResponseVerificationData {
    pub status: ProjectDomainEditResponseVerificationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainGetResponseValidationData {
    pub method: ProjectDomainGetResponseValidationDataMethod,

    pub status: ProjectDomainGetResponseValidationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_value: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDomainGetResponseVerificationData {
    pub status: ProjectDomainGetResponseVerificationDataStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentDeploymentTriggerMetadata {
    /// Where the trigger happened.
    pub branch: String,

    /// Whether the deployment trigger commit was dirty.
    pub commit_dirty: bool,

    /// Hash of the deployment trigger commit.
    pub commit_hash: String,

    /// Message of the deployment trigger commit.
    pub commit_message: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSourceConfig {
    /// Whether to enable automatic deployments when pushing to the source repository.
    /// When disabled, no deployments (production or preview) will be triggered
    /// automatically.
    /// Deprecated: Use `production_deployments_enabled` and
    /// `preview_deployment_setting` for more granular control.
    pub deployments_enabled: bool,

    /// The owner of the repository.
    pub owner: String,

    /// The owner ID of the repository.
    pub owner_id: String,

    /// A list of paths that should be excluded from triggering a preview deployment.
    /// Wildcard syntax (`*`) is supported.
    pub path_excludes: Vec<String>,

    /// A list of paths that should be watched to trigger a preview deployment. Wildcard
    /// syntax (`*`) is supported.
    pub path_includes: Vec<String>,

    /// Whether to enable PR comments.
    pub pr_comments_enabled: bool,

    /// A list of branches that should not trigger a preview deployment. Wildcard syntax
    /// (`*`) is supported. Must be used with `preview_deployment_setting` set to
    /// `custom`.
    pub preview_branch_excludes: Vec<String>,

    /// A list of branches that should trigger a preview deployment. Wildcard syntax
    /// (`*`) is supported. Must be used with `preview_deployment_setting` set to
    /// `custom`.
    pub preview_branch_includes: Vec<String>,

    /// Controls whether commits to preview branches trigger a preview deployment.
    pub preview_deployment_setting: DeploymentSourceConfigPreviewDeploymentSetting,

    /// The production branch of the repository.
    pub production_branch: String,

    /// Whether to trigger a production deployment on commits to the production branch.
    pub production_deployments_enabled: bool,

    /// The ID of the repository.
    pub repo_id: String,

    /// The name of the repository.
    pub repo_name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreview {
    /// Whether to always use the latest compatibility date for Pages Functions.
    pub always_use_latest_compatibility_date: bool,

    /// The major version of the build image to use for Pages Functions.
    pub build_image_major_version: i64,

    /// Compatibility date used for Pages Functions.
    pub compatibility_date: String,

    /// Compatibility flags used for Pages Functions.
    pub compatibility_flags: Vec<String>,

    /// Environment variables used for builds and Pages Functions.
    pub env_vars: ::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewEnvVar>,

    /// Whether to fail open when the deployment config cannot be applied.
    pub fail_open: bool,

    /// The usage model for Pages Functions.
    /// Deprecated: All new projects now use the Standard usage model.
    pub usage_model: ProjectDeploymentConfigsPreviewUsageModel,

    /// Constellation bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewAIBinding>>,

    /// Analytics Engine bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_engine_datasets: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewAnalyticsEngineDataset>>,

    /// Browser bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewBrowser>>,

    /// D1 databases used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewD1Database>>,

    /// Durable Object namespaces used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_object_namespaces: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewDurableObjectNamespace>>,

    /// Hyperdrive bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperdrive_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewHyperdriveBinding>>,

    /// KV namespaces used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewKVNamespace>>,

    /// Limits for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ProjectDeploymentConfigsPreviewLimits>,

    /// mTLS bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_certificates: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewMTLSCertificate>>,

    /// Placement setting used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ProjectDeploymentConfigsPreviewPlacement>,

    /// Queue Producer bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_producers: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewQueueProducer>>,

    /// R2 buckets used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewR2Bucket>>,

    /// Services used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewService>>,

    /// Vectorize bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectorize_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsPreviewVectorizeBinding>>,

    /// Hash of the Wrangler configuration used for the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrangler_config_hash: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProduction {
    /// Whether to always use the latest compatibility date for Pages Functions.
    pub always_use_latest_compatibility_date: bool,

    /// The major version of the build image to use for Pages Functions.
    pub build_image_major_version: i64,

    /// Compatibility date used for Pages Functions.
    pub compatibility_date: String,

    /// Compatibility flags used for Pages Functions.
    pub compatibility_flags: Vec<String>,

    /// Environment variables used for builds and Pages Functions.
    pub env_vars: ::std::collections::HashMap<String, ProjectDeploymentConfigsProductionEnvVar>,

    /// Whether to fail open when the deployment config cannot be applied.
    pub fail_open: bool,

    /// The usage model for Pages Functions.
    /// Deprecated: All new projects now use the Standard usage model.
    pub usage_model: ProjectDeploymentConfigsProductionUsageModel,

    /// Constellation bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionAIBinding>>,

    /// Analytics Engine bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_engine_datasets: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionAnalyticsEngineDataset>>,

    /// Browser bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browsers: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionBrowser>>,

    /// D1 databases used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d1_databases: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionD1Database>>,

    /// Durable Object namespaces used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_object_namespaces: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionDurableObjectNamespace>>,

    /// Hyperdrive bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperdrive_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionHyperdriveBinding>>,

    /// KV namespaces used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_namespaces: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionKVNamespace>>,

    /// Limits for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ProjectDeploymentConfigsProductionLimits>,

    /// mTLS bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtls_certificates: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionMTLSCertificate>>,

    /// Placement setting used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ProjectDeploymentConfigsProductionPlacement>,

    /// Queue Producer bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_producers: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionQueueProducer>>,

    /// R2 buckets used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2_buckets: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionR2Bucket>>,

    /// Services used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionService>>,

    /// Vectorize bindings used for Pages Functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectorize_bindings: Option<::std::collections::HashMap<String, ProjectDeploymentConfigsProductionVectorizeBinding>>,

    /// Hash of the Wrangler configuration used for the deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrangler_config_hash: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSourceConfig {
    /// Whether to enable automatic deployments when pushing to the source repository.
    /// When disabled, no deployments (production or preview) will be triggered
    /// automatically.
    /// Deprecated: Use `production_deployments_enabled` and
    /// `preview_deployment_setting` for more granular control.
    pub deployments_enabled: bool,

    /// The owner of the repository.
    pub owner: String,

    /// The owner ID of the repository.
    pub owner_id: String,

    /// A list of paths that should be excluded from triggering a preview deployment.
    /// Wildcard syntax (`*`) is supported.
    pub path_excludes: Vec<String>,

    /// A list of paths that should be watched to trigger a preview deployment. Wildcard
    /// syntax (`*`) is supported.
    pub path_includes: Vec<String>,

    /// Whether to enable PR comments.
    pub pr_comments_enabled: bool,

    /// A list of branches that should not trigger a preview deployment. Wildcard syntax
    /// (`*`) is supported. Must be used with `preview_deployment_setting` set to
    /// `custom`.
    pub preview_branch_excludes: Vec<String>,

    /// A list of branches that should trigger a preview deployment. Wildcard syntax
    /// (`*`) is supported. Must be used with `preview_deployment_setting` set to
    /// `custom`.
    pub preview_branch_includes: Vec<String>,

    /// Controls whether commits to preview branches trigger a preview deployment.
    pub preview_deployment_setting: ProjectSourceConfigPreviewDeploymentSetting,

    /// The production branch of the repository.
    pub production_branch: String,

    /// Whether to trigger a production deployment on commits to the production branch.
    pub production_deployments_enabled: bool,

    /// The ID of the repository.
    pub repo_id: String,

    /// The name of the repository.
    pub repo_name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewEnvVar {
    pub r#type: ProjectDeploymentConfigsPreviewEnvVarsType,

    /// Environment variable value.
    pub value: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewAIBinding {
    pub project_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewAnalyticsEngineDataset {
    /// Name of the dataset.
    pub dataset: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewBrowser {
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewD1Database {
    /// UUID of the D1 database.
    pub id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewDurableObjectNamespace {
    /// ID of the Durable Object namespace.
    pub namespace_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewHyperdriveBinding {
    pub id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewKVNamespace {
    /// ID of the KV namespace.
    pub namespace_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewLimits {
    /// CPU time limit in milliseconds.
    pub cpu_ms: i64,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewMTLSCertificate {
    pub certificate_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewPlacement {
    /// Placement mode.
    pub mode: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewQueueProducer {
    /// Name of the Queue.
    pub name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewR2Bucket {
    /// Name of the R2 bucket.
    pub name: String,

    /// Jurisdiction of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewService {
    /// The Service environment.
    pub environment: String,

    /// The Service name.
    pub service: String,

    /// The entrypoint to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsPreviewVectorizeBinding {
    pub index_name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionEnvVar {
    pub r#type: ProjectDeploymentConfigsProductionEnvVarsType,

    /// Environment variable value.
    pub value: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionAIBinding {
    pub project_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionAnalyticsEngineDataset {
    /// Name of the dataset.
    pub dataset: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionBrowser {
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionD1Database {
    /// UUID of the D1 database.
    pub id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionDurableObjectNamespace {
    /// ID of the Durable Object namespace.
    pub namespace_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionHyperdriveBinding {
    pub id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionKVNamespace {
    /// ID of the KV namespace.
    pub namespace_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionLimits {
    /// CPU time limit in milliseconds.
    pub cpu_ms: i64,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionMTLSCertificate {
    pub certificate_id: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionPlacement {
    /// Placement mode.
    pub mode: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionQueueProducer {
    /// Name of the Queue.
    pub name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionR2Bucket {
    /// Name of the R2 bucket.
    pub name: String,

    /// Jurisdiction of the R2 bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionService {
    /// The Service environment.
    pub environment: String,

    /// The Service name.
    pub service: String,

    /// The entrypoint to bind to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDeploymentConfigsProductionVectorizeBinding {
    pub index_name: String,

}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    /// value is "preview"
    #[serde(rename = "preview")]
    Preview,
    /// value is "production"
    #[serde(rename = "production")]
    Production,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StageName {
    /// value is "queued"
    #[serde(rename = "queued")]
    Queued,
    /// value is "initialize"
    #[serde(rename = "initialize")]
    Initialize,
    /// value is "clone_repo"
    #[serde(rename = "clone_repo")]
    CloneRepo,
    /// value is "build"
    #[serde(rename = "build")]
    Build,
    /// value is "deploy"
    #[serde(rename = "deploy")]
    Deploy,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StageStatus {
    /// value is "success"
    #[serde(rename = "success")]
    Success,
    /// value is "idle"
    #[serde(rename = "idle")]
    Idle,
    /// value is "active"
    #[serde(rename = "active")]
    Active,
    /// value is "failure"
    #[serde(rename = "failure")]
    Failure,
    /// value is "canceled"
    #[serde(rename = "canceled")]
    Canceled,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainNewResponseCertificateAuthority {
    /// value is "google"
    #[serde(rename = "google")]
    Google,
    /// value is "lets_encrypt"
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainNewResponseStatus {
    /// value is "initializing"
    #[serde(rename = "initializing")]
    Initializing,
    /// value is "pending"
    #[serde(rename = "pending")]
    Pending,
    /// value is "active"
    #[serde(rename = "active")]
    Active,
    /// value is "deactivated"
    #[serde(rename = "deactivated")]
    Deactivated,
    /// value is "blocked"
    #[serde(rename = "blocked")]
    Blocked,
    /// value is "error"
    #[serde(rename = "error")]
    Error,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainListResponseCertificateAuthority {
    /// value is "google"
    #[serde(rename = "google")]
    Google,
    /// value is "lets_encrypt"
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainListResponseStatus {
    /// value is "initializing"
    #[serde(rename = "initializing")]
    Initializing,
    /// value is "pending"
    #[serde(rename = "pending")]
    Pending,
    /// value is "active"
    #[serde(rename = "active")]
    Active,
    /// value is "deactivated"
    #[serde(rename = "deactivated")]
    Deactivated,
    /// value is "blocked"
    #[serde(rename = "blocked")]
    Blocked,
    /// value is "error"
    #[serde(rename = "error")]
    Error,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainEditResponseCertificateAuthority {
    #[serde(rename = "google")]
    ProjectDomainEditResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    ProjectDomainEditResponseCertificateAuthorityLetsEncrypt,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainEditResponseStatus {
    #[serde(rename = "initializing")]
    ProjectDomainEditResponseStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainEditResponseStatusPending,
    #[serde(rename = "active")]
    ProjectDomainEditResponseStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainEditResponseStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainEditResponseStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainEditResponseStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainGetResponseCertificateAuthority {
    #[serde(rename = "google")]
    ProjectDomainGetResponseCertificateAuthorityGoogle,
    #[serde(rename = "lets_encrypt")]
    ProjectDomainGetResponseCertificateAuthorityLetsEncrypt,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainGetResponseStatus {
    #[serde(rename = "initializing")]
    ProjectDomainGetResponseStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainGetResponseStatusPending,
    #[serde(rename = "active")]
    ProjectDomainGetResponseStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainGetResponseStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainGetResponseStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainGetResponseStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentDeploymentTriggerType {
    #[serde(rename = "github:push")]
    DeploymentDeploymentTriggerTypeGitHubPush,
    #[serde(rename = "ad_hoc")]
    DeploymentDeploymentTriggerTypeADHoc,
    #[serde(rename = "deploy_hook")]
    DeploymentDeploymentTriggerTypeDeployHook,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentEnvVarsType {
    #[serde(rename = "plain_text")]
    DeploymentEnvVarsTypePlainText,
    #[serde(rename = "secret_text")]
    DeploymentEnvVarsTypeSecretText,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentSourceType {
    #[serde(rename = "github")]
    DeploymentSourceTypeGitHub,
    #[serde(rename = "gitlab")]
    DeploymentSourceTypeGitlab,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectSourceType {
    #[serde(rename = "github")]
    ProjectSourceTypeGitHub,
    #[serde(rename = "gitlab")]
    ProjectSourceTypeGitlab,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainNewResponseValidationDataMethod {
    #[serde(rename = "http")]
    ProjectDomainNewResponseValidationDataMethodHTTP,
    #[serde(rename = "txt")]
    ProjectDomainNewResponseValidationDataMethodTXT,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainNewResponseValidationDataStatus {
    #[serde(rename = "initializing")]
    ProjectDomainNewResponseValidationDataStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainNewResponseValidationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainNewResponseValidationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainNewResponseValidationDataStatusDeactivated,
    #[serde(rename = "error")]
    ProjectDomainNewResponseValidationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainNewResponseVerificationDataStatus {
    #[serde(rename = "pending")]
    ProjectDomainNewResponseVerificationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainNewResponseVerificationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainNewResponseVerificationDataStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainNewResponseVerificationDataStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainNewResponseVerificationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainListResponseValidationDataMethod {
    #[serde(rename = "http")]
    ProjectDomainListResponseValidationDataMethodHTTP,
    #[serde(rename = "txt")]
    ProjectDomainListResponseValidationDataMethodTXT,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainListResponseValidationDataStatus {
    #[serde(rename = "initializing")]
    ProjectDomainListResponseValidationDataStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainListResponseValidationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainListResponseValidationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainListResponseValidationDataStatusDeactivated,
    #[serde(rename = "error")]
    ProjectDomainListResponseValidationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainListResponseVerificationDataStatus {
    #[serde(rename = "pending")]
    ProjectDomainListResponseVerificationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainListResponseVerificationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainListResponseVerificationDataStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainListResponseVerificationDataStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainListResponseVerificationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainEditResponseValidationDataMethod {
    #[serde(rename = "http")]
    ProjectDomainEditResponseValidationDataMethodHTTP,
    #[serde(rename = "txt")]
    ProjectDomainEditResponseValidationDataMethodTXT,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainEditResponseValidationDataStatus {
    #[serde(rename = "initializing")]
    ProjectDomainEditResponseValidationDataStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainEditResponseValidationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainEditResponseValidationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainEditResponseValidationDataStatusDeactivated,
    #[serde(rename = "error")]
    ProjectDomainEditResponseValidationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainEditResponseVerificationDataStatus {
    #[serde(rename = "pending")]
    ProjectDomainEditResponseVerificationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainEditResponseVerificationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainEditResponseVerificationDataStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainEditResponseVerificationDataStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainEditResponseVerificationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainGetResponseValidationDataMethod {
    #[serde(rename = "http")]
    ProjectDomainGetResponseValidationDataMethodHTTP,
    #[serde(rename = "txt")]
    ProjectDomainGetResponseValidationDataMethodTXT,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainGetResponseValidationDataStatus {
    #[serde(rename = "initializing")]
    ProjectDomainGetResponseValidationDataStatusInitializing,
    #[serde(rename = "pending")]
    ProjectDomainGetResponseValidationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainGetResponseValidationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainGetResponseValidationDataStatusDeactivated,
    #[serde(rename = "error")]
    ProjectDomainGetResponseValidationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDomainGetResponseVerificationDataStatus {
    #[serde(rename = "pending")]
    ProjectDomainGetResponseVerificationDataStatusPending,
    #[serde(rename = "active")]
    ProjectDomainGetResponseVerificationDataStatusActive,
    #[serde(rename = "deactivated")]
    ProjectDomainGetResponseVerificationDataStatusDeactivated,
    #[serde(rename = "blocked")]
    ProjectDomainGetResponseVerificationDataStatusBlocked,
    #[serde(rename = "error")]
    ProjectDomainGetResponseVerificationDataStatusError,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentSourceConfigPreviewDeploymentSetting {
    #[serde(rename = "all")]
    DeploymentSourceConfigPreviewDeploymentSettingAll,
    #[serde(rename = "none")]
    DeploymentSourceConfigPreviewDeploymentSettingNone,
    #[serde(rename = "custom")]
    DeploymentSourceConfigPreviewDeploymentSettingCustom,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDeploymentConfigsPreviewUsageModel {
    #[serde(rename = "standard")]
    ProjectDeploymentConfigsPreviewUsageModelStandard,
    #[serde(rename = "bundled")]
    ProjectDeploymentConfigsPreviewUsageModelBundled,
    #[serde(rename = "unbound")]
    ProjectDeploymentConfigsPreviewUsageModelUnbound,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDeploymentConfigsProductionUsageModel {
    #[serde(rename = "standard")]
    ProjectDeploymentConfigsProductionUsageModelStandard,
    #[serde(rename = "bundled")]
    ProjectDeploymentConfigsProductionUsageModelBundled,
    #[serde(rename = "unbound")]
    ProjectDeploymentConfigsProductionUsageModelUnbound,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectSourceConfigPreviewDeploymentSetting {
    #[serde(rename = "all")]
    ProjectSourceConfigPreviewDeploymentSettingAll,
    #[serde(rename = "none")]
    ProjectSourceConfigPreviewDeploymentSettingNone,
    #[serde(rename = "custom")]
    ProjectSourceConfigPreviewDeploymentSettingCustom,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDeploymentConfigsPreviewEnvVarsType {
    #[serde(rename = "plain_text")]
    ProjectDeploymentConfigsPreviewEnvVarsTypePlainText,
    #[serde(rename = "secret_text")]
    ProjectDeploymentConfigsPreviewEnvVarsTypeSecretText,
}


#[cfg(any(feature = "full", feature = "with-pages"))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectDeploymentConfigsProductionEnvVarsType {
    #[serde(rename = "plain_text")]
    ProjectDeploymentConfigsProductionEnvVarsTypePlainText,
    #[serde(rename = "secret_text")]
    ProjectDeploymentConfigsProductionEnvVarsTypeSecretText,
}

