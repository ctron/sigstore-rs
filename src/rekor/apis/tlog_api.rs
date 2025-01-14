/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::rekor::apis::ResponseContent;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`get_log_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogInfoError {
    DefaultResponse(crate::rekor::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_proof`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogProofError {
    Status400(crate::rekor::models::Error),
    DefaultResponse(crate::rekor::models::Error),
    UnknownValue(serde_json::Value),
}

/// Returns the current root hash and size of the merkle tree used to store the log entries.
pub async fn get_log_info(
    configuration: &configuration::Configuration,
) -> Result<crate::rekor::models::LogInfo, Error<GetLogInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogInfoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of hashes for specified tree sizes that can be used to confirm the consistency of the transparency log
pub async fn get_log_proof(
    configuration: &configuration::Configuration,
    last_size: i32,
    first_size: Option<&str>,
    tree_id: Option<&str>,
) -> Result<crate::rekor::models::ConsistencyProof, Error<GetLogProofError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log/proof", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = first_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("firstSize", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("lastSize", &last_size.to_string())]);
    if let Some(ref local_var_str) = tree_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("treeID", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogProofError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
