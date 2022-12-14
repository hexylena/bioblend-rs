/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateHistoryContentPayload : Base model definition with common configuration used by all derived models.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateHistoryContentPayload {
    /// The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`.
    #[serde(rename = "collection_type", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// Depending on the `source` it can be: - The encoded id from the library dataset - The encoded id from the library folder - The encoded id from the HDA - The encoded id from the HDCA 
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// If the source is a collection, whether to copy child HDAs into the target history as well, defaults to False but this is less than ideal and may be changed in future releases.
    #[serde(rename = "copy_elements", skip_serializing_if = "Option::is_none")]
    pub copy_elements: Option<bool>,
    /// TODO
    #[serde(rename = "dbkey", skip_serializing_if = "Option::is_none")]
    pub dbkey: Option<String>,
    /// List of elements that should be in the new collection.
    #[serde(rename = "element_identifiers", skip_serializing_if = "Option::is_none")]
    pub element_identifiers: Option<Vec<crate::models::CollectionElementIdentifier>>,
    /// The ID of the history that will contain the collection. Required if `instance_type=library`.
    #[serde(rename = "folder_id", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Whether to mark the original HDAs as hidden.
    #[serde(rename = "hide_source_items", skip_serializing_if = "Option::is_none")]
    pub hide_source_items: Option<bool>,
    /// The ID of the history that will contain the collection. Required if `instance_type=history`.
    #[serde(rename = "history_id", skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
    /// The type of the instance, either `history` (default) or `library`.
    #[serde(rename = "instance_type", skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<Box<crate::models::DatasetCollectionInstanceType>>,
    /// The name of the new collection.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The source of the content. Can be other history element to be copied or library elements.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::HistoryContentSource>>,
    /// The type of content to be created in the history.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::HistoryContentType>>,
}

impl CreateHistoryContentPayload {
    /// Base model definition with common configuration used by all derived models.
    pub fn new() -> CreateHistoryContentPayload {
        CreateHistoryContentPayload {
            collection_type: None,
            content: None,
            copy_elements: None,
            dbkey: None,
            element_identifiers: None,
            folder_id: None,
            hide_source_items: None,
            history_id: None,
            instance_type: None,
            name: None,
            source: None,
            r#type: None,
        }
    }
}


