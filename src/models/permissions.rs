/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Permissions : Role-based access and manage control permissions for the dataset.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Permissions {
    /// The set of roles (encoded IDs) that can access this dataset.
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<String>>,
    /// The set of roles (encoded IDs) that can manage this dataset.
    #[serde(rename = "manage", skip_serializing_if = "Option::is_none")]
    pub manage: Option<Vec<String>>,
}

impl Permissions {
    /// Role-based access and manage control permissions for the dataset.
    pub fn new() -> Permissions {
        Permissions {
            access: None,
            manage: None,
        }
    }
}

