/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobExportHistoryIdAnyOf {
    #[serde(rename = "latest")]
    Latest,

}

impl ToString for JobExportHistoryIdAnyOf {
    fn to_string(&self) -> String {
        match self {
            Self::Latest => String::from("latest"),
        }
    }
}

impl Default for JobExportHistoryIdAnyOf {
    fn default() -> JobExportHistoryIdAnyOf {
        Self::Latest
    }
}




