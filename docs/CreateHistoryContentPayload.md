# CreateHistoryContentPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**content** | Option<**String**> | Depending on the `source` it can be: - The encoded id from the library dataset - The encoded id from the library folder - The encoded id from the HDA - The encoded id from the HDCA  | [optional]
**copy_elements** | Option<**bool**> | If the source is a collection, whether to copy child HDAs into the target history as well, defaults to False but this is less than ideal and may be changed in future releases. | [optional][default to false]
**dbkey** | Option<**String**> | TODO | [optional]
**element_identifiers** | Option<[**Vec<crate::models::CollectionElementIdentifier>**](CollectionElementIdentifier.md)> | List of elements that should be in the new collection. | [optional]
**folder_id** | Option<**String**> | The ID of the history that will contain the collection. Required if `instance_type=library`. | [optional]
**hide_source_items** | Option<**bool**> | Whether to mark the original HDAs as hidden. | [optional][default to false]
**history_id** | Option<**String**> | The ID of the history that will contain the collection. Required if `instance_type=history`. | [optional]
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](DatasetCollectionInstanceType.md)> | The type of the instance, either `history` (default) or `library`. | [optional]
**name** | Option<**String**> | The name of the new collection. | [optional]
**source** | Option<[**crate::models::HistoryContentSource**](HistoryContentSource.md)> | The source of the content. Can be other history element to be copied or library elements. | [optional]
**r#type** | Option<[**crate::models::HistoryContentType**](HistoryContentType.md)> | The type of content to be created in the history. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


