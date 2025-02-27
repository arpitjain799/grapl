/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "IsMultiregion", skip_serializing_if = "Option::is_none")]
    pub is_multiregion: Option<bool>,
    #[serde(rename = "JobCreateIndex", skip_serializing_if = "Option::is_none")]
    pub job_create_index: Option<i32>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "JobSpecModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_spec_modify_index: Option<i32>,
    #[serde(rename = "JobVersion", skip_serializing_if = "Option::is_none")]
    pub job_version: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "TaskGroups", skip_serializing_if = "Option::is_none")]
    pub task_groups: Option<::std::collections::HashMap<String, crate::models::DeploymentState>>,
}

impl Deployment {
    pub fn new() -> Deployment {
        Deployment {
            create_index: None,
            ID: None,
            is_multiregion: None,
            job_create_index: None,
            job_id: None,
            job_modify_index: None,
            job_spec_modify_index: None,
            job_version: None,
            modify_index: None,
            namespace: None,
            status: None,
            status_description: None,
            task_groups: None,
        }
    }
}
