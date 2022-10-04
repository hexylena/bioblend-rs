/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActiveContents : Contains the number of active, deleted or hidden items in the History.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActiveContents {
    /// Number of active datasets.
    #[serde(rename = "active")]
    pub active: i32,
    /// Number of deleted datasets.
    #[serde(rename = "deleted")]
    pub deleted: i32,
    /// Number of hidden datasets.
    #[serde(rename = "hidden")]
    pub hidden: i32,
}

impl ActiveContents {
    /// Contains the number of active, deleted or hidden items in the History.
    pub fn new(active: i32, deleted: i32, hidden: i32) -> ActiveContents {
        ActiveContents {
            active,
            deleted,
            hidden,
        }
    }
}

