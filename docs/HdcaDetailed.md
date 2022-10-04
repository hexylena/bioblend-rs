# HdcaDetailed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_id** | **String** | The encoded ID of the dataset collection associated with this HDCA. | 
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**contents_url** | **String** | The relative URL to access the contents of this History. | 
**create_time** | Option<**String**> | The time and date this item was created. | [optional]
**deleted** | **bool** | Whether this item is marked as deleted. | 
**element_count** | Option<**i32**> | The number of elements contained in the dataset collection. It may be None or undefined if the collection could not be populated. | [optional]
**elements** | Option<[**Vec<crate::models::DceSummary>**](DCESummary.md)> | The summary information of each of the elements inside the dataset collection. | [optional][default to []]
**elements_datatypes** | **Vec<String>** | A set containing all the different element datatypes in the collection. | 
**hid** | **i32** | The index position of this item in the History. | 
**history_content_type** | [**crate::models::HistoryContentType**](HistoryContentType.md) | The type of this item. | 
**history_id** | **String** | The encoded ID of the history associated with this item. | 
**id** | **String** | The encoded ID of this entity. | 
**job_source_id** | Option<**String**> | The encoded ID of the Job that produced this dataset collection. Used to track the state of the job. | [optional]
**job_source_type** | Option<[**crate::models::JobSourceType**](JobSourceType.md)> | The type of job (model class) that produced this dataset collection. Used to track the state of the job. | [optional]
**job_state_summary** | Option<[**crate::models::JobStateSummary**](Job_State_Summary.md)> |  | [optional]
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**name** | Option<**String**> | The name of the item. | [optional]
**populated** | **bool** | Whether the dataset collection elements (and any subcollections elements) were successfully populated. | 
**populated_state** | [**crate::models::PopulatedStates**](populated_states.md) | Indicates the general state of the elements in the dataset collection:- 'new': new dataset collection, unpopulated elements.- 'ok': collection elements populated (HDAs may or may not have errors).- 'failed': some problem populating, won't be populated. | 
**populated_state_message** | Option<**String**> | Optional message with further information in case the population of the dataset collection failed. | [optional]
**tags** | **Vec<String>** | The collection of tags associated with an item. | 
**r#type** | Option<**String**> | This is always `collection` for dataset collections. | [optional]
**type_id** | Option<**String**> | The type and the encoded ID of this item. Used for caching. | [optional]
**update_time** | Option<**String**> | The last time and date this item was updated. | [optional]
**url** | **String** | The relative URL to access this item. | 
**visible** | **bool** | Whether this item is visible or hidden to the user by default. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


