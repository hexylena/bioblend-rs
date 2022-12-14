/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PrepareStoreDownloadPayload : Base model definition with common configuration used by all derived models.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrepareStoreDownloadPayload {
    /// Include file contents for deleted datasets (if include_files is True).
    #[serde(rename = "include_deleted", skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    /// include materialized files in export when available
    #[serde(rename = "include_files", skip_serializing_if = "Option::is_none")]
    pub include_files: Option<bool>,
    /// Include file contents for hidden datasets (if include_files is True).
    #[serde(rename = "include_hidden", skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
    /// format of model store to export
    #[serde(rename = "model_store_format", skip_serializing_if = "Option::is_none")]
    pub model_store_format: Option<Box<crate::models::ModelStoreFormat>>,
}

impl PrepareStoreDownloadPayload {
    /// Base model definition with common configuration used by all derived models.
    pub fn new() -> PrepareStoreDownloadPayload {
        PrepareStoreDownloadPayload {
            include_deleted: None,
            include_files: None,
            include_hidden: None,
            model_store_format: None,
        }
    }
}


