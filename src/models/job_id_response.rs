/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JobIdResponse : Contains the ID of the job associated with a particular request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobIdResponse {
    /// The encoded database ID of the job that is currently processing a particular request.
    #[serde(rename = "job_id")]
    pub job_id: String,
}

impl JobIdResponse {
    /// Contains the ID of the job associated with a particular request.
    pub fn new(job_id: String) -> JobIdResponse {
        JobIdResponse {
            job_id,
        }
    }
}

