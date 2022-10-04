# ShareWithStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<**Vec<String>**> | Collection of messages indicating that the resource was not shared with some (or all users) due to an error. | [optional][default to []]
**extra** | Option<[**crate::models::Extra**](Extra.md)> |  | [optional]
**id** | **String** | The encoded ID of the resource to be shared. | 
**importable** | **bool** | Whether this resource can be published using a link. | 
**published** | **bool** | Whether this resource is currently published. | 
**title** | **String** | The title or name of the resource. | 
**username_and_slug** | Option<**String**> | The relative URL in the form of /u/{username}/{resource_single_char}/{slug} | [optional]
**users_shared_with** | Option<[**Vec<crate::models::UserEmail>**](UserEmail.md)> | The list of encoded ids for users the resource has been shared. | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


