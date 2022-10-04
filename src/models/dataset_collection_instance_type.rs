/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DatasetCollectionInstanceType : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DatasetCollectionInstanceType {
    #[serde(rename = "history")]
    History,
    #[serde(rename = "library")]
    Library,

}

impl ToString for DatasetCollectionInstanceType {
    fn to_string(&self) -> String {
        match self {
            Self::History => String::from("history"),
            Self::Library => String::from("library"),
        }
    }
}

impl Default for DatasetCollectionInstanceType {
    fn default() -> DatasetCollectionInstanceType {
        Self::History
    }
}




