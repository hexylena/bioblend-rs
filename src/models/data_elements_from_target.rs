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
pub struct DataElementsFromTarget {
    /// Decompress compressed data before sniffing?
    #[serde(rename = "auto_decompress", skip_serializing_if = "Option::is_none")]
    pub auto_decompress: Option<bool>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::Destination>,
    #[serde(rename = "elements_from")]
    pub elements_from: crate::models::ElementsFromType,
    #[serde(rename = "ftp_path", skip_serializing_if = "Option::is_none")]
    pub ftp_path: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "server_dir", skip_serializing_if = "Option::is_none")]
    pub server_dir: Option<String>,
    #[serde(rename = "src")]
    pub src: crate::models::ItemsFromSrc,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DataElementsFromTarget {
    pub fn new(destination: crate::models::Destination, elements_from: crate::models::ElementsFromType, src: crate::models::ItemsFromSrc) -> DataElementsFromTarget {
        DataElementsFromTarget {
            auto_decompress: None,
            destination: Box::new(destination),
            elements_from,
            ftp_path: None,
            path: None,
            server_dir: None,
            src,
            url: None,
        }
    }
}

