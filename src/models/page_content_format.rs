/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PageContentFormat : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PageContentFormat {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,

}

impl ToString for PageContentFormat {
    fn to_string(&self) -> String {
        match self {
            Self::Markdown => String::from("markdown"),
            Self::Html => String::from("html"),
        }
    }
}

impl Default for PageContentFormat {
    fn default() -> PageContentFormat {
        Self::Markdown
    }
}




