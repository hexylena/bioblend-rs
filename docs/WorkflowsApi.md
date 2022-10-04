# \WorkflowsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_link_access_api_workflows_id_disable_link_access_put**](WorkflowsApi.md#disable_link_access_api_workflows_id_disable_link_access_put) | **PUT** /api/workflows/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**disable_link_access_api_workflows_id_disable_link_access_put_0**](WorkflowsApi.md#disable_link_access_api_workflows_id_disable_link_access_put_0) | **PUT** /api/workflows/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**enable_link_access_api_workflows_id_enable_link_access_put**](WorkflowsApi.md#enable_link_access_api_workflows_id_enable_link_access_put) | **PUT** /api/workflows/{id}/enable_link_access | Makes this item accessible by a URL link.
[**enable_link_access_api_workflows_id_enable_link_access_put_0**](WorkflowsApi.md#enable_link_access_api_workflows_id_enable_link_access_put_0) | **PUT** /api/workflows/{id}/enable_link_access | Makes this item accessible by a URL link.
[**index_api_workflows_get**](WorkflowsApi.md#index_api_workflows_get) | **GET** /api/workflows | Lists stored workflows viewable by the user.
[**index_api_workflows_get_0**](WorkflowsApi.md#index_api_workflows_get_0) | **GET** /api/workflows | Lists stored workflows viewable by the user.
[**prepare_store_download_api_invocations_invocation_id_prepare_store_download_post**](WorkflowsApi.md#prepare_store_download_api_invocations_invocation_id_prepare_store_download_post) | **POST** /api/invocations/{invocation_id}/prepare_store_download | Prepare a worklfow invocation export-style download.
[**prepare_store_download_api_invocations_invocation_id_prepare_store_download_post_0**](WorkflowsApi.md#prepare_store_download_api_invocations_invocation_id_prepare_store_download_post_0) | **POST** /api/invocations/{invocation_id}/prepare_store_download | Prepare a worklfow invocation export-style download.
[**publish_api_workflows_id_publish_put**](WorkflowsApi.md#publish_api_workflows_id_publish_put) | **PUT** /api/workflows/{id}/publish | Makes this item public and accessible by a URL link.
[**publish_api_workflows_id_publish_put_0**](WorkflowsApi.md#publish_api_workflows_id_publish_put_0) | **PUT** /api/workflows/{id}/publish | Makes this item public and accessible by a URL link.
[**set_slug_api_workflows_id_slug_put**](WorkflowsApi.md#set_slug_api_workflows_id_slug_put) | **PUT** /api/workflows/{id}/slug | Set a new slug for this shared item.
[**set_slug_api_workflows_id_slug_put_0**](WorkflowsApi.md#set_slug_api_workflows_id_slug_put_0) | **PUT** /api/workflows/{id}/slug | Set a new slug for this shared item.
[**share_with_users_api_workflows_id_share_with_users_put**](WorkflowsApi.md#share_with_users_api_workflows_id_share_with_users_put) | **PUT** /api/workflows/{id}/share_with_users | Share this item with specific users.
[**share_with_users_api_workflows_id_share_with_users_put_0**](WorkflowsApi.md#share_with_users_api_workflows_id_share_with_users_put_0) | **PUT** /api/workflows/{id}/share_with_users | Share this item with specific users.
[**sharing_api_workflows_id_sharing_get**](WorkflowsApi.md#sharing_api_workflows_id_sharing_get) | **GET** /api/workflows/{id}/sharing | Get the current sharing status of the given item.
[**sharing_api_workflows_id_sharing_get_0**](WorkflowsApi.md#sharing_api_workflows_id_sharing_get_0) | **GET** /api/workflows/{id}/sharing | Get the current sharing status of the given item.
[**unpublish_api_workflows_id_unpublish_put**](WorkflowsApi.md#unpublish_api_workflows_id_unpublish_put) | **PUT** /api/workflows/{id}/unpublish | Removes this item from the published list.
[**unpublish_api_workflows_id_unpublish_put_0**](WorkflowsApi.md#unpublish_api_workflows_id_unpublish_put_0) | **PUT** /api/workflows/{id}/unpublish | Removes this item from the published list.
[**write_store_api_invocations_invocation_id_write_store_post**](WorkflowsApi.md#write_store_api_invocations_invocation_id_write_store_post) | **POST** /api/invocations/{invocation_id}/write_store | Prepare a worklfow invocation export-style download and write to supplied URI.
[**write_store_api_invocations_invocation_id_write_store_post_0**](WorkflowsApi.md#write_store_api_invocations_invocation_id_write_store_post_0) | **POST** /api/invocations/{invocation_id}/write_store | Prepare a worklfow invocation export-style download and write to supplied URI.



## disable_link_access_api_workflows_id_disable_link_access_put

> crate::models::SharingStatus disable_link_access_api_workflows_id_disable_link_access_put(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_workflows_id_disable_link_access_put_0

> crate::models::SharingStatus disable_link_access_api_workflows_id_disable_link_access_put_0(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_workflows_id_enable_link_access_put

> crate::models::SharingStatus enable_link_access_api_workflows_id_enable_link_access_put(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_workflows_id_enable_link_access_put_0

> crate::models::SharingStatus enable_link_access_api_workflows_id_enable_link_access_put_0(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_workflows_get

> Vec<serde_json::Value> index_api_workflows_get(show_deleted, show_hidden, missing_tools, show_published, show_shared, sort_by, sort_desc, limit, offset, search, skip_step_counts, run_as)
Lists stored workflows viewable by the user.

Lists stored workflows viewable by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_deleted** | Option<**bool**> | Whether to restrict result to deleted workflows. |  |[default to false]
**show_hidden** | Option<**bool**> | Whether to restrict result to hidden workflows. |  |[default to false]
**missing_tools** | Option<**bool**> | Whether to include a list of missing tools per workflow entry |  |[default to false]
**show_published** | Option<**bool**> |  |  |
**show_shared** | Option<**bool**> |  |  |
**sort_by** | Option<[**crate::models::WorkflowSortByEnum**](.md)> | In unspecified, default ordering depends on other parameters but generally the user's own workflows appear first based on update time |  |
**sort_desc** | Option<**bool**> | Sort in descending order? |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |[default to 0]
**search** | Option<**String**> | A mix of free text and GitHub-style tags used to filter the index operation.  ## Query Structure  GitHub-style filter tags (not be confused with Galaxy tags) are tags of the form `<tag_name>:<text_no_spaces>` or `<tag_name>:'<text with potential spaces>'`. The tag name *generally* (but not exclusively) corresponds to the name of an attribute on the model being indexed (i.e. a column in the database).  If the tag is quoted, the attribute will be filtered exactly. If the tag is unquoted, generally a partial match will be used to filter the query (i.e. in terms of the implementation this means the database operation `ILIKE` will typically be used).  Once the tagged filters are extracted from the search query, the remaing text is just used to search various documented attributes of the object.  ## GitHub-style Tags Available  `name` : The stored workflow's name. (The tag `n` can be used a short hand alias for this tag to filter on this attribute.)  `tag` : The workflow's tag, if the tag contains a colon an approach will be made to match the key and value of the tag separately. (The tag `t` can be used a short hand alias for this tag to filter on this attribute.)  `user` : The stored workflow's owner's username. (The tag `u` can be used a short hand alias for this tag to filter on this attribute.)  `is:published` : Include only published workflows in the final result. Be sure the the query parameter `show_published` is set to `true` if to include all published workflows and not just the requesting user's.  `is:share_with_me` : Include only workflows shared with the requesting user.  Be sure the the query parameter `show_shared` is set to `true` if to include shared workflows.  ## Free Text  Free text search terms will be searched against the following attributes of the Stored Workflows: `name`, `tag`, `user`.   |  |
**skip_step_counts** | Option<**bool**> | Set this to true to skip joining workflow step counts and optimize the resulting index query. Response objects will not contain step counts. |  |[default to false]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_workflows_get_0

> Vec<serde_json::Value> index_api_workflows_get_0(show_deleted, show_hidden, missing_tools, show_published, show_shared, sort_by, sort_desc, limit, offset, search, skip_step_counts, run_as)
Lists stored workflows viewable by the user.

Lists stored workflows viewable by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_deleted** | Option<**bool**> | Whether to restrict result to deleted workflows. |  |[default to false]
**show_hidden** | Option<**bool**> | Whether to restrict result to hidden workflows. |  |[default to false]
**missing_tools** | Option<**bool**> | Whether to include a list of missing tools per workflow entry |  |[default to false]
**show_published** | Option<**bool**> |  |  |
**show_shared** | Option<**bool**> |  |  |
**sort_by** | Option<[**crate::models::WorkflowSortByEnum**](.md)> | In unspecified, default ordering depends on other parameters but generally the user's own workflows appear first based on update time |  |
**sort_desc** | Option<**bool**> | Sort in descending order? |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |[default to 0]
**search** | Option<**String**> | A mix of free text and GitHub-style tags used to filter the index operation.  ## Query Structure  GitHub-style filter tags (not be confused with Galaxy tags) are tags of the form `<tag_name>:<text_no_spaces>` or `<tag_name>:'<text with potential spaces>'`. The tag name *generally* (but not exclusively) corresponds to the name of an attribute on the model being indexed (i.e. a column in the database).  If the tag is quoted, the attribute will be filtered exactly. If the tag is unquoted, generally a partial match will be used to filter the query (i.e. in terms of the implementation this means the database operation `ILIKE` will typically be used).  Once the tagged filters are extracted from the search query, the remaing text is just used to search various documented attributes of the object.  ## GitHub-style Tags Available  `name` : The stored workflow's name. (The tag `n` can be used a short hand alias for this tag to filter on this attribute.)  `tag` : The workflow's tag, if the tag contains a colon an approach will be made to match the key and value of the tag separately. (The tag `t` can be used a short hand alias for this tag to filter on this attribute.)  `user` : The stored workflow's owner's username. (The tag `u` can be used a short hand alias for this tag to filter on this attribute.)  `is:published` : Include only published workflows in the final result. Be sure the the query parameter `show_published` is set to `true` if to include all published workflows and not just the requesting user's.  `is:share_with_me` : Include only workflows shared with the requesting user.  Be sure the the query parameter `show_shared` is set to `true` if to include shared workflows.  ## Free Text  Free text search terms will be searched against the following attributes of the Stored Workflows: `name`, `tag`, `user`.   |  |
**skip_step_counts** | Option<**bool**> | Set this to true to skip joining workflow step counts and optimize the resulting index query. Response objects will not contain step counts. |  |[default to false]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_invocations_invocation_id_prepare_store_download_post

> crate::models::AsyncFile prepare_store_download_api_invocations_invocation_id_prepare_store_download_post(invocation_id, prepare_store_download_payload, run_as)
Prepare a worklfow invocation export-style download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invocation_id** | **String** | The encoded database identifier of the Invocation. | [required] |
**prepare_store_download_payload** | [**PrepareStoreDownloadPayload**](PrepareStoreDownloadPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_invocations_invocation_id_prepare_store_download_post_0

> crate::models::AsyncFile prepare_store_download_api_invocations_invocation_id_prepare_store_download_post_0(invocation_id, prepare_store_download_payload, run_as)
Prepare a worklfow invocation export-style download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invocation_id** | **String** | The encoded database identifier of the Invocation. | [required] |
**prepare_store_download_payload** | [**PrepareStoreDownloadPayload**](PrepareStoreDownloadPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_workflows_id_publish_put

> crate::models::SharingStatus publish_api_workflows_id_publish_put(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_workflows_id_publish_put_0

> crate::models::SharingStatus publish_api_workflows_id_publish_put_0(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_workflows_id_slug_put

> set_slug_api_workflows_id_slug_put(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
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


## set_slug_api_workflows_id_slug_put_0

> set_slug_api_workflows_id_slug_put_0(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
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


## share_with_users_api_workflows_id_share_with_users_put

> crate::models::ShareWithStatus share_with_users_api_workflows_id_share_with_users_put(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
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


## share_with_users_api_workflows_id_share_with_users_put_0

> crate::models::ShareWithStatus share_with_users_api_workflows_id_share_with_users_put_0(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
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


## sharing_api_workflows_id_sharing_get

> crate::models::SharingStatus sharing_api_workflows_id_sharing_get(id, run_as)
Get the current sharing status of the given item.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_workflows_id_sharing_get_0

> crate::models::SharingStatus sharing_api_workflows_id_sharing_get_0(id, run_as)
Get the current sharing status of the given item.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_workflows_id_unpublish_put

> crate::models::SharingStatus unpublish_api_workflows_id_unpublish_put(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_workflows_id_unpublish_put_0

> crate::models::SharingStatus unpublish_api_workflows_id_unpublish_put_0(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the Stored Workflow. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_invocations_invocation_id_write_store_post

> crate::models::AsyncTaskResultSummary write_store_api_invocations_invocation_id_write_store_post(invocation_id, write_store_to_payload, run_as)
Prepare a worklfow invocation export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invocation_id** | **String** | The encoded database identifier of the Invocation. | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_invocations_invocation_id_write_store_post_0

> crate::models::AsyncTaskResultSummary write_store_api_invocations_invocation_id_write_store_post_0(invocation_id, write_store_to_payload, run_as)
Prepare a worklfow invocation export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invocation_id** | **String** | The encoded database identifier of the Invocation. | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

