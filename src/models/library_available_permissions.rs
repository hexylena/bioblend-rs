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
pub struct LibraryAvailablePermissions {
    /// Current page .
    #[serde(rename = "page")]
    pub page: i32,
    /// Maximum number of items per page.
    #[serde(rename = "page_limit")]
    pub page_limit: i32,
    /// A list available roles that can be assigned to a particular permission.
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::BasicRoleModel>,
    /// Total number of items
    #[serde(rename = "total")]
    pub total: i32,
}

impl LibraryAvailablePermissions {
    pub fn new(page: i32, page_limit: i32, roles: Vec<crate::models::BasicRoleModel>, total: i32) -> LibraryAvailablePermissions {
        LibraryAvailablePermissions {
            page,
            page_limit,
            roles,
            total,
        }
    }
}

