# \FoldersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_folders_id_post**](FoldersApi.md#create_api_folders_id_post) | **POST** /api/folders/{id} | Create a new library folder underneath the one specified by the ID.
[**create_api_folders_id_post_0**](FoldersApi.md#create_api_folders_id_post_0) | **POST** /api/folders/{id} | Create a new library folder underneath the one specified by the ID.
[**delete_api_folders_id_delete**](FoldersApi.md#delete_api_folders_id_delete) | **DELETE** /api/folders/{id} | Marks the specified library folder as deleted (or undeleted).
[**delete_api_folders_id_delete_0**](FoldersApi.md#delete_api_folders_id_delete_0) | **DELETE** /api/folders/{id} | Marks the specified library folder as deleted (or undeleted).
[**get_permissions_api_folders_id_permissions_get**](FoldersApi.md#get_permissions_api_folders_id_permissions_get) | **GET** /api/folders/{id}/permissions | Gets the current or available permissions of a particular library folder.
[**get_permissions_api_folders_id_permissions_get_0**](FoldersApi.md#get_permissions_api_folders_id_permissions_get_0) | **GET** /api/folders/{id}/permissions | Gets the current or available permissions of a particular library folder.
[**set_permissions_api_folders_id_permissions_post**](FoldersApi.md#set_permissions_api_folders_id_permissions_post) | **POST** /api/folders/{id}/permissions | Sets the permissions to manage a library folder.
[**set_permissions_api_folders_id_permissions_post_0**](FoldersApi.md#set_permissions_api_folders_id_permissions_post_0) | **POST** /api/folders/{id}/permissions | Sets the permissions to manage a library folder.
[**show_api_folders_id_get**](FoldersApi.md#show_api_folders_id_get) | **GET** /api/folders/{id} | Displays information about a particular library folder.
[**show_api_folders_id_get_0**](FoldersApi.md#show_api_folders_id_get_0) | **GET** /api/folders/{id} | Displays information about a particular library folder.
[**update_api_folders_id_patch**](FoldersApi.md#update_api_folders_id_patch) | **PATCH** /api/folders/{id} | Update
[**update_api_folders_id_patch_0**](FoldersApi.md#update_api_folders_id_patch_0) | **PATCH** /api/folders/{id} | Update
[**update_api_folders_id_put**](FoldersApi.md#update_api_folders_id_put) | **PUT** /api/folders/{id} | Updates the information of an existing library folder.
[**update_api_folders_id_put_0**](FoldersApi.md#update_api_folders_id_put_0) | **PUT** /api/folders/{id} | Updates the information of an existing library folder.



## create_api_folders_id_post

> crate::models::LibraryFolderDetails create_api_folders_id_post(id, create_library_folder_payload, run_as)
Create a new library folder underneath the one specified by the ID.

Returns detailed information about the newly created library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**create_library_folder_payload** | [**CreateLibraryFolderPayload**](CreateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_folders_id_post_0

> crate::models::LibraryFolderDetails create_api_folders_id_post_0(id, create_library_folder_payload, run_as)
Create a new library folder underneath the one specified by the ID.

Returns detailed information about the newly created library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**create_library_folder_payload** | [**CreateLibraryFolderPayload**](CreateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_folders_id_delete

> crate::models::LibraryFolderDetails delete_api_folders_id_delete(id, undelete, run_as)
Marks the specified library folder as deleted (or undeleted).

Marks the specified library folder as deleted (or undeleted).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**undelete** | Option<**bool**> | Whether to restore a deleted library folder. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_folders_id_delete_0

> crate::models::LibraryFolderDetails delete_api_folders_id_delete_0(id, undelete, run_as)
Marks the specified library folder as deleted (or undeleted).

Marks the specified library folder as deleted (or undeleted).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**undelete** | Option<**bool**> | Whether to restore a deleted library folder. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions_api_folders_id_permissions_get

> crate::models::ResponseGetPermissionsApiFoldersIdPermissionsGet get_permissions_api_folders_id_permissions_get(id, scope, page, page_limit, q, run_as)
Gets the current or available permissions of a particular library folder.

Gets the current or available permissions of a particular library. The results can be paginated and additionally filtered by a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**scope** | Option<[**crate::models::LibraryPermissionScope**](.md)> | The scope of the permissions to retrieve. Either the `current` permissions or the `available`. |  |
**page** | Option<**i32**> | The page number to retrieve when paginating the available roles. |  |[default to 1]
**page_limit** | Option<**i32**> | The maximum number of permissions per page when paginating. |  |[default to 10]
**q** | Option<**String**> | Optional search text to retrieve only the roles matching this query. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseGetPermissionsApiFoldersIdPermissionsGet**](Response_Get_Permissions_Api_Folders__Id__Permissions_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions_api_folders_id_permissions_get_0

> crate::models::ResponseGetPermissionsApiFoldersIdPermissionsGet get_permissions_api_folders_id_permissions_get_0(id, scope, page, page_limit, q, run_as)
Gets the current or available permissions of a particular library folder.

Gets the current or available permissions of a particular library. The results can be paginated and additionally filtered by a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**scope** | Option<[**crate::models::LibraryPermissionScope**](.md)> | The scope of the permissions to retrieve. Either the `current` permissions or the `available`. |  |
**page** | Option<**i32**> | The page number to retrieve when paginating the available roles. |  |[default to 1]
**page_limit** | Option<**i32**> | The maximum number of permissions per page when paginating. |  |[default to 10]
**q** | Option<**String**> | Optional search text to retrieve only the roles matching this query. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseGetPermissionsApiFoldersIdPermissionsGet**](Response_Get_Permissions_Api_Folders__Id__Permissions_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_permissions_api_folders_id_permissions_post

> crate::models::LibraryFolderCurrentPermissions set_permissions_api_folders_id_permissions_post(id, library_folder_permissions_payload, action, run_as)
Sets the permissions to manage a library folder.

Sets the permissions to manage a library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**library_folder_permissions_payload** | [**LibraryFolderPermissionsPayload**](LibraryFolderPermissionsPayload.md) |  | [required] |
**action** | Option<[**crate::models::LibraryFolderPermissionAction**](.md)> | Indicates what action should be performed on the Library. Currently only `set_permissions` is supported. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderCurrentPermissions**](LibraryFolderCurrentPermissions.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_permissions_api_folders_id_permissions_post_0

> crate::models::LibraryFolderCurrentPermissions set_permissions_api_folders_id_permissions_post_0(id, library_folder_permissions_payload, action, run_as)
Sets the permissions to manage a library folder.

Sets the permissions to manage a library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**library_folder_permissions_payload** | [**LibraryFolderPermissionsPayload**](LibraryFolderPermissionsPayload.md) |  | [required] |
**action** | Option<[**crate::models::LibraryFolderPermissionAction**](.md)> | Indicates what action should be performed on the Library. Currently only `set_permissions` is supported. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderCurrentPermissions**](LibraryFolderCurrentPermissions.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_folders_id_get

> crate::models::LibraryFolderDetails show_api_folders_id_get(id, run_as)
Displays information about a particular library folder.

Returns detailed information about the library folder with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_folders_id_get_0

> crate::models::LibraryFolderDetails show_api_folders_id_get_0(id, run_as)
Displays information about a particular library folder.

Returns detailed information about the library folder with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_folders_id_patch

> crate::models::LibraryFolderDetails update_api_folders_id_patch(id, update_library_folder_payload, run_as)
Update

Updates the information of an existing library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**update_library_folder_payload** | [**UpdateLibraryFolderPayload**](UpdateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_folders_id_patch_0

> crate::models::LibraryFolderDetails update_api_folders_id_patch_0(id, update_library_folder_payload, run_as)
Update

Updates the information of an existing library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**update_library_folder_payload** | [**UpdateLibraryFolderPayload**](UpdateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_folders_id_put

> crate::models::LibraryFolderDetails update_api_folders_id_put(id, update_library_folder_payload, run_as)
Updates the information of an existing library folder.

Updates the information of an existing library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**update_library_folder_payload** | [**UpdateLibraryFolderPayload**](UpdateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_folders_id_put_0

> crate::models::LibraryFolderDetails update_api_folders_id_put_0(id, update_library_folder_payload, run_as)
Updates the information of an existing library folder.

Updates the information of an existing library folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded identifier of the library folder. | [required] |
**update_library_folder_payload** | [**UpdateLibraryFolderPayload**](UpdateLibraryFolderPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::LibraryFolderDetails**](LibraryFolderDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

