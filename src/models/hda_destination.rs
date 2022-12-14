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
pub struct HdaDestination {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl HdaDestination {
    pub fn new(r#type: RHashType) -> HdaDestination {
        HdaDestination {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "hdas")]
    Hdas,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Hdas
    }
}

