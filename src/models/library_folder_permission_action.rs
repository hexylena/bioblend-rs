/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LibraryFolderPermissionAction : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LibraryFolderPermissionAction {
    #[serde(rename = "set_permissions")]
    SetPermissions,

}

impl ToString for LibraryFolderPermissionAction {
    fn to_string(&self) -> String {
        match self {
            Self::SetPermissions => String::from("set_permissions"),
        }
    }
}

impl Default for LibraryFolderPermissionAction {
    fn default() -> LibraryFolderPermissionAction {
        Self::SetPermissions
    }
}



