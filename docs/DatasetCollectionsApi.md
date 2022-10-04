# \DatasetCollectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attributes_api_dataset_collections_id_attributes_get**](DatasetCollectionsApi.md#attributes_api_dataset_collections_id_attributes_get) | **GET** /api/dataset_collections/{id}/attributes | Returns `dbkey`/`extension` attributes for all the collection elements.
[**attributes_api_dataset_collections_id_attributes_get_0**](DatasetCollectionsApi.md#attributes_api_dataset_collections_id_attributes_get_0) | **GET** /api/dataset_collections/{id}/attributes | Returns `dbkey`/`extension` attributes for all the collection elements.
[**contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get**](DatasetCollectionsApi.md#contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get) | **GET** /api/dataset_collections/{hdca_id}/contents/{parent_id} | Returns direct child contents of indicated dataset collection parent ID.
[**contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get_0**](DatasetCollectionsApi.md#contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get_0) | **GET** /api/dataset_collections/{hdca_id}/contents/{parent_id} | Returns direct child contents of indicated dataset collection parent ID.
[**copy_api_dataset_collections_id_copy_post**](DatasetCollectionsApi.md#copy_api_dataset_collections_id_copy_post) | **POST** /api/dataset_collections/{id}/copy | Copy the given collection datasets to a new collection using a new `dbkey` attribute.
[**copy_api_dataset_collections_id_copy_post_0**](DatasetCollectionsApi.md#copy_api_dataset_collections_id_copy_post_0) | **POST** /api/dataset_collections/{id}/copy | Copy the given collection datasets to a new collection using a new `dbkey` attribute.
[**create_api_dataset_collections_post**](DatasetCollectionsApi.md#create_api_dataset_collections_post) | **POST** /api/dataset_collections | Create a new dataset collection instance.
[**create_api_dataset_collections_post_0**](DatasetCollectionsApi.md#create_api_dataset_collections_post_0) | **POST** /api/dataset_collections | Create a new dataset collection instance.
[**download_dataset_collection_api_dataset_collections_id_download_get**](DatasetCollectionsApi.md#download_dataset_collection_api_dataset_collections_id_download_get) | **GET** /api/dataset_collections/{id}/download | Download the content of a dataset collection as a `zip` archive.
[**prepare_collection_download_api_dataset_collections_id_prepare_download_post**](DatasetCollectionsApi.md#prepare_collection_download_api_dataset_collections_id_prepare_download_post) | **POST** /api/dataset_collections/{id}/prepare_download | Prepare an short term storage object that the collection will be downloaded to.
[**show_api_dataset_collections_id_get**](DatasetCollectionsApi.md#show_api_dataset_collections_id_get) | **GET** /api/dataset_collections/{id} | Returns detailed information about the given collection.
[**show_api_dataset_collections_id_get_0**](DatasetCollectionsApi.md#show_api_dataset_collections_id_get_0) | **GET** /api/dataset_collections/{id} | Returns detailed information about the given collection.
[**suitable_converters_api_dataset_collections_id_suitable_converters_get**](DatasetCollectionsApi.md#suitable_converters_api_dataset_collections_id_suitable_converters_get) | **GET** /api/dataset_collections/{id}/suitable_converters | Returns a list of applicable converters for all datatypes in the given collection.
[**suitable_converters_api_dataset_collections_id_suitable_converters_get_0**](DatasetCollectionsApi.md#suitable_converters_api_dataset_collections_id_suitable_converters_get_0) | **GET** /api/dataset_collections/{id}/suitable_converters | Returns a list of applicable converters for all datatypes in the given collection.



## attributes_api_dataset_collections_id_attributes_get

> crate::models::DatasetCollectionAttributesResult attributes_api_dataset_collections_id_attributes_get(id, instance_type, run_as)
Returns `dbkey`/`extension` attributes for all the collection elements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::DatasetCollectionAttributesResult**](DatasetCollectionAttributesResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attributes_api_dataset_collections_id_attributes_get_0

> crate::models::DatasetCollectionAttributesResult attributes_api_dataset_collections_id_attributes_get_0(id, instance_type, run_as)
Returns `dbkey`/`extension` attributes for all the collection elements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::DatasetCollectionAttributesResult**](DatasetCollectionAttributesResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get

> Vec<crate::models::DceSummary> contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get(hdca_id, parent_id, instance_type, limit, offset, run_as)
Returns direct child contents of indicated dataset collection parent ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hdca_id** | **String** | The encoded identifier of the dataset collection. | [required] |
**parent_id** | **String** | Parent collection ID describing what collection the contents belongs to. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**limit** | Option<**i32**> | The maximum number of content elements to return. |  |
**offset** | Option<**i32**> | The number of content elements that will be skipped before returning. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::DceSummary>**](DCESummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get_0

> Vec<crate::models::DceSummary> contents_dataset_collection_api_dataset_collections_hdca_id_contents_parent_id_get_0(hdca_id, parent_id, instance_type, limit, offset, run_as)
Returns direct child contents of indicated dataset collection parent ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hdca_id** | **String** | The encoded identifier of the dataset collection. | [required] |
**parent_id** | **String** | Parent collection ID describing what collection the contents belongs to. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**limit** | Option<**i32**> | The maximum number of content elements to return. |  |
**offset** | Option<**i32**> | The number of content elements that will be skipped before returning. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::DceSummary>**](DCESummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_api_dataset_collections_id_copy_post

> serde_json::Value copy_api_dataset_collections_id_copy_post(id, update_collection_attribute_payload, run_as)
Copy the given collection datasets to a new collection using a new `dbkey` attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the dataset collection to copy. | [required] |
**update_collection_attribute_payload** | [**UpdateCollectionAttributePayload**](UpdateCollectionAttributePayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_api_dataset_collections_id_copy_post_0

> serde_json::Value copy_api_dataset_collections_id_copy_post_0(id, update_collection_attribute_payload, run_as)
Copy the given collection datasets to a new collection using a new `dbkey` attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the dataset collection to copy. | [required] |
**update_collection_attribute_payload** | [**UpdateCollectionAttributePayload**](UpdateCollectionAttributePayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_dataset_collections_post

> crate::models::HdcaDetailed create_api_dataset_collections_post(create_new_collection_payload, run_as)
Create a new dataset collection instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_new_collection_payload** | [**CreateNewCollectionPayload**](CreateNewCollectionPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HdcaDetailed**](HDCADetailed.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_dataset_collections_post_0

> crate::models::HdcaDetailed create_api_dataset_collections_post_0(create_new_collection_payload, run_as)
Create a new dataset collection instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_new_collection_payload** | [**CreateNewCollectionPayload**](CreateNewCollectionPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HdcaDetailed**](HDCADetailed.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_dataset_collection_api_dataset_collections_id_download_get

> download_dataset_collection_api_dataset_collections_id_download_get(id, history_id, run_as)
Download the content of a dataset collection as a `zip` archive.

Download the content of a history dataset collection as a `zip` archive while maintaining approximate collection structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**history_id** | Option<**String**> | The encoded database identifier of the History. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_collection_download_api_dataset_collections_id_prepare_download_post

> crate::models::AsyncFile prepare_collection_download_api_dataset_collections_id_prepare_download_post(history_id, id, run_as)
Prepare an short term storage object that the collection will be downloaded to.

The history dataset collection will be written as a `zip` archive to the returned short term storage object. Progress tracking this file's creation can be tracked with the short_term_storage API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_dataset_collections_id_get

> crate::models::ResponseShowApiDatasetCollectionsIdGet show_api_dataset_collections_id_get(id, instance_type, run_as)
Returns detailed information about the given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowApiDatasetCollectionsIdGet**](Response_Show_Api_Dataset_Collections__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_dataset_collections_id_get_0

> crate::models::ResponseShowApiDatasetCollectionsIdGet show_api_dataset_collections_id_get_0(id, instance_type, run_as)
Returns detailed information about the given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowApiDatasetCollectionsIdGet**](Response_Show_Api_Dataset_Collections__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suitable_converters_api_dataset_collections_id_suitable_converters_get

> Vec<crate::models::SuitableConverter> suitable_converters_api_dataset_collections_id_suitable_converters_get(id, instance_type, run_as)
Returns a list of applicable converters for all datatypes in the given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::SuitableConverter>**](SuitableConverter.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suitable_converters_api_dataset_collections_id_suitable_converters_get_0

> Vec<crate::models::SuitableConverter> suitable_converters_api_dataset_collections_id_suitable_converters_get_0(id, instance_type, run_as)
Returns a list of applicable converters for all datatypes in the given collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the dataset collection. | [required] |
**instance_type** | Option<[**crate::models::DatasetCollectionInstanceType**](.md)> | The type of collection instance. Either `history` (default) or `library`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::SuitableConverter>**](SuitableConverter.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

