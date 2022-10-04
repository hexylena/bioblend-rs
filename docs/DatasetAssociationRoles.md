# DatasetAssociationRoles

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_dataset_roles** | Option<[**Vec<Vec<String>>**](array.md)> | A list of roles that can access the dataset. The user has to **have all these roles** in order to access this dataset. Users without access permission **cannot** have other permissions on this dataset. If there are no access roles set on the dataset it is considered **unrestricted**. | [optional][default to []]
**manage_dataset_roles** | Option<[**Vec<Vec<String>>**](array.md)> | A list of roles that can manage permissions on the dataset. Users with **any** of these roles can manage permissions of this dataset. If you remove yourself you will lose the ability to manage this dataset unless you are an admin. | [optional][default to []]
**modify_item_roles** | Option<[**Vec<Vec<String>>**](array.md)> | A list of roles that can modify the library item. This is a library related permission. User with **any** of these roles can modify name, metadata, and other information about this library item. | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


