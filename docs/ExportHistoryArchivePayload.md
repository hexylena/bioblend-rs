# ExportHistoryArchivePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**directory_uri** | Option<**String**> | A writable directory destination where the history will be exported using the `galaxy.files` URI infrastructure. | [optional]
**file_name** | Option<**String**> | The name of the file containing the exported history. | [optional]
**force** | Option<**bool**> | Whether to force a rebuild of the history archive. | [optional]
**gzip** | Option<**bool**> | Whether to export as gzip archive. | [optional][default to true]
**include_deleted** | Option<**bool**> | Whether to include deleted datasets in the exported archive. | [optional][default to false]
**include_hidden** | Option<**bool**> | Whether to include hidden datasets in the exported archive. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


