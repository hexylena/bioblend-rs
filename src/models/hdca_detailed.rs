/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HdcaDetailed : History Dataset Collection Association detailed information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HdcaDetailed {
    /// The encoded ID of the dataset collection associated with this HDCA.
    #[serde(rename = "collection_id")]
    pub collection_id: String,
    /// The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`.
    #[serde(rename = "collection_type", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// The relative URL to access the contents of this History.
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    /// The time and date this item was created.
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Whether this item is marked as deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// The number of elements contained in the dataset collection. It may be None or undefined if the collection could not be populated.
    #[serde(rename = "element_count", skip_serializing_if = "Option::is_none")]
    pub element_count: Option<i32>,
    /// The summary information of each of the elements inside the dataset collection.
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<crate::models::DceSummary>>,
    /// A set containing all the different element datatypes in the collection.
    #[serde(rename = "elements_datatypes")]
    pub elements_datatypes: Vec<String>,
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
    /// The encoded ID of the Job that produced this dataset collection. Used to track the state of the job.
    #[serde(rename = "job_source_id", skip_serializing_if = "Option::is_none")]
    pub job_source_id: Option<String>,
    /// The type of job (model class) that produced this dataset collection. Used to track the state of the job.
    #[serde(rename = "job_source_type", skip_serializing_if = "Option::is_none")]
    pub job_source_type: Option<Box<crate::models::JobSourceType>>,
    #[serde(rename = "job_state_summary", skip_serializing_if = "Option::is_none")]
    pub job_state_summary: Option<Box<crate::models::JobStateSummary>>,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    /// The name of the item.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the dataset collection elements (and any subcollections elements) were successfully populated.
    #[serde(rename = "populated")]
    pub populated: bool,
    /// Indicates the general state of the elements in the dataset collection:- 'new': new dataset collection, unpopulated elements.- 'ok': collection elements populated (HDAs may or may not have errors).- 'failed': some problem populating, won't be populated.
    #[serde(rename = "populated_state")]
    pub populated_state: Box<crate::models::PopulatedStates>,
    /// Optional message with further information in case the population of the dataset collection failed.
    #[serde(rename = "populated_state_message", skip_serializing_if = "Option::is_none")]
    pub populated_state_message: Option<String>,
    /// The collection of tags associated with an item.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// This is always `collection` for dataset collections.
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
    /// Whether this item is visible or hidden to the user by default.
    #[serde(rename = "visible")]
    pub visible: bool,
}

impl HdcaDetailed {
    /// History Dataset Collection Association detailed information.
    pub fn new(collection_id: String, contents_url: String, deleted: bool, elements_datatypes: Vec<String>, hid: i32, history_content_type: crate::models::HistoryContentType, history_id: String, id: String, populated: bool, populated_state: crate::models::PopulatedStates, tags: Vec<String>, url: String, visible: bool) -> HdcaDetailed {
        HdcaDetailed {
            collection_id,
            collection_type: None,
            contents_url,
            create_time: None,
            deleted,
            element_count: None,
            elements: None,
            elements_datatypes,
            hid,
            history_content_type: Box::new(history_content_type),
            history_id,
            id,
            job_source_id: None,
            job_source_type: None,
            job_state_summary: None,
            model_class: None,
            name: None,
            populated,
            populated_state: Box::new(populated_state),
            populated_state_message: None,
            tags,
            r#type: None,
            type_id: None,
            update_time: None,
            url,
            visible,
        }
    }
}


