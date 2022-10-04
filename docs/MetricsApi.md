# \MetricsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_metrics_post**](MetricsApi.md#create_api_metrics_post) | **POST** /api/metrics | Records a collection of metrics.
[**create_api_metrics_post_0**](MetricsApi.md#create_api_metrics_post_0) | **POST** /api/metrics | Records a collection of metrics.



## create_api_metrics_post

> serde_json::Value create_api_metrics_post(create_metrics_payload, run_as)
Records a collection of metrics.

Record any metrics sent and return some status object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_metrics_payload** | [**CreateMetricsPayload**](CreateMetricsPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_metrics_post_0

> serde_json::Value create_api_metrics_post_0(create_metrics_payload, run_as)
Records a collection of metrics.

Record any metrics sent and return some status object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_metrics_payload** | [**CreateMetricsPayload**](CreateMetricsPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

