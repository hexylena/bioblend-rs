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
pub struct AsyncFile {
    #[serde(rename = "storage_request_id")]
    pub storage_request_id: String,
    #[serde(rename = "task")]
    pub task: Box<crate::models::AsyncTaskResultSummary>,
}

impl AsyncFile {
    pub fn new(storage_request_id: String, task: crate::models::AsyncTaskResultSummary) -> AsyncFile {
        AsyncFile {
            storage_request_id,
            task: Box::new(task),
        }
    }
}


