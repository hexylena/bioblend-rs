# HdaSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | Option<**String**> | The time and date this item was created. | [optional]
**dataset_id** | **String** | The encoded ID of the dataset associated with this item. | 
**deleted** | **bool** | Whether this item is marked as deleted. | 
**extension** | **String** | The extension of the dataset. | 
**hid** | **i32** | The index position of this item in the History. | 
**history_content_type** | [**crate::models::HistoryContentType**](HistoryContentType.md) | The type of this item. | 
**history_id** | **String** | The encoded ID of the history associated with this item. | 
**id** | **String** | The encoded ID of this entity. | 
**name** | Option<**String**> | The name of the item. | [optional]
**purged** | **bool** | Whether this dataset has been removed from disk. | 
**state** | [**crate::models::GalaxyModelDatasetStates**](galaxy__model__Dataset__states.md) | The current state of this dataset. | 
**tags** | **Vec<String>** | The collection of tags associated with an item. | 
**r#type** | **String** | The type of this item. | 
**type_id** | Option<**String**> | The type and the encoded ID of this item. Used for caching. | [optional]
**update_time** | Option<**String**> | The last time and date this item was updated. | [optional]
**url** | **String** | The relative URL to access this item. | 
**visible** | **bool** | Whether this item is visible or hidden to the user by default. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


