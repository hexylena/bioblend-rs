# DeleteDatasetBatchPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**datasets** | [**Vec<crate::models::DatasetSourceId>**](DatasetSourceId.md) | The list of datasets IDs with their sources to be deleted/purged. | 
**purge** | Option<**bool**> | Whether to permanently delete from disk the specified datasets. *Warning*: this is a destructive operation. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


