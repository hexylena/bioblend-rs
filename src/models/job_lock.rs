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
pub struct JobLock {
    /// If active, jobs will not dispatch
    #[serde(rename = "active")]
    pub active: bool,
}

impl JobLock {
    pub fn new(active: bool) -> JobLock {
        JobLock {
            active,
        }
    }
}


