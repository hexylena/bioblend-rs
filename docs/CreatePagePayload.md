# CreatePagePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**annotation** | Option<**String**> | Annotation that will be attached to the page. | [optional]
**content** | Option<**String**> | Raw text contents of the first page revision (type dependent on content_format). | [optional][default to ]
**content_format** | Option<[**crate::models::PageContentFormat**](PageContentFormat.md)> | Either `markdown` or `html`. | [optional]
**invocation_id** | Option<**String**> | Encoded ID used by workflow generated reports. | [optional]
**slug** | **String** | The title slug for the page URL, must be unique. | 
**title** | **String** | The name of the page | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


