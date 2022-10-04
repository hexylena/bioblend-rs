# \PagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_pages_post**](PagesApi.md#create_api_pages_post) | **POST** /api/pages | Create a page and return summary information.
[**create_api_pages_post_0**](PagesApi.md#create_api_pages_post_0) | **POST** /api/pages | Create a page and return summary information.
[**delete_api_pages_id_delete**](PagesApi.md#delete_api_pages_id_delete) | **DELETE** /api/pages/{id} | Marks the specific Page as deleted.
[**delete_api_pages_id_delete_0**](PagesApi.md#delete_api_pages_id_delete_0) | **DELETE** /api/pages/{id} | Marks the specific Page as deleted.
[**disable_link_access_api_pages_id_disable_link_access_put**](PagesApi.md#disable_link_access_api_pages_id_disable_link_access_put) | **PUT** /api/pages/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**disable_link_access_api_pages_id_disable_link_access_put_0**](PagesApi.md#disable_link_access_api_pages_id_disable_link_access_put_0) | **PUT** /api/pages/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**enable_link_access_api_pages_id_enable_link_access_put**](PagesApi.md#enable_link_access_api_pages_id_enable_link_access_put) | **PUT** /api/pages/{id}/enable_link_access | Makes this item accessible by a URL link.
[**enable_link_access_api_pages_id_enable_link_access_put_0**](PagesApi.md#enable_link_access_api_pages_id_enable_link_access_put_0) | **PUT** /api/pages/{id}/enable_link_access | Makes this item accessible by a URL link.
[**index_api_pages_get**](PagesApi.md#index_api_pages_get) | **GET** /api/pages | Lists all Pages viewable by the user.
[**index_api_pages_get_0**](PagesApi.md#index_api_pages_get_0) | **GET** /api/pages | Lists all Pages viewable by the user.
[**prepare_pdf_api_pages_id_prepare_download_post**](PagesApi.md#prepare_pdf_api_pages_id_prepare_download_post) | **POST** /api/pages/{id}/prepare_download | Return a PDF document of the last revision of the Page.
[**prepare_pdf_api_pages_id_prepare_download_post_0**](PagesApi.md#prepare_pdf_api_pages_id_prepare_download_post_0) | **POST** /api/pages/{id}/prepare_download | Return a PDF document of the last revision of the Page.
[**publish_api_pages_id_publish_put**](PagesApi.md#publish_api_pages_id_publish_put) | **PUT** /api/pages/{id}/publish | Makes this item public and accessible by a URL link.
[**publish_api_pages_id_publish_put_0**](PagesApi.md#publish_api_pages_id_publish_put_0) | **PUT** /api/pages/{id}/publish | Makes this item public and accessible by a URL link.
[**set_slug_api_pages_id_slug_put**](PagesApi.md#set_slug_api_pages_id_slug_put) | **PUT** /api/pages/{id}/slug | Set a new slug for this shared item.
[**set_slug_api_pages_id_slug_put_0**](PagesApi.md#set_slug_api_pages_id_slug_put_0) | **PUT** /api/pages/{id}/slug | Set a new slug for this shared item.
[**share_with_users_api_pages_id_share_with_users_put**](PagesApi.md#share_with_users_api_pages_id_share_with_users_put) | **PUT** /api/pages/{id}/share_with_users | Share this item with specific users.
[**share_with_users_api_pages_id_share_with_users_put_0**](PagesApi.md#share_with_users_api_pages_id_share_with_users_put_0) | **PUT** /api/pages/{id}/share_with_users | Share this item with specific users.
[**sharing_api_pages_id_sharing_get**](PagesApi.md#sharing_api_pages_id_sharing_get) | **GET** /api/pages/{id}/sharing | Get the current sharing status of the given Page.
[**sharing_api_pages_id_sharing_get_0**](PagesApi.md#sharing_api_pages_id_sharing_get_0) | **GET** /api/pages/{id}/sharing | Get the current sharing status of the given Page.
[**show_api_pages_id_get**](PagesApi.md#show_api_pages_id_get) | **GET** /api/pages/{id} | Return a page summary and the content of the last revision.
[**show_api_pages_id_get_0**](PagesApi.md#show_api_pages_id_get_0) | **GET** /api/pages/{id} | Return a page summary and the content of the last revision.
[**show_pdf_api_pages_id_pdf_get**](PagesApi.md#show_pdf_api_pages_id_pdf_get) | **GET** /api/pages/{id}.pdf | Return a PDF document of the last revision of the Page.
[**show_pdf_api_pages_id_pdf_get_0**](PagesApi.md#show_pdf_api_pages_id_pdf_get_0) | **GET** /api/pages/{id}.pdf | Return a PDF document of the last revision of the Page.
[**unpublish_api_pages_id_unpublish_put**](PagesApi.md#unpublish_api_pages_id_unpublish_put) | **PUT** /api/pages/{id}/unpublish | Removes this item from the published list.
[**unpublish_api_pages_id_unpublish_put_0**](PagesApi.md#unpublish_api_pages_id_unpublish_put_0) | **PUT** /api/pages/{id}/unpublish | Removes this item from the published list.



## create_api_pages_post

> crate::models::PageSummary create_api_pages_post(create_page_payload, run_as)
Create a page and return summary information.

Get a list with details of all Pages available to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_page_payload** | [**CreatePagePayload**](CreatePagePayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::PageSummary**](PageSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_pages_post_0

> crate::models::PageSummary create_api_pages_post_0(create_page_payload, run_as)
Create a page and return summary information.

Get a list with details of all Pages available to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_page_payload** | [**CreatePagePayload**](CreatePagePayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::PageSummary**](PageSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_pages_id_delete

> delete_api_pages_id_delete(id, run_as)
Marks the specific Page as deleted.

Marks the Page with the given ID as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_pages_id_delete_0

> delete_api_pages_id_delete_0(id, run_as)
Marks the specific Page as deleted.

Marks the Page with the given ID as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_pages_id_disable_link_access_put

> crate::models::SharingStatus disable_link_access_api_pages_id_disable_link_access_put(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_pages_id_disable_link_access_put_0

> crate::models::SharingStatus disable_link_access_api_pages_id_disable_link_access_put_0(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_pages_id_enable_link_access_put

> crate::models::SharingStatus enable_link_access_api_pages_id_enable_link_access_put(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_pages_id_enable_link_access_put_0

> crate::models::SharingStatus enable_link_access_api_pages_id_enable_link_access_put_0(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_pages_get

> Vec<crate::models::PageSummary> index_api_pages_get(deleted, user_id, show_published, show_shared, sort_by, sort_desc, limit, offset, run_as)
Lists all Pages viewable by the user.

Get a list with summary information of all Pages available to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Whether to include deleted pages in the result. |  |[default to false]
**user_id** | Option<**String**> |  |  |
**show_published** | Option<**bool**> |  |  |[default to true]
**show_shared** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<[**crate::models::PageSortByEnum**](.md)> | Sort page index by this specified attribute on the page model |  |
**sort_desc** | Option<**bool**> | Sort in descending order? |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::PageSummary>**](PageSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_pages_get_0

> Vec<crate::models::PageSummary> index_api_pages_get_0(deleted, user_id, show_published, show_shared, sort_by, sort_desc, limit, offset, run_as)
Lists all Pages viewable by the user.

Get a list with summary information of all Pages available to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Whether to include deleted pages in the result. |  |[default to false]
**user_id** | Option<**String**> |  |  |
**show_published** | Option<**bool**> |  |  |[default to true]
**show_shared** | Option<**bool**> |  |  |[default to false]
**sort_by** | Option<[**crate::models::PageSortByEnum**](.md)> | Sort page index by this specified attribute on the page model |  |
**sort_desc** | Option<**bool**> | Sort in descending order? |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::PageSummary>**](PageSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_pdf_api_pages_id_prepare_download_post

> crate::models::AsyncFile prepare_pdf_api_pages_id_prepare_download_post(id, run_as)
Return a PDF document of the last revision of the Page.

Return a STS download link for this page to be downloaded as a PDF.  This feature may not be available in this Galaxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_pdf_api_pages_id_prepare_download_post_0

> crate::models::AsyncFile prepare_pdf_api_pages_id_prepare_download_post_0(id, run_as)
Return a PDF document of the last revision of the Page.

Return a STS download link for this page to be downloaded as a PDF.  This feature may not be available in this Galaxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_pages_id_publish_put

> crate::models::SharingStatus publish_api_pages_id_publish_put(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_pages_id_publish_put_0

> crate::models::SharingStatus publish_api_pages_id_publish_put_0(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_pages_id_slug_put

> set_slug_api_pages_id_slug_put(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
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


## set_slug_api_pages_id_slug_put_0

> set_slug_api_pages_id_slug_put_0(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
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


## share_with_users_api_pages_id_share_with_users_put

> crate::models::ShareWithStatus share_with_users_api_pages_id_share_with_users_put(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
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


## share_with_users_api_pages_id_share_with_users_put_0

> crate::models::ShareWithStatus share_with_users_api_pages_id_share_with_users_put_0(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
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


## sharing_api_pages_id_sharing_get

> crate::models::SharingStatus sharing_api_pages_id_sharing_get(id, run_as)
Get the current sharing status of the given Page.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_pages_id_sharing_get_0

> crate::models::SharingStatus sharing_api_pages_id_sharing_get_0(id, run_as)
Get the current sharing status of the given Page.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_pages_id_get

> crate::models::PageDetails show_api_pages_id_get(id, run_as)
Return a page summary and the content of the last revision.

Return summary information about a specific Page and the content of the last revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::PageDetails**](PageDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_pages_id_get_0

> crate::models::PageDetails show_api_pages_id_get_0(id, run_as)
Return a page summary and the content of the last revision.

Return summary information about a specific Page and the content of the last revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::PageDetails**](PageDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_pdf_api_pages_id_pdf_get

> show_pdf_api_pages_id_pdf_get(id, run_as)
Return a PDF document of the last revision of the Page.

Return a PDF document of the last revision of the Page.  This feature may not be available in this Galaxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_pdf_api_pages_id_pdf_get_0

> show_pdf_api_pages_id_pdf_get_0(id, run_as)
Return a PDF document of the last revision of the Page.

Return a PDF document of the last revision of the Page.  This feature may not be available in this Galaxy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_pages_id_unpublish_put

> crate::models::SharingStatus unpublish_api_pages_id_unpublish_put(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_pages_id_unpublish_put_0

> crate::models::SharingStatus unpublish_api_pages_id_unpublish_put_0(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Page. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

