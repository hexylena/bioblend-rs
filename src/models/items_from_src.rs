/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemsFromSrc : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemsFromSrc {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "ftp_import")]
    FtpImport,
    #[serde(rename = "server_dir")]
    ServerDir,

}

impl ToString for ItemsFromSrc {
    fn to_string(&self) -> String {
        match self {
            Self::Url => String::from("url"),
            Self::Files => String::from("files"),
            Self::Path => String::from("path"),
            Self::FtpImport => String::from("ftp_import"),
            Self::ServerDir => String::from("server_dir"),
        }
    }
}

impl Default for ItemsFromSrc {
    fn default() -> ItemsFromSrc {
        Self::Url
    }
}




