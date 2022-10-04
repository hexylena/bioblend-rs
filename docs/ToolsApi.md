# \ToolsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_json_api_tools_fetch_post_plus_fetch_form_api_tools_fetch_post**](ToolsApi.md#fetch_json_api_tools_fetch_post_plus_fetch_form_api_tools_fetch_post) | **POST** /api/tools/fetch | Upload files to Galaxy



## fetch_json_api_tools_fetch_post_plus_fetch_form_api_tools_fetch_post

> serde_json::Value fetch_json_api_tools_fetch_post_plus_fetch_form_api_tools_fetch_post(fetch_data_payload, run_as)
Upload files to Galaxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fetch_data_payload** | [**FetchDataPayload**](FetchDataPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

