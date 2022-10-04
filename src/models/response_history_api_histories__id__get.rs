/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseHistoryApiHistoriesIdGet {
    /// An annotation to provide details or to help understand the purpose and usage of this item.
    #[serde(rename = "annotation")]
    pub annotation: String,
    #[serde(rename = "contents_active")]
    pub contents_active: Box<crate::models::ActiveContents>,
    /// The relative URL to access the contents of this History.
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    /// The time and date this item was created.
    #[serde(rename = "create_time")]
    pub create_time: String,
    /// Whether this item is marked as deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// Whether this History has any content.
    #[serde(rename = "empty")]
    pub empty: bool,
    /// TODO
    #[serde(rename = "genome_build", skip_serializing_if = "Option::is_none")]
    pub genome_build: Option<String>,
    /// TODO
    #[serde(rename = "hid_counter")]
    pub hid_counter: i32,
    /// The encoded ID of this entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether this History can be imported by other users with a shared link.
    #[serde(rename = "importable")]
    pub importable: bool,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    /// The name of the history.
    #[serde(rename = "name")]
    pub name: String,
    /// Human-readable size of the contents of this history.
    #[serde(rename = "nice_size")]
    pub nice_size: String,
    /// Whether this resource is currently publicly available to all users.
    #[serde(rename = "published")]
    pub published: bool,
    /// Whether this item has been permanently removed.
    #[serde(rename = "purged")]
    pub purged: bool,
    /// The total size of the contents of this history in bytes.
    #[serde(rename = "size")]
    pub size: i32,
    /// Part of the URL to uniquely identify this History by link in a readable way.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The current state of the History based on the states of the datasets it contains.
    #[serde(rename = "state")]
    pub state: Box<crate::models::GalaxyModelDatasetStates>,
    /// A dictionary keyed to possible dataset states and valued with the number of datasets in this history that have those states.
    #[serde(rename = "state_details")]
    pub state_details: ::std::collections::HashMap<String, i32>,
    /// A dictionary keyed to possible dataset states and valued with lists containing the ids of each HDA in that state.
    #[serde(rename = "state_ids")]
    pub state_ids: ::std::collections::HashMap<String, Vec<String>>,
    /// The collection of tags associated with an item.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The last time and date this item was updated.
    #[serde(rename = "update_time")]
    pub update_time: String,
    /// The relative URL to access this item.
    #[serde(rename = "url")]
    pub url: String,
    /// The encoded ID of the user that owns this History.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The relative URL in the form of /u/{username}/h/{slug}
    #[serde(rename = "username_and_slug", skip_serializing_if = "Option::is_none")]
    pub username_and_slug: Option<String>,
}

impl ResponseHistoryApiHistoriesIdGet {
    pub fn new(annotation: String, contents_active: crate::models::ActiveContents, contents_url: String, create_time: String, deleted: bool, empty: bool, hid_counter: i32, id: String, importable: bool, name: String, nice_size: String, published: bool, purged: bool, size: i32, state: crate::models::GalaxyModelDatasetStates, state_details: ::std::collections::HashMap<String, i32>, state_ids: ::std::collections::HashMap<String, Vec<String>>, tags: Vec<String>, update_time: String, url: String, user_id: String) -> ResponseHistoryApiHistoriesIdGet {
        ResponseHistoryApiHistoriesIdGet {
            annotation,
            contents_active: Box::new(contents_active),
            contents_url,
            create_time,
            deleted,
            empty,
            genome_build: None,
            hid_counter,
            id,
            importable,
            model_class: None,
            name,
            nice_size,
            published,
            purged,
            size,
            slug: None,
            state: Box::new(state),
            state_details,
            state_ids,
            tags,
            update_time,
            url,
            user_id,
            username_and_slug: None,
        }
    }
}


