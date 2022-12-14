/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DatasetInheritanceChainEntry : Base model definition with common configuration used by all derived models.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetInheritanceChainEntry {
    /// Name of the source of the referenced dataset at this point of the inheritance chain.
    #[serde(rename = "dep")]
    pub dep: String,
    /// Name of the referenced dataset
    #[serde(rename = "name")]
    pub name: String,
}

impl DatasetInheritanceChainEntry {
    /// Base model definition with common configuration used by all derived models.
    pub fn new(dep: String, name: String) -> DatasetInheritanceChainEntry {
        DatasetInheritanceChainEntry {
            dep,
            name,
        }
    }
}


