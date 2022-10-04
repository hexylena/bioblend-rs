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
pub struct CompositeFileInfo {
    /// Summary description of the purpouse of this file
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this file is a binary file
    #[serde(rename = "is_binary")]
    pub is_binary: bool,
    /// The MIME type of this file
    #[serde(rename = "mimetype", skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
    /// The name of this composite file
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "optional")]
    pub optional: bool,
    #[serde(rename = "space_to_tab")]
    pub space_to_tab: bool,
    #[serde(rename = "substitute_name_with_metadata", skip_serializing_if = "Option::is_none")]
    pub substitute_name_with_metadata: Option<String>,
    #[serde(rename = "to_posix_lines")]
    pub to_posix_lines: bool,
}

impl CompositeFileInfo {
    pub fn new(is_binary: bool, name: String, optional: bool, space_to_tab: bool, to_posix_lines: bool) -> CompositeFileInfo {
        CompositeFileInfo {
            description: None,
            is_binary,
            mimetype: None,
            name,
            optional,
            space_to_tab,
            substitute_name_with_metadata: None,
            to_posix_lines,
        }
    }
}

