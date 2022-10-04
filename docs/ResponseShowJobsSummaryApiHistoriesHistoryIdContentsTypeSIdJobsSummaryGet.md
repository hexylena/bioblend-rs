# ResponseShowJobsSummaryApiHistoriesHistoryIdContentsTypeSIdJobsSummaryGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The encoded ID of this entity. | 
**model** | Option<**String**> | The name of the database model class. | [optional]
**populated_state** | [**crate::models::PopulatedStates**](populated_states.md) | Indicates the general state of the elements in the dataset collection:- 'new': new dataset collection, unpopulated elements.- 'ok': collection elements populated (HDAs may or may not have errors).- 'failed': some problem populating, won't be populated. | 
**states** | Option<**::std::collections::HashMap<String, i32>**> | A dictionary of job states and the number of jobs in that state. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


