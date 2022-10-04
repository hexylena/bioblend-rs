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
pub struct LegacyLibraryPermissionsPayload {
    #[serde(rename = "LIBRARY_ACCESS_in", skip_serializing_if = "Option::is_none")]
    pub library_access_in: Option<Box<crate::models::AccessIds>>,
    #[serde(rename = "LIBRARY_ADD_in", skip_serializing_if = "Option::is_none")]
    pub library_add_in: Option<Box<crate::models::ManageIds>>,
    #[serde(rename = "LIBRARY_MANAGE_in", skip_serializing_if = "Option::is_none")]
    pub library_manage_in: Option<Box<crate::models::ModifyIds>>,
    #[serde(rename = "LIBRARY_MODIFY_in", skip_serializing_if = "Option::is_none")]
    pub library_modify_in: Option<Box<crate::models::AddIds>>,
}

impl LegacyLibraryPermissionsPayload {
    pub fn new() -> LegacyLibraryPermissionsPayload {
        LegacyLibraryPermissionsPayload {
            library_access_in: None,
            library_add_in: None,
            library_manage_in: None,
            library_modify_in: None,
        }
    }
}

