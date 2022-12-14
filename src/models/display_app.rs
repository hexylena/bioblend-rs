/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DisplayApp : Basic linked information about an application that can display certain datatypes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisplayApp {
    /// The label or title of the Display Application.
    #[serde(rename = "label")]
    pub label: String,
    /// The collection of link details for this Display Application.
    #[serde(rename = "links")]
    pub links: Vec<crate::models::Hyperlink>,
}

impl DisplayApp {
    /// Basic linked information about an application that can display certain datatypes.
    pub fn new(label: String, links: Vec<crate::models::Hyperlink>) -> DisplayApp {
        DisplayApp {
            label,
            links,
        }
    }
}


