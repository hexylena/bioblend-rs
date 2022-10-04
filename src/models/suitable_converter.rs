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
pub struct SuitableConverter {
    /// The name of the converter.
    #[serde(rename = "name")]
    pub name: String,
    /// The type to convert from.
    #[serde(rename = "original_type")]
    pub original_type: String,
    /// The type to convert to.
    #[serde(rename = "target_type")]
    pub target_type: String,
    /// The ID of the tool that can perform the type conversion.
    #[serde(rename = "tool_id")]
    pub tool_id: String,
}

impl SuitableConverter {
    pub fn new(name: String, original_type: String, target_type: String, tool_id: String) -> SuitableConverter {
        SuitableConverter {
            name,
            original_type,
            target_type,
            tool_id,
        }
    }
}


