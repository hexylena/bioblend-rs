# DatasetStorageDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dataset_state** | **String** | The model state of the supplied dataset instance. | 
**description** | Option<**String**> | A description of how this dataset is stored. | [optional]
**hashes** | [**Vec<serde_json::Value>**](serde_json::Value.md) | The file contents hashes associated with the supplied dataset instance. | 
**name** | Option<**String**> | The display name of the destination ObjectStore for this dataset. | [optional]
**object_store_id** | Option<**String**> | The identifier of the destination ObjectStore for this dataset. | [optional]
**percent_used** | Option<**f32**> | The percentage indicating how full the store is. | [optional]
**sources** | [**Vec<serde_json::Value>**](serde_json::Value.md) | The file sources associated with the supplied dataset instance. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


