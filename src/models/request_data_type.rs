/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RequestDataType : Particular pieces of information that can be requested for a dataset.

/// Particular pieces of information that can be requested for a dataset.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestDataType {
    #[serde(rename = "state")]
    State,
    #[serde(rename = "converted_datasets_state")]
    ConvertedDatasetsState,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "features")]
    Features,
    #[serde(rename = "raw_data")]
    RawData,
    #[serde(rename = "track_config")]
    TrackConfig,
    #[serde(rename = "genome_data")]
    GenomeData,
    #[serde(rename = "in_use_state")]
    InUseState,

}

impl ToString for RequestDataType {
    fn to_string(&self) -> String {
        match self {
            Self::State => String::from("state"),
            Self::ConvertedDatasetsState => String::from("converted_datasets_state"),
            Self::Data => String::from("data"),
            Self::Features => String::from("features"),
            Self::RawData => String::from("raw_data"),
            Self::TrackConfig => String::from("track_config"),
            Self::GenomeData => String::from("genome_data"),
            Self::InUseState => String::from("in_use_state"),
        }
    }
}

impl Default for RequestDataType {
    fn default() -> RequestDataType {
        Self::State
    }
}




