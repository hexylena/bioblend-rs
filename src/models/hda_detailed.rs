/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HdaDetailed : History Dataset Association detailed information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HdaDetailed {
    /// Whether this item is accessible to the current user due to permissions.
    #[serde(rename = "accessible")]
    pub accessible: bool,
    /// An annotation to provide details or to help understand the purpose and usage of this item.
    #[serde(rename = "annotation")]
    pub annotation: String,
    /// TODO
    #[serde(rename = "api_type", skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    /// The time and date this item was created.
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// The basename of the output that produced this dataset.
    #[serde(rename = "created_from_basename", skip_serializing_if = "Option::is_none")]
    pub created_from_basename: Option<String>,
    /// The encoded ID of the job that created this dataset.
    #[serde(rename = "creating_job")]
    pub creating_job: String,
    /// The fully qualified name of the class implementing the data type of this dataset.
    #[serde(rename = "data_type")]
    pub data_type: String,
    /// The encoded ID of the dataset associated with this item.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Whether this item is marked as deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// Contains new-style display app urls.
    #[serde(rename = "display_apps")]
    pub display_apps: Vec<crate::models::DisplayApp>,
    /// Contains old-style display app urls.
    #[serde(rename = "display_types")]
    pub display_types: Vec<crate::models::DisplayApp>,
    /// The URL to download this item from the server.
    #[serde(rename = "download_url")]
    pub download_url: String,
    /// The extension of the dataset.
    #[serde(rename = "extension")]
    pub extension: String,
    /// The extension of the file.
    #[serde(rename = "file_ext")]
    pub file_ext: String,
    /// The full path to the dataset file.
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The file size in bytes.
    #[serde(rename = "file_size")]
    pub file_size: i32,
    /// TODO
    #[serde(rename = "genome_build", skip_serializing_if = "Option::is_none")]
    pub genome_build: Option<String>,
    /// Whether this dataset belongs to a history (HDA) or a library (LDDA).
    #[serde(rename = "hda_ldda", skip_serializing_if = "Option::is_none")]
    pub hda_ldda: Option<Box<crate::models::DatasetSourceType>>,
    /// The index position of this item in the History.
    #[serde(rename = "hid")]
    pub hid: i32,
    /// The type of this item.
    #[serde(rename = "history_content_type")]
    pub history_content_type: Box<crate::models::HistoryContentType>,
    /// The encoded ID of the history associated with this item.
    #[serde(rename = "history_id")]
    pub history_id: String,
    /// The encoded ID of this entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Collection of metadata files associated with this dataset.
    #[serde(rename = "meta_files")]
    pub meta_files: Vec<crate::models::MetadataFile>,
    /// The metadata associated with this dataset.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// TODO
    #[serde(rename = "metadata_data_lines", skip_serializing_if = "Option::is_none")]
    pub metadata_data_lines: Option<i32>,
    /// TODO
    #[serde(rename = "metadata_dbkey", skip_serializing_if = "Option::is_none")]
    pub metadata_dbkey: Option<String>,
    /// TODO
    #[serde(rename = "misc_blurb", skip_serializing_if = "Option::is_none")]
    pub misc_blurb: Option<String>,
    /// TODO
    #[serde(rename = "misc_info", skip_serializing_if = "Option::is_none")]
    pub misc_info: Option<String>,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    /// The name of the item.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A few lines of contents from the start of the file.
    #[serde(rename = "peek", skip_serializing_if = "Option::is_none")]
    pub peek: Option<String>,
    #[serde(rename = "permissions")]
    pub permissions: Box<crate::models::Permissions>,
    /// Whether this dataset has been removed from disk.
    #[serde(rename = "purged")]
    pub purged: bool,
    /// Whether the job creating this dataset can be run again.
    #[serde(rename = "rerunnable")]
    pub rerunnable: bool,
    /// Whether the job creating this dataset has been resubmitted.
    #[serde(rename = "resubmitted")]
    pub resubmitted: bool,
    /// The current state of this dataset.
    #[serde(rename = "state")]
    pub state: Box<crate::models::GalaxyModelDatasetStates>,
    /// The collection of tags associated with an item.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// This is always `file` for datasets.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The type and the encoded ID of this item. Used for caching.
    #[serde(rename = "type_id", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    /// The last time and date this item was updated.
    #[serde(rename = "update_time", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// The relative URL to access this item.
    #[serde(rename = "url")]
    pub url: String,
    /// Universal unique identifier for this dataset.
    #[serde(rename = "uuid")]
    pub uuid: String,
    /// The state of the datatype validation for this dataset.
    #[serde(rename = "validated_state")]
    pub validated_state: Box<crate::models::ValidatedStates>,
    /// The message with details about the datatype validation result for this dataset.
    #[serde(rename = "validated_state_message")]
    pub validated_state_message: String,
    /// Whether this item is visible or hidden to the user by default.
    #[serde(rename = "visible")]
    pub visible: bool,
    /// The collection of visualizations that can be applied to this dataset.
    #[serde(rename = "visualizations")]
    pub visualizations: Vec<serde_json::Value>,
}

impl HdaDetailed {
    /// History Dataset Association detailed information.
    pub fn new(accessible: bool, annotation: String, creating_job: String, data_type: String, dataset_id: String, deleted: bool, display_apps: Vec<crate::models::DisplayApp>, display_types: Vec<crate::models::DisplayApp>, download_url: String, extension: String, file_ext: String, file_size: i32, hid: i32, history_content_type: crate::models::HistoryContentType, history_id: String, id: String, meta_files: Vec<crate::models::MetadataFile>, permissions: crate::models::Permissions, purged: bool, rerunnable: bool, resubmitted: bool, state: crate::models::GalaxyModelDatasetStates, tags: Vec<String>, url: String, uuid: String, validated_state: crate::models::ValidatedStates, validated_state_message: String, visible: bool, visualizations: Vec<serde_json::Value>) -> HdaDetailed {
        HdaDetailed {
            accessible,
            annotation,
            api_type: None,
            create_time: None,
            created_from_basename: None,
            creating_job,
            data_type,
            dataset_id,
            deleted,
            display_apps,
            display_types,
            download_url,
            extension,
            file_ext,
            file_name: None,
            file_size,
            genome_build: None,
            hda_ldda: None,
            hid,
            history_content_type: Box::new(history_content_type),
            history_id,
            id,
            meta_files,
            metadata: None,
            metadata_data_lines: None,
            metadata_dbkey: None,
            misc_blurb: None,
            misc_info: None,
            model_class: None,
            name: None,
            peek: None,
            permissions: Box::new(permissions),
            purged,
            rerunnable,
            resubmitted,
            state: Box::new(state),
            tags,
            r#type: None,
            type_id: None,
            update_time: None,
            url,
            uuid,
            validated_state: Box::new(validated_state),
            validated_state_message,
            visible,
            visualizations,
        }
    }
}

