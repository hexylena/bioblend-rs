# ResponseGetPermissionsApiLibrariesIdPermissionsGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which have access to the Library. | 
**add_library_item_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can add items to the Library. | 
**manage_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can manage the Library. | 
**modify_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can modify the Library. | 
**page** | **i32** | Current page . | 
**page_limit** | **i32** | Maximum number of items per page. | 
**roles** | [**Vec<crate::models::BasicRoleModel>**](BasicRoleModel.md) | A list available roles that can be assigned to a particular permission. | 
**total** | **i32** | Total number of items | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


