# DcObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**contents_url** | Option<**String**> | The relative URL to access the contents of this History. | [optional]
**element_count** | Option<**i32**> | The number of elements contained in the dataset collection. It may be None or undefined if the collection could not be populated. | [optional]
**elements** | Option<[**Vec<crate::models::DceSummary>**](DCESummary.md)> | The summary information of each of the elements inside the dataset collection. | [optional][default to []]
**id** | **String** | The encoded ID of this entity. | 
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**populated** | Option<**bool**> | Whether the dataset collection elements (and any subcollections elements) were successfully populated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


