use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use reqwest;

use super::{configuration, Error};
use crate::google_rest_apis::compute_v1::apis::ResponseContent;

/// struct for passing parameters to the method [`compute_license_codes_get`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodLicenseCodesPeriodGetParams {
    /// Project ID for this request.
    pub project: String,
    /// Number corresponding to the License code resource to return.
    pub license_code: String,
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
}

/// struct for passing parameters to the method [`compute_license_codes_test_iam_permissions`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodLicenseCodesPeriodTestIamPermissionsParams {
    /// Project ID for this request.
    pub project: String,
    /// Name or id of the resource for this request.
    pub resource: String,
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
    pub test_permissions_request:
        Option<crate::google_rest_apis::compute_v1::models::TestPermissionsRequest>,
}

/// struct for typed errors of method [`compute_license_codes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodLicenseCodesPeriodGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`compute_license_codes_test_iam_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodLicenseCodesPeriodTestIamPermissionsError {
    UnknownValue(serde_json::Value),
}

/// Return a specified license code. License codes are mirrored across all projects that have permissions to read the License Code. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
pub async fn compute_license_codes_get(
    configuration: &configuration::Configuration,
    params: ComputePeriodLicenseCodesPeriodGetParams,
) -> Result<
    crate::google_rest_apis::compute_v1::models::LicenseCode,
    Error<ComputePeriodLicenseCodesPeriodGetError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let license_code = params.license_code;
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/global/licenseCodes/{licenseCode}",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        licenseCode = crate::google_rest_apis::compute_v1::apis::urlencode(license_code)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ComputePeriodLicenseCodesPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns permissions that a caller has on the specified resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
pub async fn compute_license_codes_test_iam_permissions(
    configuration: &configuration::Configuration,
    params: ComputePeriodLicenseCodesPeriodTestIamPermissionsParams,
) -> Result<
    crate::google_rest_apis::compute_v1::models::TestPermissionsResponse,
    Error<ComputePeriodLicenseCodesPeriodTestIamPermissionsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let resource = params.resource;
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let test_permissions_request = params.test_permissions_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/global/licenseCodes/{resource}/testIamPermissions",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        resource = crate::google_rest_apis::compute_v1::apis::urlencode(resource)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&test_permissions_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ComputePeriodLicenseCodesPeriodTestIamPermissionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}