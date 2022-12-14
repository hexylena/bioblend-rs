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
pub struct ShareWithPayload {
    /// User choice for sharing resources which its contents may be restricted:  - None: The user did not choose anything yet or no option is needed.  - make_public: The contents of the resource will be made publicly accessible.  - make_accessible_to_shared: This will automatically create a new `sharing role` allowing protected contents to be accessed only by the desired users.  - no_changes: This won't change the current permissions for the contents. The user which this resource will be shared may not be able to access all its contents. 
    #[serde(rename = "share_option", skip_serializing_if = "Option::is_none")]
    pub share_option: Option<Box<crate::models::SharingOptions>>,
    /// A collection of encoded IDs (or email addresses) of users that this resource will be shared with.
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<crate::models::UserIdentifiersInner>,
}

impl ShareWithPayload {
    pub fn new(user_ids: Vec<crate::models::UserIdentifiersInner>) -> ShareWithPayload {
        ShareWithPayload {
            share_option: None,
            user_ids,
        }
    }
}


