/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SharingOptions : Options for sharing resources that may have restricted access to all or part of their contents.

/// Options for sharing resources that may have restricted access to all or part of their contents.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharingOptions {
    #[serde(rename = "make_public")]
    MakePublic,
    #[serde(rename = "make_accessible_to_shared")]
    MakeAccessibleToShared,
    #[serde(rename = "no_changes")]
    NoChanges,

}

impl ToString for SharingOptions {
    fn to_string(&self) -> String {
        match self {
            Self::MakePublic => String::from("make_public"),
            Self::MakeAccessibleToShared => String::from("make_accessible_to_shared"),
            Self::NoChanges => String::from("no_changes"),
        }
    }
}

impl Default for SharingOptions {
    fn default() -> SharingOptions {
        Self::MakePublic
    }
}




