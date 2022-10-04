# PageDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> | Raw text contents of the first page revision (type dependent on content_format). | [optional][default to ]
**content_format** | Option<[**crate::models::PageContentFormat**](PageContentFormat.md)> | Either `markdown` or `html`. | [optional]
**deleted** | **bool** | Whether this Page has been deleted. | 
**generate_time** | Option<**String**> | The date this page was generated. | [optional]
**generate_version** | Option<**String**> | The version of Galaxy this page was generated with. | [optional]
**id** | **String** | Encoded ID of the Page. | 
**importable** | **bool** | Whether this Page can be imported. | 
**latest_revision_id** | **String** | The encoded ID of the last revision of this Page. | 
**model_class** | **String** | The class of the model associated with the ID. | 
**published** | **bool** | Whether this Page has been published. | 
**revision_ids** | **Vec<String>** | The history with the encoded ID of each revision of the Page. | 
**slug** | **String** | The title slug for the page URL, must be unique. | 
**title** | **String** | The name of the page | 
**username** | **String** | The name of the user owning this Page. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


