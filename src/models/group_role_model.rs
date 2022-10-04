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
pub struct GroupRoleModel {
    /// Encoded ID of the role
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the role
    #[serde(rename = "name")]
    pub name: String,
    /// The relative URL to access this item.
    #[serde(rename = "url")]
    pub url: String,
}

impl GroupRoleModel {
    pub fn new(id: String, name: String, url: String) -> GroupRoleModel {
        GroupRoleModel {
            id,
            name,
            url,
        }
    }
}

