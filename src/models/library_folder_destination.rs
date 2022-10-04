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
pub struct LibraryFolderDestination {
    #[serde(rename = "library_folder_id")]
    pub library_folder_id: String,
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl LibraryFolderDestination {
    pub fn new(library_folder_id: String, r#type: RHashType) -> LibraryFolderDestination {
        LibraryFolderDestination {
            library_folder_id,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "library_folder")]
    LibraryFolder,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LibraryFolder
    }
}
