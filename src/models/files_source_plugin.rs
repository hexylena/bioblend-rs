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
pub struct FilesSourcePlugin {
    /// Documentation or extended description for this plugin.
    #[serde(rename = "doc")]
    pub doc: String,
    /// The `FilesSource` plugin identifier
    #[serde(rename = "id")]
    pub id: String,
    /// The display label for this plugin.
    #[serde(rename = "label")]
    pub label: String,
    /// Only users belonging to the groups specified here can access this files source.
    #[serde(rename = "requires_groups", skip_serializing_if = "Option::is_none")]
    pub requires_groups: Option<String>,
    /// Only users with the roles specified here can access this files source.
    #[serde(rename = "requires_roles", skip_serializing_if = "Option::is_none")]
    pub requires_roles: Option<String>,
    /// The type of the plugin.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The URI root used by this type of plugin.
    #[serde(rename = "uri_root")]
    pub uri_root: String,
    /// Whether this files source plugin allows write access.
    #[serde(rename = "writable")]
    pub writable: bool,
}

impl FilesSourcePlugin {
    pub fn new(doc: String, id: String, label: String, r#type: String, uri_root: String, writable: bool) -> FilesSourcePlugin {
        FilesSourcePlugin {
            doc,
            id,
            label,
            requires_groups: None,
            requires_roles: None,
            r#type,
            uri_root,
            writable,
        }
    }
}


