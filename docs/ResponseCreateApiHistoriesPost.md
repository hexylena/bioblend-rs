# ResponseCreateApiHistoriesPost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | **String** | The time and date this item was created. | 
**exit_code** | Option<**i32**> | The exit code returned by the tool. Can be unset if the job is not completed yet. | [optional]
**galaxy_version** | **String** | The (major) version of Galaxy used to create this job. | 
**history_id** | Option<**String**> | The encoded ID of the history associated with this item. | [optional]
**id** | **String** | The encoded ID of this entity. | 
**message** | **String** | Text message containing information about the history import. | 
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**state** | [**crate::models::GalaxyModelDatasetStates**](galaxy__model__Dataset__states.md) | The current state of the History based on the states of the datasets it contains. | 
**tool_id** | **String** | Identifier of the tool that generated this job. | 
**update_time** | **String** | The last time and date this item was updated. | 
**annotation** | **String** | An annotation to provide details or to help understand the purpose and usage of this item. | 
**contents_active** | [**crate::models::ActiveContents**](Active_Contents.md) |  | 
**contents_url** | **String** | The relative URL to access the contents of this History. | 
**deleted** | **bool** | Whether this item is marked as deleted. | 
**empty** | **bool** | Whether this History has any content. | 
**genome_build** | Option<**String**> | TODO | [optional][default to ?]
**hid_counter** | **i32** | TODO | 
**importable** | **bool** | Whether this History can be imported by other users with a shared link. | 
**name** | **String** | The name of the history. | 
**nice_size** | **String** | Human-readable size of the contents of this history. | 
**published** | **bool** | Whether this resource is currently publicly available to all users. | 
**purged** | **bool** | Whether this item has been permanently removed. | 
**size** | **i32** | The total size of the contents of this history in bytes. | 
**slug** | Option<**String**> | Part of the URL to uniquely identify this History by link in a readable way. | [optional]
**state_details** | **::std::collections::HashMap<String, i32>** | A dictionary keyed to possible dataset states and valued with the number of datasets in this history that have those states. | 
**state_ids** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | A dictionary keyed to possible dataset states and valued with lists containing the ids of each HDA in that state. | 
**tags** | **Vec<String>** | The collection of tags associated with an item. | 
**url** | **String** | The relative URL to access this item. | 
**user_id** | **String** | The encoded ID of the user that owns this History. | 
**username_and_slug** | Option<**String**> | The relative URL in the form of /u/{username}/h/{slug} | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


