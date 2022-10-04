# \LibrariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_libraries_post**](LibrariesApi.md#create_api_libraries_post) | **POST** /api/libraries | Creates a new library and returns its summary information.
[**create_api_libraries_post_0**](LibrariesApi.md#create_api_libraries_post_0) | **POST** /api/libraries | Creates a new library and returns its summary information.
[**create_from_store_api_libraries_from_store_post**](LibrariesApi.md#create_from_store_api_libraries_from_store_post) | **POST** /api/libraries/from_store | Create libraries from a model store.
[**create_from_store_api_libraries_from_store_post_0**](LibrariesApi.md#create_from_store_api_libraries_from_store_post_0) | **POST** /api/libraries/from_store | Create libraries from a model store.
[**delete_api_libraries_id_delete**](LibrariesApi.md#delete_api_libraries_id_delete) | **DELETE** /api/libraries/{id} | Marks the specified library as deleted (or undeleted).
[**delete_api_libraries_id_delete_0**](LibrariesApi.md#delete_api_libraries_id_delete_0) | **DELETE** /api/libraries/{id} | Marks the specified library as deleted (or undeleted).
[**get_permissions_api_libraries_id_permissions_get**](LibrariesApi.md#get_permissions_api_libraries_id_permissions_get) | **GET** /api/libraries/{id}/permissions | Gets the current or available permissions of a particular library.
[**get_permissions_api_libraries_id_permissions_get_0**](LibrariesApi.md#get_permissions_api_libraries_id_permissions_get_0) | **GET** /api/libraries/{id}/permissions | Gets the current or available permissions of a particular library.
[**index_api_libraries_get**](LibrariesApi.md#index_api_libraries_get) | **GET** /api/libraries | Returns a list of summary data for all libraries.
[**index_api_libraries_get_0**](LibrariesApi.md#index_api_libraries_get_0) | **GET** /api/libraries | Returns a list of summary data for all libraries.
[**index_deleted_api_libraries_deleted_get**](LibrariesApi.md#index_deleted_api_libraries_deleted_get) | **GET** /api/libraries/deleted | Returns a list of summary data for all libraries marked as deleted.
[**index_deleted_api_libraries_deleted_get_0**](LibrariesApi.md#index_deleted_api_libraries_deleted_get_0) | **GET** /api/libraries/deleted | Returns a list of summary data for all libraries marked as deleted.
[**set_permissions_api_libraries_id_permissions_post**](LibrariesApi.md#set_permissions_api_libraries_id_permissions_post) | **POST** /api/libraries/{id}/permissions | Sets the permissions to access and manipulate a library.
[**set_permissions_api_libraries_id_permissions_post_0**](LibrariesApi.md#set_permissions_api_libraries_id_permissions_post_0) | **POST** /api/libraries/{id}/permissions | Sets the permissions to access and manipulate a library.
[**show_api_libraries_id_get**](LibrariesApi.md#show_api_libraries_id_get) | **GET** /api/libraries/{id} | Returns summary information about a particular library.
[**show_api_libraries_id_get_0**](LibrariesApi.md#show_api_libraries_id_get_0) | **GET** /api/libraries/{id} | Returns summary information about a particular library.
[**update_api_libraries_id_patch**](LibrariesApi.md#update_api_libraries_id_patch) | **PATCH** /api/libraries/{id} | Updates the information of an existing library.
[**update_api_libraries_id_patch_0**](LibrariesApi.md#update_api_libraries_id_patch_0) | **PATCH** /api/libraries/{id} | Updates the information of an existing library.



## create_api_libraries_post

> crate::models::LibrarySummary create_api_libraries_post(create_library_payload, run_as)
Creates a new library and returns its summary information.

Creates a new library and returns its summary information. Currently, only admin users can create libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_library_payload** | [**CreateLibraryPayload**](CreateLibraryPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_libraries_post_0

> crate::models::LibrarySummary create_api_libraries_post_0(create_library_payload, run_as)
Creates a new library and returns its summary information.

Creates a new library and returns its summary information. Currently, only admin users can create libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_library_payload** | [**CreateLibraryPayload**](CreateLibraryPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_libraries_from_store_post

> Vec<crate::models::LibrarySummary> create_from_store_api_libraries_from_store_post(create_libraries_from_store, run_as)
Create libraries from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_libraries_from_store** | [**CreateLibrariesFromStore**](CreateLibrariesFromStore.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_libraries_from_store_post_0

> Vec<crate::models::LibrarySummary> create_from_store_api_libraries_from_store_post_0(create_libraries_from_store, run_as)
Create libraries from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_libraries_from_store** | [**CreateLibrariesFromStore**](CreateLibrariesFromStore.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_libraries_id_delete

> crate::models::LibrarySummary delete_api_libraries_id_delete(id, undelete, run_as, delete_library_payload)
Marks the specified library as deleted (or undeleted).

Marks the specified library as deleted (or undeleted). Currently, only admin users can delete or restore libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**undelete** | Option<**bool**> | Whether to restore a deleted library. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_library_payload** | Option<[**DeleteLibraryPayload**](DeleteLibraryPayload.md)> |  |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_libraries_id_delete_0

> crate::models::LibrarySummary delete_api_libraries_id_delete_0(id, undelete, run_as, delete_library_payload)
Marks the specified library as deleted (or undeleted).

Marks the specified library as deleted (or undeleted). Currently, only admin users can delete or restore libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**undelete** | Option<**bool**> | Whether to restore a deleted library. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_library_payload** | Option<[**DeleteLibraryPayload**](DeleteLibraryPayload.md)> |  |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions_api_libraries_id_permissions_get

> crate::models::ResponseGetPermissionsApiLibrariesIdPermissionsGet get_permissions_api_libraries_id_permissions_get(id, scope, is_library_access, page, page_limit, q, run_as)
Gets the current or available permissions of a particular library.

Gets the current or available permissions of a particular library. The results can be paginated and additionally filtered by a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**scope** | Option<[**crate::models::LibraryPermissionScope**](.md)> | The scope of the permissions to retrieve. Either the `current` permissions or the `available`. |  |
**is_library_access** | Option<**bool**> | Indicates whether the roles available for the library access are requested. |  |
**page** | Option<**i32**> | The page number to retrieve when paginating the available roles. |  |[default to 1]
**page_limit** | Option<**i32**> | The maximum number of permissions per page when paginating. |  |[default to 10]
**q** | Option<**String**> | Optional search text to retrieve only the roles matching this query. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseGetPermissionsApiLibrariesIdPermissionsGet**](Response_Get_Permissions_Api_Libraries__Id__Permissions_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions_api_libraries_id_permissions_get_0

> crate::models::ResponseGetPermissionsApiLibrariesIdPermissionsGet get_permissions_api_libraries_id_permissions_get_0(id, scope, is_library_access, page, page_limit, q, run_as)
Gets the current or available permissions of a particular library.

Gets the current or available permissions of a particular library. The results can be paginated and additionally filtered by a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**scope** | Option<[**crate::models::LibraryPermissionScope**](.md)> | The scope of the permissions to retrieve. Either the `current` permissions or the `available`. |  |
**is_library_access** | Option<**bool**> | Indicates whether the roles available for the library access are requested. |  |
**page** | Option<**i32**> | The page number to retrieve when paginating the available roles. |  |[default to 1]
**page_limit** | Option<**i32**> | The maximum number of permissions per page when paginating. |  |[default to 10]
**q** | Option<**String**> | Optional search text to retrieve only the roles matching this query. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseGetPermissionsApiLibrariesIdPermissionsGet**](Response_Get_Permissions_Api_Libraries__Id__Permissions_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_libraries_get

> Vec<crate::models::LibrarySummary> index_api_libraries_get(deleted, run_as)
Returns a list of summary data for all libraries.

Returns a list of summary data for all libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Whether to include deleted libraries in the result. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_libraries_get_0

> Vec<crate::models::LibrarySummary> index_api_libraries_get_0(deleted, run_as)
Returns a list of summary data for all libraries.

Returns a list of summary data for all libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Whether to include deleted libraries in the result. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_libraries_deleted_get

> Vec<crate::models::LibrarySummary> index_deleted_api_libraries_deleted_get(run_as)
Returns a list of summary data for all libraries marked as deleted.

Returns a list of summary data for all libraries marked as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_libraries_deleted_get_0

> Vec<crate::models::LibrarySummary> index_deleted_api_libraries_deleted_get_0(run_as)
Returns a list of summary data for all libraries marked as deleted.

Returns a list of summary data for all libraries marked as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::LibrarySummary>**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_permissions_api_libraries_id_permissions_post

> crate::models::ResponseSetPermissionsApiLibrariesIdPermissionsPost set_permissions_api_libraries_id_permissions_post(id, payload, action, run_as)
Sets the permissions to access and manipulate a library.

Sets the permissions to access and manipulate a library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**payload** | [**Payload**](Payload.md) |  | [required] |
**action** | Option<[**crate::models::LibraryPermissionAction**](.md)> | Indicates what action should be performed on the Library. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseSetPermissionsApiLibrariesIdPermissionsPost**](Response_Set_Permissions_Api_Libraries__Id__Permissions_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_permissions_api_libraries_id_permissions_post_0

> crate::models::ResponseSetPermissionsApiLibrariesIdPermissionsPost set_permissions_api_libraries_id_permissions_post_0(id, payload, action, run_as)
Sets the permissions to access and manipulate a library.

Sets the permissions to access and manipulate a library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**payload** | [**Payload**](Payload.md) |  | [required] |
**action** | Option<[**crate::models::LibraryPermissionAction**](.md)> | Indicates what action should be performed on the Library. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseSetPermissionsApiLibrariesIdPermissionsPost**](Response_Set_Permissions_Api_Libraries__Id__Permissions_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_libraries_id_get

> crate::models::LibrarySummary show_api_libraries_id_get(id, run_as)
Returns summary information about a particular library.

Returns summary information about a particular library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_libraries_id_get_0

> crate::models::LibrarySummary show_api_libraries_id_get_0(id, run_as)
Returns summary information about a particular library.

Returns summary information about a particular library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_libraries_id_patch

> crate::models::LibrarySummary update_api_libraries_id_patch(id, update_library_payload, run_as)
Updates the information of an existing library.

Updates the information of an existing library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**update_library_payload** | [**UpdateLibraryPayload**](UpdateLibraryPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_libraries_id_patch_0

> crate::models::LibrarySummary update_api_libraries_id_patch_0(id, update_library_payload, run_as)
Updates the information of an existing library.

Updates the information of an existing library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the Library. | [required] |
**update_library_payload** | [**UpdateLibraryPayload**](UpdateLibraryPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibrarySummary**](LibrarySummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

