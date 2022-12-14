/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// QuotaDetails : Base model containing common fields for Quotas.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QuotaDetails {
    /// The amount, expressed in bytes, of this Quota.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// A list indicating which types of default user quotas, if any, are associated with this quota.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<crate::models::DefaultQuota>>,
    /// Detailed text description for this Quota.
    #[serde(rename = "description")]
    pub description: String,
    /// Human-readable representation of the `amount` field.
    #[serde(rename = "display_amount")]
    pub display_amount: String,
    /// A list of specific groups of users associated with this quota.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::GroupQuota>>,
    /// The `encoded identifier` of the quota.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    /// The name of the quota. This must be unique within a Galaxy instance.
    #[serde(rename = "name")]
    pub name: String,
    /// Quotas can have one of three `operations`:- `=` : The quota is exactly the amount specified- `+` : The amount specified will be added to the amounts of the user's other associated quota definitions- `-` : The amount specified will be subtracted from the amounts of the user's other associated quota definitions
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<crate::models::QuotaOperation>>,
    /// A list of specific users associated with this quota.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::UserQuota>>,
}

impl QuotaDetails {
    /// Base model containing common fields for Quotas.
    pub fn new(bytes: i32, description: String, display_amount: String, id: String, name: String) -> QuotaDetails {
        QuotaDetails {
            bytes,
            default: None,
            description,
            display_amount,
            groups: None,
            id,
            model_class: None,
            name,
            operation: None,
            users: None,
        }
    }
}


