# \QuotasApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_quotas_post**](QuotasApi.md#create_api_quotas_post) | **POST** /api/quotas | Creates a new quota.
[**create_api_quotas_post_0**](QuotasApi.md#create_api_quotas_post_0) | **POST** /api/quotas | Creates a new quota.
[**delete_api_quotas_id_delete**](QuotasApi.md#delete_api_quotas_id_delete) | **DELETE** /api/quotas/{id} | Deletes an existing quota.
[**delete_api_quotas_id_delete_0**](QuotasApi.md#delete_api_quotas_id_delete_0) | **DELETE** /api/quotas/{id} | Deletes an existing quota.
[**index_api_quotas_get**](QuotasApi.md#index_api_quotas_get) | **GET** /api/quotas | Displays a list with information of quotas that are currently active.
[**index_api_quotas_get_0**](QuotasApi.md#index_api_quotas_get_0) | **GET** /api/quotas | Displays a list with information of quotas that are currently active.
[**index_deleted_api_quotas_deleted_get**](QuotasApi.md#index_deleted_api_quotas_deleted_get) | **GET** /api/quotas/deleted | Displays a list with information of quotas that have been deleted.
[**index_deleted_api_quotas_deleted_get_0**](QuotasApi.md#index_deleted_api_quotas_deleted_get_0) | **GET** /api/quotas/deleted | Displays a list with information of quotas that have been deleted.
[**show_api_quotas_id_get**](QuotasApi.md#show_api_quotas_id_get) | **GET** /api/quotas/{id} | Displays details on a particular active quota.
[**show_api_quotas_id_get_0**](QuotasApi.md#show_api_quotas_id_get_0) | **GET** /api/quotas/{id} | Displays details on a particular active quota.
[**show_deleted_api_quotas_deleted_id_get**](QuotasApi.md#show_deleted_api_quotas_deleted_id_get) | **GET** /api/quotas/deleted/{id} | Displays details on a particular quota that has been deleted.
[**show_deleted_api_quotas_deleted_id_get_0**](QuotasApi.md#show_deleted_api_quotas_deleted_id_get_0) | **GET** /api/quotas/deleted/{id} | Displays details on a particular quota that has been deleted.
[**undelete_api_quotas_deleted_id_undelete_post**](QuotasApi.md#undelete_api_quotas_deleted_id_undelete_post) | **POST** /api/quotas/deleted/{id}/undelete | Restores a previously deleted quota.
[**undelete_api_quotas_deleted_id_undelete_post_0**](QuotasApi.md#undelete_api_quotas_deleted_id_undelete_post_0) | **POST** /api/quotas/deleted/{id}/undelete | Restores a previously deleted quota.
[**update_api_quotas_id_put**](QuotasApi.md#update_api_quotas_id_put) | **PUT** /api/quotas/{id} | Updates an existing quota.
[**update_api_quotas_id_put_0**](QuotasApi.md#update_api_quotas_id_put_0) | **PUT** /api/quotas/{id} | Updates an existing quota.



## create_api_quotas_post

> crate::models::CreateQuotaResult create_api_quotas_post(create_quota_params, run_as)
Creates a new quota.

Creates a new quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_quota_params** | [**CreateQuotaParams**](CreateQuotaParams.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::CreateQuotaResult**](CreateQuotaResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_quotas_post_0

> crate::models::CreateQuotaResult create_api_quotas_post_0(create_quota_params, run_as)
Creates a new quota.

Creates a new quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_quota_params** | [**CreateQuotaParams**](CreateQuotaParams.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::CreateQuotaResult**](CreateQuotaResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_quotas_id_delete

> String delete_api_quotas_id_delete(id, run_as, delete_quota_payload)
Deletes an existing quota.

Deletes an existing quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_quota_payload** | Option<[**DeleteQuotaPayload**](DeleteQuotaPayload.md)> |  |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_quotas_id_delete_0

> String delete_api_quotas_id_delete_0(id, run_as, delete_quota_payload)
Deletes an existing quota.

Deletes an existing quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_quota_payload** | Option<[**DeleteQuotaPayload**](DeleteQuotaPayload.md)> |  |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_quotas_get

> Vec<crate::models::QuotaSummary> index_api_quotas_get(run_as)
Displays a list with information of quotas that are currently active.

Displays a list with information of quotas that are currently active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::QuotaSummary>**](QuotaSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_quotas_get_0

> Vec<crate::models::QuotaSummary> index_api_quotas_get_0(run_as)
Displays a list with information of quotas that are currently active.

Displays a list with information of quotas that are currently active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::QuotaSummary>**](QuotaSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_quotas_deleted_get

> Vec<crate::models::QuotaSummary> index_deleted_api_quotas_deleted_get(run_as)
Displays a list with information of quotas that have been deleted.

Displays a list with information of quotas that have been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::QuotaSummary>**](QuotaSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_quotas_deleted_get_0

> Vec<crate::models::QuotaSummary> index_deleted_api_quotas_deleted_get_0(run_as)
Displays a list with information of quotas that have been deleted.

Displays a list with information of quotas that have been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::QuotaSummary>**](QuotaSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_quotas_id_get

> crate::models::QuotaDetails show_api_quotas_id_get(id, run_as)
Displays details on a particular active quota.

Displays details on a particular active quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::QuotaDetails**](QuotaDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_quotas_id_get_0

> crate::models::QuotaDetails show_api_quotas_id_get_0(id, run_as)
Displays details on a particular active quota.

Displays details on a particular active quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::QuotaDetails**](QuotaDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_deleted_api_quotas_deleted_id_get

> crate::models::QuotaDetails show_deleted_api_quotas_deleted_id_get(id, run_as)
Displays details on a particular quota that has been deleted.

Displays details on a particular quota that has been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::QuotaDetails**](QuotaDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_deleted_api_quotas_deleted_id_get_0

> crate::models::QuotaDetails show_deleted_api_quotas_deleted_id_get_0(id, run_as)
Displays details on a particular quota that has been deleted.

Displays details on a particular quota that has been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::QuotaDetails**](QuotaDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_api_quotas_deleted_id_undelete_post

> String undelete_api_quotas_deleted_id_undelete_post(id, run_as)
Restores a previously deleted quota.

Restores a previously deleted quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_api_quotas_deleted_id_undelete_post_0

> String undelete_api_quotas_deleted_id_undelete_post_0(id, run_as)
Restores a previously deleted quota.

Restores a previously deleted quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_quotas_id_put

> String update_api_quotas_id_put(id, update_quota_params, run_as)
Updates an existing quota.

Updates an existing quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**update_quota_params** | [**UpdateQuotaParams**](UpdateQuotaParams.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_quotas_id_put_0

> String update_api_quotas_id_put_0(id, update_quota_params, run_as)
Updates an existing quota.

Updates an existing quota.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Quota. | [required] |
**update_quota_params** | [**UpdateQuotaParams**](UpdateQuotaParams.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**String**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

