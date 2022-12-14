/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateCollectionAttributePayload : Contains attributes that can be updated for all elements in a dataset collection.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateCollectionAttributePayload {
    /// TODO
    #[serde(rename = "dbkey")]
    pub dbkey: String,
}

impl UpdateCollectionAttributePayload {
    /// Contains attributes that can be updated for all elements in a dataset collection.
    pub fn new(dbkey: String) -> UpdateCollectionAttributePayload {
        UpdateCollectionAttributePayload {
            dbkey,
        }
    }
}


