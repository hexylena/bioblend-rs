# CreateNewCollectionPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**copy_elements** | Option<**bool**> | Whether to create a copy of the source HDAs for the new collection. | [optional][default to false]
**element_identifiers** | Option<[**Vec<crate::models::CollectionElementIdentifier>**](CollectionElementIdentifier.md)> | List of elements that should be in the new collection. | [optional]
**folder_id** | Option<**String**> | The ID of the history that will contain the collection. Required if `instance_type=library`. | [optional]
**hide_source_items** | Option<**bool**> | Whether to mark the original HDAs as hidden. | [optional][default to false]
**history_id** | Option<**String**> | The ID of the history that will contain the collection. Required if `instance_type=history`. | [optional]
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](DatasetCollectionInstanceType.md)> | The type of the instance, either `history` (default) or `library`. | [optional]
**name** | Option<**String**> | The name of the new collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


