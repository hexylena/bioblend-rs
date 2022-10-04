# ResponseSetPermissionsApiLibrariesIdPermissionsPost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | **String** | The time and date this item was created. | 
**deleted** | **bool** | Whether this Library has been deleted. | 
**description** | Option<**String**> | A detailed description of the Library. | [optional][default to ]
**id** | **String** | Encoded ID of the Library. | 
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**name** | **String** | The name of the Library. | 
**root_folder_id** | **String** | Encoded ID of the Library's base folder. | 
**synopsis** | Option<**String**> | A short text describing the contents of the Library. | [optional]
**access_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which have access to the Library. | 
**add_library_item_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can add items to the Library. | 
**manage_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can manage the Library. | 
**modify_library_role_list** | [**Vec<Vec<String>>**](array.md) | A list containing pairs of role names and corresponding encoded IDs which can modify the Library. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


