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
pub struct HdcaDataItemsFromTarget {
    /// Decompress compressed data before sniffing?
    #[serde(rename = "auto_decompress", skip_serializing_if = "Option::is_none")]
    pub auto_decompress: Option<bool>,
    #[serde(rename = "collection_type", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::HdcaDestination>,
    #[serde(rename = "elements_from")]
    pub elements_from: crate::models::ElementsFromType,
    #[serde(rename = "ftp_path", skip_serializing_if = "Option::is_none")]
    pub ftp_path: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "server_dir", skip_serializing_if = "Option::is_none")]
    pub server_dir: Option<String>,
    #[serde(rename = "src")]
    pub src: crate::models::ItemsFromSrc,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HdcaDataItemsFromTarget {
    pub fn new(destination: crate::models::HdcaDestination, elements_from: crate::models::ElementsFromType, src: crate::models::ItemsFromSrc) -> HdcaDataItemsFromTarget {
        HdcaDataItemsFromTarget {
            auto_decompress: None,
            collection_type: None,
            destination: Box::new(destination),
            elements_from,
            ftp_path: None,
            name: None,
            path: None,
            server_dir: None,
            src,
            tags: None,
            url: None,
        }
    }
}


