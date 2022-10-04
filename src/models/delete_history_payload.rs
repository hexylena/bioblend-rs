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
pub struct DeleteHistoryPayload {
    /// Whether to definitely remove this history from disk.
    #[serde(rename = "purge", skip_serializing_if = "Option::is_none")]
    pub purge: Option<bool>,
}

impl DeleteHistoryPayload {
    pub fn new() -> DeleteHistoryPayload {
        DeleteHistoryPayload {
            purge: None,
        }
    }
}


