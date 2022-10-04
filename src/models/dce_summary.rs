/*
 * Galaxy API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 22.05.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DceSummary : Dataset Collection Element summary information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DceSummary {
    /// The actual name of this element.
    #[serde(rename = "element_identifier")]
    pub element_identifier: String,
    /// The position index of this element inside the collection.
    #[serde(rename = "element_index")]
    pub element_index: i32,
    /// The type of the element. Used to interpret the `object` field.
    #[serde(rename = "element_type")]
    pub element_type: Box<crate::models::DceType>,
    /// The encoded ID of this entity.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the database model class.
    #[serde(rename = "model_class", skip_serializing_if = "Option::is_none")]
    pub model_class: Option<String>,
    #[serde(rename = "object")]
    pub object: Box<crate::models::Object>,
}

impl DceSummary {
    /// Dataset Collection Element summary information.
    pub fn new(element_identifier: String, element_index: i32, element_type: crate::models::DceType, id: String, object: crate::models::Object) -> DceSummary {
        DceSummary {
            element_identifier,
            element_index,
            element_type: Box::new(element_type),
            id,
            model_class: None,
            object: Box::new(object),
        }
    }
}


