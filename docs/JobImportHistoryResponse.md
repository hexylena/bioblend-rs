# JobImportHistoryResponse

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
**state** | [**crate::models::GalaxyModelJobStates**](galaxy__model__Job__states.md) | Current state of the job. | 
**tool_id** | **String** | Identifier of the tool that generated this job. | 
**update_time** | **String** | The last time and date this item was updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


