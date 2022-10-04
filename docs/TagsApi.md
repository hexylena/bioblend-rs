# \TagsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_api_tags_put**](TagsApi.md#update_api_tags_put) | **PUT** /api/tags | Apply a new set of tags to an item.
[**update_api_tags_put_0**](TagsApi.md#update_api_tags_put_0) | **PUT** /api/tags | Apply a new set of tags to an item.



## update_api_tags_put

> update_api_tags_put(payload1, run_as)
Apply a new set of tags to an item.

Replaces the tags associated with an item with the new ones specified in the payload.  - The previous tags will be __deleted__. - If no tags are provided in the request body, the currently associated tags will also be __deleted__.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload1** | [**Payload1**](Payload1.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_tags_put_0

> update_api_tags_put_0(payload1, run_as)
Apply a new set of tags to an item.

Replaces the tags associated with an item with the new ones specified in the payload.  - The previous tags will be __deleted__. - If no tags are provided in the request body, the currently associated tags will also be __deleted__.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload1** | [**Payload1**](Payload1.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

