# \GroupUsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_groups_group_id_user_user_id_delete**](GroupUsersApi.md#delete_api_groups_group_id_user_user_id_delete) | **DELETE** /api/groups/{group_id}/user/{user_id} | Removes a user from a group
[**delete_api_groups_group_id_user_user_id_delete_0**](GroupUsersApi.md#delete_api_groups_group_id_user_user_id_delete_0) | **DELETE** /api/groups/{group_id}/user/{user_id} | Removes a user from a group
[**delete_api_groups_group_id_users_user_id_delete**](GroupUsersApi.md#delete_api_groups_group_id_users_user_id_delete) | **DELETE** /api/groups/{group_id}/users/{user_id} | Removes a user from a group
[**delete_api_groups_group_id_users_user_id_delete_0**](GroupUsersApi.md#delete_api_groups_group_id_users_user_id_delete_0) | **DELETE** /api/groups/{group_id}/users/{user_id} | Removes a user from a group
[**group_user_api_groups_group_id_user_id_get**](GroupUsersApi.md#group_user_api_groups_group_id_user_id_get) | **GET** /api/groups/{group_id}/user/{id} | Displays information about a group user.
[**group_user_api_groups_group_id_user_id_get_0**](GroupUsersApi.md#group_user_api_groups_group_id_user_id_get_0) | **GET** /api/groups/{group_id}/user/{id} | Displays information about a group user.
[**group_user_api_groups_group_id_users_id_get**](GroupUsersApi.md#group_user_api_groups_group_id_users_id_get) | **GET** /api/groups/{group_id}/users/{id} | Displays information about a group user.
[**group_user_api_groups_group_id_users_id_get_0**](GroupUsersApi.md#group_user_api_groups_group_id_users_id_get_0) | **GET** /api/groups/{group_id}/users/{id} | Displays information about a group user.
[**index_api_groups_group_id_users_get**](GroupUsersApi.md#index_api_groups_group_id_users_get) | **GET** /api/groups/{group_id}/users | Displays a collection (list) of groups.
[**index_api_groups_group_id_users_get_0**](GroupUsersApi.md#index_api_groups_group_id_users_get_0) | **GET** /api/groups/{group_id}/users | Displays a collection (list) of groups.
[**update_api_groups_group_id_user_user_id_put**](GroupUsersApi.md#update_api_groups_group_id_user_user_id_put) | **PUT** /api/groups/{group_id}/user/{user_id} | Adds a user to a group
[**update_api_groups_group_id_user_user_id_put_0**](GroupUsersApi.md#update_api_groups_group_id_user_user_id_put_0) | **PUT** /api/groups/{group_id}/user/{user_id} | Adds a user to a group
[**update_api_groups_group_id_users_user_id_put**](GroupUsersApi.md#update_api_groups_group_id_users_user_id_put) | **PUT** /api/groups/{group_id}/users/{user_id} | Adds a user to a group
[**update_api_groups_group_id_users_user_id_put_0**](GroupUsersApi.md#update_api_groups_group_id_users_user_id_put_0) | **PUT** /api/groups/{group_id}/users/{user_id} | Adds a user to a group



## delete_api_groups_group_id_user_user_id_delete

> crate::models::GroupUserModel delete_api_groups_group_id_user_user_id_delete(group_id, user_id, run_as)
Removes a user from a group

DELETE /api/groups/{encoded_group_id}/users/{encoded_user_id} Removes a user from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_groups_group_id_user_user_id_delete_0

> crate::models::GroupUserModel delete_api_groups_group_id_user_user_id_delete_0(group_id, user_id, run_as)
Removes a user from a group

DELETE /api/groups/{encoded_group_id}/users/{encoded_user_id} Removes a user from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_groups_group_id_users_user_id_delete

> crate::models::GroupUserModel delete_api_groups_group_id_users_user_id_delete(group_id, user_id, run_as)
Removes a user from a group

DELETE /api/groups/{encoded_group_id}/users/{encoded_user_id} Removes a user from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_groups_group_id_users_user_id_delete_0

> crate::models::GroupUserModel delete_api_groups_group_id_users_user_id_delete_0(group_id, user_id, run_as)
Removes a user from a group

DELETE /api/groups/{encoded_group_id}/users/{encoded_user_id} Removes a user from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_user_api_groups_group_id_user_id_get

> crate::models::GroupUserModel group_user_api_groups_group_id_user_id_get(group_id, id, run_as)
Displays information about a group user.

GET /api/groups/{encoded_group_id}/users/{encoded_user_id} Displays information about a group user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_user_api_groups_group_id_user_id_get_0

> crate::models::GroupUserModel group_user_api_groups_group_id_user_id_get_0(group_id, id, run_as)
Displays information about a group user.

GET /api/groups/{encoded_group_id}/users/{encoded_user_id} Displays information about a group user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_user_api_groups_group_id_users_id_get

> crate::models::GroupUserModel group_user_api_groups_group_id_users_id_get(group_id, id, run_as)
Displays information about a group user.

GET /api/groups/{encoded_group_id}/users/{encoded_user_id} Displays information about a group user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_user_api_groups_group_id_users_id_get_0

> crate::models::GroupUserModel group_user_api_groups_group_id_users_id_get_0(group_id, id, run_as)
Displays information about a group user.

GET /api/groups/{encoded_group_id}/users/{encoded_user_id} Displays information about a group user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_groups_group_id_users_get

> Vec<crate::models::GroupUserModel> index_api_groups_group_id_users_get(group_id, run_as)
Displays a collection (list) of groups.

GET /api/groups/{encoded_group_id}/users Displays a collection (list) of groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::GroupUserModel>**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_groups_group_id_users_get_0

> Vec<crate::models::GroupUserModel> index_api_groups_group_id_users_get_0(group_id, run_as)
Displays a collection (list) of groups.

GET /api/groups/{encoded_group_id}/users Displays a collection (list) of groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::GroupUserModel>**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_user_user_id_put

> crate::models::GroupUserModel update_api_groups_group_id_user_user_id_put(group_id, user_id, run_as)
Adds a user to a group

PUT /api/groups/{encoded_group_id}/users/{encoded_user_id} Adds a user to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_user_user_id_put_0

> crate::models::GroupUserModel update_api_groups_group_id_user_user_id_put_0(group_id, user_id, run_as)
Adds a user to a group

PUT /api/groups/{encoded_group_id}/users/{encoded_user_id} Adds a user to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_users_user_id_put

> crate::models::GroupUserModel update_api_groups_group_id_users_user_id_put(group_id, user_id, run_as)
Adds a user to a group

PUT /api/groups/{encoded_group_id}/users/{encoded_user_id} Adds a user to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_users_user_id_put_0

> crate::models::GroupUserModel update_api_groups_group_id_users_user_id_put_0(group_id, user_id, run_as)
Adds a user to a group

PUT /api/groups/{encoded_group_id}/users/{encoded_user_id} Adds a user to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**user_id** | **String** | The ID of the user | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupUserModel**](GroupUserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

