# \GroupRolesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_groups_group_id_roles_role_id_delete**](GroupRolesApi.md#delete_api_groups_group_id_roles_role_id_delete) | **DELETE** /api/groups/{group_id}/roles/{role_id} | Removes a role from a group
[**delete_api_groups_group_id_roles_role_id_delete_0**](GroupRolesApi.md#delete_api_groups_group_id_roles_role_id_delete_0) | **DELETE** /api/groups/{group_id}/roles/{role_id} | Removes a role from a group
[**group_role_api_groups_group_id_roles_id_get**](GroupRolesApi.md#group_role_api_groups_group_id_roles_id_get) | **GET** /api/groups/{group_id}/roles/{id} | Displays information about a group role.
[**group_role_api_groups_group_id_roles_id_get_0**](GroupRolesApi.md#group_role_api_groups_group_id_roles_id_get_0) | **GET** /api/groups/{group_id}/roles/{id} | Displays information about a group role.
[**index_api_groups_group_id_roles_get**](GroupRolesApi.md#index_api_groups_group_id_roles_get) | **GET** /api/groups/{group_id}/roles | Displays a collection (list) of groups.
[**index_api_groups_group_id_roles_get_0**](GroupRolesApi.md#index_api_groups_group_id_roles_get_0) | **GET** /api/groups/{group_id}/roles | Displays a collection (list) of groups.
[**update_api_groups_group_id_roles_role_id_put**](GroupRolesApi.md#update_api_groups_group_id_roles_role_id_put) | **PUT** /api/groups/{group_id}/roles/{role_id} | Adds a role to a group
[**update_api_groups_group_id_roles_role_id_put_0**](GroupRolesApi.md#update_api_groups_group_id_roles_role_id_put_0) | **PUT** /api/groups/{group_id}/roles/{role_id} | Adds a role to a group



## delete_api_groups_group_id_roles_role_id_delete

> crate::models::GroupRoleModel delete_api_groups_group_id_roles_role_id_delete(group_id, role_id, run_as)
Removes a role from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**role_id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_groups_group_id_roles_role_id_delete_0

> crate::models::GroupRoleModel delete_api_groups_group_id_roles_role_id_delete_0(group_id, role_id, run_as)
Removes a role from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**role_id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_role_api_groups_group_id_roles_id_get

> crate::models::GroupRoleModel group_role_api_groups_group_id_roles_id_get(group_id, id, run_as)
Displays information about a group role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_role_api_groups_group_id_roles_id_get_0

> crate::models::GroupRoleModel group_role_api_groups_group_id_roles_id_get_0(group_id, id, run_as)
Displays information about a group role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_groups_group_id_roles_get

> Vec<crate::models::GroupRoleModel> index_api_groups_group_id_roles_get(group_id, run_as)
Displays a collection (list) of groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::GroupRoleModel>**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_groups_group_id_roles_get_0

> Vec<crate::models::GroupRoleModel> index_api_groups_group_id_roles_get_0(group_id, run_as)
Displays a collection (list) of groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::GroupRoleModel>**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_roles_role_id_put

> crate::models::GroupRoleModel update_api_groups_group_id_roles_role_id_put(group_id, role_id, run_as)
Adds a role to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**role_id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_groups_group_id_roles_role_id_put_0

> crate::models::GroupRoleModel update_api_groups_group_id_roles_role_id_put_0(group_id, role_id, run_as)
Adds a role to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group | [required] |
**role_id** | **String** | The ID of the role | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::GroupRoleModel**](GroupRoleModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

