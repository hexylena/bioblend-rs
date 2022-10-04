# WriteStoreToPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | Include file contents for deleted datasets (if include_files is True). | [optional][default to false]
**include_files** | Option<**bool**> | include materialized files in export when available | [optional][default to true]
**include_hidden** | Option<**bool**> | Include file contents for hidden datasets (if include_files is True). | [optional][default to false]
**model_store_format** | Option<[**crate::models::ModelStoreFormat**](ModelStoreFormat.md)> | format of model store to export | [optional]
**target_uri** | **String** | Galaxy Files URI to write mode store content to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


