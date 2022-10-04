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
pub struct FtpImportElement {
    #[serde(rename = "MD5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// Decompress compressed data before sniffing?
    #[serde(rename = "auto_decompress", skip_serializing_if = "Option::is_none")]
    pub auto_decompress: Option<bool>,
    #[serde(rename = "collection_type", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    #[serde(rename = "created_from_basename", skip_serializing_if = "Option::is_none")]
    pub created_from_basename: Option<String>,
    #[serde(rename = "dbkey", skip_serializing_if = "Option::is_none")]
    pub dbkey: Option<String>,
    #[serde(rename = "deferred", skip_serializing_if = "Option::is_none")]
    pub deferred: Option<bool>,
    #[serde(rename = "elements_from", skip_serializing_if = "Option::is_none")]
    pub elements_from: Option<crate::models::ElementsFromType>,
    #[serde(rename = "ext", skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
    #[serde(rename = "extra_files", skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Box<crate::models::ExtraFiles>>,
    #[serde(rename = "ftp_path")]
    pub ftp_path: String,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "space_to_tab", skip_serializing_if = "Option::is_none")]
    pub space_to_tab: Option<bool>,
    #[serde(rename = "src")]
    pub src: Src,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "to_posix_lines", skip_serializing_if = "Option::is_none")]
    pub to_posix_lines: Option<bool>,
}

impl FtpImportElement {
    pub fn new(ftp_path: String, src: Src) -> FtpImportElement {
        FtpImportElement {
            md5: None,
            auto_decompress: None,
            collection_type: None,
            created_from_basename: None,
            dbkey: None,
            deferred: None,
            elements_from: None,
            ext: None,
            extra_files: None,
            ftp_path,
            info: None,
            name: None,
            space_to_tab: None,
            src,
            tags: None,
            to_posix_lines: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Src {
    #[serde(rename = "ftp_import")]
    FtpImport,
}

impl Default for Src {
    fn default() -> Src {
        Self::FtpImport
    }
}

