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
pub struct CreateLibraryPayload {
    /// A detailed description of the Library.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the Library.
    #[serde(rename = "name")]
    pub name: String,
    /// A short text describing the contents of the Library.
    #[serde(rename = "synopsis", skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,
}

impl CreateLibraryPayload {
    pub fn new(name: String) -> CreateLibraryPayload {
        CreateLibraryPayload {
            description: None,
            name,
            synopsis: None,
        }
    }
}

