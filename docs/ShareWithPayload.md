# ShareWithPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**share_option** | Option<[**crate::models::SharingOptions**](SharingOptions.md)> | User choice for sharing resources which its contents may be restricted:  - None: The user did not choose anything yet or no option is needed.  - make_public: The contents of the resource will be made publicly accessible.  - make_accessible_to_shared: This will automatically create a new `sharing role` allowing protected contents to be accessed only by the desired users.  - no_changes: This won't change the current permissions for the contents. The user which this resource will be shared may not be able to access all its contents.  | [optional]
**user_ids** | [**Vec<crate::models::UserIdentifiersInner>**](User_Identifiers_inner.md) | A collection of encoded IDs (or email addresses) of users that this resource will be shared with. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


