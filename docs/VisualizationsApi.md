# \VisualizationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_link_access_api_visualizations_id_disable_link_access_put**](VisualizationsApi.md#disable_link_access_api_visualizations_id_disable_link_access_put) | **PUT** /api/visualizations/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**disable_link_access_api_visualizations_id_disable_link_access_put_0**](VisualizationsApi.md#disable_link_access_api_visualizations_id_disable_link_access_put_0) | **PUT** /api/visualizations/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**enable_link_access_api_visualizations_id_enable_link_access_put**](VisualizationsApi.md#enable_link_access_api_visualizations_id_enable_link_access_put) | **PUT** /api/visualizations/{id}/enable_link_access | Makes this item accessible by a URL link.
[**enable_link_access_api_visualizations_id_enable_link_access_put_0**](VisualizationsApi.md#enable_link_access_api_visualizations_id_enable_link_access_put_0) | **PUT** /api/visualizations/{id}/enable_link_access | Makes this item accessible by a URL link.
[**publish_api_visualizations_id_publish_put**](VisualizationsApi.md#publish_api_visualizations_id_publish_put) | **PUT** /api/visualizations/{id}/publish | Makes this item public and accessible by a URL link.
[**publish_api_visualizations_id_publish_put_0**](VisualizationsApi.md#publish_api_visualizations_id_publish_put_0) | **PUT** /api/visualizations/{id}/publish | Makes this item public and accessible by a URL link.
[**set_slug_api_visualizations_id_slug_put**](VisualizationsApi.md#set_slug_api_visualizations_id_slug_put) | **PUT** /api/visualizations/{id}/slug | Set a new slug for this shared item.
[**set_slug_api_visualizations_id_slug_put_0**](VisualizationsApi.md#set_slug_api_visualizations_id_slug_put_0) | **PUT** /api/visualizations/{id}/slug | Set a new slug for this shared item.
[**share_with_users_api_visualizations_id_share_with_users_put**](VisualizationsApi.md#share_with_users_api_visualizations_id_share_with_users_put) | **PUT** /api/visualizations/{id}/share_with_users | Share this item with specific users.
[**share_with_users_api_visualizations_id_share_with_users_put_0**](VisualizationsApi.md#share_with_users_api_visualizations_id_share_with_users_put_0) | **PUT** /api/visualizations/{id}/share_with_users | Share this item with specific users.
[**sharing_api_visualizations_id_sharing_get**](VisualizationsApi.md#sharing_api_visualizations_id_sharing_get) | **GET** /api/visualizations/{id}/sharing | Get the current sharing status of the given Page.
[**sharing_api_visualizations_id_sharing_get_0**](VisualizationsApi.md#sharing_api_visualizations_id_sharing_get_0) | **GET** /api/visualizations/{id}/sharing | Get the current sharing status of the given Page.
[**unpublish_api_visualizations_id_unpublish_put**](VisualizationsApi.md#unpublish_api_visualizations_id_unpublish_put) | **PUT** /api/visualizations/{id}/unpublish | Removes this item from the published list.
[**unpublish_api_visualizations_id_unpublish_put_0**](VisualizationsApi.md#unpublish_api_visualizations_id_unpublish_put_0) | **PUT** /api/visualizations/{id}/unpublish | Removes this item from the published list.



## disable_link_access_api_visualizations_id_disable_link_access_put

> crate::models::SharingStatus disable_link_access_api_visualizations_id_disable_link_access_put(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_visualizations_id_disable_link_access_put_0

> crate::models::SharingStatus disable_link_access_api_visualizations_id_disable_link_access_put_0(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_visualizations_id_enable_link_access_put

> crate::models::SharingStatus enable_link_access_api_visualizations_id_enable_link_access_put(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_visualizations_id_enable_link_access_put_0

> crate::models::SharingStatus enable_link_access_api_visualizations_id_enable_link_access_put_0(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_visualizations_id_publish_put

> crate::models::SharingStatus publish_api_visualizations_id_publish_put(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_visualizations_id_publish_put_0

> crate::models::SharingStatus publish_api_visualizations_id_publish_put_0(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_visualizations_id_slug_put

> set_slug_api_visualizations_id_slug_put(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**set_slug_payload** | [**SetSlugPayload**](SetSlugPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_visualizations_id_slug_put_0

> set_slug_api_visualizations_id_slug_put_0(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**set_slug_payload** | [**SetSlugPayload**](SetSlugPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_with_users_api_visualizations_id_share_with_users_put

> crate::models::ShareWithStatus share_with_users_api_visualizations_id_share_with_users_put(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**share_with_payload** | [**ShareWithPayload**](ShareWithPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ShareWithStatus**](ShareWithStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_with_users_api_visualizations_id_share_with_users_put_0

> crate::models::ShareWithStatus share_with_users_api_visualizations_id_share_with_users_put_0(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**share_with_payload** | [**ShareWithPayload**](ShareWithPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ShareWithStatus**](ShareWithStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_visualizations_id_sharing_get

> crate::models::SharingStatus sharing_api_visualizations_id_sharing_get(id, run_as)
Get the current sharing status of the given Page.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_visualizations_id_sharing_get_0

> crate::models::SharingStatus sharing_api_visualizations_id_sharing_get_0(id, run_as)
Get the current sharing status of the given Page.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_visualizations_id_unpublish_put

> crate::models::SharingStatus unpublish_api_visualizations_id_unpublish_put(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_visualizations_id_unpublish_put_0

> crate::models::SharingStatus unpublish_api_visualizations_id_unpublish_put_0(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Visualization. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

