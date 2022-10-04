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
pub struct ResponseSetPermissionsApiLibrariesIdPermissionsPost {
    /// The time and date this item was created.
    #[serde(rename = "create_time")]
    pub create_time: String,
    /// Whether this Library has been deleted.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    /// A detailed description of the Library.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Encoded ID of the Library.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    /// The name of the Library.
    #[serde(rename = "name")]
    pub name: String,
    /// Encoded ID of the Library's base folder.
    #[serde(rename = "root_folder_id")]
    pub root_folder_id: String,
    /// A short text describing the contents of the Library.
    #[serde(rename = "synopsis", skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,
    /// A list containing pairs of role names and corresponding encoded IDs which have access to the Library.
    #[serde(rename = "access_library_role_list")]
    pub access_library_role_list: Vec<Vec<String>>,
    /// A list containing pairs of role names and corresponding encoded IDs which can add items to the Library.
    #[serde(rename = "add_library_item_role_list")]
    pub add_library_item_role_list: Vec<Vec<String>>,
    /// A list containing pairs of role names and corresponding encoded IDs which can manage the Library.
    #[serde(rename = "manage_library_role_list")]
    pub manage_library_role_list: Vec<Vec<String>>,
    /// A list containing pairs of role names and corresponding encoded IDs which can modify the Library.
    #[serde(rename = "modify_library_role_list")]
    pub modify_library_role_list: Vec<Vec<String>>,
}

impl ResponseSetPermissionsApiLibrariesIdPermissionsPost {
    pub fn new(create_time: String, deleted: bool, id: String, name: String, root_folder_id: String, access_library_role_list: Vec<Vec<String>>, add_library_item_role_list: Vec<Vec<String>>, manage_library_role_list: Vec<Vec<String>>, modify_library_role_list: Vec<Vec<String>>) -> ResponseSetPermissionsApiLibrariesIdPermissionsPost {
        ResponseSetPermissionsApiLibrariesIdPermissionsPost {
            create_time,
            deleted,
            description: None,
            id,
            model_class: None,
            name,
            root_folder_id,
            synopsis: None,
            access_library_role_list,
            add_library_item_role_list,
            manage_library_role_list,
            modify_library_role_list,
        }
    }
}


