/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`index_api_ftp_files_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexApiFtpFilesGetError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`index_api_ftp_files_get_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexApiFtpFilesGet0Error {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`index_api_remote_files_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexApiRemoteFilesGetError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`index_api_remote_files_get_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexApiRemoteFilesGet0Error {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`plugins_api_remote_files_plugins_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginsApiRemoteFilesPluginsGetError {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`plugins_api_remote_files_plugins_get_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginsApiRemoteFilesPluginsGet0Error {
    Status422(crate::models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Lists all remote files available to the user from different sources.
pub async fn index_api_ftp_files_get(configuration: &configuration::Configuration, target: Option<&str>, format: Option<crate::models::RemoteFilesFormat>, recursive: Option<bool>, disable: Option<crate::models::RemoteFilesDisableMode>, run_as: Option<&str>) -> Result<Vec<serde_json::Value>, Error<IndexApiFtpFilesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/ftp_files", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = target {
        local_var_req_builder = local_var_req_builder.query(&[("target", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = format {
        local_var_req_builder = local_var_req_builder.query(&[("format", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = disable {
        local_var_req_builder = local_var_req_builder.query(&[("disable", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IndexApiFtpFilesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all remote files available to the user from different sources.
pub async fn index_api_ftp_files_get_0(configuration: &configuration::Configuration, target: Option<&str>, format: Option<crate::models::RemoteFilesFormat>, recursive: Option<bool>, disable: Option<crate::models::RemoteFilesDisableMode>, run_as: Option<&str>) -> Result<Vec<serde_json::Value>, Error<IndexApiFtpFilesGet0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/ftp_files", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = target {
        local_var_req_builder = local_var_req_builder.query(&[("target", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = format {
        local_var_req_builder = local_var_req_builder.query(&[("format", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = disable {
        local_var_req_builder = local_var_req_builder.query(&[("disable", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IndexApiFtpFilesGet0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all remote files available to the user from different sources.
pub async fn index_api_remote_files_get(configuration: &configuration::Configuration, target: Option<&str>, format: Option<crate::models::RemoteFilesFormat>, recursive: Option<bool>, disable: Option<crate::models::RemoteFilesDisableMode>, run_as: Option<&str>) -> Result<Vec<serde_json::Value>, Error<IndexApiRemoteFilesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/remote_files", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = target {
        local_var_req_builder = local_var_req_builder.query(&[("target", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = format {
        local_var_req_builder = local_var_req_builder.query(&[("format", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = disable {
        local_var_req_builder = local_var_req_builder.query(&[("disable", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IndexApiRemoteFilesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all remote files available to the user from different sources.
pub async fn index_api_remote_files_get_0(configuration: &configuration::Configuration, target: Option<&str>, format: Option<crate::models::RemoteFilesFormat>, recursive: Option<bool>, disable: Option<crate::models::RemoteFilesDisableMode>, run_as: Option<&str>) -> Result<Vec<serde_json::Value>, Error<IndexApiRemoteFilesGet0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/remote_files", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = target {
        local_var_req_builder = local_var_req_builder.query(&[("target", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = format {
        local_var_req_builder = local_var_req_builder.query(&[("format", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = disable {
        local_var_req_builder = local_var_req_builder.query(&[("disable", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IndexApiRemoteFilesGet0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Display plugin information for each of the gxfiles:// URI targets available.
pub async fn plugins_api_remote_files_plugins_get(configuration: &configuration::Configuration, run_as: Option<&str>) -> Result<Vec<crate::models::FilesSourcePlugin>, Error<PluginsApiRemoteFilesPluginsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/remote_files/plugins", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PluginsApiRemoteFilesPluginsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Display plugin information for each of the gxfiles:// URI targets available.
pub async fn plugins_api_remote_files_plugins_get_0(configuration: &configuration::Configuration, run_as: Option<&str>) -> Result<Vec<crate::models::FilesSourcePlugin>, Error<PluginsApiRemoteFilesPluginsGet0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/remote_files/plugins", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("key", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = run_as {
        local_var_req_builder = local_var_req_builder.header("run-as", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PluginsApiRemoteFilesPluginsGet0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

