# CollectionElementIdentifier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**element_identifiers** | Option<[**Vec<crate::models::CollectionElementIdentifier>**](CollectionElementIdentifier.md)> | List of elements that should be in the new sub-collection. | [optional]
**id** | Option<**String**> | The encoded ID of the element. | [optional]
**name** | Option<**String**> | The name of the element. | [optional]
**src** | [**crate::models::ColletionSourceType**](ColletionSourceType.md) | The source of the element. | 
**tags** | Option<**Vec<String>**> | The list of tags associated with the element. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


