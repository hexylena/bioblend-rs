/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateContentItem : Used for updating a particular history item. All fields are optional.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateContentItem {
    /// The type of this item.
    #[serde(rename = "history_content_type")]
    pub history_content_type: Box<crate::models::HistoryContentType>,
    /// The encoded ID of this entity.
    #[serde(rename = "id")]
    pub id: String,
}

impl UpdateContentItem {
    /// Used for updating a particular history item. All fields are optional.
    pub fn new(history_content_type: crate::models::HistoryContentType, id: String) -> UpdateContentItem {
        UpdateContentItem {
            history_content_type: Box::new(history_content_type),
            id,
        }
    }
}

