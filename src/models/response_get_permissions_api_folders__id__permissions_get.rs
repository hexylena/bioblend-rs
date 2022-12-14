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
pub struct ResponseGetPermissionsApiFoldersIdPermissionsGet {
    /// A list containing pairs of role names and corresponding encoded IDs which can add items to the Library folder.
    #[serde(rename = "add_library_item_role_list")]
    pub add_library_item_role_list: Vec<Vec<String>>,
    /// A list containing pairs of role names and corresponding encoded IDs which can manage the Library folder.
    #[serde(rename = "manage_folder_role_list")]
    pub manage_folder_role_list: Vec<Vec<String>>,
    /// A list containing pairs of role names and corresponding encoded IDs which can modify the Library folder.
    #[serde(rename = "modify_folder_role_list")]
    pub modify_folder_role_list: Vec<Vec<String>>,
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

impl ResponseGetPermissionsApiFoldersIdPermissionsGet {
    pub fn new(add_library_item_role_list: Vec<Vec<String>>, manage_folder_role_list: Vec<Vec<String>>, modify_folder_role_list: Vec<Vec<String>>, page: i32, page_limit: i32, roles: Vec<crate::models::BasicRoleModel>, total: i32) -> ResponseGetPermissionsApiFoldersIdPermissionsGet {
        ResponseGetPermissionsApiFoldersIdPermissionsGet {
            add_library_item_role_list,
            manage_folder_role_list,
            modify_folder_role_list,
            page,
            page_limit,
            roles,
            total,
        }
    }
}


